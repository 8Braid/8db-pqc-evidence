# Protect messages before handing them to the network

**Participating 8DB nodes can establish post-quantum message protection at the
endpoints that hold the data.** In the RequiredMeshNode composition reviewed
here, the network carries a recipient-specific encrypted message. The receiving
node opens it and stores its typed content under its own local protection.

This explanation is bound to engine source
`fa4911581ba4dc901be5582730c4c2813d221c16`, reviewed September 16, 2026. It
describes the inspected implementation and its trust boundaries. The
[dated qualification matrix](QUALIFICATION.md) separates this source review
from executed tests, historical public measurements and pending release work.

**Current admission scope:** this RequiredMeshNode profile accepts only
canonical native WordNet lemma sections, with the schema paths
`lexical/wordnet-lemma` or `wordnet/lemma`. The importer enforces this allowlist
before proposing a replicated mutation. It is an implementation limit, not
just the choice of test data. The underlying message-sealing wrapper accepts
opaque bytes; that generality does not qualify chat or other application
schemas through this protected-storage composition. Separate application
messaging and MLS paths require their own source and qualification review.

## The path between two authorized nodes

| Step | What happens |
|---|---|
| Admit the operation | The sender checks current authority and the typed section. Peer identities and public keys must already be trusted; an announcement does not enroll a new identity. |
| Seal for the recipient | A fresh ML-KEM-1024 encapsulation establishes a message secret. HKDF-SHA384 derives the message key; AES-256-GCM with the selected provider's key-commitment protection encrypts the payload. |
| Authenticate the frame | ML-DSA-87 authenticates the sender and a descriptor binding the recipient, session, channel, suite, sequence, ciphertext digest and length. Context is also bound through authenticated encryption. |
| Carry encrypted bytes | The byte transport forwards the protected frame. A relay without the inner keys can route it without opening its payload. |
| Verify, open and apply | The recipient verifies the pinned identity, signature, session challenge, suite and replay conditions, then authenticates and opens the payload. Current authority is checked at admission and again at durable persistence/application boundaries. |
| Protect the local copy | The receiver seals the replicated records under local storage protection and applies typed content with the durable replication watermark. |

The transmitted payload is newly encrypted endpoint content. It is not the
sender's unchanged on-disk ciphertext. This path does not send the sender's
storage root or at-rest data key to the peer. Separately rooted nodes can
therefore hold the same logical records without sharing a storage root.

The inspected Required storage profile selects HKDF-SHA384, AWS-LC AES-256-GCM,
HMAC-SHA384 native anchoring, the required lexical index, committed operations
and durable GCM key-instance allocation. The claim belongs to this composition;
a generic protected-storage constructor still leaves transport selection to
its caller.

## What the network learns

Breaking an outer classical connection would, by itself, leave the attacker
with the inner protected message. That conclusion assumes the message
cryptography and endpoint keys remain secure. It is an architectural consequence
of the inspected layering, not a new cryptographic security proof.

The concrete relay uses TCP and custom framing. Addresses, node/channel
identifiers, message lengths, timing and routing remain observable. Network
failure or denial of service can still prevent delivery. An intermediary that
cannot decrypt the payload cannot perform general plaintext content inspection;
that inspection belongs at an authorized endpoint or an explicitly trusted
gateway.

Byte-preserving routing can carry the frame without understanding its inner
cryptography. An enterprise requiring outer TLS/mTLS or HTTP-only egress needs
a suitable adapter and tested configuration. This note does not establish
interoperability with every proxy, firewall or load balancer. The physical
recovery experiment below used its declared network configuration.

## Keys, providers and custody

Local data keys are derived from secret root material and context. The reviewed
composition operates in software without a required HSM or cloud key-service
call. It still depends on trusted bootstrap, entropy, pinned identities and
protection of secrets at the endpoints. An authorized receiving node can read
the plaintext; a customer-operated cloud node remains an endpoint whose host
must be trusted.

The selected paths reuse RustCrypto `ml-kem` 0.2.3 and `ml-dsa` 0.1.0-rc.9,
with AWS-LC supplying the selected symmetric operations. These are embedded
third-party implementations. Software deployment without an external key
service does not remove provider dependencies or the need for implementation
assurance.

The recipient retains its decapsulation key for the endpoint lifetime. Fresh
encapsulation per frame does not establish forward secrecy against later
compromise of that retained key. A per-message ratchet or live key-rotation
schedule was not established by this source review. Replay protection and
key-erasure assurance are distinct requirements.

ML-KEM-1024 and ML-DSA-87 are NIST Category 5 parameter sets selected for their
respective roles in NSA's CNSA 2.0 suite.[1] Product conformance, protocol
qualification and cryptographic-module validation require additional evidence.
The [qualification matrix](QUALIFICATION.md) records the assurance work still
open.

## Evidence and the next evaluation

The [two-workstation recovery record](../required-two-host-2026-09-15/) preserves
a historical Required-profile run at source `7db11180…`: both hosts finished
with 512 exact typed records after a receiver process kill and same-store
reopen. Its public Rust verifier checks the retained artifacts. Its wire report
checks declared frame structure and absence of fixture plaintext markers;
it is not an independent cryptographic verification of every recorded frame.

Each application frame in the reviewed composition performs fresh encapsulation
and signing. The [same-AEAD record measurements](../../bench/p38/2026-09-14/)
describe a different operation. A matched durable peer workload is still needed
to measure message bytes, setup and rotation, CPU, throughput, latency and
recovery costs.

Bring a representative dataset, participating endpoints and a recovery or
latency constraint to [Ashley at 8Braid](mailto:ashley@8braid.com?subject=8DB%20protected-node%20evaluation).
We can define the failure conditions, key-custody assumptions and evidence
needed for that deployment decision.

[1] NIST [FIPS 203](https://csrc.nist.gov/pubs/fips/203/final) and
[FIPS 204](https://csrc.nist.gov/pubs/fips/204/final); NSA's
[CNSA 2.0 FAQ, December 2024 version 2.1](https://media.defense.gov/2022/Sep/07/2003071836/-1/-1/1/CSI_CNSA_2.0_FAQ_.PDF).
Category 5 is an algorithm security category, not a database certification.
