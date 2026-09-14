# 8DB results and measurement scope

Use this guide to assess 8DB's cryptographic tests, storage costs and migration
behavior. Each section identifies the evidence available in this repository
and the additional material available with an evaluation.

## Algorithm correctness and interoperability

| Test | Recorded result | Evidence |
|---|---|---|
| ACVP demonstration service | ML-KEM-1024 keyGen and encapDecap, and ML-DSA-87 sigVer passed in session 744723 on June 16, 2026, with `isSample = false`. | [Server disposition](acvp/NIST-ACVP-Disposition-2026-06-16.json), [session log](acvp/Live-Session-Log-744723.txt) |
| Offline known-answer suite | 35 of 35 checks passed across HKDF, AES-GCM, AES-GCM-SIV, HMAC, ML-KEM-1024 and ML-DSA-87, using NIST and RFC test vectors. | [Offline report](acvp/Offline-KAT-Report-35of35.txt), [published PQC vectors](kat/) |
| OpenSSL 3.5 interoperability | 15 of 15 checks passed on each of x86_64 and aarch64, including both directions and negative controls. | [Interoperability guide and runs](interop/README.md) |

The interoperability checks cover matching shared secrets for ML-KEM-1024 and
signature verification for ML-DSA-87. OpenSSL verifies 8DB's signatures; 8DB
verifies OpenSSL's deterministic, hedged and context-bound signatures. These
results describe the tested implementations and operations. Formal validation
status is recorded below.

## Storage and processing costs

The design shares key-establishment objects across records and stores an
ML-DSA-87 signature once per batch. The useful comparison depends on which
cost is being measured.

| Measurement | Result | Scope |
|---|---|---|
| Same-AEAD record size | A 115-byte input produces 131 bytes with AES-256-GCM-SIV. | Identical for the compared classical-KDF and ML-KEM-1024 plus X25519 key-establishment paths. The 16 added bytes are the AEAD tag. |
| Compliance-profile record size | 185 bytes for the same 115-byte input. | AES-256-GCM with the profile's 48-byte key-commitment tag and framing, as recorded in the internal measurement record. |
| KEM-specific object | 1,616 bytes. | A 1,568-byte ML-KEM-1024 ciphertext plus a 48-byte wrapped key, per established key. |
| ML-DSA-87 signature | 4,627 bytes. | Signature bytes alone. The stored batch representation, inclusion proofs and other framing have additional costs. |

At 100,000 records of 131 bytes each, the 1,616-byte KEM-specific object is
0.0123% of the encrypted record bytes. This ratio measures that object alone;
use the full representation when budgeting storage.

For a design that attaches a 4,627-byte signature to every 115-byte record,
signature bytes alone add approximately 4,023%. Sharing a signature across a
batch changes that cost. The appropriate batch size and inclusion-proof cost
depend on the workload.

### Published release runs

The [benchmark files](bench/) contain three release runs: Windows x86_64 on
September 11, Linux x86_64 on September 11, and Linux aarch64 on September 12,
2026. Each file identifies its host, build and command. The Windows and
aarch64 files also record host load; both runs were measured under load.

In their same-AEAD comparisons, median per-record encrypt and decrypt times for
classical and post-quantum key establishment agree within 1%. A fourth,
historical July run is retained internally; its decrypt medians differ by
100 ns, approximately 14%, with the cause unresolved. Quote a named run when
using a latency result.

The internal compliance-profile comparison records approximately one additional
microsecond per record on both architectures, attributed to the HMAC-SHA-384
key-commitment path. Request that run with the evaluation materials before
using this estimate in a performance decision.

These are operation-level microbenchmarks. The files also contain comparisons
with per-row signing and a decrypt-and-scan search baseline. Those configurations
answer specific design questions; workload-level database comparisons require
matched durability, indexing, concurrency and protection settings.

## Migration

The recorded transition re-encrypts stored records from page keys derived with
HKDF-SHA256 into a new generation using HKDF-SHA384. Each record is verified
before the new generation is activated atomically. The old generation remains
readable during the copy.

The 100,000-record run was recorded on Linux x86_64 on September 14, 2026.
Its raw results and the engine harness are available to evaluators on request.
This establishes a specific change in key derivation with record re-encryption.
Additional transitions, interruption recovery and production operating limits
should be agreed and tested against the intended deployment.

### Per-row signing comparison

The internal record also contains storage measurements for a composition that
adds one ML-DSA-87 signature per row at the application layer, with encryption
at rest:

| Configuration | Recorded storage |
|---|---|
| PostgreSQL 18.4 with pgcrypto | 579,452,928 bytes |
| MongoDB | 500,445,184 bytes |

The same record set was measured on a Windows host on July 19, 2026. The internal
record reports reproduction within 1.2% in Linux containers on September 14,
with a second-party check. Request the raw runs and configuration details for
review. These figures compare the stated per-row-signing composition; they
measure a different representation from 8DB's batch-signing design.

## Timing analysis

The published suite screens for input-dependent timing with positive and
negative controls. Its results belong to the named build, host, operation and
sample count.

| Run or path | Recorded reading |
|---|---|
| Windows x86_64, September 12, harness v2.1 | At 50,000 measurements per class, all gated readings were at or below 2.79, below the 4.5 threshold. |
| Linux aarch64, September 12, harness v2.1 | One ML-KEM case, S04-10, was inconclusive at 7.64. |
| Linux aarch64, September 14, `fips` | The compliance-profile S04-38 case was inconclusive at 5.61; the research-profile gated cases were at or below 3.52. |
| Linux x86_64, September 14, `fips`, 50,000 per class | S04-35 crossed the failure threshold at 15.71. This finding remains unresolved. |
| Linux x86_64, September 14, `fips`, 200,000 per class | The repeat was recorded as invalid because of its negative control. It does not resolve the preceding finding. |

The [timing guide](timing/README.md) provides the complete run history,
thresholds and public-seed attribution rule. Statistical screening gives
evidence at a measured sample size; constant-time proofs and broader
side-channel assessment require other methods. The gated ML-DSA signing tests
use the hedged variant. The production signature path described in this
package uses deterministic signing, which has a separate informational row.

## Builds and validation status

| Area | Status in this evidence package |
|---|---|
| Post-quantum parameter sets | ML-KEM-1024 (FIPS 203) and ML-DSA-87 (FIPS 204), the Category 5 parameter sets specified in CNSA 2.0. |
| Default record AEAD | AES-256-GCM-SIV (RFC 8452). This mode is outside the NIST-approved AES-GCM profile. |
| `fips` record AEAD | AES-256-GCM (SP 800-38D), with HMAC-SHA-384 key commitment. The profile name identifies a build configuration. |
| Cryptographic providers | The compliance path uses aws-lc-rs for AES, HKDF, HMAC and ML-KEM; ML-DSA-87 uses RustCrypto. Any underlying module certificate applies only to its specified version, configuration and boundary. |
| 8DB validation | No 8DB CAVP certificate or CMVP module validation is held for the implementation described here. ACVP demonstration results establish the tested algorithm behavior. |
| Transport | The described device-mesh path transfers sealed record objects. The TLS layer in this package uses a Category 3 hybrid group; transport qualification is assessed separately. |

The record-level descriptions apply to the paths covered by this package.
For deployment review, confirm the selected build, write and import paths,
managed artifacts, recovery behavior and key lifecycle.

[Request the evaluation materials](mailto:ashley@8braid.com?subject=8DB%20PQC%20evaluation)
or start with [How to verify](HOW-TO-VERIFY.md).
