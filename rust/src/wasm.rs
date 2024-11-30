use base64::{prelude::BASE64_STANDARD, Engine as Base64Engine};
use nova_aadhaar_qr::{
    circuit::{AadhaarAgeProofCircuit, OP_CODE_LAST},
    qr::{parse_aadhaar_qr_data, DOB_LENGTH_BYTES},
};
use nova_snark::{
    provider::{PallasEngine, VestaEngine},
    traits::{circuit::TrivialCircuit, snark::RelaxedR1CSSNARKTrait, Engine},
    CompressedSNARK, PublicParams, RecursiveSNARK,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::js_sys::Uint8Array;
use web_time::Instant;

use crate::{console_log, utils::set_panic_hook};

type E1 = PallasEngine;
type E2 = VestaEngine;
type EE1 = nova_snark::provider::ipa_pc::EvaluationEngine<E1>;
type EE2 = nova_snark::provider::ipa_pc::EvaluationEngine<E2>;
type S1 = nova_snark::spartan::snark::RelaxedR1CSSNARK<E1, EE1>;
type S2 = nova_snark::spartan::snark::RelaxedR1CSSNARK<E2, EE2>;
type C1 = AadhaarAgeProofCircuit<<E1 as Engine>::Scalar>;
type C2 = TrivialCircuit<<E2 as Engine>::Scalar>;

#[wasm_bindgen]
pub async fn generate_public_parameters() -> Uint8Array {
    set_panic_hook();
    let circuit_primary: C1 = AadhaarAgeProofCircuit::default();
    let circuit_secondary: C2 = TrivialCircuit::default();

    let param_gen_timer = Instant::now();
    console_log!("Producing public parameters...");
    let pp = PublicParams::<E1, E2, C1, C2>::setup(
        &circuit_primary,
        &circuit_secondary,
        &*S1::ck_floor(),
        &*S2::ck_floor(),
    )
    .unwrap();
    console_log!("PublicParams::setup, took {:?} ", param_gen_timer.elapsed());

    let serialized_pp = bincode::serialize(&pp).unwrap();
    return Uint8Array::from(serialized_pp.as_slice());
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AadhaarAgeProof {
    pub version: u32,
    pub pp_hash: String,
    pub num_steps: usize,
    pub current_date_ddmmyyyy: String, // Current date in DD-MM-YYYY format
    pub snark_proof: String,
}

#[wasm_bindgen]
pub async fn generate_proof(
    pp_bytes: Uint8Array,
    qr_data_bytes: Uint8Array,
    current_date_bytes: &[u8],
    _demo: bool,
) -> JsValue {
    assert_eq!(current_date_bytes.len(), DOB_LENGTH_BYTES);

    let timer = Instant::now();
    console_log!("Started deserialization of public parameters");
    let pp_bytes_vec = pp_bytes.to_vec();
    let pp: PublicParams<E1, E2, C1, C2> = bincode::deserialize(&pp_bytes_vec[..]).unwrap();
    console_log!("Public parameters deserialized in {:?}", timer.elapsed());

    console_log!(
        "Number of constraints per step (primary circuit): {}",
        pp.num_constraints().0
    );
    console_log!(
        "Number of constraints per step (secondary circuit): {}",
        pp.num_constraints().1
    );

    console_log!(
        "Number of variables per step (primary circuit): {}",
        pp.num_variables().0
    );
    console_log!(
        "Number of variables per step (secondary circuit): {}",
        pp.num_variables().1
    );

    let aadhaar_qr_data = parse_aadhaar_qr_data(qr_data_bytes.to_vec()).unwrap();
    let primary_circuit_sequence = C1::new_state_sequence(&aadhaar_qr_data);

    let z0_primary = C1::calc_initial_primary_circuit_input(current_date_bytes);
    let z0_secondary = vec![<E2 as Engine>::Scalar::zero()];

    let circuit_secondary: C2 = TrivialCircuit::default();

    let proof_gen_timer = Instant::now();
    // produce a recursive SNARK
    console_log!("Generating a RecursiveSNARK...");
    let mut recursive_snark: RecursiveSNARK<E1, E2, C1, C2> =
        RecursiveSNARK::<E1, E2, C1, C2>::new(
            &pp,
            &primary_circuit_sequence[0],
            &circuit_secondary,
            &z0_primary,
            &z0_secondary,
        )
        .unwrap();

    let start = Instant::now();
    for (i, circuit_primary) in primary_circuit_sequence.iter().enumerate() {
        let step_start = Instant::now();
        let res = recursive_snark.prove_step(&pp, circuit_primary, &circuit_secondary);
        assert!(res.is_ok());
        console_log!(
            "RecursiveSNARK::prove_step {}: {:?}, took {:?}",
            i,
            res.is_ok(),
            step_start.elapsed()
        );
    }
    console_log!(
        "Total time taken by RecursiveSNARK::prove_steps: {:?}",
        start.elapsed()
    );

    // verify the recursive SNARK
    console_log!("Verifying a RecursiveSNARK...");
    let start = Instant::now();
    let num_steps = primary_circuit_sequence.len();
    let res = recursive_snark.verify(&pp, num_steps, &z0_primary, &z0_secondary);
    console_log!(
        "RecursiveSNARK::verify: {:?}, took {:?}",
        res.is_ok(),
        start.elapsed()
    );
    // assert!(res.is_ok());

    // produce a compressed SNARK
    console_log!("Generating a CompressedSNARK using Spartan with IPA-PC...");
    let (pk, _vk) = CompressedSNARK::<_, _, _, _, S1, S2>::setup(&pp).unwrap();

    let start = Instant::now();
    let res = CompressedSNARK::<_, _, _, _, S1, S2>::prove(&pp, &pk, &recursive_snark);
    console_log!(
        "CompressedSNARK::prove: {:?}, took {:?}",
        res.is_ok(),
        start.elapsed()
    );
    assert!(res.is_ok());
    console_log!("Total proving time is {:?}", proof_gen_timer.elapsed());

    let compressed_snark = res.unwrap();
    let snark_proof_bytes = bincode::serialize(&compressed_snark).unwrap();
    let snark_proof = BASE64_STANDARD.encode(snark_proof_bytes);

    let mut hasher = Sha256::new();
    hasher.update(pp_bytes_vec);
    let pp_hash_bytes: [u8; 32] = hasher.finalize().try_into().unwrap();
    let pp_hash = hex::encode(pp_hash_bytes);

    let nova_aadhaar_proof = AadhaarAgeProof {
        version: 1,
        pp_hash,
        num_steps,
        current_date_ddmmyyyy: String::from_utf8(current_date_bytes.to_vec()).unwrap(),
        snark_proof,
    };

    return serde_wasm_bindgen::to_value(&nova_aadhaar_proof).unwrap();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AadhaarAgeVerifyResult {
    pub success: bool,
    pub message: String,
    pub nullifier: String,
}

macro_rules! return_verify_error {
    ($predicate:expr, $msg:literal) => {
        if $predicate {
            let result = AadhaarAgeVerifyResult {
                success: false,
                message: String::from($msg),
                nullifier: String::new(),
            };
            return serde_wasm_bindgen::to_value(&result).unwrap();
        }
    };
}

#[wasm_bindgen]
pub async fn verify_proof(pp_bytes: Uint8Array, aadhaar_age_proof: JsValue) -> JsValue {
    let res = serde_wasm_bindgen::from_value::<AadhaarAgeProof>(aadhaar_age_proof);
    return_verify_error!(res.is_err(), "Proof deserialization failed.");

    let nova_aadhaar_proof = res.unwrap();
    return_verify_error!(
        nova_aadhaar_proof.version != 1,
        "Proof version is not the expected value of 1"
    );

    let pp_bytes = pp_bytes.to_vec();

    console_log!("Checking if hash of generated public parameters matches the hash in proof");
    let mut hasher = Sha256::new();
    hasher.update(&pp_bytes);
    let pp_hash_bytes: [u8; 32] = hasher.finalize().try_into().unwrap();
    let pp_hash = hex::encode(pp_hash_bytes);
    return_verify_error!(
        nova_aadhaar_proof.pp_hash != pp_hash,
        "Public parameters hash did not match expected value."
    );
    console_log!("Hashes matched");

    let timer = Instant::now();
    console_log!("Started deserialization of public parameters");
    let res = bincode::deserialize(&pp_bytes[..]);
    return_verify_error!(res.is_err(), "Public parameters deserialization failed.");
    let pp: PublicParams<E1, E2, C1, C2> = res.unwrap();
    console_log!("Public parameters deserialized in {:?}", timer.elapsed());

    let res = CompressedSNARK::<_, _, _, _, S1, S2>::setup(&pp);
    return_verify_error!(res.is_err(), "Verifier key setup failed.");
    let (_pk, vk) = res.unwrap();

    // verify the compressed SNARK
    console_log!("Verifying a CompressedSNARK...");
    let start = Instant::now();

    let current_date_bytes: [u8; DOB_LENGTH_BYTES] = nova_aadhaar_proof
        .current_date_ddmmyyyy
        .as_bytes()
        .try_into()
        .unwrap();
    let z0_primary = C1::calc_initial_primary_circuit_input(&current_date_bytes);
    let z0_secondary = vec![<E2 as Engine>::Scalar::zero()];
    let res = BASE64_STANDARD.decode(nova_aadhaar_proof.snark_proof);
    return_verify_error!(
        res.is_err(),
        "Base64 decoding of SNARK proof string failed."
    );
    let snark_proof_bytes: Vec<u8> = res.unwrap();
    let res = bincode::deserialize(&snark_proof_bytes);
    return_verify_error!(res.is_err(), "SNARK proof deserialization failed.");
    let compressed_snark: CompressedSNARK<_, _, _, _, S1, S2> = res.unwrap();

    let res = compressed_snark.verify(
        &vk,
        nova_aadhaar_proof.num_steps,
        &z0_primary,
        &z0_secondary,
    );
    console_log!(
        "CompressedSNARK::verify: {:?}, took {:?}",
        res.is_ok(),
        start.elapsed()
    );
    return_verify_error!(res.is_err(), "SNARK proof invalid.");

    let final_outputs = res.unwrap().0;
    let final_opcode = final_outputs[0];
    return_verify_error!(
        final_opcode != <E1 as Engine>::Scalar::from(OP_CODE_LAST + 1),
        "Final opcode is incorrect."
    );

    let result = AadhaarAgeVerifyResult {
        success: true,
        message: String::from("Proof verification succeeded."),
        nullifier: format!("{:?}", final_outputs[1]),
    };

    return serde_wasm_bindgen::to_value(&result).unwrap();
}
