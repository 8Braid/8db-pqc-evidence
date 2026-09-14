# Timing-leak suite evidence (P7, GYM-S04)

Each subdirectory is one run of
`scripts/pqc_timing_leak.sh` (the driver for `tests/gym_s04_pqc_timing_leak.rs`),
named `<arch>-<os>_<date>`. Inside a run:

- `host.txt`: host, CPU, commit, rustc, samples per class, seed, the number of
  other compilers alive and the load right before the measurement, finish time
  and the suite's exit code.
- `report.txt`: the suite's table, one line per comparison: samples per class
  after cropping, mean and median per class in nanoseconds, the largest |t| and
  the crop it came from, and the reading against the pre-registered lines.
- `SHA256SUMS.txt`: the three files, plus the full output (`suite.txt`) and the
  compiler log (`build.txt`), which stay in the workflow artifact and are not
  copied here.

The registry `cam::pqc_timing_metrics::recorded_timing_runs()` carries one
`RecordedTimingRun` per directory here, with the `ClaimedMetric` constructors
that let a |t| travel with its host, date, sample count and command.

## Method

dudect (Reparaz, Balasch, Verbauwhede, "Dude, is my code constant time?", DATE
2017), on the Security Gym's shared timing harness (`tests/common/timing.rs`, the
GYM-S01 and GYM-S03 lineage; one harness, not three). For each comparison: two
classes of input to one secret-holding operation; the class of each measurement is
chosen by a coin so drift and neighbours land on both classes alike; the pooled
measurements are cropped at the upper tail at 0, 1, 5, 10 and 20 percent, the same
absolute cut applied to both classes; Welch's t is computed on each crop and the
largest |t| is reported.

## The lines, pre-registered before the first run

`cam::pqc_timing_metrics` holds them as a `substrate::core::pre_registration::PendingClaim`
emitted 2026-09-12T17:00:00Z at main a4905c30, before the suite had produced a
measurement; the run's largest gated |t| is attached as the anchor by the host that
ran the suite, and the verdict comes out of `bio::topology::gate_outcome`, not out
of an assertion someone typed after reading the table.

| reading | line | source |
|---|---|---|
| no leak detected at this N | \|t\| <= 4.5 | dudect's "no evidence of leakage" line |
| inconclusive: repeat with more samples | 4.5 < \|t\| <= 10 | between the two lines; never read as a pass |
| leak detected: the suite fails | \|t\| > 10 | dudect's `t_threshold_moderate` |

A run is evidence only if, in the same run on the same host, the positive control
(an early-exit byte compare, deliberately variable-time) reads above 10 and the
negative control (both classes the same input) reads at or below 10. A run needs at
least 10,000 measurements per class for a verdict; below that the table is printed
and nothing is concluded.

## Comparisons

| id | operation | class A | class B | role |
|---|---|---|---|---|
| S04-00 | ML-KEM-1024 decapsulation | one valid ciphertext | the same ciphertext | negative control |
| S04-01 | early-exit 32-byte compare, 64 repeats | equal to the secret | random | positive control |
| S04-10 | ML-KEM-1024 decapsulation | fixed valid ciphertext | random valid ciphertexts | gated |
| S04-11 | ML-KEM-1024 decapsulation | fixed valid ciphertext | random bytes (implicit rejection path) | gated |
| S04-12 | ML-KEM-1024 key decode and decapsulation | fixed key | random keys | gated unless attributed (rule below) |
| S04-13 | ML-KEM-1024 key decode only | fixed key | random keys | gated |
| S04-15 | ML-KEM-1024 encapsulation, fixed m (no secret) | fixed ek | random eks | informational |
| S04-16 | ML-KEM-1024 decode and decapsulation, fixed ciphertext | key 0 | key 0 with its secret part (dk_PKE, z) from a random key | gated |
| S04-17 | ML-KEM-1024 decode and decapsulation, fixed ciphertext | key 0 | key 0 with its public part (ek, H(ek)) from a random key | informational |
| S04-20 | ML-DSA-87 hedged signing, fixed message | fixed key | random keys | gated |
| S04-21 | ML-DSA-87 hedged signing, fixed key | fixed message | random messages | gated |
| S04-22 | ML-DSA-87 deterministic signing (`SignatureProvider::sign`) | fixed message | random messages | informational |
| S04-23 | ML-DSA-87 verification | fixed valid pair | random valid pairs | informational |
| S04-30 | `encrypt_page`, fixed key | fixed 4 KiB page | random pages | gated |
| S04-31 | `encrypt_page`, fixed page | fixed key | random keys | gated |
| S04-32 | `decrypt_page`, fixed key | fixed valid ciphertext | random valid ciphertexts | gated |
| S04-33 | `decrypt_page`, fixed key, both fail authentication | tag bit flipped at the first tag byte | tag bit flipped at a random tag byte | gated |
| S04-34 | `decrypt_page`, fixed key | valid ciphertext | one tag bit flipped (rejected) | informational |

Why two ML-DSA readings are informational: FIPS 204 Algorithm 2 rejection-samples,
so the loop count of deterministic signing is a function of the key and the message
by design; a fixed message has one loop count and random messages have a
distribution of them, and the difference is not a secret leaking. The gated signing
comparisons use the hedged interface with fresh randomness in both classes, which
makes the loop count identically distributed across classes and leaves only timing
inside an iteration. Verification holds no secret. S04-34 is informational because
whether a ciphertext was accepted is public to the caller; an accept-versus-reject
cost difference is the error path, not a secret. The gated tag test (S04-33) asks
the question that matters: does the position of the mismatch change the time.

## The S04-12 attribution rule (fixed 2026-09-12T19:00Z, before run 3)

Run 2 (harness v2, x86_64) read S04-12 at |t| = 901.70 while every
ciphertext-varying case read below 1. FIPS 203 decapsulation re-encrypts, and
re-encryption expands the matrix from the key's public seed rho by rejection
sampling (`NttMatrix::sample_uniform` in the ml-kem crate): for a fixed key that
accept/reject branch sequence repeats exactly and a branch predictor learns it;
for random keys it is random. That would be timing on public data, not a leak,
and it is the kind of claim that must be decided by a measurement fixed in
advance, not by an explanation written after the table. The rule: **S04-12 is
excluded from a run's gate only when, in the same run, S04-17 (public part of
the key varies, secret part fixed) reads above 10 and S04-16 (secret part varies,
public part fixed) reads at or below 10. Otherwise S04-12 is gated.** The
spliced keys reach the implicit-rejection path, which S04-11 checks against
acceptance in the same run. S04-13 (decode only) and S04-15 (encapsulation, no
secret at all) are recorded alongside so the location of the gap is visible.

## What a run shows, and what it does not

**Shows:** on the named host, at the recorded sample count, the gated operations
did or did not exhibit input-dependent timing that a Welch t-test can see, with the
detector's power demonstrated in the same run.

**Does not show:** constant-time execution as a proof, behaviour on other hardware
or compilers, resistance to power, electromagnetic or micro-architectural channels,
or anything a FIPS 140-3 lab measures under ISO/IEC 17825. No run here is
validation or certification. Say "no leak detected at N = 50,000 per class on
<host>", never "constant time" and never "validated".

## Run history and harness changes

Changes to the harness after a run are listed here, with the run that prompted
them, so nobody has to reconstruct the order from git.

- **Run 1, x86_64 Windows, 2026-09-12, harness v1** (`x86_64-windows_2026-09-12_run1-harness-v1/`):
  controls fine, five gated cases read as leaks, all of them cases whose random
  class drew from a 0.8 to 2 MB input pool while the fixed class stayed cache-hot.
  Recorded as `Indeterminate` with that invalidating condition; see its `NOTE.md`.
- **Run 1, aarch64 Linux, 2026-09-12, harness v1** (`aarch64-linux_2026-09-12_run1-harness-v1/`,
  CI run 34709203023 on the ARM64 runner): the same pattern on the second
  architecture, which is what settled the diagnosis. Controls fine; every pooled
  case a "leak" (S04-12 at 1,371 with an 8 us mean gap); the 1 ms ML-DSA cases and
  the L1-sized key pool clean. Recorded as `Indeterminate`.
- **Harness v2, same day:** both classes are written into one input slot before
  each timed call (dudect's own arrangement), so the operation reads identical
  addresses whichever class it is. S04-33 redefined to compare two failing
  decryptions (tag mismatch at the first byte versus at a random byte); the
  accept-versus-reject cost became the informational S04-34. The pre-registered
  lines were not changed.
- **Run 2, x86_64 Windows, 2026-09-12, harness v2** (`x86_64-windows_2026-09-12_run2-harness-v2/`):
  controls 0.95 and 2,121; every AEAD and ciphertext-varying ML-KEM case between
  0.40 and 2.16, both hedged ML-DSA cases clean; S04-12 (decode key and
  decapsulate, whole key varying) 901.70 with a 6 us mean gap. `Refuted` as the
  suite stood; recorded that way.
- **Suite v3, same day, before run 3:** S04-13, S04-15, S04-16, S04-17 added and
  the S04-12 attribution rule above fixed. The lines were not changed.
- **Run 3, x86_64 Windows, 2026-09-12, suite v3** (`x86_64-windows_2026-09-12_run3-suite-v3/`):
  `Confirmed`. Controls 0.51 and 1,092. The rule attributed S04-12 (91.29) to
  public data: S04-17 read 99.77 and S04-15 110.73 with the same 6 us mean gap,
  S04-16 read 1.36 and S04-13 2.06. Every gated case inside the window (largest
  6.05, S04-30); S04-30 and S04-33 (4.50) sit in the inconclusive band and are
  quoted as "repeat with more samples", not as passes.
- **Run 2, aarch64 Linux, 2026-09-12, suite v3** (`aarch64-linux_2026-09-12_run2-suite-v3/`,
  CI run 34713968258): `Refuted` as the suite stood. The public-seed effect
  reproduced at full strength on Neoverse-V1 (S04-17 1,537 and S04-15 1,628, an
  8 us gap), but the rule needs S04-16 at or below 10 and it read 32 with a 22 ns
  gap, the same 23 ns that S04-13 (decode only, no decapsulation at all) showed;
  S04-32 read 28 with 8 ns and S04-30 10.8 with 1.2 ns. The gaps follow how many
  bytes class B's fill copied from cold pool memory before the timed call (a
  3 KB key: 22 ns; a 4 KB page copied: 8 ns; a 4 KB page generated in place: 1 ns;
  32 bytes: 0.2), which is a harness asymmetry a nanosecond timer on a quiet ARM
  host can see at N = 50,000.
- **Harness v2.1, same day, before the next runs:** class A now draws its fixed
  input from a pool of identical copies, so both classes' fills stream from
  equally cold memory and evict alike. Nothing else changes; the lines and the
  S04-12 rule stand.
- **Run 4, x86_64 Windows, 2026-09-12, harness v2.1** (`x86_64-windows_2026-09-12_run4-harness-v2.1/`):
  `Confirmed` with no case in the inconclusive band; every gated reading at or
  below 2.79 (S04-16), controls 1.91 and 1,075. The rule fired: S04-17 811 and
  S04-15 632 with the secret fixed or absent, S04-16 2.79 and S04-13 0.53
  without the public part varying.
- **Run 3, aarch64 Linux, 2026-09-12, harness v2.1** (`aarch64-linux_2026-09-12_run3-harness-v2.1/`,
  CI run 34717091734): `Confirmed`. The symmetric fills removed the gaps where
  predicted (S04-13 36 to 2.24, S04-16 32 to 0.92, S04-32 28 to 1.64, S04-30
  10.8 to 3.78) while the public-seed effect stayed at full strength (S04-17
  1,342, S04-15 1,437) and the rule fired. Controls 3.08 and 53,122. S04-10
  (7.64) sits in the inconclusive band on this host.
- **Run 1, x86_64 Linux (WSL2 on the same Ryzen 9 9950X3D), 2026-09-12, harness v2.1**
  (`x86_64-linux_2026-09-12_run1-harness-v2.1/`, CI run 34719913662, the x86_64
  leg of PR #3499): conclusive pass, every gated case at or below 2.83, controls
  0.54 and 13,220, the rule firing (S04-17 274, S04-15 640 against S04-16 2.39,
  S04-13 2.23). A nanosecond clock on the same silicon as the Windows runs, under
  a load average of 24 with ten other compilers alive; a busy-host result.

- **Run 4, aarch64 Linux, 2026-09-14, features fips** (`aarch64-linux_2026-09-14_run4-fips/`,
  CI run 34806633427, the first run of the compliance profile's AEAD cases S04-35
  to S04-39): pass with one inconclusive case. Controls 1.55 and
  49,298. Compliance: S04-35 2.16, S04-36 3.31, S04-37 1.26, S04-38 5.61 (two
  failing GCM decryptions, tag mismatch at the first byte versus a random byte;
  a 1.2 ns mean gap at 2,740 ns, medians equal; repeat before quoting), S04-39
  9,347 informational as expected (the commitment refusal happens before AES).
  Every research-profile gated case at or below 3.52; the rule fired (S04-17
  1,217 and S04-15 1,278 against S04-16 0.80 and S04-13 2.64).
- **Run 2, x86_64 Linux, 2026-09-14, features fips** (`x86_64-linux_2026-09-14_run2-fips/`,
  the x86_64 leg of the same CI run, on x86_64-ci-runner-a with eleven other compilers
  alive): `Refuted` as recorded. S04-35 (compliance encrypt, fixed page versus
  random pages) read 15.71 at the 20 percent crop from a 2.1 ns mean gap at
  1,154 ns with identical medians (1,162 / 1,162 ns). Everything else was clean:
  the research-profile analog S04-30 on the same two pools read 1.13 in the same
  run, S04-36 1.43, S04-37 1.92, S04-38 3.26, every other gated case at or below
  3.43, controls 1.15 and 601.78, the rule fired (S04-17 424 and S04-15 89
  against S04-16 1.96 and S04-13 2.75). On aarch64 from the same commit the same
  day S04-35 read 2.16. The reading is recorded as a refutation on this host at
  this N, not attributed and not reworded.
- **Run 3, x86_64 Linux, 2026-09-14, features fips, 200,000 per class**
  (`x86_64-linux_2026-09-14_run3-fips-200k-invalid/`, CI run 34818892186, the
  pre-registered S04-35 repeat): INVALID. The negative control read 10.00 with an
  8 ns mean gap on two identical classes, so the host (x86_64-ci-runner-c, load average 9.54,
  a shared CI box that was also carrying other work) had no quiet floor. S04-35
  read 13.29 with a 0.9 ns mean gap (means 2,407.2 vs 2,408.1 ns, medians 2,404 /
  2,405 ns), smaller than the control's own gap. Positive control 73,628.70. The
  run is recorded and not quoted either way.

### The S04-35 repeat rule (fixed 2026-09-14T07:40Z, before the repeat)

One reading above the line on one busy host, with the same comparison clean on
the other architecture and the same two input pools clean through the research
provider in the same run, is repeated, not argued with. The repeat is the x86_64
leg at 200,000 per class, features fips, same seed, same commit family, and its
reading is attributed in advance:

- S04-35 above 10 with the negative control at or below 4.5: a finding against
  the compliance provider's encrypt path (`cam::cnsa2_compliance_provider` over
  aws-lc-rs AES-256-GCM and the HMAC-SHA-384 commitment). The P22I timing row
  stays refuted, and two attribution cases are added to the suite before any
  further run: aws-lc-rs AES-256-GCM seal alone on the same two pools, and the
  provider's framing with the AEAD call replaced by a copy, so the gap is placed
  in the library, in the framing, or in neither.
- S04-35 at or below 4.5: run 2's reading is recorded as not reproduced at four
  times the samples, and the compliance rows are quoted with both runs named.
- Between the two: inconclusive, and the repeat is run again at the same N on a
  quieter x86_64 host before anything is quoted.

Whatever it reads, the repeat is a new row; run 2 stays.

**Outcome of the first repeat (2026-09-14, run 3):** invalid, not a resolution. The
repeat host had no quiet floor (negative control 10.00), so S04-35 is neither
confirmed nor cleared. It stays flagged on loaded x86 hosts (15.71 at 50k, 13.29
at 200k, both with sub-2 ns mean gaps and both on hosts whose own controls were
noisy) and clean on aarch64 (2.16). The real repeat needs a genuinely quiet
x86 host with no other work on it during the measurement, which this run did not
have. The 0.9 ns S04-35 gap being smaller than the 8 ns negative-control gap is
consistent with host noise, but that is an observation, not the pass the rule
requires.

Standing reading after these eight runs, in the tri-state words the 2026-09-12
evidence review asked for: on x86_64 (Windows and Linux) a **conclusive pass**,
every gated case at or below 4.5; on aarch64 a **pass with one inconclusive
case** (S04-10 at 7.64). At 50,000 per class, no leak detected in ML-KEM-1024
decapsulation (ciphertext, implicit rejection, secret-key dependence), ML-KEM key
decode, ML-DSA-87 **hedged** signing (key and message) and the page AEAD (page,
key, ciphertext, tag-mismatch position), with the whole-key ML-KEM reading
attributed to public-seed matrix expansion by the pre-registered rule. The
deployed signing path is the deterministic variant (`SignatureProvider::sign`),
whose loop count is a function of key and message; the hedged rows do not
describe it . That paragraph, with the host names and N, is the claim;
not "constant time", not "validated".

## Reproducing

    scripts/pqc_timing_leak.sh                    # 50,000 per class, results under pqc-timing-results/
    scripts/pqc_timing_leak.sh --seed 0x<seed>    # the seed from a run's host.txt reproduces its class order and pools
    scripts/pqc_timing_leak.sh --publish          # also copy report.txt and host.txt here

CI: `.github/workflows/pqc-timing-leak.yml` runs the x86_64 leg on every change to
the providers, the page AEAD or the suite, weekly, and on dispatch; the aarch64 leg
runs on dispatch or on a pull request carrying the `arm64` label
(`docs/RUNNER-ARM64.md`).

## Completion of the evidence sets, 2026-09-13

Every set's `SHA256SUMS.txt` has always listed four raw files (`host.txt`,
`build.txt`, `suite.txt`, `report.txt`), but the publish step of
`scripts/pqc_timing_leak.sh` copied only `host.txt` and `report.txt`, so the
evaluator kit (`pqc_evidence_check`) read every set as
INCONCLUSIVE: two listed files absent. On 2026-09-13 the missing `build.txt` and
`suite.txt` were restored into all eight sets from their originals (the four
Windows runs from this host's `pqc-timing-results/` folders; the three aarch64
runs and the x86_64 Linux run from the CI artifacts of runs 34709203023,
34713968258, 34717091734 and 34719913662) and `sha256sum -c` confirms every
digest recorded at run time. No recorded file was altered; the publish step now
copies all four. The kit found this; that is what it is for.
