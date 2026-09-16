# R5: original availability rejection

The September 15, 2026 R5 run completed the 100,000-record HKDF-SHA256 to HKDF-SHA384 transformation and exact cold typed/index check. Its original independent audit rejected read availability: 16 scheduled deadlines were missed, including one request that expired before service. The producer exited 101, auditor exited 1 and final custody checks exited 0.

All 944 scheduled requests have retained outcomes. The audit separately accounts for 39 requests scheduled during the deliberate process outage and one qualifying interrupted in-flight request. Final dispositions were 99,998 copied, one superseded write, one superseded deletion and zero failed attempts. These positives remain part of the result; they do not change its rejected availability verdict.

The [original audit report](original-audit-report.json), [original chain result](original-chain-result.json), [pre-copy oracle custody](oracle-prerun-custody.json) and nine files in [evidence](evidence/) are byte-identical copies of the closed run. The full original corpus-manifest digest and source/executable pins appear in [CUSTODY-PINS.json](../CUSTODY-PINS.json). The private engine store and diagnostic archive remain separate evaluation artifacts.

Run `bash verify-package.sh` from the [parent directory](../) to reproduce both the R5 rejection and R6 acceptance with the same auditor. A matching rejection is the correct replay of this run. The later result has its own source and evidence and does not revise R5.
