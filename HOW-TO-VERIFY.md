# How to verify this evidence

Everything in this repository can be checked by a second evaluator. The parts
that need the 8DB engine are marked; the rest stand on public tools and NIST's
own vectors.

## Algorithm correctness, without the engine

**ML-KEM-1024 and ML-DSA-87 known-answer vectors** are under `kat/`. The
ML-DSA-87 signature-generation and signature-verification vectors carry their
NIST provenance in `kat/ml-dsa-87/SOURCE.md`. To check the implementation:

- Load the vector inputs and the recorded outputs and compare against any FIPS
  203 and FIPS 204 reference implementation, or against NIST's ACVP vectors
  directly.
- The offline suite result is `acvp/Offline-KAT-Report-35of35.txt`; the ACVP
  demonstration-server disposition is `acvp/NIST-ACVP-Disposition-2026-06-16.json`
  and the session transcript is `acvp/Live-Session-Log-744723.txt`.

## Interoperability, with OpenSSL 3.5 or liboqs

The `interop/` folder holds ciphertexts and signatures produced by 8DB and by
OpenSSL 3.5, each intended to be verified by the other, on x86_64 and aarch64.
A `NOTE` in that folder records the exact commands and versions. To reproduce:

- Use OpenSSL 3.5 (or liboqs) to encapsulate to 8DB's ML-KEM-1024 public key and
  confirm 8DB derives the same shared secret, and the reverse.
- Use OpenSSL 3.5 to verify 8DB's ML-DSA-87 signatures over the recorded
  messages, and have 8DB verify OpenSSL's deterministic, hedged and
  context-bound signatures.

## Sizes and latencies

The `bench/` files are release-mode runs; the stored-data transition and head-to-head comparison rows are stated with their scope in `CLAIMS-AND-SCOPE.md` and their raw run records are held in the internal tree, available to evaluators on request. Each records its host,
date, sample size and build profile in its own header. The sizes (record 131 or
185 bytes, key-establishment object 1,616 bytes, ML-DSA-87 signature 4,627 bytes)
follow from FIPS 203 and FIPS 204 and can be checked against those standards
directly. The latency medians are reproduced by the one-command harness inside
the 8DB engine.

## The reading rules for the timing screen

The `timing/` folder holds a dudect-style statistical screen and its
`README.md`. The pre-registered reading lines are: a Welch t at or below 4.5
reads "no leak detected at this sample size", above 10 reads "leak detected", and
between the two reads "inconclusive". A run counts as evidence only if its
positive control (a deliberately variable-time compare) reads above 10 and its
negative control (two identical input classes) does not. Runs that fail their own
negative control are marked invalid and are not quoted. Read the per-run reports
against these rules, not as bare numbers.

## The parts that need the engine

The full transition, a stored dataset re-encrypted from one algorithm generation
to another while reads continue, and its one-command reproducibility harness, run
inside the 8DB engine, which is not open source. An evaluator who wants to run
that end to end can request access to the engine and the harness. The recorded
outputs of those runs are included here and are checkable as they stand; only the
re-execution needs the engine.

Contact: Ashley Dunfield, 8Braid Inc., ashley@8braid.com.
