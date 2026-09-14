# Evaluator kit: verify this checkout's PQC evidence without trusting its authors

The kit answers three questions an evaluator can ask of this repository on any
host, offline:

1. Is every file an evidence set says it carries present and unaltered?
2. Does every recorded timing run still say what the registry in
   `src/cam/pqc_timing_metrics.rs` says it says? The per-case |t| values are
   re-read from the set's `report.txt`, the gated maximum and the two controls
   are re-derived with the pre-registered S04-12 attribution rule, and the
   tri-state reading is recomputed with the registry's own `aggregate_label`
   and compared with the recorded row. Numbers cannot be edited in one place.
3. Do the algorithm-correctness vectors still pass on this host (35 of 35
   offline NIST ACVP known answers for ML-KEM-1024 and ML-DSA-87)?

It does not measure anything new unless asked (`--bench`, `--timing`,
`--interop` re-run the underlying harnesses and write their own folders). It is
not validation or certification. Its outcomes are:

| Outcome | Meaning |
|---|---|
| PASS | Present, unaltered, in agreement with the record |
| FAIL | A checksum mismatch, a registry disagreement, a FAIL line, an unparseable report |
| INCONCLUSIVE | Something needed for a verdict is missing: no `SHA256SUMS`, a listed file absent, a timing directory with no registry row, a hook script not present. Never upgraded to PASS |

The summary is derived from the per-check lines by the tool. There is no field
to type a verdict into.

## Run it

```bash
scripts/pqc_evaluator_kit.sh                 # default: host facts, evidence tree, ceilings, KATs, lifecycle hook
scripts/pqc_evaluator_kit.sh --self-test     # also prove the kit reports INCONCLUSIVE and FAIL
scripts/pqc_evaluator_kit.sh --check-only    # host facts, evidence tree, ceilings and the matrix citations (minutes; what CI runs)
scripts/pqc_modality_matrix_check.sh --self-test   # P24: the matrix cites code that exists; prove FAIL and INCONCLUSIVE
scripts/pqc_evaluator_kit.sh --interop       # also re-run the OpenSSL 3.5 interop (needs OpenSSL 3.5 or docker)
scripts/pqc_evaluator_kit.sh --bench         # also re-run the release bench (long compile first time)
scripts/pqc_evaluator_kit.sh --timing        # also record a new timing run (never overwrites a recorded one)
scripts\pqc_evaluator_kit.ps1 --self-test    # Windows: the same script through Git Bash, same exit codes
```

Or the checker alone:

```bash
cargo run --release --bin pqc_evidence_check -- --root .
cargo run --release --bin pqc_evidence_check -- --set docs/security/acvp-evidence/timing/aarch64-linux_2026-09-12_run3-harness-v2.1
cargo run --release --bin pqc_evidence_check -- --root . --ceilings   # P7A: 95 percent effect-size ceilings per timing set
```

In CI the x86_64 job of `pqc-interop.yml` runs `--check-only --self-test` on
every change to the evidence tree, the kit or the registry, and fails only on a
FAIL line (its overall summary is INCONCLUSIVE by design while the lifecycle hook
is absent and the KAT step is skipped).

Exit codes: 0 PASS, 1 FAIL, 2 INCONCLUSIVE.

## What it writes

```
pqc-evaluator-results/<arch>-<os>_<utc>/
  host.txt                cpu, os, commit, dirty count, toolchain, openssl, docker, flags
  evidence_check.txt      one line per check over docs/security/acvp-evidence, then the derived summary
  run_acvp_demo.txt       the KAT run, ends with "Local KAT result: 35 / 35 passed" and EXIT=<code>
  lifecycle_fixtures.txt  the lifecycle hook's lines, when the hook exists
  selftest_*.txt          with --self-test: the three broken-set runs
  interop/ bench/ timing/ with the optional flags: the underlying harnesses' own folders
  steps.txt               the step outcomes the summary is derived from
  summary.txt             steps plus the derived summary line
  SHA256SUMS.txt          sums of every file above
```

## What each check means, and what it does not

- **sha256 <file>** per evidence set: the file is present and equals the digest
  recorded when the set was written. A mismatch is FAIL; an absent file is
  INCONCLUSIVE. It says nothing about whether the recorded digest was right when
  written; that is what the registry comparison and the second-evaluator run
  (P27F) are for.
- **gated max |t|, negative control |t|, positive control |t|** per timing set:
  recomputed from `report.txt` and compared with the registry row within 0.011
  (the report prints two decimals). The S04-12 rule is applied from the S04-16
  and S04-17 numbers in the same report, never from the prose column.
- **aggregate reading**: `aggregate_label(gated max, controls, invalidated)`
  recomputed and compared with `aggregate_reading(row)`. A run the registry
  marks with an invalidating condition (harness v1) must recompute to INVALID;
  a Refuted run must recompute to REFUTED. The failure corpus README lists
  which sets are expected to read which way.
- **modality-matrix** (`scripts/pqc_modality_matrix_check.sh`):
  every `name` (`file.rs:line`) citation in
  `docs/security/operation-by-modality-matrix.md` is parsed; the cited file
  must resolve to exactly one path under `src/`, `crates/` and `tests/`, and the
  name must be declared there as `fn`, `struct`, `enum`, `trait` or `type`.
  PASS when every cited name is declared, FAIL when one is missing or its file
  is ambiguous, INCONCLUSIVE when the matrix is absent. A line number that has
  moved is an INFO line, not a failure. The check says the matrix names real
  code; it does not run the cited tests and does not say a cell's claim is
  true. `--self-test` renames one cited test in a copy and removes the file to
  show the FAIL and INCONCLUSIVE outputs.
- **ceilings** (`--ceilings`): for every gated case the 95 percent
  upper bound on the mean timing difference the data could still hide, derived
  from the printed means and |t| (Welch's t is the difference over its standard
  error, so the standard error is recovered as `|delta| / |t|`; the derivation
  uses `|t| - 0.005`, the smallest value the two-decimal figure could stand for,
  so the ceiling errs large). Rows whose |t| prints below 0.05 get no ceiling;
  only retained samples can give one (P7K). A ceiling is informational: it says
  how small a leak the run could have missed, not that there is none, and it is
  never a constant-time claim.
- **registry**: INCONCLUSIVE when no `RecordedTimingRun` names the directory.
  New runs are evidence only once a reviewed row exists; the kit will not vouch
  for an unregistered folder.
- **summary PASS lines, negative controls** per interop set: 15 PASS lines,
  at least 3 direction-E lines, no FAIL line.
- **131 = 131, 1616 B, 4627 B** per bench output: the byte equalities the
  grounding document quotes are present in the raw output.
- **kat-35-of-35**: `run_acvp_demo` on this host. Algorithm correctness against
  offline vectors; not CAVP.
- **lifecycle-fixtures**: the lifecycle hook `scripts/pqc_lifecycle_fixture_check.sh`
  re-checks the transition and restore fixtures when they are on
  main. Absent hook: INCONCLUSIVE, by design.
