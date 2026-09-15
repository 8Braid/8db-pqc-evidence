# 512 exact records after a killed replica

On September 15, 2026, two physical workstations running 8DB finished with the same 512 exact typed records after the receiver was killed and reopened from its retained store. The receiver retained 172 records and read new content 1,074 ms after reopening, within the preregistered 30,000 ms limit. Both actors completed with matching content and drained commit/applied watermarks of 513.

This package makes the fixed synthetic-corpus result inspectable without engine access. Start with [the readable machine summary](summary.json), then inspect the [expected manifest](originals/expected-manifest.json), [host A final manifest](originals/a-final.json) and [host B final manifest](originals/b-final.json). The [prior outcomes](negative-history.json) retain the earlier 293-of-512 completion failure and an unpaired launch refusal.

The tested profile combines Required native AES-256-GCM, HKDF-SHA384 and HMAC-SHA384 with a Compliance channel using ML-KEM-1024 and trusted ML-DSA-87 sender authentication. The 240-second campaign included a final 30-second drain, a real SIGKILL at 45 seconds, a 15-second outage and a same-store reopen. It used a 250 ms proposal interval and at most eight outstanding distinct admitted records.

The source was frozen at `7db11180d1a71f7d95cd9698c72ffd1f044d1e6e`. The shared executable SHA-256 was `9596da2abf37ee0e22c714ecf407db138acfe4153fcbd96093b779f911377009`. Rust 1.94.1 built the physical executable at test opt-level 2 with debug assertions and overflow checks enabled. The public artifact verifier below is separate software and contains no 8DB implementation.

## Verify the published record with Rust

From this directory, with Rust and Cargo installed:

```sh
cargo run --locked --manifest-path verify/Cargo.toml --target-dir /tmp/8db-r9-verifier -- .
```

On Windows PowerShell, use `--target-dir "$env:TEMP/8db-r9-verifier"`. Keep compiled artifacts outside the evidence checkout so the repository's separate complete-file inventory remains clean.

The verifier hashes all listed evidence files, rehashes the 512 exact serialized expected rows, compares every final row digest on both hosts, checks the retained/missing partition, process exits and recovery timing, and checks the completed clock and wire reports. A successful result prints `PASS_PUBLISHED_ARTIFACTS`; a detected inconsistency exits 1. Missing dependencies or tools must be resolved before this check can run.

Both 270-second clock observations completed without wall or monotonic regressions, with 47,704,992 and 48,471,168 samples. The retained wire report covers four streams and 2,236 declared Compliance frames, with zero invalid frames, plaintext fixture markers or partial bytes. This is a framing report; independent endpoint signature verification is covered by the separate public cryptographic evidence, rather than by this reader.

The verifier was run on the published candidate and on seven deliberately altered disposable copies: a missing file, modified bytes, an altered typed row with its artifact hash refreshed, conflicting wrapper outcomes, a clock regression with its artifact hash refreshed, an unsafe manifest path and an unlisted original. All seven were refused; [the recorded verifier checks](VERIFIER-CHECKS.json) retain their outcomes. These checks exercise the artifact reader, not the database.

## Original custody and scope

[ORIGINAL-PROVENANCE.json](ORIGINAL-PROVENANCE.json) records each original hash, the published hash and every transformation. Expected records, clock reports, the A wrapper and the wire report are byte-identical originals. Public final/open manifests omit a private binary evidence encoding and an internal API identifier. Local paths in the two reopen manifests are replaced with hashes of their original UTF-8 values, preserving equality comparisons. The B wrapper's executable path and the private test module name are replaced with explicit labels. All remaining typed data, outcomes, counts and times are preserved.

The independent internal audit checked the exact source inventories, raw captures, process and clock receipts, both original publication manifests and the committed peer bytes before preparing these copies. [PUBLICATION-MANIFEST.json](PUBLICATION-MANIFEST.json) establishes the integrity of the public copies. Its hashes are not a signature or an independent attestation of the hosts.

Acceptance applies to this fixed 512-row WordNet GNSE corpus and current admitted native copies. Physical power loss, endurance, historical all-copy coverage, every modality and general throughput need their own tests. No module certification or full deployment approval follows from this run. Private seeds, credentials, stores, authority floors, raw packets, engine source and local path names are excluded.

To run a fresh engine experiment, bring a workload and a recovery or migration constraint to [Ashley at 8Braid](mailto:ashley@8braid.com?subject=8DB%20recovery%20evaluation). We can agree on the operation, injected failure and acceptance evidence.
