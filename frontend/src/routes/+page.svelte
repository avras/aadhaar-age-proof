<script>
  import { page_subtitle } from '$lib/stores';

  $page_subtitle = '';
</script>

<div>
  <h2>About this site</h2>
  <p>
    Welcome. This site allows you to <b>prove that an Aadhaar holder is at least 18 years old</b> using
    zero-knowledge proofs. No other information about the holder is revealed by the generated proof.
    An image of the holder's Aadhaar QR code is required to generate the proof.
  </p>
  <p>
    This site was made by <a target="_blank" href="https://www.ee.iitb.ac.in/~sarva/"
      >Saravanan Vijayakumaran</a
    >
    of IIT Bombay. <a target="_blank" href="https://x.com/ayush_modi_">Ayush Modi</a> of Christ
    College, Bengaluru helped with the initial design during his summer internship at
    <a target="_blank" href="https://trustlab.iitb.ac.in/">IITB Trust Lab</a>.
  </p>

  <h3>What is the Aadhaar QR code?</h3>
  An Aadhaar QR code is a QR code containing digitally signed data about an Aadhaar holder. It can be
  found on the back of an Aadhaar card. It can also be downloaded using the mAadhaar app (<a
    href="https://play.google.com/store/apps/details?id=in.gov.uidai.mAadhaarPlus"
    target="_blank">Google Play</a
  >, <a href="https://apps.apple.com/in/app/maadhaar/id1435469474" target="_blank">App Store</a>).
  It contains the following information about the holder.
  <ul>
    <li>Last 4 digits of Aadhaar number</li>
    <li>Name</li>
    <li>Date of birth</li>
    <li>Gender</li>
    <li>Address</li>
    <li>Last 4 digits of mobile number</li>
    <li>Photo (in JPEG format)</li>
  </ul>
  Additionally, it contains a timestamp indicating the date and time at which the QR code was created.

  <h3>FAQs</h3>
  <ul>
    <li>
      <p><b>Is my Aadhaar QR code stored by this site?</b></p>
      <p>
        No. The entire proof generation occurs in the browser. The QR code data is not sent to any
        server.
      </p>
    </li>
    <li>
      <p>
        <b
          >I am not convinced that this site does not store my Aadhaar QR code data. Where can I
          find the source code of this site?</b
        >
      </p>

      <p>
        Source code will be released soon. In the meantime, you can use the demo files. See next
        question.
      </p>
    </li>
    <li>
      <p>
        <b
          >Can I see a demo of the proof generation and verification without uploading my Aadhaar QR
          code?</b
        >
      </p>
      <p>
        Yes. Use the demo file options in the <a href="/prove">Prove</a> and
        <a href="/verify">Verify</a> pages.
      </p>
    </li>
    <li>
      <p><b>How long does the proof generation and verification take?</b></p>
      <p>
        Prior to proof generation and verification, this site needs to generate some public
        parameters which takes less than a minute. A rule of thumb for calculating the proof
        generation time is to multiply the public parameter generation time by 15.
      </p>
      <p>
        On a desktop/laptop computer, <b>proof generation</b> takes about 3 minutes. On a mobile device,
        it can take 10 minutes or more.
      </p>
      <p>
        On a desktop/laptop computer,
        <b>proof verification</b> takes about 15 seconds. On a mobile phone, it can takes about a minute.
      </p>
    </li>
    <li>
      <p><b> How big are the proofs?</b></p>
      <p>Proof files are about 32 KB in size.</p>
    </li>
    <li>
      <p><b>Proof generation is taking a long time. How can I monitor its progress?</b></p>
      <p>
        If you are using a browser on a desktop/laptop computer, you can monitor progress of proof
        generation using the Javascript console in the browser's developer tools. It can be opened
        by pressing <b>F12</b> on Chrome and Firefox. If you are using these browsers on a Mac, you
        need to press <b>Cmd+Opt+J</b>. On Safari, you need to first enable showing the developer
        menu first and then press
        <b>Cmd+Opt+C</b>. See
        <a target="_blank" href="https://javascript.info/devtools">instructions here</a>.
      </p>
      <p>On a mobile phone, there is currently no way to monitor progress.</p>
      <b>WARNING: </b>Opening developer tools can <b>significantly slow down</b> proof generation in
      some browsers (Chrome, Brave). We have observed proof generation taking 3 times longer when developer
      tools was open.
    </li>
  </ul>
</div>
