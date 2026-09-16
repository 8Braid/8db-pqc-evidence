# Release worker rejects clock faults; larger rehearsal verifies 16,512 records

**Updated September 16, 2026, 16:06 UTC.** The corrected candidate's retained release
executable passed one positive case and all three injected worker clock-fault
cases. Separately, an optimized test build completed a local delivery rehearsal
with 16,512 exact records. Independent review checked both sets of original
results. Their source, build profile and acceptance scope remain distinct.

## Clock faults on the corrected release executable

The Linux x86_64 `protected-gcm` release executable from source
`326ab0a1333dd7f6f13d4c0fff503f9d5eefaa5e` ran the four cases without a Rust
compiler. The positive case completed two deliveries. Each fault case completed
the first delivery, refused the second offer for the expected reason, and
cold-reopened with one terminal delivery, one pending delivery and one accepted
receiver record.

| Case | Observed outcome |
|---|---|
| Ordinary clock, no preload | Two completed deliveries; cold state contained two terminal and two accepted records, with none pending. |
| Frozen wall clock | The worker refused after its elapsed-time permit expired. The test waited through the actual 30-second deadline; the case took 30.589 seconds. |
| Unavailable wall clock | The worker refused because the authority clock was unavailable. |
| Wall clock below the retained anchor | The worker refused because wall time was outside its retained temporal anchor. |

The supplement completed at 15:42:09 UTC. Independent review replayed the
unchanged named-result parser and semantic checks, joined the worker's process,
thread and fault generation to the clock witnesses, checked control history and
cold-state markers, and verified the runtime and process shutdown. It rehashed
32 execution originals before and after review and checked all 82 retained
copies against their originals. The original probe's `NOT_ESTABLISHED` label
remains intact; the bounded result is established by this separate validation.

| Identity | Value |
|---|---|
| Source tree | `372602bc07b1233008184c55246d7b6702646ec3` |
| Release producer run / attempt / job | `35102476544` / `1` / `104815090601` |
| Retained release executable SHA-256 | `7936623d4e83a8271866eb3b54deefb968acf879006fed8689da02ad86135711` |
| Supplemental result SHA-256 | `ed0df8025bf2673d01214cf7e75384136236dc9401d6e17bfe190f8b84fafc40` |
| Independent result review SHA-256 | `912f3e6f8ad2d090ce1b462f47a6cdfdcf1c31d7f3af4f2992c00e2aa95a5df9` |
| Private source-free replay archive SHA-256 | `31d70d2a462442772a0f365e47326221f87b7fce98187089d3468d476520b749` |

The pinned clock shim was built separately from the exact candidate's C source.
Execution used a non-root user, no network, a two-CPU quota and a 16 GiB memory
limit. These four cases establish bounded worker behavior under process-local
clock injection. The retained bundle requires its recorded Linux runtime; it
does not establish arbitrary-host portability or sustained 24-hour operation.

## Larger local delivery rehearsal

The separately identified optimized test build delivered 16,384 queued records
plus 128 distinct records submitted during draining. All 16,512 records,
totalling 30,998,528 payload bytes, matched the expected content. Independent
review freshly verified all 16,512 ML-DSA-87 receipt signatures and joined each
receipt to its request, source completion and cold-reopened stores.

The maximum observed in-flight count was eight, with 16,512 send attempts and
16,512 issued receipts. The disconnected interval was 136.165 seconds; reconnect
and drain took 257.889 seconds. The complete native observation took 406.200
seconds. These are measurements of this synthetic local rehearsal, without a
matched throughput comparison. The worker's incomplete-scan pending gauge
remained 16,512; the separate exact durable cold audit established zero pending
records.

| Identity | Value |
|---|---|
| Test source | `a96ab656c54efcdc6e4dadb24a0386180d81883b` |
| Source tree | `21ae7e20efd9106680b546f89c0dc1eea29c090a` |
| Runtime wrapper | `b611037b4319c71efceafaedd9250fac81d6a23a` |
| Native executable SHA-256 | `6c832cdf4c9d560f3a617a06e9e80caf154997d4517f8d3551b6c1bacc2d8329` |
| Build profile | Test `opt_level=2`, debug assertions and overflow checks enabled; features `protected-gcm,two-host-probe`. |
| Original audit SHA-256 | `d3c83582db142fbfcd06d0d6b3a6b2945a965bcc91b873eb6c651529b8c7dc67` |
| Independent rehearsal review SHA-256 | `50a0c6ef5e922aa6d5a1ae93f33f3377a1f1ad1a9fdeef1d4462e947cc4aa8f2` |
| Fresh independent receipt replay SHA-256 | `8cc5e0039def079f635e883b0b99aca322141c153955e0038dce404817f715ed` |

The rehearsal used known synthetic keys, Linux TCP loopback in the same process,
and a separate cold-reopen child. It adds no tests to the previously recorded
177 active controls and three faults. The earlier wrapper README provenance
exception remains in the source review. This test build's results are separate
from qualification of the retained `326ab` release executable above.

## Remaining release work

The corrected candidate's default native job and both release-profile jobs are
complete. The remaining native producer roles, CI review, normal main merge,
main-built evaluator package and platform acceptance remain open. Windows
package delivery is still pending.

The first startup attempt for the next short mission refused before engine or
Docker execution because the Windows guardian could not resolve `Get-FileHash`.
Its child was reaped and its execution lock released. After that dependency
was corrected, the second attempt completed its native phase in 51.033 seconds.
Its external audit refused an unexpected native time namespace; both containers
stopped and cleanup was confirmed. The owner is correcting the actual offset
capture while preserving the clock checks. Neither attempt is an accepted short
mission. **The 24-hour mission has not started.** Earlier source-specific
failures and the [15:34 checkpoint](NATIVE-CHECKPOINT-1529Z.md) remain dated
observations; this update does not rewrite them.

This page summarizes private retained evidence. It publishes no executable,
stores or raw mission records. The source-free four-case bundle and matching
records can be supplied for an agreed evaluation. Use the
[current qualification status](../../QUALIFICATION-STATUS.md) to select a public
replay package or request the engine and harness for a fresh test.
