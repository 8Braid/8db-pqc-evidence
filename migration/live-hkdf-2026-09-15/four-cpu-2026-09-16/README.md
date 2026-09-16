# Four-CPU migration: exact recovery in four runs, three deadline rejections

**A migration can recover every record and still fail its application's availability requirement.** These four fresh 100,000-record runs make that distinction testable. Every run cold-reopened and verified the exact typed records and lexical index. The unchanged Rust auditor rejected three runs for missed one-second scheduled-read deadlines and accepted the fourth.[1]

All four ran the packaged 8DB engine under a four-CPU quota and 16 GiB memory limit, without network access, engine source or a compiler on the runtime PATH. The workload changed page-key derivation from HKDF-SHA256 to HKDF-SHA384, retained AES-256-GCM-SIV, and included a process kill, cold resume, concurrent reads, a changed record, an addition and a deletion. The fixed schedule remained two requests per second with two workers and a one-second scheduled-to-response budget.[2]

## Recorded results, September 16, 2026 UTC

| Run and engine source | Auditor verdict | Issued / retained outcomes | Nonexcluded deadline observations | Deadline misses | Expired before service, included in misses | Maximum scheduled-to-outcome latency |
|---|---|---:|---:|---:|---:|---:|
| [R6 offline, `206f1a6`](r6-offline-206f/original-audit-report.json) | Rejected | 884 / 884 | 843 | 2 | 0 | 1,038.203 ms |
| [Completion batching, `0280898`](completion-028/original-audit-report.json) | Rejected | 922 / 922 | 882 | 26 | 8 | 3,398.321 ms |
| [Copy batching, `41ca1c8`](copy-41ca/original-audit-report.json) | Rejected | 759 / 759 | 724 | 9 | 3 | 1,752.428 ms |
| [Diagnostic capture, `9c5ae6c`](diagnostic-9c5/original-audit-report.json) | Accepted | 728 / 728 | 689 | 0 | 0 | 867.176 ms |

The denominator subtracts only requests scheduled during the declared process outage and the qualifying interrupted in-flight request. Those counts were respectively 40 + 1, 39 + 1, 34 + 1 and 38 + 1. Expired requests remain failures. A failure can have several diagnostic labels; the miss column counts unique scheduled requests.

The accepted run's 689 observations had a nearest-rank p95 of 629.668 ms and p99 of 739.035 ms. Its slowest response had 132.824 ms of margin against the fixed deadline. Of its responses, 688 stayed wholly within one recorded phase and one crossed a phase boundary. All 689 were on time.

Every run recorded **100,000 authenticated, indexed and exact typed records after cold reopen, with zero cold-validation errors**. The final copy accounting was 99,998 copied, one superseded write, one superseded deletion and zero failed attempts. These content results remain useful when the separate availability verdict is rejected.[3]

## What changed between runs

The `0280898` engine used the existing native batch-write operation during completion. The `41ca1c8` engine also used it during copy, preserving the 128-source-candidate and 512 KiB ciphertext limits, entity metadata and authority ordering. Entity metadata adds memory beyond the ciphertext limit. These changes exercised existing storage machinery under the same acceptance contract.

The final `9c5ae6c` run added bounded owner-stage and parent-ledger diagnostics. Its [tracked product-input comparison](diagnostic-9c5/tracked-product-equivalence.json) records that all tracked inputs outside five diagnostic harness files and two documentation notes matched `41ca1c8`. This establishes the scope of the change, while the separate source and executable hashes identify the actual run.

The last pass does **not establish that the earlier nine misses were fixed**. These are four individual observations on a shared workstation, with different source revisions or instrumentation and uncontrolled cache and host scheduling conditions. The resource limit is a CPU quota, rather than a claim of four dedicated cores. The record supports the accepted run and preserves the failed ones; repeatability across the intended deployment remains the next performance question.

## Replay all verdicts with the same Rust auditor

From the parent package directory, with the [documented Rust 1.94.1 prerequisites](../README.md#replay-offline-with-rust) installed:

```sh
bash verify-package.sh
```

The command runs the same 31 controls, then replays the original R5/R6 ledgers and these four additional ledgers. It requires each report to match its original bytes and each verdict to retain its original exit code. Package verification succeeds when all six recorded outcomes reproduce, including the four rejected runs. The unchanged auditor continues to return exit 1 for each rejection.

Each run contains the same nine auditor-input types as the earlier package, including the expected typed-record fixture and the oracle captured before copying. [Original/public provenance](ORIGINAL-PROVENANCE.json) maps every copied file byte-for-byte; [custody pins](CUSTODY-PINS.json) identify the engine source, executable, oracle and closed original corpus. Per-run `HOST-SCOPE.json` files select the recorded limits and terminal state. Original runtime-isolation attestations and cold/counter results are also retained.

Replay verifies the published observations and recorded cold result. A fresh engine run and cold reopen require the evaluation engine and harness. Use these ledgers to inspect the acceptance conditions before choosing a deployment test.

[1] Original independent audit reports are linked in the table. Their `availability_latency_ns` arrays contain the stated nonexcluded populations. Percentiles use the sorted observations at rank `ceil(p × n)`; maxima include retained failures rather than replacing them with successful-response times. All report bytes and verifier source are covered by the parent `SHA256SUMS`.

[2] The four original `evidence/schedule-contract.json` files retain the unchanged 500 ms period, 1,000 ms budget, two workers and queue size four per worker. These are preregistered application criteria. The parent package explains their relationship to the stored-data migration example.

[3] See each run's `cold-validation.json`, `counter-validation.json` and `evidence/operation-result.json`. These are original engine observations, checked for consistency by public replay. The operation changed the derivation used for page keys; its result alone does not establish replacement of every cryptographic algorithm or qualification of every data modality.
