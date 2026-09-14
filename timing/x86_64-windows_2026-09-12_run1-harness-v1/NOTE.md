# Run 1, harness v1: recorded as indeterminate, not as a leak

This is the first execution of the suite (x86_64, Windows, 2026-09-12, 50,000
measurements per class, exit 101). Its controls behaved (negative 1.51, positive
989.13), and the gated readings split exactly along one line of the harness, not
of the algorithms:

- every comparison whose class B drew its input from a pool of 0.8 to 2 MB while
  class A reused one cache-hot input read as a leak: S04-12 (155, decapsulation
  with a 512-key pool), S04-30 (220, 4 KiB pages), S04-32 (270), S04-33 (673);
  S04-11 (9.88) and S04-10 (4.50) sat on the lines with 0.8 MB pools;
- every comparison whose operation dwarfs a cache miss (ML-DSA-87 signing, about
  500 us) or whose pool fits in L1 read clean: S04-20 (1.91), S04-21 (1.08),
  S04-23 (3.77); S04-31 (15.27) used a 16 KB key pool and sits in between.

dudect's own harness writes both classes' input into the same buffer before each
measurement for this reason. Harness v2 (the next commit) does the same: one input
slot per case, filled outside the timed region for both classes, so the operation
reads identical addresses whichever class it is. S04-33 was also redefined: valid
versus tampered compares two public outcomes whose error-path cost differs by
construction; v2 compares a tag mismatch at the first byte with a mismatch at a
random byte (both classes fail), and records the accept-versus-reject cost as an
informational case.

The pre-registered lines were not touched. This run is in the registry
(`cam::pqc_timing_metrics`) with an invalidating condition and reads
`Indeterminate` through `gate_outcome`; it is evidence of neither a leak nor its
absence.
