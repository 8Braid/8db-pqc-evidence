# 100,000-record migration with scheduled reads, a process kill and cold resume

**8DB met its preregistered one-second scheduled-read deadline while changing page-key derivation in a 100,000-record store and recovering from a process kill.** The declared process outage and one qualifying interrupted request are accounted for separately. The September 15, 2026 R6 run passed the unchanged independent Rust auditor; its original R5 predecessor remains a recorded rejection.

The run changed HKDF-SHA256 to HKDF-SHA384 while keeping AES-256-GCM-SIV unchanged. Applications read through copying, index construction, verification and active serving. A changed record, an added record and a deletion exercised the live facade. The owner was killed after 50,048 source dispositions and resumed in a new process. The final engine check cold-reopened the store and compared all 100,000 exact typed records and the lexical index.

This provides a concrete experiment for stored-data crypto agility: changing protection, accounting for completion and checking what applications can read throughout the operation.[1] The two-requests-per-second schedule and one-second response budget are our preregistered application criteria.

## Original results

| Measurement | [R5](r5/) | [R6](r6/) |
|---|---|---|
| Independent availability verdict | Rejected | Accepted |
| Scheduled requests / recorded outcomes | 944 / 944 | 877 / 877 |
| Scheduled deadline misses | 16, including one expired before service | 0 |
| Scheduled during deliberate outage | 39 | 41 |
| Qualifying interrupted in-flight requests | 1 | 1 |
| Nonexcluded deadline observations | 904 | 835 |
| Qualifying stable-phase successes | See original audit report | 833 |
| Exact cold typed/index result reported by engine | 100,000 records, true | 100,000 records, true |
| Copied / superseded write / superseded delete / failed attempts | 99,998 / 1 / 1 / 0 | 99,998 / 1 / 1 / 0 |
| Producer / auditor / final custody exit | 101 / 1 / 0 | 0 / 0 / 0 |

The deadline-observation count and stable-phase success count are different fields in the audit. All scheduled requests have retained outcomes. The R6 audit also confirmed a real protected response callback overlapping the owner's activation attempt, with its 25 ms hold included in the ordinary response deadline. It recorded zero expected retired-source refusals.

R5 completed its transformation and cold checks but missed the read-availability requirement. Both runs retain their original oracle, observations, counter result and audit report. Compare the [exact custody pins](CUSTODY-PINS.json) before interpreting a difference. These are one measured run per implementation on the recorded host, not a general throughput comparison.

## Replay offline with Rust

The package includes the unchanged Rust auditor, all 31 positive and negative controls and its locked public dependencies. With Rust 1.94.1 already installed through rustup, Bash and GNU core utilities, run from this directory:

```sh
bash verify-package.sh
```

No network access or 8DB engine is required. The script checks every package file, runs all 31 controls and independently replays both ledgers. It places builds and reports in a new temporary directory outside the evidence checkout and prints that location.

Package verification exits **0** when the files, controls and original verdicts match: R5 rejected and R6 accepted. It exits **1** for a mismatch or failed check, and **2** when required files or tools are missing. An incomplete attempt is never reported as a pass. The underlying auditor preserves its own verdict: exit 1 for R5, exit 0 for R6. Read its JSON reports and compare them with the byte-identical [R5 original](r5/original-audit-report.json) and [R6 original](r6/original-audit-report.json).

For a direct audit, use the binary built by the script and the three original external pins:

```sh
/path/to/replay-output/target/debug/p11-availability-audit r6/evidence \
  dc16933f75a04fe3af6baaad3a4dcbe4454a9ef485156eadd79395ef363faee7 \
  2d92663bb69608ecc911d9f2a08bf807775982e4b3f5a8360202df1a870a753b \
  6fada689973c19dc9ed7772f9f96ba2c9c67c3902802dfc4cd05b79671770135
```

The [repository verification guide](https://github.com/8Braid/8db-pqc-evidence/blob/1a9b750013e03e10d9fb3f60708216d1b61a3c0f/HOW-TO-VERIFY.md) separately checks the recorded cryptographic artifacts and OpenSSL interoperability. Its global raw-file manifest and this package's complete `SHA256SUMS` have complementary coverage.

## What the replay establishes

The auditor includes dispatch delay in latency. It checks response content and endpoint identity, source/executable joins, observed authority-floor continuity and protected emission overlapping activation. It counts the observed process outage separately. A response whose deadline had already expired before the kill stays late. Returned content is checked even during the outage. Expected retired-source refusals cannot count as successful reads.

The 31 controls challenge missing and duplicated outcomes, concealed dispatch delay, wrong content, authority regression, fabricated overlap and an oracle rewritten after the run. The pre-copy oracle receipts remain outside the observation directory. No acceptance logic or threshold was changed for this publication.

Public replay checks the retained record of cold validation. Reopening and validating an actual protected store requires the evaluation engine and harness. Engine source and executable hashes identify the vendor-run implementation; matching those identities is distinct from rebuilding the engine or independently attesting the host. Publication provenance records every original/public hash and retains the inputs byte-identically.

This run uses a synthetic Wordnet lexical workload on Linux x86_64, with 16 guest logical processors and an AMD Ryzen 9 9950X3D reported CPU. The [recorded host context](r6/HOST-SCOPE.json) identifies a shared workstation rather than an isolated benchmark host. Separate engine tests cover additional authorization and restoration cases; this page's availability result comes from the full campaign and independent audit. The broader cryptographic algorithm and module validation status remains in [claims and scope](https://github.com/8Braid/8db-pqc-evidence/blob/1a9b750013e03e10d9fb3f60708216d1b61a3c0f/CLAIMS-AND-SCOPE.md).

Which additional acceptance condition would make this lifecycle example useful for your evaluation? We can provide the engine and harness for a fresh run against an agreed workload. Contact [Ashley Dunfield](mailto:ashley@8braid.com?subject=8DB%20stored-data%20migration%20evaluation).

[1] NIST, [Considerations for Achieving Crypto Agility, updated June 29, 2026](https://nvlpubs.nist.gov/nistpubs/CSWP/NIST.CSWP.39-upd1.pdf), addresses replacing algorithms protecting stored data and maintaining operation during change. The application deadline here is not a NIST latency standard or certification result.
