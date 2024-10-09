<script>
  import { green } from '$lib/util';

  export let labeltext;
</script>

<button class="btn btn-primary mt-2 mb-2" style="background-color: {green};" type="button" disabled>
  <span class="spinner-border spinner-border-sm" aria-hidden="true"></span>
  <span role="status">{labeltext}</span>
</button>
