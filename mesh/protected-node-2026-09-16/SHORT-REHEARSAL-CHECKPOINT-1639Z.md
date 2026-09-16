# Short delivery rehearsal passes its execution and audit checks

**Updated September 16, 2026, 16:39 UTC.** The third short rehearsal completed
2,176 exact synthetic records and passed its separate audit. Independent review
checked the retained results, process and clock observations, resource limits
and cleanup. This closes the short rehearsal on the identified optimized test
build. Main-source integration and the 24-hour mission remain open.

## What completed

The local Linux TCP rehearsal used source `a96ab656c54efcdc6e4dadb24a0386180d81883b`
and wrapper `b611037b4319c71efceafaedd9250fac81d6a23a`. Its sender and receiver
ran in the same process, followed by a separate cold-reopen child. The test used
known synthetic keys and the previously identified optimized executable.

The run's separate pinned verifier checked all 2,176 ML-DSA-87 receipt
signatures. The audit joined receipts to their requests, source completion and
exact receiver content, including the cold-reopened stores. Total payload was
4,083,712 bytes. The disconnected interval was 16.684 seconds; reconnect and
drain took 33.557 seconds. The native observation took 51.785 seconds. These
measurements describe this bounded local rehearsal.

The audit checked the actual native and audit processes' time namespaces and
observed zero namespace offsets for the monotonic and boot-time clocks.
Independent review rehashed 657 retained files
and four container-log readbacks, then checked the result, process identities,
resource limits, original execution-slot custody and completed cleanup. That
review inspected the original signature-verifier result; it did not rerun
cryptography or the engine.

| Identity | Value |
|---|---|
| Test source | `a96ab656c54efcdc6e4dadb24a0386180d81883b` |
| Runtime wrapper | `b611037b4319c71efceafaedd9250fac81d6a23a` |
| Native executable SHA-256 | `6c832cdf4c9d560f3a617a06e9e80caf154997d4517f8d3551b6c1bacc2d8329` |
| Composite result SHA-256 | `64bc3804aee557199ddf56f19703f64a701929c8255783b9d630ad5d1ac96b8f` |
| Original audit SHA-256 | `eb686f6f51cdd59b6d4a2665383cf2f050b441183d83faadbe106772c0dbc47c` |
| Process and clock capture SHA-256 | `cdeba94250b00f214e03c27fc7cb2991ed7e1e6b6447f17c508cc1c6820a758a` |
| Independent review SHA-256 | `5efb10a0f8e6ae8bf8ebc094ea00fb41f2ec3275a3dee5423f3ae4364a05ad1d` |

## Earlier refusals and the next gate

The first attempt refused before engine or container execution because its
Windows guardian could not resolve `Get-FileHash`. The second ran its native
phase, but its audit refused the captured time-namespace identity. Both outcomes
and their cleanup records remain retained. The third attempt passed after the
capture path recorded the actual process namespaces and offsets; the clock
checks remained enforced. The [earlier checkpoint](WORKER-AND-DELIVERY-CHECKPOINT-1556Z.md)
preserves the status before this result.

This source-specific result adds no passes to the corrected `326ab` release
executable's inventory. It establishes neither remote two-host delivery nor
Windows engine qualification. The selected production source still needs its
remaining CI and native gates, normal main integration, and a main-bound
executable containing the required mission harness before the sustained run.
**The 24-hour mission has not started.** Main-built evaluator delivery and
platform acceptance also remain open.

This update publishes a summary of private retained evidence. It adds no raw
mission records, executable or database files to the public replay packages.
Use the [current qualification status](../../QUALIFICATION-STATUS.md) to select
a published verification package or request the matching evaluation records.
