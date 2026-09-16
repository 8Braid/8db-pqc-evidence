# Evaluate the evidence, then the workload

**Updated September 16, 2026, 13:32 UTC.** The public package supports immediate
review of cryptographic interoperability, record costs, live migration and
recovery. The latest integrated engine is frozen for qualification. Its results
will be recorded against that exact source before we mark its release complete.

## Public results you can check now

| Question | Published result | Reproduce the evidence |
|---|---|---|
| Do the implementations interoperate? | The recorded OpenSSL 3.5 campaigns passed 15 checks on each architecture. The public verifier checks artifact hashes, recorded ML-KEM secret agreement, and valid and modified ML-DSA signatures. | [Cryptographic verification instructions](HOW-TO-VERIFY.md#run-the-public-verifier) and [original interoperability records](interop/README.md). |
| What does PQC add to the encrypted record? | In the measured same-cipher comparison, a 115-byte record occupies 131 encrypted bytes under either key-establishment method. Key envelopes, signatures, proofs and database structures have separate costs. | [Same-cipher measurements](bench/p38/2026-09-14/) and [protection-profile measurements](bench/profile-cost/2026-09-14/). |
| Can a stored dataset change protection while scheduled reads continue? | Six retained 100,000-record migration runs reproduce four rejected outcomes and two accepted outcomes under the same auditor. The newest accepted run had zero missed deadlines among 689 nonexcluded observations. | [Offline Rust replay](migration/live-hkdf-2026-09-15/) and [four-CPU supplement](migration/live-hkdf-2026-09-15/four-cpu-2026-09-16/). |
| Can a killed replica recover the exact records? | Two physical hosts cold-finished with all 512 exact typed records. The killed receiver retained 172 and read new content 1,074 ms after same-store reopen. | [Recovery package and Rust verifier](mesh/required-two-host-2026-09-15/). |

Each package identifies its tested source, workload and acceptance conditions.
Artifact replay checks those retained observations. A fresh engine experiment
requires the evaluation engine and harnesses.

From the repository root, the three verification entry points are:

```sh
# Python 3.10+ and OpenSSL 3.5+: manifest and cryptographic examples
python verify-evidence.py

# Bash, Rust 1.94.1 and the documented native tools: all six migration outcomes
bash migration/live-hkdf-2026-09-15/verify-package.sh

# Rust/Cargo: exact content and recovery observations in the two-host package
cargo run --locked \
  --manifest-path mesh/required-two-host-2026-09-15/verify/Cargo.toml \
  --target-dir /tmp/8db-r9-verifier \
  -- mesh/required-two-host-2026-09-15
```

Use an external Cargo target directory, as shown, to keep build output outside
the evidence inventory. The migration replay must reproduce the original
rejections as well as the acceptances. A later passing run leaves earlier
deadline misses in the record; it does not establish their resolution.

## Latest engine qualification

The integrated source `9467eb1d2cb9aea2625f3e6b6f8d189d447c4172` completed
its AES-GCM release job: 67 named protected-node tests passed, generated-client
checks passed, and six local sender/receiver observations each matched all
2,048 expected records. An independent review rehashed the consumed originals
and replayed the recorded-content checks. The [dated result and provenance](mesh/protected-node-2026-09-16/RELEASE-CHECKPOINT-1332Z.md)
identify the exact source, executable, job and scope.

The next candidate is `326ab0a1333dd7f6f13d4c0fff503f9d5eefaa5e`, tree
`372602bc07b1233008184c55246d7b6702646ec3`. It corrects a test fixture whose
cold-read audit unnecessarily reserved a write context, advancing the authority
revision it was checking. The corrected fixture retains the initial write
reservation and explicitly checks the unchanged revision after cold reopen.
Production code and the original fault assertions are unchanged. Static review
passed; native qualification of this exact candidate remains open.

The earlier baseline at `fa4911581ba4dc901be5582730c4c2813d221c16`, tree
`d8b87230e9d57b20604f08d05a93ff7b89c36615`, has since completed its native
producer. Retained internal results include 742 default-profile native controls
and 810 AES-GCM-profile native controls, seven integration controls per profile,
and the Required-mesh controls and owned crash/recovery exercise. That complete
tree was merged as `b05f7ae35840b2d7f5116164f80664b8056f163c`. These are internal
execution records available for evaluation; this update adds no new public raw
test package and transfers none of those passes to the successor.

| Release gate | Current state | Evidence needed to close it |
|---|---|---|
| Integrated candidate | Source `326ab` selected after static review. Predecessor `9467` has a completed AES-GCM release job. | Completed native and generated-client tests bound to the corrected candidate source, binaries and execution records. |
| Evaluator delivery | Recipes prepared; final assembly and offline replay await the completed producer records. | Successful assembly, durable artifact publication, clean extraction and replay, followed by the required main-built and platform checks. |
| Temporal and delivery-worker behavior | The predecessor release passed its ordinary worker positive and receipt-observer controls; three injected clock faults were explicitly deferred in that job. The cold-audit fixture correction is ready for execution. | Actual positive and isolated fault runs on the retained corrected executable, with their clock and authority evidence. |
| Sustained operation | The 24-hour mission has not started. | A full 86,400-second run, independent receipt/content accounting, failure and cold-recovery observations, and a completed audit. |

The [earlier protected-node snapshot](mesh/protected-node-2026-09-16/QUALIFICATION.md)
preserves its source-specific architecture and schema limits. The latest
candidate does not establish every application schema or deployment topology
by composition alone.

## Findings carried forward

The [timing record](timing/README.md) still carries the unresolved historical
x86 compliance-profile failure and ARM inconclusive case. Cryptographic
correctness, timing assurance and operational qualification remain separate
evidence requirements. The [validation record](CLAIMS-AND-SCOPE.md#builds-and-validation-status)
also distinguishes Category 5 algorithm choices from CAVP certification and
CMVP module validation; 8DB holds neither certificate for this implementation.

For an engine evaluation, [contact Ashley](mailto:ashley@8braid.com?subject=8DB%20technical%20evaluation)
with the workload and failure condition you want to test. We can supply the
matching engine, harness and retained records, and agree on the result that
would answer your question.
