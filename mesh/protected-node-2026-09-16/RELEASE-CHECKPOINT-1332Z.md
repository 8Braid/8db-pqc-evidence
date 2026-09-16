# Protected-node release checkpoint

**September 16, 2026, 13:32 UTC.** The AES-GCM release job completed at
13:19:34 UTC. It passed 67 named protected-node tests, including the ordinary
same-boot delivery-worker positive and five receipt-observer controls.

| Executed or reviewed boundary | Recorded result |
|---|---|
| Protected-node tests | 67 passes from a 70-test protected inventory. Three injected clock-fault tests were explicitly deferred in this job. |
| Generated clients | Indexed and AES-GCM clients each checked 128 records; delivery and receiver clients each checked two records, with retained abrupt-exit and reopen observations. |
| Local mission preparation and recovery | Sender and receiver roles each retained 2,048 exact records, totaling 3,842,560 payload bytes per role. All six recorded produce, reopen and final observations matched the fixture. |
| Independent review | 84 consumed original files were rehashed before and after review against a closed 7,395-member inventory. The original named-test parser, generated-client validators and six recorded-content checks passed. |

The mission used `linux-same-boot-zero-offset-v1`. These local role checks do
not establish authenticated delivery between physical hosts or a completed
24-hour mission. The independent reviewer replayed evidence validators; it did
not rerun the engine. The complete release workflow was later canceled when
the corrected successor was selected. Its completed AES-GCM job remains a
successful, source-specific result.

## Exact identities

| Item | Identity |
|---|---|
| Tested source | `9467eb1d2cb9aea2625f3e6b6f8d189d447c4172` |
| Source tree | `0288528b2b947c3a810d82ce1ec590707ed7704a` |
| GitHub run / attempt | `35098543316` / `1` |
| Completed job | `104801857319`, Release node / AES-GCM Python qualification |
| Native Rust test executable SHA-256 | `9c30d5a46f67ebe26e5518f8f44c50e9850c67f87bbf22e1a75d0e75635251f9` |
| Original release receipt SHA-256 | `57b121cde9735a2e24568d3039c8c21e4765984bb992c63c9dd2cfe2c9b87b4e` |
| Closed original inventory SHA-256 | `7aa8a21bca77507d3b7e8ec863b058664e5dceae6fded18a371e027eaa50d354` |
| Independent review SHA-256 | `bb669208fd5a68b7b9b8dac51ee89c122a5bfdd51adb5e6f796add1a1ad704c1` |

These are internally retained execution records, identified here for evaluation
and custody. This checkpoint publishes a result summary; it adds no raw engine,
executable or test-log package. The Python job name describes the generated
client qualification tooling; the database engine and native tests are Rust.

## Correction and next qualification

Source `326ab0a1333dd7f6f13d4c0fff503f9d5eefaa5e` changes one Linux test
fixture. Its cold audit now reads historical encrypted data without allocating
a new write context. Setup retains the initial write reservation, and cold
reopen asserts the retained authority revision. All four original test bodies
and their fault checks remain unchanged; production code is identical to the
tested predecessor.

The corrected source passed static analysis and a separate 48-check retained
evidence audit. All 66 portable static-evidence members were also independently
rehashed. Those checks establish the correction's source and static results.
Execution of the corrected positive and injected-fault cases, full native
qualification, evaluator assembly and the real 86,400-second mission remain
tracked in the [current qualification status](../../QUALIFICATION-STATUS.md).
Earlier negative runs remain in the internal evidence record.
