<script>
  import { page_subtitle } from '$lib/stores';
  import { green } from '$lib/util';
</script>

<svelte:head>
  <title>Aadhaar Age Proof {$page_subtitle}</title>
</svelte:head>

<!-- As a link -->
<div class="container">
  <div class="row justify-content-center">
    <div class="col-md-8">
      <nav
        class="navbar navbar-expand-sm sticky-top navbar-dark"
        style="background-color: {green};"
      >
        <div class="container-fluid">
          <a class="navbar-brand" href="/">Aadhaar Age Proof</a>
          <button
            class="navbar-toggler"
            type="button"
            data-bs-toggle="collapse"
            data-bs-target="#navbarSupportedContent"
            aria-controls="navbarSupportedContent"
            aria-expanded="false"
            aria-label="Toggle navigation"
          >
            <span class="navbar-toggler-icon"></span>
          </button>
          <div class="collapse navbar-collapse justify-content-end" id="navbarSupportedContent">
            <div class="navbar-nav">
              <a class="nav-link" aria-current="page" href="/">Home</a>
              <a class="nav-link" href="/prove">Prove</a>
              <a class="nav-link" href="/verify">Verify</a>
            </div>
          </div>
        </div>
      </nav>

      <slot />
    </div>
  </div>
</div>
