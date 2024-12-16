# Aadhaar-based Age Proof
**TLDR:** [Anon Aadhaar](https://pse.dev/en/projects/anon-aadhaar) in the browser using [Nova](https://github.com/microsoft/Nova) instead of Groth16. Based on the Rust code at https://github.com/avras/nova-aadhaar-qr. Try the demo [here](https://age-proof.vercel.app/).

The `frontend` directory has a SvelteKit-based static website that generates age proofs using WASM versions of Nova-based Rust code.

The `rust` directory has the glue logic needed to generate WASM bindings.

> [!WARNING]
> This code has not been audited. Use with care.


## Building and running the code
Run the following commands to build and run a local version of the website.

```
cd frontend
npm run build:wasm
npm run dev
```

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-
2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall b
e
dual licensed as above, without any additional terms or conditions.
