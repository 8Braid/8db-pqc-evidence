# Interoperability with OpenSSL 3.5

Both recorded runs passed all 15 checks for ML-KEM-1024 and ML-DSA-87 exchanges
between 8DB and OpenSSL. They cover both directions, context-bound signatures
and rejection of modified inputs. Integration teams can inspect the formats
and verify published signatures directly with OpenSSL.

## Recorded runs

| Run | Host | OpenSSL | Checks | Result |
|---|---|---|---|---|
| [`x86_64-windows_2026-09-11`](x86_64-windows_2026-09-11/) | Windows 11 x86_64, Ryzen 9 9950X3D; rustc 1.94.1 x86_64-pc-windows-gnullvm; commit `e18a9a3bcae8f541ac07716fc59c410a392a6d63`, 9 dirty paths | 3.5.7 (host binary, Git for Windows) | A 3, B 2, C 3, D 4, E 3 | 15 of 15 checks PASS |
| [`aarch64-linux_2026-09-12`](aarch64-linux_2026-09-12/) | AWS Graviton3 c7g.2xlarge, Ubuntu 24.04 aarch64; rustc 1.94.1 aarch64-unknown-linux-gnu; GitHub Actions run 34662891115, PR #3487 head e6fdd5ec | 3.5.8 (`alpine:3.22` container, Docker fallback) | A 3, B 2, C 3, D 4, E 3 | 15 of 15 checks PASS |

## Inspect a run

Each folder contains the output of the engine's
`scripts/pqc_interop_openssl.sh`, which drives `src/bin/pqc_interop.rs`.

- `host.txt`: host, commit, rustc, OpenSSL version and execution mode.
- `summary.txt`: one `PASS` or `FAIL` line per graded check.
- `SHA256SUMS`: the original manifest. See [publication history](../REDACTIONS.md)
  for published files whose hashes changed after identifier removal.
- `A_kem_8db_keygen/`: 8DB's encapsulation key as SubjectPublicKeyInfo DER,
  OpenSSL's ciphertext and shared secret, and 8DB's decapsulated secret.
- `B_kem_openssl_keygen/`: OpenSSL's encapsulation key, 8DB's ciphertext and
  shared secret, and OpenSSL's decapsulated secret.
- `C_dsa_8db_sign/`: 8DB's verifying key as SubjectPublicKeyInfo DER, message,
  signature and OpenSSL's verification output.
- `D_dsa_openssl_sign/`: OpenSSL's verifying key, message, three signatures
  (deterministic, hedged and bound to context `8db-interop`) and the context in hex.
- `E_negative/`: modified signatures and ciphertext used as negative controls.

The public-key encodings use the LAMPS SubjectPublicKeyInfo form, with NIST CSOR
OIDs `2.16.840.1.101.3.4.4.3` and `2.16.840.1.101.3.4.3.19` and absent parameters.
The 8DB providers under test were `cam::ml_kem1024_provider` and
`cam::ml_dsa87_provider`.

## Verify a published signature

With OpenSSL 3.5 on your PATH, change into either run folder and run:

```bash
openssl pkeyutl -verify -pubin -keyform DER -inkey C_dsa_8db_sign/vk_8db.der -rawin -in C_dsa_8db_sign/msg.bin -sigfile C_dsa_8db_sign/sig_8db.bin
```

The expected result is `Signature Verified Successfully`. Repeat with the
modified signature:

```bash
openssl pkeyutl -verify -pubin -keyform DER -inkey C_dsa_8db_sign/vk_8db.der -rawin -in C_dsa_8db_sign/msg.bin -sigfile E_negative/sig_8db_flipped.bin
```

That check should fail verification and return a nonzero exit code. The
[OpenSSL command reference](https://docs.openssl.org/3.5/man1/openssl-pkeyutl/)
also describes context-string options for the signatures in `D_dsa_openssl_sign/`.

## Repeat the complete exchange

A fresh run in both directions requires the 8DB evaluation engine and interop
harness. The original test keys, including ML-KEM decapsulation keys, the
ML-DSA seed and OpenSSL private keys, are retained privately. The published
shared-secret files can be compared directly; re-decapsulating the original
ciphertexts requires those keys.

The engine repository's `.github/workflows/pqc-interop.yml` defines repeat runs
for provider changes and scheduled checks. That workflow and the engine source
are outside this public checkout. Contact [8Braid](mailto:ashley@8braid.com)
to arrange an evaluation on your platform.

These results cover primitive-level agreement on the named hosts. Timing,
database integration and certification status have separate evidence and scope
in [claims and scope](../CLAIMS-AND-SCOPE.md).