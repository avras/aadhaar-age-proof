<script>
  import { public_params_generated, public_params_generation_time } from '$lib/stores';
  import Spinner from './Spinner.svelte';
</script>

{#if !$public_params_generated}
  <p>Generating public parameters. It takes about a minute...</p>
  <Spinner />
{:else}
  <p>
    Public parameters generated in {($public_params_generation_time / 1000).toFixed(2)} seconds.
  </p>
{/if}
