<script>
  import { wrap } from 'comlink';
  import { onMount } from 'svelte';
  import init from '$lib/pkg/nova_aadhaar_qr_browser.js';
  import { browser } from '$app/environment';
  import { green, millisToMinutesAndSeconds, readFile } from '$lib/util';
  import {
    public_params_generated,
    public_params_generation_in_progress,
    public_params_generation_time,
    public_params_store,
    proof_verification_in_progress,
    proof_generation_in_progress,
    selected_json_files,
    proof_verified,
    proof_verification_time,
    verifier_proof_object,
    json_parse_status_message,
    proof_is_correct,
    proof_verification_message,
    verifier_demo,
    page_subtitle,
    proof_nullifier
  } from '$lib/stores';
  import PublicParams from '$lib/components/PublicParams.svelte';
  import Spinner from '$lib/components/Spinner.svelte';
  import SpinnerButton from '$lib/components/SpinnerButton.svelte';

  $page_subtitle = ' | Verify';
  let wasmInitialized = false;
  /**
   * @type {import("comlink").Remote<any>}
   */
  let worker;

  /**
   * @type {File}
   */
  let proof_file;

  $: if ($selected_json_files) {
    proof_file = $selected_json_files[0];
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

  async function verifyProof() {
    if (browser) {
      if (window.Worker) {
        $proof_verification_in_progress = true;
        const start = performance.now();
        $proof_verified = false;
        let res = await worker.verify_proof($public_params_store, $verifier_proof_object);
        $proof_is_correct = res.success;
        $proof_verification_message = res.message;
        $proof_nullifier = res.nullifier;
        $proof_verification_time = performance.now() - start;
        $proof_verified = true;
        $proof_verification_in_progress = false;
      }
    }
  }

  /**
   * @param {File} f
   */
  async function parseProofFile(f) {
    if ($proof_verification_in_progress == false) {
      var proof_file_name = '';
      var proof_json_str = '';
      if ($verifier_demo) {
        const response = await fetch('/demo-proof.json');
        proof_json_str = await response.text();
        proof_file_name = 'demo-proof.json';
      } else {
        proof_json_str = await readFile(f);
        proof_file_name = f.name;
      }

      try {
        $verifier_proof_object = JSON.parse(proof_json_str);
        console.log('success while parsing proof file');
        $json_parse_status_message = `File <b>${proof_file_name}</b> read successfully.`;
      } catch (error) {
        console.log('error while parsing proof file');
        $json_parse_status_message = `Failed to parse <b>${proof_file_name}</b>. \
        Check the file and try again.`;
        console.log(error);
      }
    }
  }

  function resetChoice() {
    $selected_json_files = null;
    $verifier_proof_object = {};
    $proof_verified = false;
    $proof_verification_in_progress = false;
    $verifier_demo = false;
  }

  function useDemoProof() {
    $verifier_demo = true;
  }
</script>

<div>
  <h2>Verify Age Proof</h2>
  {#if wasmInitialized}
    <PublicParams />
    {#if $public_params_generated}
      {#if $proof_generation_in_progress}
        Proof generation in progress. Go to <a href="/prove">Prove</a> page.
        <Spinner />
      {:else}
        {#if ($selected_json_files && proof_file) || $verifier_demo}
          <div>
            {#await parseProofFile(proof_file)}
              <p>Parsing proof file..</p>
            {:then}
              {@html $json_parse_status_message}
              {#if !$proof_verification_in_progress}
                <a href={'#'} on:click={resetChoice}>Choose another file</a>.
              {/if}
            {/await}
          </div>
        {:else}
          <label for="fileUpload"
            >Choose the proof JSON file. Or <a href={'#'} on:click={useDemoProof}
              >use a demo proof file</a
            >.</label
          >
          <input
            type="file"
            class="form-control mt-2 mb-2"
            id="fileUpload"
            accept=".json"
            bind:files={$selected_json_files}
            disabled={$proof_verification_in_progress}
          />
        {/if}

        {#if Object.keys($verifier_proof_object).length != 0}
          {#if !$proof_verification_in_progress}
            <button
              class="btn btn-primary mt-2 mb-2"
              style="background-color: {green};"
              on:click={verifyProof}
              disabled={$proof_verification_in_progress}>Verify Proof</button
            >
          {:else}
            <SpinnerButton labeltext="Verifying Proof..." />
          {/if}
          {#if $proof_verified}
            {#if $proof_is_correct}
              <p>
                Proof verification succeeded in {millisToMinutesAndSeconds(
                  $proof_verification_time
                )}.
                <br />
                Nullifier value is {$proof_nullifier}.
              </p>
            {:else}
              <p style="color:red;">
                Proof verification failed. <b>Reason:</b>
                {$proof_verification_message}
              </p>
            {/if}
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
