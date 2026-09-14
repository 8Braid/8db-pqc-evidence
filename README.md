# 8DB storage-layer CNSA 2.0 evidence

Reproducible evidence for the post-quantum cryptography in 8DB, an embedded
database that protects stored records with a CNSA 2.0 Category 5 algorithm set
and migrates already-stored data from one algorithm to another while it keeps
serving reads. This repository holds the evidence a second evaluator can check,
not the product.

**Algorithm set.** ML-KEM-1024 (FIPS 203) for key establishment, ML-DSA-87
(FIPS 204) for signatures, AES-256 (SP 800-38D GCM in the compliance build,
RFC 8452 GCM-SIV in the default build) for the record AEAD, and SHA-384
(FIPS 180-4) for the boundary digest and HKDF (SP 800-56C).

## What this is, and what it is not

- **Algorithm-correctness evidence.** ML-KEM-1024 keyGen and encapDecap and
  ML-DSA-87 sigVer passed NIST's ACVP **demonstration** server, session 744723,
  isSample = false (2026-06-16). An offline known-answer suite passes 35 of 35
  vectors. This is algorithm-correctness evidence, not module validation.
- **Not a certificate.** Nothing here is CAVP-tested or CMVP-validated, and no
  certificate number exists. The compliance build path runs AES, HKDF, HMAC and
  ML-KEM through aws-lc-rs, which carries its own CMVP validation; ML-DSA-87 is
  the RustCrypto crate. We do not claim "validated", "certified", "approved" or
  "first".
- **Transport scope.** The record is protected at rest and travels as a sealed
  object on the device-mesh sync path. This is not a claim about the TLS layer,
  which today negotiates a Category 3 hybrid group.

## Why storage-layer migration

OMB M-26-15 tells agencies to re-encrypt long-lived sensitive data under
PQC-protected keys and prove it. NIST CSWP 39 section 5.1 asks for a mechanism
to change the algorithm protecting data at rest without interrupting the system,
and SP 1800-38B lists database protection and record integrity among the
data-at-rest cases. A record written years ago has no handshake to upgrade; the
upgrade has to be a database operation. 8DB performs that operation: every
ciphertext carries an algorithm version, two versions run side by side, a
quarantined algorithm refuses new writes while still serving reads, and a batch
is authenticated by one ML-DSA-87 signature over a SHA-384 Merkle root with a
per-record inclusion proof.

## Contents

| Path | What it holds |
|---|---|
| `acvp/` | The ACVP demonstration-server disposition, the session log, and the offline KAT report (35 of 35). |
| `kat/ml-kem-1024/`, `kat/ml-dsa-87/` | The known-answer vectors and our outputs, so the algorithm implementations can be checked against NIST's own vectors. |
| `interop/` | Cross-implementation interoperability against OpenSSL 3.5, both directions, on x86_64 and aarch64. |
| `bench/` | Release-mode CNSA 2.0 microbenchmarks on several hosts: record sizes, per-record encrypt and decrypt medians, key-establishment object size. |
| `timing/` | The statistical timing-leak screening (dudect-style) of the ML-KEM, ML-DSA and page-AEAD paths, with its reading rules and per-run reports. |
| `CLAIMS-AND-SCOPE.md` | Every quotable number with its scope and harness, and the claims that carry a qualifier. |
| `HOW-TO-VERIFY.md` | What a second evaluator can reproduce independently, and how. |
| `REDACTIONS.md` | The identifier scrubs applied for publication, with before and after hashes; no measurement changed. |

## How a second evaluator checks this

- **Algorithm correctness.** The `kat/` vectors are NIST's own known-answer
  values and our responses. Verify our ML-KEM-1024 and ML-DSA-87 outputs against
  the NIST vectors with any FIPS 203 / 204 reference, or re-run the ACVP
  demonstration server against the disposition in `acvp/`.
- **Interoperability.** The `interop/` artifacts are ciphertexts and signatures
  produced by our implementation and by OpenSSL 3.5, each verified by the other.
  Reproduce with OpenSSL 3.5 or liboqs.
- **Sizes and latencies.** The `bench/` numbers are release-mode
  runs with the host recorded in each file. The one-command harness that
  produces them runs inside the 8DB engine; the engine is available to evaluators
  on request (see below). The recorded outputs here are checkable as they stand.

Every reading in `timing/` and every number in `CLAIMS-AND-SCOPE.md` carries its
host, date, sample size and build profile. A reading with no quiet-host control,
or a run marked invalid, is labelled as such and is not quoted.

## Requesting the runnable engine

The reproducibility harness for the full transition runs inside the 8DB engine,
which is not open source. An evaluator who wants to run it end to end can request
access. Contact: Ashley Dunfield, 8Braid Inc., ashley@8braid.com.

## Provenance

These artifacts are exported from 8Braid's internal evidence tree. The internal
tree carries additional runs and working notes that are not part of this package.
Each file here is a copy of a recorded run in which only host, path, account
and internal-tracker identifiers were replaced with neutral tokens; the change
is itemised with before-and-after hashes in `REDACTIONS.md`. No measurement,
vector or digest was altered.
