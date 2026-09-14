# ML-DSA-87 sigGen / sigVer KAT vectors — PROVENANCE (R-ACVP-PRECISION)

**Status: AUTHENTIC NIST ACVP-Server vectors (NOT self-generated).**

These `ml_dsa_87_nist_siggen_*` and `ml_dsa_87_nist_sigver_*` files are the
official NIST ACVP-Server known-answer vectors for the **ML-DSA-87** parameter
set, extracted verbatim (hex → raw bytes) from:

- Repo: <https://github.com/usnistgov/ACVP-Server>
- Commit: `15c0f3deeefbfa8cb6cd32a99e1ca3b738c66bf0` (master, 2026-04-16)
- sigGen file: `gen-val/json-files/ML-DSA-sigGen-FIPS204/internalProjection.json` (vsId 42)
- sigVer file: `gen-val/json-files/ML-DSA-sigVer-FIPS204/internalProjection.json` (vsId 42)

They are the same authoritative CAVP vectors against which `aws-lc-fips`,
`BoringSSL`, and the RustCrypto `ml-dsa` crate are validated.

## Which test group

Both use the **`ML-DSA-87`, `signatureInterface: "internal"`, `deterministic:
true`, `externalMu: false`, `preHash: "none"`** group (sigGen tgId 12, sigVer
tgId 12). This maps byte-exactly onto the FIPS 204 internal interface exposed
by the `ml-dsa` crate:

- **sigGen** — `ExpandedSigningKey::<MlDsa87>::from_expanded(sk).sign_internal(&[msg], &rnd)`
  with `rnd = [0u8; 32]` (deterministic ⇒ all-zero per-signature randomness).
  FIPS 204 Algorithm 7 (ML-DSA.Sign_internal). The produced 4627-byte signature
  is byte-identical to the NIST `signature` field.
- **sigVer** — `VerifyingKey::<MlDsa87>::verify_internal(msg, sig)`. FIPS 204
  Algorithm 8 (ML-DSA.Verify_internal). The verdict must equal the NIST
  `testPassed` field.

## Files (tcId 166–180 sigGen; 166–180 sigVer)

- `ml_dsa_87_nist_siggen_tc{n}_sk.bin`  — NIST signing key (4896 B, FIPS 204 §6.1 skEncode)
- `ml_dsa_87_nist_siggen_tc{n}_msg.bin` — NIST message (variable length)
- `ml_dsa_87_nist_siggen_tc{n}_sig.bin` — NIST expected signature (4627 B) — the known answer
- `ml_dsa_87_nist_sigver_tc{n}_pk.bin`  — NIST verifying key (2592 B)
- `ml_dsa_87_nist_sigver_tc{n}_msg.bin` — NIST message
- `ml_dsa_87_nist_sigver_tc{n}_sig.bin` — NIST signature under test
- `ml_dsa_87_nist_sigmeta.json`         — per-tc `expected_pass` verdict + failure `reason`
  (12 of the 15 sigVer cases are deliberately-invalid — modified z / commitment /
  hint / message — and MUST be rejected; 3 are valid and MUST be accepted).

## Verification

The vault-core H7 harness (`vault_core::vv::h7_acvp_kat`):
- **sigGen KAT** — loads `sk`, re-signs `msg` via the internal deterministic
  interface, and asserts the signature is byte-identical to the recorded NIST
  `sig`. A mismatch FAILS H7.
- **sigVer KAT** — verifies `sig` under `pk` via the internal interface and
  asserts the boolean verdict equals the NIST `expected_pass`. A wrong verdict
  (accept-an-invalid or reject-a-valid) FAILS H7.

This closes the R-ACVP-PRECISION honest gap for ML-DSA-87 sigGen/sigVer: the
signature-generation and signature-verification boundaries are now attested by
the official NIST vectors byte-for-byte, not by a self-consistency KAT.
