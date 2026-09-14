# aarch64 run 1, harness v1: indeterminate

First execution of the suite on aarch64 (AWS Graviton3 c7g.2xlarge, 8 vCPU, Ubuntu
24.04, CI runner aarch64-ci-runner, workflow run 34709203023 on the PR
#3499 merge commit a39c3a14 of 0d409127; 50,000 measurements per class; load
average 9.64 right after the runner's own release compile; exit 101).

Same harness, same pattern as the x86_64 run 1 in the sibling directory, on a
different architecture and operating system: controls behaved (negative 2.15,
positive 78,858); every comparison whose class B came from a 0.8 to 2 MB pool
read as a leak (S04-10 67, S04-11 68, S04-12 1,371 with an 8 us mean gap, S04-30
414, S04-32 401, S04-33 3,212); the two hedged ML-DSA-87 cases whose 1 ms call
dwarfs a cache miss read clean (1.43, 0.68), and so did S04-31 (3.85), whose
16 KB key pool fits L1. The harness measured its own memory locality on both
hosts. See the x86_64 note for the fix (harness v2, same-slot inputs).

The CPU field of host.txt is empty because /proc/cpuinfo on this kernel carries no
"model name" line for aarch64; the helper now falls back to lscpu.
