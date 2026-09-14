# Cross-implementation interoperability evidence (P12)

Each subdirectory is one run of
`scripts/pqc_interop_openssl.sh` (the driver for `src/bin/pqc_interop.rs`), named
`<arch>-<os>_<date>`. Inside a run:

- `host.txt`: host, commit, rustc, OpenSSL version and mode (host binary or an
  `alpine:3.22` container).
- `summary.txt`: one `PASS` or `FAIL` line per graded check.
- `SHA256SUMS`: every file in the set.
- `A_kem_8db_keygen/`: 8DB encapsulation key as SubjectPublicKeyInfo DER, OpenSSL's
  ciphertext and shared secret, 8DB's decapsulated secret.
- `B_kem_openssl_keygen/`: OpenSSL's encapsulation key, 8DB's ciphertext and secret,
  OpenSSL's decapsulated secret.
- `C_dsa_8db_sign/`: 8DB verifying key as SubjectPublicKeyInfo DER, message, 8DB's
  signature, OpenSSL's verify output.
- `D_dsa_openssl_sign/`: OpenSSL's verifying key, message, three OpenSSL signatures
  (deterministic, hedged, and bound to the context `8db-interop`), the context in hex.
- `E_negative/`: the flipped signatures and ciphertext used as negative controls.

The test keys (ML-KEM decapsulation key, ML-DSA seed, OpenSSL PEM private keys) stay
in the run's `private/` directory, which is not published.

**What a set shows:** byte-level agreement on FIPS 203 / FIPS 204 public keys,
ciphertexts, shared secrets and signatures between 8DB's CAM providers
(`cam::ml_kem1024_provider`, `cam::ml_dsa87_provider`) and OpenSSL 3.5 on the named
host, in both directions, with negative controls. **What it does not show:** CAVP
validation, timing behaviour, or anything about how the store uses the primitives.
The public-key encoding is the LAMPS SubjectPublicKeyInfo form (NIST CSOR OIDs
2.16.840.1.101.3.4.4.3 and 2.16.840.1.101.3.4.3.19, parameters absent).

CI: `.github/workflows/pqc-interop.yml` repeats the run on x86_64 for every change to
the providers, weekly, and on aarch64 once the ARM64 runner in `docs/RUNNER-ARM64.md`
is registered. Workflow artifacts are the CI copies of these sets.

## Recorded runs

| Run | Host | OpenSSL | Checks | Result |
|---|---|---|---|---|
| [`x86_64-windows_2026-09-11`](x86_64-windows_2026-09-11/) | Windows 11 x86_64, Ryzen 9 9950X3D; rustc 1.94.1 x86_64-pc-windows-gnullvm; commit in this PR | 3.5.7 (host binary, Git for Windows) | A 3, B 2, C 3, D 4, E 3 | 15 of 15 checks PASS |
| [`aarch64-linux_2026-09-12`](aarch64-linux_2026-09-12/) | AWS Graviton3 c7g.2xlarge, Ubuntu 24.04 aarch64, self-hosted runner aarch64-ci-runner (GitHub Actions run 34662891115, PR #3487 head e6fdd5ec); rustc 1.94.1 aarch64-unknown-linux-gnu | 3.5.8 (alpine:3.22 container, docker fallback) | A 3, B 2, C 3, D 4, E 3 | 15 of 15 checks PASS |
