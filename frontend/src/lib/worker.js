import { expose } from 'comlink';
import { threads } from 'wasm-feature-detect';

const AadhaarAgeProofWorker = {
  async gen_params() {
    const multiThread = await import('$lib/pkg/nova_aadhaar_qr_browser');
    await multiThread.default();
    if (await threads()) {
      await multiThread
        .initThreadPool(navigator.hardwareConcurrency)
        .catch(() =>
          console.log('Failed to initialize thread pool. Could be due to repeated initialization.')
        );
    }

    return await multiThread.generate_public_parameters();
  },

  /**
   * @param {string} pp_str
   * @param {Uint8Array} qr_data_bytes
   * @param {Uint8Array} current_date_bytes
   * @param {boolean} demo
   */
  async gen_proof(pp_str, qr_data_bytes, current_date_bytes, demo) {
    const multiThread = await import('$lib/pkg/nova_aadhaar_qr_browser');
    await multiThread.default();
    if (await threads()) {
      await multiThread
        .initThreadPool(navigator.hardwareConcurrency)
        .catch(() =>
          console.log('Failed to initialize thread pool. Could be due to repeated initialization.')
        );
    }
    return await multiThread.generate_proof(pp_str, qr_data_bytes, current_date_bytes, demo);
  },

  /**
   * @param {string} pp_str
   * @param {any} nova_aadhaar_proof
   */
  async verify_proof(pp_str, nova_aadhaar_proof) {
    const multiThread = await import('$lib/pkg/nova_aadhaar_qr_browser');
    await multiThread.default();
    if (await threads()) {
      await multiThread
        .initThreadPool(navigator.hardwareConcurrency)
        .catch(() =>
          console.log('Failed to initialize thread pool. Could be due to repeated initialization.')
        );
    }
    return await multiThread.verify_proof(pp_str, nova_aadhaar_proof);
  }
};

expose(AadhaarAgeProofWorker);
