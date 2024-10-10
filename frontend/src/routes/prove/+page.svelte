<script>
  import { wrap } from 'comlink';
  import { onMount } from 'svelte';
  import init from '$lib/pkg/nova_aadhaar_qr_browser.js';
  import { browser } from '$app/environment';
  import { green, bigint_to_bytes } from '$lib/util';
  import {
    public_params_generated,
    public_params_generation_in_progress,
    public_params_generation_time,
    public_params_store,
    proof_generation_in_progress,
    proof_generation_time,
    proof_generated,
    prover_proof_object,
    proof_verification_in_progress,
    prover_demo,
    qr_code_data,
    qr_parse_status_message,
    selected_image_files,
    page_subtitle
  } from '$lib/stores';
  import PublicParams from '$lib/components/PublicParams.svelte';
  import Spinner from '$lib/components/Spinner.svelte';
  import SpinnerButton from '$lib/components/SpinnerButton.svelte';

  $page_subtitle = ' | Prove';
  let wasmInitialized = false;
  import {} from '$lib/stores';
  import jsQR from 'jsqr';
  import pako from 'pako';

  /**
   * @type {import("comlink").Remote<any>}
   */
  let worker;

  /**
   * @type {File}
   */
  let qr_image_file;

  $: if ($selected_image_files) {
    qr_image_file = $selected_image_files[0];
    checkQRCodeImageFile(qr_image_file);
  }

  async function initWebWorker() {
    if (browser) {
      if (window.Worker) {
        const w = new Worker(new URL('$lib/worker', import.meta.url), { type: 'module' });
        worker = wrap(w);
      }
    }
  }

  onMount(async () => {
    await init({});
    wasmInitialized = true;
    await initWebWorker();
    await genParams();
  });

  async function genParams() {
    if (browser) {
      if (window.Worker) {
        if ($public_params_generation_in_progress == false && $public_params_store.length == 0) {
          $public_params_generation_in_progress = true;
          const start = performance.now();
          $public_params_store = await worker.gen_params();
          $public_params_generation_time = performance.now() - start;
          $public_params_generated = true;
          $public_params_generation_in_progress = false;
        }
      }
    }
  }

  async function genProof() {
    if (browser) {
      if (window.Worker) {
        if ($proof_generation_in_progress == false) {
          const start = performance.now();
          $proof_generated = false;
          $proof_generation_in_progress = true;
          $prover_proof_object = await worker.gen_proof(
            $public_params_store,
            $qr_code_data,
            Uint8Array.from(Array.from('10-10-2024').map((letter) => letter.charCodeAt(0))),
            $prover_demo
          );
          $proof_generation_time = performance.now() - start;
          $proof_generation_in_progress = false;
          $proof_generated = true;
        }
      }
    }
  }

  function getProofFileLink() {
    const blob = new Blob([JSON.stringify($prover_proof_object, null, 2)], {
      type: 'application/json'
    });
    const blobURL = URL.createObjectURL(blob);
    return blobURL;
  }

  /**
   * @param {File} f
   */
  function checkQRCodeImageFile(f) {
    if (f) {
      let reader = new FileReader();
      reader.readAsDataURL(f);
      reader.onload = (e) => {
        if (e.target && e.target.result) {
          const image = new Image();
          image.onload = () => {
            try {
              const canvas = document.createElement('canvas');
              canvas.width = image.width;
              canvas.height = image.height;

              const ctx = canvas.getContext('2d');
              if (!ctx) throw Error('Image cannot be reconstructed');

              ctx.drawImage(image, 0, 0);
              const image_data = ctx.getImageData(0, 0, image.width, image.height);

              const qr_value = jsQR(image_data.data, image.width, image.height);
              if (!qr_value) throw Error('QR code could not be read from image');

              const compressed_bytes = bigint_to_bytes(BigInt(qr_value.data));
              // @ts-ignore
              $qr_code_data = pako.inflate(compressed_bytes);
              if (!$qr_code_data) throw Error('Failed to decompress QR code data');

              $qr_parse_status_message = `File <b>${f.name}</b> read successfully.`;
            } catch (error) {
              $qr_parse_status_message = `<span style="color:red;">Failed to parse <b>${f.name}</b>.`;
              console.log(error);
            }
          };
          image.src = e.target.result.toString();
        }
      };
    }
  }

  function resetChoice() {
    $selected_image_files = null;
    $proof_generated = false;
    $prover_demo = false;
  }

  async function useDemoQR() {
    $prover_demo = true;
    const demo_qr_code_filename = 'demo-qr.png';
    const response = await fetch('/' + demo_qr_code_filename);
    const data = await response.blob();
    const qr_image_file = new File([data], demo_qr_code_filename, { type: 'image/png' });
    checkQRCodeImageFile(qr_image_file);
  }
</script>

<div>
  <h2>Generate Age Proof</h2>
  {#if wasmInitialized}
    <PublicParams />
    {#if $public_params_generated}
      {#if $proof_verification_in_progress}
        Proof verification in progress. Go to <a href="/verify">Verify</a> page.
        <Spinner />
      {:else}
        {#if ($selected_image_files && qr_image_file) || $prover_demo}
          <div>
            {@html $qr_parse_status_message}
            {#if !$proof_generation_in_progress}
              <a href={'#'} on:click={resetChoice}>Choose another file</a>.
            {/if}
          </div>
        {:else}
          <label for="fileUpload"
            >Choose Aadhaar QR code image file. Or <a href={'#'} on:click={useDemoQR}
              >use a demo QR code</a
            >.</label
          >
          <br />
          <input
            type="file"
            class="form-control mt-2 mb-2"
            id="fileUpload"
            accept=".png,.jpeg,.jpg"
            bind:files={$selected_image_files}
            disabled={$proof_generation_in_progress}
          />
        {/if}
        {#if $qr_code_data && (qr_image_file || $prover_demo)}
          {#if !$proof_generation_in_progress}
            <button
              class="btn btn-primary mt-2 mb-2"
              style="background-color: {green};"
              on:click={genProof}
              disabled={$proof_generation_in_progress}>Generate Proof</button
            >
          {:else}
            <SpinnerButton labeltext="Generating Proof..." />
          {/if}
          {#if $proof_generated && !$proof_generation_in_progress}
            <p>
              Proof generation succeeded in {($proof_generation_time / 1000).toFixed(2)} seconds. Click
              <a href={getProofFileLink()} download="aadhaar-age-proof.json">here</a> to download
              the proof.
              <br />
              {#if $prover_demo}
                <span style="color:red;">
                  <b>Note:</b> This proof will fail verification because the demo QR code has dummy data.</span
                >
              {/if}
            </p>
          {/if}
        {/if}
      {/if}
    {/if}
  {:else}
    <div>
      <p>Initializing WASM module...</p>
      <Spinner />
    </div>
  {/if}
</div>
