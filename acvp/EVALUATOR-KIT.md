# Evaluator kit

The evaluator kit helps a technical reviewer check the evidence trail, compare
recorded timing results with the engine's registry and run the offline
correctness suite on a second host.

**The commands below run from an 8DB evaluation-engine checkout.** The scripts,
Rust source and registry are supplied through evaluation access and are absent
from this public repository. For checks you can perform on the published files
alone, start with [Review and reproduce the evidence](../HOW-TO-VERIFY.md).

## Checks and outcomes

The kit checks three things:

1. Files listed by each evidence set are present and match its recorded hashes.
2. Timing reports agree with the registry in `src/cam/pqc_timing_metrics.rs`.
   The checker reads the per-case |t| values from `report.txt`, applies the
   S04-12 attribution rule, derives the gated maximum and controls, and
   recomputes the aggregate reading.
3. The local correctness suite passes its 35 checks across ML-KEM, ML-DSA,
   HKDF, AES-GCM, AES-GCM-SIV and HMAC using NIST and RFC vectors.

The default run checks existing evidence and runs the correctness suite.
Optional `--bench`, `--timing` and `--interop` flags collect new measurements
in separate result folders.

| Outcome | Meaning |
|---|---|
| PASS | All required checks completed and agreed with the record |
| FAIL | A checksum mismatch, registry disagreement, FAIL line or unparseable report |
| INCONCLUSIVE | A required item is missing, such as a manifest, listed file, registry row or hook script |

The tool derives the summary from individual check results. Exit codes are
0 for PASS, 1 for FAIL and 2 for INCONCLUSIVE. These outcomes describe the
evaluation checks; certification status is documented in
[claims and scope](../CLAIMS-AND-SCOPE.md).

For this public edition, consult [publication history](../REDACTIONS.md) when
reconciling hashes. The kit's original manifests refer to the internal evidence;
some public copies have redacted identifiers or revised editorial text.

## Run from the evaluation-engine checkout

```bash
scripts/pqc_evaluator_kit.sh                 # host facts, evidence tree, ceilings, KATs, lifecycle hook
scripts/pqc_evaluator_kit.sh --self-test     # also exercise INCONCLUSIVE and FAIL outcomes
scripts/pqc_evaluator_kit.sh --check-only    # checks used by CI; skips the KAT run
scripts/pqc_modality_matrix_check.sh --self-test
scripts/pqc_evaluator_kit.sh --interop       # OpenSSL 3.5 or Docker required
scripts/pqc_evaluator_kit.sh --bench         # release benchmark; initial compile can be long
scripts/pqc_evaluator_kit.sh --timing        # new timing run in a new folder
scripts\pqc_evaluator_kit.ps1 --self-test    # Windows wrapper through Git Bash
```

To run the evidence checker separately:

```bash
cargo run --release --bin pqc_evidence_check -- --root .
cargo run --release --bin pqc_evidence_check -- --set docs/security/acvp-evidence/timing/aarch64-linux_2026-09-12_run3-harness-v2.1
cargo run --release --bin pqc_evidence_check -- --root . --ceilings
```

In the engine's `pqc-interop.yml` workflow, the x86_64 job runs
`--check-only --self-test` for changes to the evidence tree, kit or registry.
That job fails on a FAIL line. Its overall summary remains INCONCLUSIVE while
the lifecycle hook is absent and the KAT step is skipped.

## Results folder

```text
pqc-evaluator-results/<arch>-<os>_<utc>/
  host.txt                CPU, OS, commit, dirty count, toolchain, OpenSSL, Docker, flags
  evidence_check.txt      per-check results over docs/security/acvp-evidence and summary
  run_acvp_demo.txt       KAT output, "Local KAT result: 35 / 35 passed", EXIT=<code>
  lifecycle_fixtures.txt  lifecycle hook results, when available
  selftest_*.txt          broken-set runs when --self-test is selected
  interop/ bench/ timing/ results from optional harness runs
  steps.txt               step outcomes
  summary.txt             steps and derived summary
  SHA256SUMS.txt          hashes for the files above
```

## Interpret individual checks

- **sha256 <file>**: compares a file with the digest in its evidence set.
  A mismatch is FAIL; an absent file is INCONCLUSIVE. This establishes
  consistency with the recorded digest. The registry comparison and a second
  evaluator's run provide additional checks on the record.
- **gated max |t| and controls**: recomputes values from `report.txt` and compares
  them with the registry within 0.011, allowing for two-decimal report values.
  The S04-12 attribution rule uses the S04-16 and S04-17 values in the same report.
- **aggregate reading**: compares
  `aggregate_label(gated max, controls, invalidated)` with
  `aggregate_reading(row)`. Invalidated harness-v1 runs must read INVALID;
  Refuted runs must read REFUTED. See the [timing history](../timing/README.md)
  for the recorded outcomes.
- **modality-matrix**: checks each named `file.rs:line` citation in the engine's
  `docs/security/operation-by-modality-matrix.md`. The file must resolve
  uniquely under `src/`, `crates/` or `tests/`, and the cited name must be
  declared as a `fn`, `struct`, `enum`, `trait` or `type`. Missing names or
  ambiguous files are FAIL; an absent matrix is INCONCLUSIVE. A moved line
  number produces INFO. This checks citation accuracy; executing the cited tests
  is a separate step. The self-test renames a cited test in a copy and removes
  the matrix file to exercise both failure outcomes.
- **ceilings**: estimates a 95 percent upper bound on the mean timing difference
  each gated case could have missed. The calculation recovers the standard
  error as `|delta| / |t|` and uses `|t| - 0.005` to account conservatively
  for rounding. Rows with printed |t| below 0.05 need retained samples for a
  ceiling. Use these bounds to assess sensitivity at the recorded sample size.
- **registry**: an evidence directory without a `RecordedTimingRun` row is
  INCONCLUSIVE. A new run needs a reviewed registry entry.
- **interop summaries and negative controls**: checks for 15 PASS lines, at
  least three direction-E lines and no FAIL line.
- **benchmark byte counts**: checks for the recorded `131 = 131`,
  `1616 B` and `4627 B` equalities cited by the engine's grounding document.
- **kat-35-of-35**: runs `run_acvp_demo` on the evaluator's host and checks the
  offline correctness results.
- **lifecycle-fixtures**: invokes `scripts/pqc_lifecycle_fixture_check.sh`
  to re-check transition and restore fixtures when available on the engine's
  main branch. An absent hook produces INCONCLUSIVE.

Contact [8Braid](mailto:ashley@8braid.com) to request evaluation access and agree
which checks support your review.