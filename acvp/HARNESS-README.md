# Re-run the 8DB evidence harness

The harness collects release benchmarks, offline correctness results and the
available algorithm-transition test in one results folder. It records the host
and build alongside the measurements, so an evaluator can compare a new run
with the published evidence.

**Run these commands from an 8DB evaluation-engine checkout.** The scripts,
tests and engine source are supplied through evaluation access. This public
repository contains recorded outputs and the instructions for reviewing them.

## Test sequence

| Step | Command | Result recorded |
|---|---|---|
| 1 | `cargo test --release --features v2-mera --test cnsa_m1_bench -- --ignored --nocapture` | Storage, write, read and search costs at N = 100,000; ML-DSA-87 unit costs; byte equalities 131 = 131, 1,616, 4,627 and 7,332 |
| 2 | `cargo run --release --features v2-mera --bin run_acvp_demo` | 35 offline checks across PQC and symmetric primitives using NIST and RFC vectors |
| 3 | `cargo test --release --features v2-mera --test pqc_live_algorithm_swap -- --ignored --nocapture` | Algorithm-transition test, when that test file is present in the engine checkout; otherwise an absence note |

The benchmark and KAT commands share `--features v2-mera` to reuse the library
build. The correctness suite lives in `cam::acvp` and also runs with
`cargo run --bin run_acvp_demo` without that feature.

## Run the harness

Linux, macOS or Windows Git Bash:

```bash
scripts/pqc_evidence.sh
scripts/pqc_evidence.sh --publish
```

Windows PowerShell:

```powershell
scripts\pqc_evidence.ps1
scripts\pqc_evidence.ps1 -Publish
```

Results go to `pqc-evidence-results/<arch>-<os>_<utc>/`. Set `PQC_EVIDENCE_OUT`
to choose another results root. Allow time for the first release build: the
recorded 4-vCPU Linux container took about an hour to compile and 39 seconds
to run.

## Read the results

The harness builds first, logging
`cargo build --release --features v2-mera --test cnsa_m1_bench --bin run_acvp_demo`
to `build.txt`. It then records CPU load and the count of other cargo or rustc
processes immediately before the timed steps. Build time is excluded from the
measurements. Concurrent compilation leaves the byte counts usable but requires
the latency results to be identified as measured under load.

```text
pqc-evidence-results/<arch>-<os>_<utc>/
  host.txt                    CPU, cores, memory, OS, rustc, cargo, commit, branch,
                              dirty count, profile, features, compiled_at,
                              concurrent compiler count and CPU load
  build.txt                   build output and EXIT=<code>
  cnsa_m1_bench.txt            host, rustc, UTC, commit, command, raw output, exit code
  run_acvp_demo.txt            correctness output and "Local KAT result: 35 / 35 passed"
  pqc_live_algorithm_swap.txt  transition output or note that the test is absent
  summary.txt                 exit codes and result summaries
  SHA256SUMS.txt              hashes for the text files above
```

Preserve the raw result files. Record interpretation and corrections in the
accompanying documentation.

## Add a run to the engine's evidence register

1. Compare the byte counts in `cnsa_m1_bench.txt` with the 2026-07-19 run.
   Investigate and document any difference before updating a claim.
2. Use `--publish` to copy the benchmark output into the engine's evidence
   directory as `cnsa_m1_release_<arch>-<os>_<YYYY-MM-DD>.txt`.
3. Add the host, command, commit, profile, execution time, raw-output path and
   reproduced equalities to the internal run register.
4. Add a `RecordedRun` to `src/cam/pqc_claimed_metrics.rs` with the four median
   latencies, keeping each value with its `ClaimedMetric` scope.
5. Append the run to section 4 of the engine's `pqc-claims-grounding.md`.
   Cite latencies with host and date. Describe repeated byte equalities with
   the number of hosts that reproduced them.

These paths refer to the engine repository. Publishing a run there is separate
from releasing a reviewed copy in this public repository.

## Recorded harness status, September 11, 2026

Both wrapper scripts were syntax-checked, and the PowerShell steps were
exercised individually. The first Windows run,
`cnsa_m1_release_x86_64-windows_2026-09-11.txt`, used the compiled benchmark
and KAT binaries directly because concurrent cargo builds held the target-directory
lock. At that point, a complete wrapper run on an idle Windows host remained
outstanding. Read each later run's own host and execution record for its status.

## Measurement scope

The sealed-index search result uses a 2,000-term demonstration. The M1 boundary
benchmark signs uniform amplitudes rather than record bytes. Cost comparisons
need those workload details and the recorded host; broader database and
certification claims are covered in [claims and scope](../CLAIMS-AND-SCOPE.md).

## Windows toolchain note, September 11, 2026

On a Windows host without Visual Studio, the task-local
`x86_64-pc-windows-gnu` toolchain linked through llvm-mingw's clang produced
binaries larger than about 1 MB that exited with STATUS_ACCESS_VIOLATION before
`main`. The `x86_64-pc-windows-gnullvm` host toolchain
(`rustup toolchain install 1.94.1-x86_64-pc-windows-gnullvm`) linked and ran
correctly with llvm-mingw on PATH.

For that setup, set `CC`, `CXX` and `AR` to llvm-mingw's clang, clang++ and
llvm-ar for C dependencies. Keep llvm-mingw's `bin` directory on PATH at runtime
so the binaries can load `libunwind.dll`; otherwise they exit with
0xC0000135 (STATUS_DLL_NOT_FOUND). The harness records the toolchain as
`rustc_host` in `host.txt`.

Cargo built the crate's bin targets when building the integration test. The
first Linux build took 61 minutes on the recorded 4-vCPU host. Build duration
will depend on the evaluator's machine and cached artifacts.