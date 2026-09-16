# Independent live HKDF availability ledger check

This Rust executable checks the evidence from the exact 100,000-record
HKDF-SHA256 to HKDF-SHA384 acceptance campaign. It does not import 8DB or the
producer's acceptance function. The actual product run remains a separate step.

```sh
cargo test --locked --manifest-path Cargo.toml
cargo run --locked --manifest-path Cargo.toml -- EVIDENCE_DIR SOURCE_SHA256 BINARY_SHA256 PRE_CAMPAIGN_ORACLE_SHA256
```

Supply the source inventory and executable SHA256 from the independently
retained build custody, not values copied from an unverified result. An external
supervisor must pin response-oracle.json during initial import/preflight, before
copy/start exists, and retain that receipt outside the fixture. The verifier also
rehashes expected-typed-sections.bincode. The program
prints a JSON report even when acceptance fails and exits nonzero on failure.
Every input file's hash is included in that report. The build used Rust1.94.1.

The contract fixes one request every500ms, two workers, four queued requests per
worker, and a1s scheduled-to-response deadline. The one declared activation
probe uses the normal schedule, reads changed record0 through the authenticated
facade, and includes its25ms consumer hold in the same latency budget. Other
requests cycle through the retained, changed, added and deleted record samples.
ReadyToActivate is a coordination phase, not an index-build or verification scan.

The verifier requires every scheduled tick through the recorded stop, one
outcome per request, immutable issued identities, correct typed response bytes,
source/binary and endpoint joins, nonregressing authority custody, and successful
reads during copying, index construction, actual verification and Active
serving. A child-clock receipt must show a real protected emission callback
overlapping the owner's activation attempt. Child and parent clock epochs are
never compared; the callback duration must fit within the measured parent RPC.

Requests scheduled during the deliberate process outage are counted separately.
An earlier in-flight request qualifies as interrupted only when its ID and
killed endpoint match the kill snapshot, it has no response, and its deadline
had not already expired at the kill. Returned responses are checked for identity
and content even during the outage. Expected retired-source refusals are counted
separately; they never count as successful facade reads.

The retained final operation must report100,000 exact typed/indexed records after
cold reopen,99,998 copied source records, one superseding write, one deletion and
zero failed attempts. This verifier checks that result's consistency; validating
the full native store requires the separate engine run and its retained source
and binary evidence. Hashes establish identity, not independent host attestation.

The control suite covers missing/duplicate/unissued outcomes, shifted or omitted
schedule ticks, hidden dispatch delay, wrong values and generations, expired
pre-kill requests, corrupt outage responses, absent crash evidence and fabricated
activation overlap. A successful synthetic control is not a database result.
