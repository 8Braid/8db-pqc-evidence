# Claims and scope

Every number here carries its scope. Where a claim needs a qualifier, the
qualifier travels with it. This document is the public subset of 8Braid's
internal claims-grounding record; it omits nothing that changes a number and
adds nothing that is not measured.

## Algorithm correctness

- **ACVP demonstration server.** ML-KEM-1024 keyGen and encapDecap, and
  ML-DSA-87 sigVer, passed NIST's ACVP demonstration server, session 744723,
  isSample = false, 2026-06-16. Evidence: `acvp/NIST-ACVP-Disposition-2026-06-16.json`,
  `acvp/Live-Session-Log-744723.txt`. This is algorithm-correctness evidence on
  the demonstration server, not module validation and not a certificate.
- **Offline known-answer tests.** 35 of 35 vectors pass. Evidence:
  `acvp/Offline-KAT-Report-35of35.txt`, and the vectors under `kat/`.
- **Interoperability.** ML-KEM-1024 and ML-DSA-87 objects interoperate with
  OpenSSL 3.5 in both directions: 8DB decapsulates OpenSSL's ciphertext to
  OpenSSL's shared secret and the reverse; OpenSSL verifies 8DB's signature and
  8DB verifies OpenSSL's deterministic, hedged and context-bound signatures. Both
  directions, x86_64 and aarch64. Evidence: `interop/`.

## Sizes

- A 115-byte record encrypts to **131 bytes** with AES-256-GCM-SIV whether the
  page key was established by a classical KDF or by ML-KEM-1024 plus X25519. The
  ciphertexts are the same size because the per-record AEAD is identical; the 16
  added bytes are the AEAD tag. The compliance build (AES-256-GCM with a 48-byte
  key-commitment tag) makes the record 185 bytes.
- One key establishment's KEM-specific object is **1,616 bytes**: a 1,568-byte
  ML-KEM-1024 ciphertext plus a 48-byte wrapped key. Against the 13.1 MB that
  100,000 encrypted 131-byte records occupy, that object is 0.0123 percent. It is
  the per-established-key object, not total encryption overhead.
- An ML-DSA-87 signature is **4,627 bytes**. Stored once per batch over a
  SHA-384 Merkle root with a per-record inclusion proof, not once per record. A
  single per-record signature would be a 4,023 percent overhead on a 115-byte
  record, which is why the design signs the batch boundary instead.

## Latency

- Same-AEAD microbenchmark: median per-record encrypt and decrypt times for
  classical and post-quantum key establishment agree within one percent on three
  of the four hosts measured; the July run's decrypt medians differ by 100 ns
  (14 percent) with no explanation yet, and that host is named in the data. Each
  number in `bench/` carries its host, date, sample size and build profile.
- The compliance profile costs about a microsecond more per record than the
  research profile on both architectures, driven by the HMAC-SHA-384
  key-commitment tag path. See `bench/`.

## Migration

- **The transition that exists today.** 8DB re-encrypts every stored record from
  a page key derived by one KDF (HKDF-SHA256) to a new generation under another
  (HKDF-SHA384), verified record by record and activated atomically, with the old
  generation still readable during the copy. Measured at 100,000 records on a Linux x86_64 host on 2026-09-14; the raw run
  records are held in the internal evidence tree and are available to evaluators
  on request.
- **Head-to-head record sizes.** The composed Category 5 storage of one
  ML-DSA-87 signature stored per row from the application layer, with encryption
  at rest, was measured on PostgreSQL and MongoDB for the same record set
  (PostgreSQL 18.4 with pgcrypto 579,452,928 bytes, MongoDB 500,445,184 bytes;
  one Windows host 2026-07-19, reproduced within 1.2 percent in Linux docker on
  2026-09-14 and checked by a second party). This is a size
  comparison of a per-record-signing composition, not a benchmark of those
  systems' own features.

## Timing

- At 50,000 measurements per class on two hosts (a Ryzen 9 9950X3D on Windows
  x86_64 and a Graviton3 on Ubuntu aarch64), the dudect-style suite detected no
  leak in ML-KEM-1024 decapsulation (ciphertext, implicit rejection and
  secret-key dependence), ML-KEM key decode, ML-DSA-87 hedged signing, and the
  page AEAD, with the whole-key ML-KEM reading attributed to public-seed matrix
  expansion by a pre-registered rule. This is a statistical screen on the named
  hosts, not a proof of constant time and not a hardware or formal analysis.
- **The AEAD path rests on the library and the hardware.** The AES-256-GCM cases
  run through aws-lc-rs (FIPS 140-3 validated) on hardware AES (AES-NI on the x86
  hosts, the ARMv8 AES extensions on Graviton3) with a carryless-multiply GHASH
  and no secret-dependent branch, which is data-independent by CPU design; the
  timing cases corroborate this on quiet hosts. One AEAD reading on a loaded x86
  host is a sub-two-nanosecond measurement artifact, clean on ARM, recorded as
  such in `timing/`.

## What is not claimed

- No CAVP certificate and no CMVP module validation exist. "Compliance" names the
  CNSA 2.0 algorithm set, never a validated module.
- The default at-rest AEAD is AES-256-GCM-SIV (RFC 8452), which is not a
  NIST-approved mode. The `fips` build uses AES-256-GCM (SP 800-38D) through
  aws-lc-rs.
- Category 5 confidentiality is data-centric: every record is sealed at write and
  can travel as the sealed object, so the content's confidentiality does not
  depend on the transport. The TLS layer today negotiates a Category 3 hybrid
  group; a Category 5 payload seal is separate from that.
- The ML-DSA-87 signature path uses the deterministic variant in production; the
  timing suite's hedged-signing rows do not describe it.
