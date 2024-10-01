<script>
  import { qr_code_data, qr_parse_status_message, selected_image_files } from '$lib/stores';
  import { bigint_to_bytes } from '$lib/util';
  import jsQR from 'jsqr';
  import pako from 'pako';

  /**
   * @type {File}
   */
  let qr_image_file;

  $: if ($selected_image_files) {
    qr_image_file = $selected_image_files[0];
    checkQRCodeImageFile(qr_image_file);
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

              console.log($qr_code_data);
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
  }
</script>

<div>
  <h1>Aadhaar-based Age Proof</h1>
  {#if $selected_image_files && qr_image_file}
    <div>
      {@html $qr_parse_status_message}
      <a href={'#'} on:click={resetChoice}>Choose another file</a>.
    </div>
  {:else}
    <label for="fileUpload">Choose Aadhaar QR code image file</label>
    <br />
    <input
      type="file"
      id="fileUpload"
      accept=".png,.jpeg,.jpg"
      bind:files={$selected_image_files}
    />
  {/if}
</div>
