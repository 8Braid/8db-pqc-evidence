# 8DB: post-quantum protection for stored data

**Change stored-data protection while keeping records usable and verifiable.**

8DB is 8Braid's embedded database. It combines post-quantum key establishment,
record encryption and batch signatures with a way to migrate existing records
as cryptographic requirements change.

Applications still need to read retained records while their protection changes.
This package brings together the implementation tests, interoperability results
and measured costs behind 8DB, so your team can assess it for a workload of its
own.

The design keeps AES encryption on the record path and shares post-quantum
key-establishment and signature work across keys and batches. In the measured
same-cipher comparison, a 115-byte record occupies 131 encrypted bytes with
either classical or post-quantum-established keys. That is an exact
**zero-byte increment per record**. The key envelopes, signatures, proofs and
database structures still need their own storage and processing budget.

The harder result is operational: changing protection on records already held,
checking the result, recovering from interruption and enforcing current
authority when managed copies return. This package provides measured examples
and reproducible evidence for those operations. Their combination in an
embedded engine is the capability to evaluate; whole-database cost parity and
competitive uniqueness remain questions for matched tests.

[Review the results](CLAIMS-AND-SCOPE.md) ·
[Verify the evidence](HOW-TO-VERIFY.md) ·
[Request an evaluation](mailto:ashley@8braid.com?subject=8DB%20PQC%20evaluation)

**Evaluator update, September 16:** [current qualification status](QUALIFICATION-STATUS.md)
links the public replay packages, the newly frozen engine source and the
remaining release tests. Each result stays attached to the version that ran it.

For data moving between nodes, read the
[protected-node architecture](mesh/protected-node-2026-09-16/) and its
[dated qualification matrix](mesh/protected-node-2026-09-16/QUALIFICATION.md).
The reviewed composition seals messages for the receiving endpoint before
network transit, then protects the received content under local storage keys.
At the reviewed source, that Required profile admits canonical native WordNet
lemma sections; additional application schemas need integration and qualification.
Its per-frame cryptographic work has a separate cost from the at-rest
key-and-batch measurements below. Current integrated release qualification
remains in progress.

## Built for the data you already hold

Long-lived records can outlast the keys, algorithms and policies that first
protected them. Updating those records takes work inside the storage system.
8DB records the algorithm version with the ciphertext and supports old and new
generations side by side during a transition.

In a recorded managed-copy test on 100,000 stored records, 8DB re-encrypted the data from
HKDF-SHA256-derived page keys into a new generation using HKDF-SHA384, verified
each record and activated the new generation atomically. That run's concurrent
read probe recorded zero served reads.[^1]

The separate live-migration API now has a
[100,000-record run with verified scheduled reads](migration/live-hkdf-2026-09-15/).
On September 15, 2026, it met a one-second scheduled-to-response budget while
changing HKDF-SHA256 to HKDF-SHA384, including an owned process kill and cold
resume. The declared process outage and one qualifying interrupted request are
accounted for separately. All 877 scheduled requests have retained outcomes.
The package includes the unchanged independent Rust auditor, its 31 controls,
offline dependencies and the rejected predecessor run. See the
[dated migration scope](CLAIMS-AND-SCOPE.md#key-derivation-transition) for the
distinction from the earlier managed-copy operation.

**September 16 update:** the [four-CPU replay supplement](migration/live-hkdf-2026-09-15/four-cpu-2026-09-16/)
retains four newer 100,000-record runs. Every run cold-verified the exact content;
the unchanged auditor rejected three for missed deadlines (2/843, 26/882 and
9/724 nonexcluded observations) and accepted the latest (0/689). That final run
added diagnostics to the preceding product inputs. It establishes a measured
pass while leaving the earlier misses unresolved. All six original ledgers
remain independently replayable with the same Rust auditor.

This work addresses a practical part of the migration agenda. NIST's
[crypto-agility guidance](https://csrc.nist.gov/pubs/cswp/39/upd1/considerations-for-achieving-crypto-agility/final)
covers replacing algorithms and handling data already encrypted under them.
[OMB M-26-15](https://www.whitehouse.gov/wp-content/uploads/2026/06/M-26-15-Execution-of-the-Migration-to-Post-Quantum-Cryptography.pdf)
prioritizes re-encryption of long-lived sensitive data using PQC-protected keys
for US federal civilian agencies. 8DB brings the update and verification work
into the database.

## Results to start your review

| Question | Evidence |
|---|---|
| Do the algorithms produce the expected results? | ML-KEM-1024 key generation and encapsulation/decapsulation, and ML-DSA-87 signature verification passed NIST's ACVP demonstration-server tests. The offline suite passed all 35 checks across post-quantum and symmetric primitives.[^2] |
| Do the cryptographic outputs interoperate? | [OpenSSL 3.5 interoperability](interop/README.md): 15 of 15 checks passed on each of x86_64 and aarch64, covering both directions and negative controls. |
| What are the storage and processing costs? | The [September 14 same-cipher runs](bench/p38/2026-09-14/) report summaries of 2,000 timed calls per arm on x86_64 and ARM, with clock/load details: 131-byte ciphertexts from 115-byte records and encrypt/decrypt medians within 1%. A [separate protection-profile comparison](bench/profile-cost/2026-09-14/) measures commitment/framing costs. Earlier [release microbenchmarks](bench/) remain available.[^3] |
| Can a replica recover its exact protected records after a process kill? | [September 15 two-host recovery](mesh/required-two-host-2026-09-15/): both hosts finished with all 512 exact typed records; the receiver retained 172 and read new content 1,074 ms after same-store reopen. The package includes originals, explicit redaction provenance, negative history and a Rust artifact verifier. |
| Can stored-data protection change while scheduled reads continue through a crash and resume? | [September 15 live HKDF migration](migration/live-hkdf-2026-09-15/): the R6 100,000-record run passed the unchanged auditor with zero missed deadlines outside the declared exclusions. The original R5 rejection and an offline Rust replay package remain available. |
| How can I inspect the security evidence? | [Known-answer vectors](kat/), [test results](acvp/), and [timing reports](timing/README.md) include the inputs, methods and recorded outcomes needed for a technical review. |

## How 8DB protects records

8DB establishes keys with ML-KEM-1024, encrypts records with AES-256, and uses
ML-DSA-87 to authenticate batches. ML-KEM-1024 and ML-DSA-87 are the Category 5
parameter sets specified in CNSA 2.0.[^4]

A signature over a SHA-384 Merkle root, together with an inclusion proof,
lets a reviewer check a record's membership in the signed batch. Storing the
signature once per batch reduces the signature storage required by a design
that signs every row separately. Key-establishment objects are also shared
across the records protected by that key.

The [measurement guide](CLAIMS-AND-SCOPE.md#storage-and-processing-costs)
separates those costs and the remaining whole-database comparison, so you can
assess the design against your record sizes, batch sizes and access patterns.

## Evaluate 8DB for your workload

Bring one representative dataset and the operating constraint that matters
most: read availability, migration time, recovery, or storage cost. We can use
those to agree on a test and the evidence your team needs to assess it.

[Contact Ashley Dunfield at 8Braid](mailto:ashley@8braid.com?subject=8DB%20PQC%20evaluation)
to discuss a workload evaluation. For an implementation review, request the
engine and test harnesses.

To check the published files with Python 3.10+ and OpenSSL 3.5+, run:

```bash
python verify-evidence.py
```

The verifier checks the published artifact hashes, matching ML-KEM shared
secrets, and valid and modified ML-DSA signatures on both recorded architectures.
Follow [How to verify](HOW-TO-VERIFY.md) for setup and result interpretation.
[Publication provenance](REDACTIONS.md) records identifier redactions and
editorial changes separately from test data.

[^1]: The September 14, 2026 Linux x86_64 run changed page-key derivation from HKDF-SHA256 to HKDF-SHA384. Full run records are available on request; re-execution requires the evaluation engine. See [migration scope](CLAIMS-AND-SCOPE.md#migration).
[^2]: ACVP session 744723, June 16, 2026, used the demonstration service with `isSample = false`. 8DB has no CAVP certificate or CMVP module validation. The [35-check offline report](acvp/Offline-KAT-Report-35of35.txt) covers NIST and RFC vectors across ML-KEM, ML-DSA, HKDF, AES and HMAC.
[^3]: This is the per-record AEAD result. Key establishment, batch signatures, inclusion proofs, framing and indexes have separate costs. [Build-specific measurements](CLAIMS-AND-SCOPE.md#storage-and-processing-costs) provide the context for reuse.
[^4]: The default build uses AES-256-GCM-SIV; the `fips` build uses AES-256-GCM. Algorithm selection, module validation and deployment qualification are separate assessments. See [builds and validation status](CLAIMS-AND-SCOPE.md#builds-and-validation-status) and the [timing review](CLAIMS-AND-SCOPE.md#timing-analysis), including its unresolved compliance-profile findings.
