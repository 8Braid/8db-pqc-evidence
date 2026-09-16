# R6: original availability acceptance

The September 15, 2026 R6 run passed the original fixed schedule and independent auditor. All 877 scheduled requests have recorded outcomes. The audit separately accounts for 41 requests scheduled during the deliberate process outage and one qualifying interrupted in-flight request. All 835 nonexcluded deadline observations met the one-second scheduled-to-response budget. Its stable-phase success counters sum to 833; these are a separate field from deadline observations.

The engine completed HKDF-SHA256 to HKDF-SHA384 replacement with AES-256-GCM-SIV unchanged. It cold-reopened and checked all 100,000 exact typed records and the lexical index. Final dispositions were 99,998 copied, one superseded write, one superseded deletion and zero failed attempts. The producer, independent auditor and final custody checks each exited 0. The closed chain finished at 2026-09-15 23:36:44 UTC.

The [original audit report](original-audit-report.json), [original chain result](original-chain-result.json), [pre-copy oracle custody](oracle-prerun-custody.json) and nine files in [evidence](evidence/) retain their original bytes. The oracle was pinned by the supervisor before copying started. [CUSTODY-PINS.json](../CUSTODY-PINS.json) identifies the exact engine source, executable, oracle, auditor and closed corpus manifest.

Run `bash verify-package.sh` from the [parent directory](../). It checks the same 31 auditor controls and replays this run alongside the [R5 rejection](../r5/). Public artifact replay and engine re-execution are distinct; the [parent README](../) explains their scopes.
