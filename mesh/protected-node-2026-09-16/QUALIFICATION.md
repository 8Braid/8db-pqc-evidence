# Protected-node capability and qualification record

**Current update:** see [qualification status](../../QUALIFICATION-STATUS.md).
The September 16, 06:56 UTC snapshot below is retained as history. Its native
producer status has since advanced; the successor engine has its own source
identity and qualification gates.

**Snapshot: September 16, 2026, 06:56 UTC.** An implemented path, an executed
test and a qualified release answer different questions. This table keeps each
claim attached to the source, profile and evidence that supports it.

The current architectural source review is bound to
`fa4911581ba4dc901be5582730c4c2813d221c16`, tree
`d8b87230e9d57b20604f08d05a93ff7b89c36615`. Its complete producer qualification
and the broader successor release remain open at this snapshot. Publishing
this explanation does not declare either release complete.

| Capability or question | Evidence available | Boundary and remaining work |
|---|---|---|
| Message protection before network transit | The [RequiredMeshNode source review](README.md) identifies recipient-specific ML-KEM-1024 establishment, AES-256-GCM protection, ML-DSA-87 authentication, replay checks and current-authority boundaries at `fa491158…`. | Source inspection establishes the implemented construction. It is not an independent protocol proof or an all-constructor/all-modality qualification. |
| Admitted data schemas | At the reviewed source, the RequiredMeshNode importer accepts only canonical native WordNet lemma sections under `lexical/wordnet-lemma` or `wordnet/lemma`. Other schemas are refused before the replicated mutation. | This is an enforced capability limit, beyond the historical fixture's test scope. The generic byte-sealing wrapper does not establish chat or arbitrary multimodal integration through this composition. Separate application messaging and MLS paths are outside this review. |
| Local storage roots and network keys | The inspected typed-replication path opens content at the authorized recipient and reseals locally; it does not transmit the sender's storage root or at-rest data key. | Endpoint secrets and trusted bootstrap remain necessary. Production provisioning, rotation, backup/escrow and later endpoint-key compromise need explicit deployment acceptance tests. |
| Recovery on two physical hosts | [Public 512-record package](../required-two-host-2026-09-15/), source `7db11180d1a71f7d95cd9698c72ffd1f044d1e6e`: Required native GCM storage plus Compliance message channel; both hosts cold-finished with all 512 exact typed records. | The public Rust verifier checks recorded content, custody and recovery observations. Fresh engine execution, physical power loss, endurance and every modality remain separate tests. This earlier source is not relabeled as `fa491158…`. |
| Current native producer | At `fa491158…`, the default role completed 742 native controls plus seven integration controls and both seven-case evaluator modes. The Required-mesh role completed 30 named controls plus its owned crash/cold-recovery exercise. | These are internally retained executed results, not new public raw packages. The AWS-LC native role and dependent evaluator roles are still pending; the whole producer has not passed. No evaluator assembly is qualified by partial role success. |
| Stored-data migration with scheduled reads | [Public four-CPU supplement](../../migration/live-hkdf-2026-09-15/four-cpu-2026-09-16/) retains HKDF-SHA256 to HKDF-SHA384 runs at `206f1a6…`, `0280898…`, `41ca1c8…` and `9c5ae6c…`, with AES-256-GCM-SIV unchanged. Every run cold-verified 100,000 exact typed records and the index. | The unchanged auditor rejected the first three for 2/843, 26/882 and 9/724 missed nonexcluded deadlines; the last passed with 0/689. The last change added diagnostics to equivalent product inputs and does not prove the earlier misses are resolved. The one-second budget and two requests per second are declared application criteria. |
| Receipt-preserving delivery and migration | A separately tested source at `9a8806499531781b0a6a2bd7af7ff4dd0cfd9bb2` adds reserved-namespace guards after a counterexample showed that generic owner mutation could remove pending capture state. A combined successor is prepared. | This is a delivery-state integrity issue, separate from message encryption. The counterexample was not executed against the reviewed `fa491158…` baseline; analogous unguarded paths exist there. Combined exact-source qualification, durable quota behavior and the broader delivery work remain open. Historical source-specific tests do not qualify the combined successor. |
| Key erasure and side-channel assurance | Exact-provider inspection found a missing ML-KEM zeroize feature in the reviewed source. A correction and related standalone signing-key lifetime changes are prepared with independent source review. The [published timing record](../../timing/README.md) retains its earlier findings. | Runtime qualification of the key-lifetime changes is pending. Enabling key-type destruction does not prove erasure of every temporary or copy. The historical x86 compliance timing failure and ARM inconclusive case remain unresolved; their source/profile scope must be retained. The locked RustCrypto crate documentation reports no independent audit of those implementations.[1] |
| Algorithm and module validation | [ACVP demonstration results and interoperability](../../CLAIMS-AND-SCOPE.md#algorithm-correctness-and-interoperability) provide bounded correctness evidence. The selected message algorithms use Category 5 parameter sets. | No 8DB CAVP certificate or CMVP module validation is held for this implementation. A build feature named `fips` and an underlying provider certificate do not validate the complete PQC construction or deployment. |
| Cost and network compatibility | [Published same-cipher](../../bench/p38/2026-09-14/) and [provider-profile](../../bench/profile-cost/2026-09-14/) measurements identify their actual component costs. The inspected relay carries framed messages over TCP. | The record does not establish whole-database or peer-network cost parity. Enterprise TLS/HTTP adapters, routing topologies, throughput and workload latency require their own matched tests. |

## How to use this record

Public artifact replay checks the retained observations and their declared
acceptance conditions. It does not execute the private engine or substitute
for qualifying a changed build. The public packages link their raw inputs,
source identifiers, executable hashes and verifiers where available. The two
current-source review documents contain no private engine source, runtime
stores, private keys or private test logs.

For an evaluation, record the chosen engine source and feature profile,
authorized endpoints, key-custody model, workload and failure conditions before
execution. Keep message confidentiality, local storage, migration availability,
delivery-state integrity and resource limits as separate acceptance criteria.
Update this dated record when those gates close; retain the earlier outcomes.

[1] Exact-version upstream documentation: [ML-KEM 0.2.3](https://docs.rs/ml-kem/0.2.3/ml_kem/)
and [ML-DSA 0.1.0-rc.9](https://docs.rs/ml-dsa/0.1.0-rc.9/ml_dsa/).
This describes those implementations' documented audit status, not all versions
or all software from their maintainers.
