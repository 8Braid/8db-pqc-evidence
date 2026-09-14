# Review and reproduce the evidence

Use this guide to check the published results and plan an evaluation of 8DB on
your own workload. The public files support a document review, vector inspection
and independent signature verification. Re-running 8DB requires access to the
evaluation engine and its harnesses.

## Run the public verifier

With Python 3.10+ and OpenSSL 3.5+ on your PATH, run this from the repository:

```bash
python verify-evidence.py
```

If OpenSSL is installed elsewhere, specify its executable:

```bash
python verify-evidence.py --openssl /path/to/openssl
```

The [verifier](verify-evidence.py) checks all 249 raw artifacts against the
[published manifest](ARTIFACT-SHA256SUMS), compares four recorded ML-KEM
shared-secret pairs, and checks valid and modified 8DB ML-DSA signatures with
OpenSSL in both run folders. It reads the files without changing them.

Exit codes are 0 when all checks pass, 1 for a detected artifact or check
failure, and 2 when verification is incomplete, such as when OpenSSL is
unavailable. A successful run establishes agreement with the published
artifact manifest and the checked interoperability examples. Re-executing the
8DB engine, its known-answer suite or its migration remains a separate step.

Use a clean checkout: extra files in the raw-artifact formats are reported as
unlisted. The script was tested with Python 3.12 and OpenSSL 3.5.7, including
corrupted files, a missing file, unsafe manifest paths and a signature that
fails cryptographic verification despite matching an updated hash.

## Start with the question you need to answer

| Review question | Evidence to read | Access needed to repeat the measurement |
|---|---|---|
| Do the cryptographic implementations agree with known answers? | [Offline correctness report](acvp/Offline-KAT-Report-35of35.txt), [vectors](kat/) | 8DB evaluation engine to re-run its suite; public vectors can be checked with a compatible independent implementation |
| Does 8DB exchange valid keys, ciphertexts and signatures with OpenSSL? | [Interop guide](interop/README.md) and the two run folders | OpenSSL 3.5 can verify the published signatures; a fresh run in both directions needs the evaluation engine |
| What are the storage and processing costs? | [Benchmarks](bench/), [claims and scope](CLAIMS-AND-SCOPE.md) | Evaluation engine and benchmark harness |
| Can a stored dataset change algorithms while reads continue? | The transition results in [claims and scope](CLAIMS-AND-SCOPE.md) | Evaluation engine, transition harness and detailed run records, available on request |
| What does the timing screen show? | [Timing guide](timing/README.md), then the relevant `report.txt` and `host.txt` | Evaluation engine and timing harness on the target host |

## Check the published files

Read [REDACTIONS.md](REDACTIONS.md) before checking hashes. Some public copies
have identifiers removed, and editorial documents have been revised. The
original run manifests remain part of the record; reconcile any mismatch
with the publication history.

For algorithm correctness, the recorded offline suite passed **35 of 35 checks**
across ML-KEM, ML-DSA, HKDF, AES-GCM, AES-GCM-SIV and HMAC using NIST and RFC
vectors. The separate ML-DSA signature-generation and signature-verification
fixtures have their source, interface parameters and expected verdicts documented
in the [vector provenance](kat/ml-dsa-87/ml_dsa_87_nist_sig_SOURCE.md).
Match those parameters when using an independent implementation, especially the
internal signature interface and deterministic randomness.

The [demonstration-server response](acvp/NIST-ACVP-Disposition-2026-06-16.json)
and [session transcript](acvp/Live-Session-Log-744723.txt) record a separate
ACVP demonstration-server session. Certification status is addressed in
[claims and scope](CLAIMS-AND-SCOPE.md).

For interoperability, follow the OpenSSL commands in the [interop guide](interop/README.md).
Public keys, messages and signatures are sufficient for those verification
checks. The original runs' private keys are withheld, so re-decapsulation of the
published ciphertexts requires access beyond this checkout.

## Read costs in context

The release benchmark headers identify the host, date, sample size, command and
build. Keep those details with any latency you quote. The 131-byte and 185-byte
record sizes and the 1,616-byte key-establishment object describe 8DB's measured
formats. The key-establishment object includes a 1,568-byte ML-KEM-1024
ciphertext and a 48-byte wrapped key. ML-DSA-87's 4,627-byte signature size is
defined by FIPS 204. Compare these costs at the same storage or operation boundary.

Detailed transition and comparison run records are available to evaluators on
request. The public claim table gives the recorded results and their scope.

## Read timing results by profile and host

The timing screen uses pre-registered thresholds: a maximum absolute Welch t
value at or below 4.5 means "no leak detected at this sample size"; above 10
means "leak detected"; the interval between them is inconclusive. Controls and
any invalidating conditions determine whether a run can support a conclusion.

The September 14 compliance-profile result on x86_64 remains **refuted** for
S04-35. Its first repeat was invalid and leaves that finding unresolved. On
aarch64, compliance case S04-38 remains **inconclusive**. Read the current
interpretation and complete history in the [timing guide](timing/README.md)
before citing an earlier passing run.

## Arrange an evaluation

Contact [Ashley Dunfield at 8Braid](mailto:ashley@8braid.com) with your target
platform, data volume and the decision this review needs to support. Useful
starting points include migration while reads continue, storage overhead,
OpenSSL interoperability and review of an unresolved timing case.

The [evaluator kit](acvp/EVALUATOR-KIT.md) describes the integrity checks and
the [evidence harness](acvp/HARNESS-README.md) describes new measurement runs.
Their scripts, source registry and engine are supplied through evaluation access.
