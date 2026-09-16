# Delivery receipts, recovery and corrected release checks

**September 16, 2026, 14:38 UTC.** The corrected engine candidate has passed its
default release checks. A separate optimized test build has also completed a
2,176-record delivery rehearsal whose receipts and recovered content were
independently verified.

## Corrected default release

| Check | Recorded result |
|---|---|
| Native Rust protected-node tests | All 44 named tests passed. |
| Generated clients | Indexed migration and recovery checked 128 records; delivery and receiver clients checked two records each, including abrupt exit and reopen. |
| Local sender and receiver | Each role retained 2,048 exact records and 3,842,560 payload bytes. All six produce, reopen and final observations matched the fixture. |
| Independent review | 80 consumed original files rehashed before and after review against a closed 5,265-member inventory. Original test, generated-client and recorded-content validators passed. |

This job uses the default research-SIV profile. Its result is separate from the
AES-GCM release job and the full native producer, which remain in qualification.
The local role checks use the required same-boot temporal profile. They do not
establish delivery between physical hosts or a completed 24-hour mission.

| Identity | Value |
|---|---|
| Source | `326ab0a1333dd7f6f13d4c0fff503f9d5eefaa5e` |
| Source tree | `372602bc07b1233008184c55246d7b6702646ec3` |
| GitHub run / attempt / completed job | `35102476544` / `1` / `104815090392` |
| Rust test executable SHA-256 | `00b5d445113c6f3d01b17932f0370721f4aceca096bef78c7f8e99f632742d7f` |
| Original release receipt SHA-256 | `278cdf833c99a3ee56b50d011daed5818bc05fb20414c62b62646fc47cbd4852` |
| Closed inventory SHA-256 | `08512167c43d6a74757b3138812c6a6225ed0591bbb32fbcb23e1fb78d08e0b4` |
| Independent review SHA-256 | `023ac34c51043406724e6d08a6d5d95bc8e7a608812033a1861431f20ff9a5db` |

## Authenticated delivery and cold recovery

The mission test overlay adds a harness to the selected candidate without
changing its production code. Its optimized Rust test build retains debug
assertions and overflow checks. It passed 177 active controls: 155 collector
cases, seven tracked feature helpers and 15 streaming-mission controls. Three
separately executed clock-fault tests also passed, with observed process/thread
and clock witnesses and the expected cold-owner checks.

The delivery rehearsal retained 2,048 offline records and accepted another 128
during draining. All 2,176 were accounted for in the original signed receipts,
source completion records and the receiver's exact content. A separate cold
process found no pending records. Independent replay rebuilt the expected
requests from the original fixtures and freshly verified all 2,176 ML-DSA-87
signatures using the pinned portable Rust verifier.

| Rehearsal | Records | Sender attempts | Reconnect | Whole native process |
|---|---:|---:|---:|---:|
| Initial optimized run | 64 offline + 16 during draining | 81 | 30.790 s | 39.524 s |
| Larger run, 128 in flight | 2,048 offline + 128 during draining | 2,202 | 107.734 s | 126.519 s |
| Same larger workload, eight in flight | 2,048 offline + 128 during draining | 2,176 | 33.858 s | 52.622 s |

Changing the maximum number of in-flight deliveries from 128 to eight reduced
the observed sender scans from 5,005 pages to 26 and completed every record with
one send attempt in this run. The source, executable and fixtures stayed the
same; this was the sole semantic plan change. Independent replay again verified
all 2,176 signatures and the original content, completion and cold-state joins.
These single-run measurements guide the next scale test; they are not a general
throughput guarantee.

Retries and scan costs remain part of the evidence. The final worker scan was
incomplete in both larger runs and reported 22 and 2,170 pending respectively.
Those partial scan samples are distinct from completion accounting: each
separately verified cold state established zero pending. The initial optimized
run also drained more slowly than the
earlier debug rehearsal; compiler optimization alone did not establish faster
delivery.

These were same-process Linux TCP loopback experiments with known synthetic
inputs and a separate cold process. They qualify that bounded workload and
profile. Physical two-host, sustained-operation and release-binary tests have
their own gates.

| Identity | Value |
|---|---|
| Test overlay source | `a96ab656c54efcdc6e4dadb24a0386180d81883b` |
| Source tree | `21ae7e20efd9106680b546f89c0dc1eea29c090a` |
| Optimized Rust test executable SHA-256 | `6c832cdf4c9d560f3a617a06e9e80caf154997d4517f8d3551b6c1bacc2d8329` |
| Original 2,176-record audit SHA-256 | `257724f1c3992b7d6ee76f1b6dbcf069035fbc6ea7da043ed58c6c486ba19079` |
| Independent review SHA-256 | `9adf0d68c59e8dbd039825957eddc3d04580b3c8a4da8ca5c3470c52b7b180f1` |
| Original eight-in-flight audit SHA-256 | `375a023bc613976070a577627d416f7aff819ecf135a7c1157b0e3de0e6919de` |
| Fresh eight-in-flight receipt-verifier output SHA-256 | `665ad351ba4878832012c006993c8dab972a03317c357a40967875097adb479d` |
| Independent eight-in-flight review SHA-256 | `5d7370efffb0688cd485de2db21b38560e61b90563593289237145ce0f38cdf8` |
| Mission wrapper source | `b611037b4319c71efceafaedd9250fac81d6a23a` |

The wrapper review matched all 22 executable Python files to their retained
source and Git identities. Its control-time README differs from the published
README; that documentation provenance exception is retained. The wrapper is
qualification tooling around the native Rust engine and verifier.

## Next qualification

The original clock tests first reached their required worker refusal but failed
because the cold audit allocated a new write context. The corrected fixture
preserves the initial reservation and verifies that cold reading leaves the
authority revision unchanged. The original failures remain retained.

The remaining work includes the selected candidate's AES-GCM release and full
native producer, the release-executable fault supplement, evaluator delivery,
main-built and platform checks, and the full 86,400-second mission. The longer
mission is being prepared on a host with capacity to retain its original
evidence; it has not started.

These summaries identify private execution records available for evaluation.
They do not add a publicly replayable native engine package. Existing public
crypto, migration and two-host replay instructions remain in the
[current qualification status](../../QUALIFICATION-STATUS.md).
