# The measured cost of changing the protection profile

**The committed-GCM profile returned 185 bytes for a 115-byte record; the framed research-SIV profile returned 136 bytes.** These September 14, 2026 runs measure the additional framing and processing of two different protection profiles. Cipher selection and key-commitment checks change together. Their difference is not the isolated cost of PQC key establishment.[1]

| Provider operation or output | Xeon Platinum 8375C, Linux x86_64 | Linux ARM |
|---|---:|---:|
| Committed-GCM encrypt / decrypt median | 1,637 / 1,674 ns | 1,661 / 1,654 ns |
| Research-SIV encrypt / decrypt median | 584 / 574 ns | 1,021 / 980 ns |
| Committed-GCM / research-SIV returned buffer | 185 / 136 bytes | 185 / 136 bytes |
| Pure ML-KEM-1024 establishment / recovery median, measured separately | 87,954 / 99,325 ns | 79,542 / 89,556 ns |

The committed-GCM profile uses AES-256-GCM and a 48-byte HMAC-SHA-384 key-commitment tag, checked before decryption. Its returned buffer comprises a five-byte version header, one frame byte, the commitment tag, 115-byte ciphertext and 16-byte GCM tag. The research-SIV buffer has the five-byte version header, ciphertext and 16-byte tag. The 49-byte difference is the added frame byte and commitment tag. The [same-AEAD key-establishment comparison](../../p38/2026-09-14/) measures a different boundary: 131-byte page output without the CAM version header.

## What was timed

Both profiles ran in one process on each host at source `cc994fc6e1ecdede60eb679c3c964fafc2c92946`, release build with feature `fips`. Record encrypt/decrypt used one fixed key, a nonce counter per operation and 200 samples after 16 warm-ups. The separate pure ML-KEM measurements used 50 samples each. Establishment had four warm-ups; recovery had no separate warm-up loop, despite sharing the original output's establishment-extent label. The profile name and build feature identify the tested configuration; they do not establish an 8DB module-validation certificate.

These provider measurements do not call the full protected-store admission, ingestion or indexing path. They do not establish native database size, sustained throughput, a matched traditional-encryption baseline, or migration cost. The output's 1,568-byte KEM row measures the ciphertext alone; its 1,616-byte research row is ciphertext plus wrapped key. Complete envelopes, recipients, epochs and serialization need their own accounting. Both hosts recorded load before and after the build/run; no separate quiet-host or clock-resolution claim accompanies this experiment.

## Check the original rows and their meaning

Read the [x86_64 rows](x86_64-linux/rows.txt) and [ARM rows](aarch64-linux/rows.txt) with their `host.txt`, `output.txt` and `PROVENANCE.json`. The original printed numeric rows have no metric IDs. Their order in the pinned benchmark source is: KEM establishment, KEM recovery, KEM ciphertext bytes, research partial KEM-object bytes, committed-GCM encrypt, committed-GCM decrypt, committed-GCM buffer bytes, research-SIV encrypt, research-SIV decrypt and research-SIV buffer bytes. Each provenance record preserves that original-source mapping and the exact row-file hash. No timing row has been rewritten.

From this directory, with GNU `sha256sum` available:

```sh
sha256sum --check SHA256SUMS
```

This checks the public bytes. Each host's provenance retains source, original run/job/artifact identities, successful step records, original archive hash and per-file original/public hashes. Build-root paths in `output.txt` were replaced with `[build-root-redacted]`; all other original bytes are preserved. The original `SHA256SUMS.txt` retains hashes of the unredacted files, so use this directory's complete `SHA256SUMS` to check public copies. Engine source and executable are supplied through evaluation access for a fresh run.

[1] Run `34812350038`: [x86_64 provenance](x86_64-linux/PROVENANCE.json), job `103876031495`, artifact `10336634404`, output rows at 07:31:21 UTC; [ARM provenance](aarch64-linux/PROVENANCE.json), job `103876031605`, artifact `10335763016`, output rows at 06:52:00 UTC. Each artifact contains four original files. This package publishes their measured outcomes and scoped provenance; the source-to-build association is retained evidence rather than a fresh independent engine rebuild.
