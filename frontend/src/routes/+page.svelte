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
        On a desktop/laptop computer, <b>proof generation</b> takes about 2 minutes. On a mobile device,
        it can take 10 minutes or more.
      </p>
      <p>
        On a desktop/laptop computer,
        <b>proof verification</b> takes about 8 seconds. On a mobile phone, it can takes about a minute.
      </p>
    </li>
    <li>
      <p><b> How big are the proofs?</b></p>
      <p>Proof files are about 15 KB in size.</p>
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
      <p>
        <b>WARNING: </b>Opening developer tools can <b>significantly slow down</b> proof generation in
        some browsers (Chrome, Brave). We have observed proof generation taking 3 times longer when Javascript
        console was open.
      </p>
    </li>
    <li>
      <p>
        <b
          >If the proofs reveal no information about the holder except the fact that they are an
          adult, what prevents multiple people from reusing a single valid proof?
        </b>
      </p>
      <p>
        The generated proof contains a <b>nullifier</b> which is a cryptographic hash of the data in
        the QR code (excluding a time stamp that changes everytime a QR code is requested). This nullifier
        is unique to a holder as long as they don't change their name, address, date of birth, gender,
        address, photo, or mobile number. So an application can use the nullifier to prevent reuse of
        a holder's QR code.
      </p>
      <p>
        A couple of caveats. Firstly, if a holder changes their identifying data then the nullifier
        will change. To prevent multiple valid proofs from the same holder's data, the verifier can
        require that the timestamp in the QR code be recent (for e.g., within the last week). This
        feature is not implemented in the present demo.
      </p>
      <p>
        Secondly, the nullifier can be used to track a user across applications. This kind of
        tracking can be prevented by generating an application-specific nullifier. The prover will
        accept an application name as input and generate a proof that contains a nullifier that will
        depend on both the application's name and the user's data. This feature is also not
        implemented in the present demo.
      </p>
    </li>
  </ul>
</div>
