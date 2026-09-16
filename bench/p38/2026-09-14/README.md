# PQC-established keys, unchanged record ciphertext

**A key established with PQC produced the same 131-byte encrypted record as a classically derived key.** In these September 14, 2026 paired measurements, encrypt and decrypt medians matched within one percent on Linux x86_64 and ARM. The record cipher stayed AES-256-GCM-SIV; key establishment happened before the timed operations.[1]

This is the part of the cost model that can remain unchanged as key establishment moves to PQC. The measurements below expose the individual operations, clock resolution and host load so the claim can be assessed at that boundary.

| Measurement, classical / PQC | Ryzen 9 9950X3D, WSL2 Linux x86_64 | Neoverse-V1, Linux ARM |
|---|---:|---:|
| Plaintext bytes | 115 / 115 | 115 / 115 |
| Returned ciphertext bytes | 131 / 131 | 131 / 131 |
| Encrypt median | 451 / 451 ns | 1,172 / 1,171 ns |
| Encrypt p95 | 470 / 461 ns | 1,198 / 1,195 ns |
| Decrypt median | 441 / 441 ns | 1,188 / 1,188 ns |
| Decrypt p95 | 461 / 451 ns | 1,225 / 1,226 ns |
| Samples per arm, after 16 warm-ups | 2,000 | 2,000 |
| Smallest observed nonzero clock delta | 10 ns | 33 ns |

Read the [x86_64 output](x86_64-wsl2/cnsa_m1_bench.txt) and [ARM output](aarch64-linux/cnsa_m1_bench.txt), especially their `PER-RECORD` and `PARITY` blocks. Both use source `4efb7653b91216d0ed6e8c5aa463e7bb1806a211`, Rust 1.94.1 and the release `v2-mera` build. These are vendor-run measurements. The x86_64 row is WSL2 Linux, distinct from the older native-Windows and Xeon rows elsewhere in this repository.

The 16 extra bytes are the AEAD tag, shared by both arms. Zero incremental ciphertext bytes does not remove key-establishment objects, signatures, stored indices or database framing. For a comparison that changes both cipher and key-commitment protection, see the separate [provider-profile results](../../profile-cost/2026-09-14/).

## Timing interpretation

The experiment times the same page encrypt/decrypt functions using keys established through two different paths. It does not include governed-store admission, key derivation on each write, indexing, durable database commits, key rotation or migration. The printed `N=100000` is the key-object amortization denominator, rather than 100,000 timed database writes. The separate search fixture has 2,000 terms.

The reported one-nanosecond ARM median difference is below the observed 33 ns clock delta and does not establish a speed advantage. The retained files contain summarized medians and p95s, without the individual samples needed to recompute quantiles or an equivalence interval. The arms run sequentially. The x86_64 host recorded three compiler processes and load averages of 8.37, 17.74 and 15.92; ARM recorded zero compiler processes at its snapshot but load averages of 9.58, 8.71 and 6.00 on eight CPUs. These observations do not establish host isolation or a deployment latency guarantee.

The July 700/600 ns decrypt medians remain a separate historical observation. This later protocol adds a clock-resolution probe and a minimum of 2,000 samples per arm. It does not retroactively explain that difference or reproduce every earlier host.

## Check the retained evidence

From this directory, with GNU `sha256sum` available:

```sh
sha256sum --check SHA256SUMS
```

The command checks the published bytes. Each host's `PROVENANCE.json` binds the original artifact ZIP, run, job, source, successful harness step, host context and original/public file hashes. Build-root paths were replaced with `[build-root-redacted]` only where that file's provenance records an occurrence; measurements and other bytes are preserved. The original ZIPs remain retained privately. Their original `SHA256SUMS.txt` files are historical receipts: they contain original hashes, include a duplicated `host.txt` entry, and will not validate a redacted log. Use this directory's complete `SHA256SUMS` for the public copies.

The raw harness also printed other benchmark arms and their original interpretation. Use this README's scoped table when interpreting its legacy capability labels and speedup projections. Password derivation with PBKDF and ML-KEM establishment perform different functions, so their ratio is not an equivalent-function speedup. The `four-way` frame is 136 bytes, a different object from the 131-byte page result. Its projected per-row signature totals and boundary-signature timers do not measure verification of every original record. Its sealed-index versus decrypt-and-scan search compares different query plans. None is a matched conventional-database or full-verification comparison. Those outputs remain in the record and are outside the paired same-AEAD claim above. The `migration_head_to_head` control was a plumbing skeleton, not a measured database transition.

Use these exact records to inspect the per-record comparison. Re-executing the engine benchmark requires the evaluation source and harness; evaluating total storage and end-to-end performance requires matched protection, indexing, durability and workload conditions.

[1] [x86_64 provenance](x86_64-wsl2/PROVENANCE.json): run `34802540928`, job `103847986771`, artifact `10334020988`, harness 04:35:42–05:03:39 UTC. [ARM provenance](aarch64-linux/PROVENANCE.json): run `34816572382`, job `103888377965`, artifact `10336934273`, harness 07:25:27–07:51:10 UTC. Each original artifact has seven files. Source/job associations come from retained API records and are distinct from independently rebuilding the engine. Private repository permissions may be required to open the original workflow links.
