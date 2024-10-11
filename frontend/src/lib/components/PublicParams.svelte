<script>
  import { public_params_generated, public_params_generation_time } from '$lib/stores';
  import { millisToMinutesAndSeconds } from '$lib/util';
  import Spinner from './Spinner.svelte';
</script>

{#if !$public_params_generated}
  <p>Generating public parameters. It takes about a minute...</p>
  <Spinner />
{:else}
  <p>
    Public parameters generated in {millisToMinutesAndSeconds($public_params_generation_time)}.
  </p>
{/if}
