# 8DB results and measurement scope

Use this guide to assess 8DB's cryptographic tests, storage costs and migration
behavior. Each section identifies the evidence available in this repository
and the additional material available with an evaluation.

For node-to-node protection, use the
[September 16 architecture note](mesh/protected-node-2026-09-16/) and
[source/profile qualification matrix](mesh/protected-node-2026-09-16/QUALIFICATION.md).
They distinguish message encryption, local storage, recovery and the current
release gates.

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
| Commitment-profile buffer size | 185 bytes for a 115-byte input, versus 136 bytes through the research-profile wrapper. | The [separate provider comparison](bench/profile-cost/2026-09-14/) measures AES-256-GCM with key commitment and framing versus AES-256-GCM-SIV. These are returned buffers from a different interface than the 131-byte same-AEAD row, not full database sizes. |
| Measured KEM/wrapped-key subtotal | 1,616 bytes. | A 1,568-byte ML-KEM-1024 ciphertext plus a 48-byte wrapped key, per established key. Other envelope fields and serialization are outside this subtotal. |
| ML-DSA-87 signature | 4,627 bytes. | Signature bytes alone. The stored batch representation, inclusion proofs and other framing have additional costs. |

At 100,000 records of 131 bytes each, the 1,616-byte KEM/wrapped-key subtotal is
0.0123% of the encrypted record bytes. This ratio measures those two fields alone;
use the full representation when budgeting storage.

### What the zero-byte result makes possible

Post-quantum key establishment can protect the keys used by the same compact
symmetric record cipher. The measured 131-byte ciphertext therefore need not
grow when the key-establishment method changes. Sharing a batch signature also
avoids attaching a large signature to every row. These choices keep the
post-quantum work at key and batch boundaries; they do not remove that work.

The adoption question is whether the complete engine can deliver the needed
protection, migration and recovery behavior within a workload's storage and
latency budget. The [live migration evidence](migration/live-hkdf-2026-09-15/four-cpu-2026-09-16/)
addresses that operation's correctness and scheduled-read deadline. It is a
separate result from the same-cipher size and timing comparisons.

### What a whole-database comparison still needs

A claim of no additional database storage or performance cost requires a matched
baseline. Use the same engine, plaintext, record cipher, indexes, compression,
durability, authorization, hardware and offered load. Change key establishment
and batch authentication in separately identified arms. Count complete
serialized envelopes, signatures, proofs, metadata, indexes, logs, retained
copies and migration's peak space. Report key setup, signing and full-data
verification separately from steady-state reads and writes.

Run repeated, alternated arms with retained request samples, scheduled-to-response
latency, p50/p95/p99, deadline misses, throughput, CPU, memory and storage I/O.
Set the acceptable performance difference before execution and report its
uncertainty. Equal rounded medians do not establish a zero penalty. A comparison
with another database additionally needs equivalent protection and persistence
guarantees, including an efficient batch-signing baseline.
[Envelope encryption](https://docs.cloud.google.com/kms/docs/envelope-encryption)
and [signed Merkle trees with inclusion proofs](https://www.rfc-editor.org/rfc/rfc9162.html)
are established techniques. 8DB's integrated lifecycle and its measured
operating behavior are the capabilities to compare.

**Status, September 16, 2026:** exact per-record ciphertext-size parity is
established for the recorded same-AEAD construction. Full delivered-profile and
whole-database cost parity remain open; the AES-GCM compliance-profile buffer
and its commitment work are measured separately below. These open measurements
do not change the achieved migration and recovery results.

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

The [September 14 reruns](bench/p38/2026-09-14/) add 2,000 samples per arm,
p95 readings and clock-resolution checks on WSL2 x86_64 and Linux aarch64.
Encrypt medians are 451/451 ns and 1,172/1,171 ns (classical/PQC); decrypt
medians are 441/441 ns and 1,188/1,188 ns. The measured minimum nonzero clock
steps are 10 ns and 33 ns. These observations support close component timings;
sub-clock differences do not establish a speed advantage, and retained summary
statistics alone do not establish statistical equivalence.

The [September 14 commitment-profile comparison](bench/profile-cost/2026-09-14/)
records GCM encrypt/decrypt medians of 1,637/1,674 ns versus SIV's 584/574 ns
on x86_64, and 1,661/1,654 ns versus 1,021/980 ns on aarch64. Each uses 200
timed operations with a fixed key. This comparison changes both the cipher and
key-commitment behavior, so it measures those profiles rather than an isolated
PQC penalty. It still does not measure the delivered store's complete
authorization, derivation, index, ingestion and recovery work.

These are operation-level microbenchmarks. The files also contain comparisons
with per-row signing and a decrypt-and-scan search baseline. Those configurations
answer specific design questions; workload-level database comparisons require
matched durability, indexing, concurrency and protection settings.

## Migration

### Two-host recovery, September 15, 2026

The [Required-profile recovery package](mesh/required-two-host-2026-09-15/)
records a real process kill and same-store reopen across two physical
workstations. Both hosts finished with 512 exact typed records. The receiver
retained 172 records and observed new readable content in 1,074 ms, within the
preregistered 30,000 ms limit. Both final commit/applied watermarks were 513.
The package identifies the exact source, executable, profile, workload,
independent expected records, redactions and earlier failed attempts.

This is a fixed-corpus crash-and-rejoin qualification of current admitted native
copies. The Rust artifact reader checks its published record; fresh execution,
endurance, physical power loss and historical all-copy coverage are separate
evaluations. This recovery result does not establish read availability during
a key-derivation migration.

### Key-derivation transition

**Scope correction, September 15, 2026:** the historical 100,000-record
Linux x86_64 run from September 14 used the managed-copy operation. It
re-encrypted stored records from page keys derived with HKDF-SHA256 into a new
generation using HKDF-SHA384, verified each record and activated the new
generation atomically. Its concurrent-read probe recorded zero served reads.
The measured result establishes the key-derivation change, re-encryption and
record verification. Read availability requires evidence from the separate
live-migration operation.

**Measured live result, September 15, 2026:** the separate live-migration API
passed a 100,000-record HKDF-SHA256 to HKDF-SHA384 campaign with scheduled reads,
an owned process kill, cold resume and a protected response callback overlapping
activation. AES-256-GCM-SIV remained unchanged. The [complete public evidence](migration/live-hkdf-2026-09-15/)
retains 877 issued requests and outcomes, 41 requests scheduled during the
declared process outage and one qualifying interrupted in-flight request. All
835 nonexcluded deadline observations met the fixed one-second
scheduled-to-response budget. The application target was preregistered at one
request every 500 ms with two workers; it is not a NIST latency requirement.

The engine cold-reopened and verified 100,000 exact typed records and the
lexical index. Its final dispositions were 99,998 copied, one superseded write,
one superseded deletion and zero failed attempts. The independent Rust auditor
and all 31 controls are public, with locked dependencies for offline replay.
The predecessor R5 run remains rejected for 16 deadline misses. Matching public
reports checks artifact consistency; fresh engine execution requires evaluation
access. This measured result covers the stated synthetic lexical workload and
does not establish every workload, modality or backup policy.

**Four-CPU update, September 16, 2026:** the [additional original ledgers](migration/live-hkdf-2026-09-15/four-cpu-2026-09-16/)
cover four fresh packaged-engine runs under a four-CPU quota and 16 GiB limit.
The fixed one-second budget and two-request-per-second schedule remained
unchanged. The auditor rejected runs with 2/843, 26/882 and 9/724 missed
nonexcluded deadlines, then accepted a run with 0/689. Every run cold-verified
100,000 exact typed records and the lexical index. The latest source added
diagnostic capture to the preceding product inputs; its pass does not resolve
the cause of earlier misses or establish repeatability. All four original
reports, inputs and source/executable bindings are public, alongside the
earlier R5/R6 history.

The historical raw results and both engine harnesses are available to evaluators
on request. Additional transitions and production operating limits should be
agreed and tested against the intended deployment.

### Composed signatures and database allocation

**Interpretation correction, September 16, 2026:** the internal record contains
two different experiments. The July 19 application-composed runs used encryption
and signatures. The September 14 allocation runs used signature-sized random
fields without encryption or actual signing. They cannot be combined into a
single cryptographic-performance result.

| Experiment, 100,000 records | PostgreSQL allocation | MongoDB collection allocation |
|---|---:|---:|
| July 19, per-record signatures | 579,452,928 bytes | 495,194,112 bytes |
| July 19, one batch signature | 24,477,696 bytes | 19,369,984 bytes |
| September 14, random signature-sized fields | 572,801,024 bytes | 500,445,184 bytes |

The July scripts do not establish a matched whole-database Category 5
comparison. They contain no ML-KEM or node replication. The PostgreSQL recipe
does not override pgcrypto's default cipher options, so its AES-256 label is
unsupported. The verification loops reconstruct fixture values and check
in-memory signatures rather than rereading and decrypting the persisted
database. MongoDB's batched encryption preparation and fsync also sit outside
its timed write region, and its collection allocation excludes indexes, key
vault and the separate batch-signature collection.

The September experiment measures actual database allocation for random bytes.
Its proximity to July sizes does not reproduce July's cryptographic work or
verification semantics. The archived July output does not independently pin
its historical 8DB source commit; inspecting current code cannot repair that
source-identity gap. Original records are retained and available to evaluators
on request.

Both competitors' July batch configurations materially reduce signature
storage. Per-row signing is one design choice, not a mandatory cost of PQC.
A decisive comparison needs competent batch baselines, matched protection and
durability, complete allocation accounting, and cold full-data verification as
specified above.

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
| Cryptographic providers | In the Required mesh composition reviewed at `fa491158…`, ML-KEM-1024 and ML-DSA-87 use RustCrypto; the selected symmetric operations use AWS-LC. Any underlying module certificate applies only to its specified version, configuration and boundary. See the [exact-source provider and custody scope](mesh/protected-node-2026-09-16/README.md#keys-providers-and-custody). |
| 8DB validation | No 8DB CAVP certificate or CMVP module validation is held for the implementation described here. ACVP demonstration results establish the tested algorithm behavior. |
| Transport | The reviewed Required mesh path seals typed endpoint content in a recipient-specific Category 5 message construction; the receiver opens it and reseals under local storage keys. It does not forward the sender's unchanged on-disk ciphertext. The separate hybrid TLS group's Category 3 designation does not describe this inner message layer. See the [source/profile matrix](mesh/protected-node-2026-09-16/QUALIFICATION.md). |

The record-level descriptions apply to the paths covered by this package.
For deployment review, confirm the selected build, write and import paths,
managed artifacts, recovery behavior and key lifecycle.

[Request the evaluation materials](mailto:ashley@8braid.com?subject=8DB%20PQC%20evaluation)
or start with [How to verify](HOW-TO-VERIFY.md).
