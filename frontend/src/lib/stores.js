import { writable } from 'svelte/store';

export const page_subtitle = writable('');
export const public_params_generated = writable(false);
export const public_params_generation_in_progress = writable(false);
export const public_params_generation_time = writable(0);
export const public_params_store = writable(new Uint8Array());

export const proof_verification_in_progress = writable(false);
export const proof_generation_in_progress = writable(false);
export const proof_generation_time = writable(0);
export const proof_verification_time = writable(0);
export const proof_generated = writable(false);
export const proof_verified = writable(false);
export const proof_is_correct = writable(false);
export const prover_proof_object = writable({});
export const verifier_proof_object = writable({});
export const proof_verification_message = writable('');

export const selected_image_files = writable();
export const qr_parse_status_message = writable('');
export const qr_code_data = writable(new Uint8Array());

export const selected_json_files = writable();
export const json_parse_status_message = writable('');
export const prover_demo = writable(false);
export const verifier_demo = writable(false);
