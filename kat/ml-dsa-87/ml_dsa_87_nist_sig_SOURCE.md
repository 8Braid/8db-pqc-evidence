# ML-DSA-87 signature vector provenance

The `ml_dsa_87_nist_siggen_*` and `ml_dsa_87_nist_sigver_*` fixtures contain
NIST ACVP-Server vectors for ML-DSA-87, converted from hexadecimal to raw bytes.
They let reviewers compare signature generation and verification with the
source vectors at the same cryptographic interface.

## Source

- Repository: [NIST ACVP-Server](https://github.com/usnistgov/ACVP-Server)
- Recorded commit: `15c0f3deeefbfa8cb6cd32a99e1ca3b738c66bf0` (master, 2026-04-16)
- Signature generation: `gen-val/json-files/ML-DSA-sigGen-FIPS204/internalProjection.json`, vsId 42
- Signature verification: `gen-val/json-files/ML-DSA-sigVer-FIPS204/internalProjection.json`, vsId 42

## Match the interface parameters

Both sets use test group 12 with these parameters:

```text
parameterSet: ML-DSA-87
signatureInterface: internal
deterministic: true
externalMu: false
preHash: none
```

For signature generation, the engine uses
`ExpandedSigningKey::<MlDsa87>::from_expanded(sk).sign_internal(&[msg], &rnd)`
with `rnd = [0u8; 32]`. This maps to FIPS 204 Algorithm 7,
ML-DSA.Sign_internal. The generated 4,627-byte signature is compared byte for
byte with the NIST `signature` field.

For signature verification, the engine uses
`VerifyingKey::<MlDsa87>::verify_internal(msg, sig)`, corresponding to FIPS 204
Algorithm 8, ML-DSA.Verify_internal. Its verdict is compared with the NIST
`testPassed` field.

## Fixture files

There are 15 signature-generation cases and 15 signature-verification cases,
each numbered 166 through 180.

| File pattern | Contents |
|---|---|
| `ml_dsa_87_nist_siggen_tc{n}_sk.bin` | NIST signing key, 4,896 bytes, encoded with FIPS 204 section 6.1 skEncode |
| `ml_dsa_87_nist_siggen_tc{n}_msg.bin` | Message, variable length |
| `ml_dsa_87_nist_siggen_tc{n}_sig.bin` | Expected signature, 4,627 bytes |
| `ml_dsa_87_nist_sigver_tc{n}_pk.bin` | Verifying key, 2,592 bytes |
| `ml_dsa_87_nist_sigver_tc{n}_msg.bin` | Message |
| `ml_dsa_87_nist_sigver_tc{n}_sig.bin` | Signature under test |
| `ml_dsa_87_nist_sigmeta.json` | Expected verdict and failure reason for each case |

Of the 15 verification cases, three are valid and must be accepted. Twelve
contain deliberately modified z, commitment, hint or message values and must
be rejected.

## Reproduce the checks

The evaluation engine's `vault_core::vv::h7_acvp_kat` harness loads these files,
re-signs the generation inputs through the deterministic internal interface and
checks the expected verification verdicts. A byte mismatch or incorrect verdict
fails the corresponding check.

The fixtures are public and can also be checked with an independent implementation
that exposes the same internal interface. Engine and harness access is available
from [8Braid](mailto:ashley@8braid.com). These fixtures are separate from the
[35-check offline suite report](../../acvp/Offline-KAT-Report-35of35.txt).