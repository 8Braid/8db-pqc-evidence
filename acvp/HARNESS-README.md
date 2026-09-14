# PQC evidence harness: one command, one results folder

## What it runs

| Step | Command | What it establishes |
|---|---|---|
| 1 | `cargo test --release --features v2-mera --test cnsa_m1_bench -- --ignored --nocapture` | Release-mode storage, write, read and search costs for plaintext, classical KDF and PQC KEM key establishment at N = 100,000; ML-DSA-87 unit costs; the byte equalities (131 = 131, 1,616, 4,627, 7,332) |
| 2 | `cargo run --release --features v2-mera --bin run_acvp_demo` | 35 of 35 offline NIST ACVP known-answer vectors for ML-KEM-1024 and ML-DSA-87. Algorithm correctness, not module validation |
| 3 | `cargo test --release --features v2-mera --test pqc_live_algorithm_swap -- --ignored --nocapture` | Runs only when work package A has landed the test file; otherwise recorded as not present |

Both steps use `--features v2-mera` so the library compiles once. The KAT suite
lives in `cam::acvp` and does not depend on that feature; `cargo run --bin
run_acvp_demo` gives the same 35 of 35.

## How to run

Linux, macOS, or Windows Git Bash:

```bash
scripts/pqc_evidence.sh            # results under pqc-evidence-results/<arch>-<os>_<utc>/
scripts/pqc_evidence.sh --publish  # also copies the bench output into this folder (see below)
```

Windows PowerShell:

```powershell
scripts\pqc_evidence.ps1
scripts\pqc_evidence.ps1 -Publish
```

Set `PQC_EVIDENCE_OUT` to change the results root. Expect one long compile on
the first run; the recorded 4-vCPU Linux container took about an hour to
compile and 39 seconds to run.

## What it writes

The harness compiles first (`cargo build --release --features v2-mera --test
cnsa_m1_bench --bin run_acvp_demo`, logged to `build.txt`) and only then runs
the timed steps, so the measurements exclude build time. Immediately before the
timed steps it records how many other cargo or rustc processes are alive and the
CPU load; a run with concurrent compiles is still valid for the byte equalities
but its latencies must be marked as not quiesced.

```
pqc-evidence-results/<arch>-<os>_<utc>/
  host.txt              cpu, cores, memory, OS, rustc, cargo, commit, branch, dirty count, profile, features,
                        compiled_at, concurrent cargo/rustc count and load immediately before the timed steps
  build.txt             cargo build output, EXIT=<code>
  cnsa_m1_bench.txt     header (host | rustc | utc | commit | cmd), full raw output, EXIT=<code>
  run_acvp_demo.txt     same shape, ends with "Local KAT result: 35 / 35 passed"
  pqc_live_algorithm_swap.txt   raw output, or the note that package A is not present
  summary.txt           exit codes and the one-line results
  SHA256SUMS.txt        sums of every text file above
```

The raw files, not this README, are the evidence. Keep them unedited.

## Publishing a run

1. Review `cnsa_m1_bench.txt`. Every byte count and equality must match the
   2026-07-19 run; if a value differs, write down which one is wrong and why
   before changing anything.
2. `--publish` copies it to `cnsa_m1_release_<arch>-<os>_<YYYY-MM-DD>.txt` in
   this folder, next to `cnsa_m1_release_x86_64-linux_2026-09-11.txt`.
3. Add a row to the internal run register mirroring
   `RUN-PQC3STATE-TRUSTDB-20260718`: host, command, commit, profile, execution
   time, raw-output path, and the reproduced equalities.
4. Add a `RecordedRun` to `src/cam/pqc_claimed_metrics.rs` with the four median
   latencies, so the numbers exist only as scoped `ClaimedMetric`s.
5. Append the run to `../pqc-claims-grounding.md` section 4. Latencies are
   host-bound and are quoted only with host and date; equalities may be quoted
   with "N hosts".

## Status of the harness itself

Both scripts are syntax-checked and the PowerShell one was exercised step by
step on 2026-09-11, but the first Windows run
(`cnsa_m1_release_x86_64-windows_2026-09-11.txt`) executed the cargo-built
bench and KAT binaries directly, because other agents' cargo builds held the
target-dir lock the whole time. The first end-to-end `pqc_evidence.ps1` or
`pqc_evidence.sh` run on an idle host is still to be done and should replace
that run's latencies.

## What a passing run does not prove

No CAVP or CMVP status, no "validated" or "certified" wording, no deployment
claim, and no statement about hosts the harness has not run on. The sealed-index
search figure is a 2,000-term demo; the M1 boundary bench signs uniform
amplitudes, not record bytes.

## Windows toolchain note (2026-09-11)

On a Windows host without Visual Studio, the task-local `x86_64-pc-windows-gnu`
toolchain linked through llvm-mingw's clang produced binaries larger than about
1 MB that exit with STATUS_ACCESS_VIOLATION before `main`. The
`x86_64-pc-windows-gnullvm` host toolchain (`rustup toolchain install
1.94.1-x86_64-pc-windows-gnullvm`) with llvm-mingw on PATH links and runs
correctly; set `CC`, `CXX` and `AR` to the llvm-mingw clang, clang++ and llvm-ar
for the C dependencies. Binaries built this way import `libunwind.dll` from the
llvm-mingw `bin` directory, so that directory must stay on PATH when they run
(exit code 0xC0000135, STATUS_DLL_NOT_FOUND, otherwise). Record the toolchain in
`host.txt` (it is captured under `rustc_host`).

Cargo builds every bin target of the crate whenever it builds an integration
test, so the first `cargo test --test cnsa_m1_bench` compiles the whole bin
catalogue. On the 4-vCPU Linux host that was 61 minutes; on a 16-core desktop
it is a few minutes.
