# Workpack 05 - Account Issuer Key and Store Custody

<!-- agent-capsule -->

> Plan: `protected-capability-custody-foundation-plan`
> Workpack: `05-account-issuer-key-and-store-custody`
> Kind: current-v2 Account issuer owner and protected broker composition.
> Proves: the reviewed ownership, wire, source/test topology, and the exact
> remaining implementation blocker.
> Does not prove: a constructible Account admission, external enrollment,
> operational signing, test results, proof, READY, or DONE.

<!-- /agent-capsule -->

## Purpose

Own the broker-side private composition between Protected Custody and the
Account currentness transaction. The current implementation already contains
the Account-owned `crates/account-issuer-owner` crate, typed protected
protocol/client/broker RPC, the family-owned single SQLite currentness/receipt
host, TPM-native P-256 source, Parent Runtime facade, and real test files.

This workpack does not own Cloudflare's Worker consumer or D1 state. Those
current-v2 roots belong to Cloudflare WP06. It also does not move Account
authority into the broker: family-identity-core remains the owner of
`VerifiedAccountIdentityAuthority`, the Account repository, and the single
`BEGIN IMMEDIATE` transaction/currentness host.

## Live-call correction - 2026-08-29

The earlier reviewed-implementation claim is withdrawn. File presence and
fail-closed helpers do not complete the owner seam:

- `BrokerAuthorizedClientTranscript` is minted and revalidated by Protected
  Custody, but `protected-capability-custody-broker/src/windows_ipc/peer.rs`
  drops it before Account issuer dispatch;
- `AccountIssuerOwner::authorize_protected_request` is intentionally
  fail-closed because no owner-issued Account admission exists;
- the fixed Account issuer mount is unavailable;
- the protected enrollment record binds OS peer/process/token/image/service/TPM
  state, but carries no Account/provider/service-binding identity or Account
  key generation;
- family-identity-core cannot import Protected Custody to manufacture the
  admission without creating a crate cycle, and its admission fields are
  intentionally private.

The current source is therefore a durable fail-closed substrate, not a complete
Protected-to-Account producer.

## Required source outcome

The source order is:

```text
Protected WP01 foundation
-> WP02 external Enrollment/SCM/TPM owner binding
-> WP03 hardware monotonic currentness
-> WP04 retained fixed-pipe peer/session transcript
-> WP05 private transcript-to-Account admission and owner consumption
-> Account WP08 sealed authority/currentness contract
-> Account WP09 family lifecycle composition
-> Cloudflare WP06 authenticated delivery/current-key consumer
```

WP05 must preserve the authenticated transcript through broker dispatch and
consume one opaque, non-cloneable, request-scoped Account admission. That
admission must bind exact peer/session provenance, Account/service binding,
request correlation and idempotency, current public-key identity, enrollment
and authority generations, currentness, and revocation. It becomes invalid on
peer, enrollment, service-key, authority, session, or generation drift and
must reconcile the issuer/outbox after restart.

WP02 owns the immutable external Account binding that makes such an admission
authoritative. WP03 owns hardware currentness. WP04 owns transport and retained
OS provenance. WP05 owns only the private bridge into
`account-issuer-owner` and the one family transaction facade.

## Non-negotiable boundary

The following are invalid:

- public raw fields or a public admission constructor;
- boolean, closure, static-key, environment, request-header, or caller-selector
  authorization;
- a family-to-Protected dependency cycle;
- a second Account database connection or direct access to
  `custody.sqlite`;
- caller-supplied key, path, SQL, signer, generation, identity, or receipt;
- software key fallback, mock owner, no-op provider, or in-memory authority.

Unsupported TPM capability or absent external enrollment stays fail-closed.

## Frozen v2 contract

The selected wire remains self-contained ECDSA P-256 v2:

- producer `ocentra.account-authority-producer.v2`;
- audience `ocentra.account.authority.v2`;
- service `ocentra.account-authority-producer.cloudflare.v2`;
- algorithm `ecdsa-p256-sha256-p1363`;
- canonical 65-byte SEC1 public key and exact 64-byte low-S P1363 signature;
- algorithm-aware `sha256:ecdsa-p256:<hex>` key identifiers;
- protected envelope kinds `AccountIssuerRequest` and
  `AccountIssuerResponse`;
- `IssueCurrentAuthority` and `AcknowledgeReceipt` as the initial
  cross-process operations; verification remains owner-local.

Ed25519 v1 remains historical parse/verify only. Account SQLite owns
authoritative key, receipt, and outbox state. D1 owns public verifier
currentness/CAS and inbound idempotency only.

## Current production roots

The bounded owner seam is concentrated in:

```text
crates/account-issuer-owner/src/rpc.rs
crates/account-issuer-owner/src/signing.rs
crates/account-issuer-owner/src/startup.rs
crates/account-issuer-owner/src/recovery.rs
crates/protected-capability-custody-broker/src/windows_ipc/peer.rs
crates/protected-capability-custody-broker/src/account_issuer.rs
crates/protected-capability-custody-broker/src/account_issuer_rpc.rs
crates/protected-capability-custody-core/src/account_issuer_signing.rs
crates/family-identity-core/src/account_identity_authority_issuer_client.rs
crates/parent-runtime-core/src/account_issuer_owner.rs
```

The wider code-map retains registration, protocol, repository, FFI, and
manifest hosts needed by these files. They are topology, not a readiness claim.

## Current real test roots

All seven Protected/Account test roots are present but unexecuted:

```text
crates/account-issuer-owner/tests/account_issuer_owner.rs
crates/protected-capability-custody-protocol/tests/account_issuer.rs
crates/protected-capability-custody-client/tests/account_issuer.rs
crates/protected-capability-custody-broker/tests/account_issuer.rs
crates/ocentra-protected-capability-custody-windows-ffi/tests/account_issuer_p256.rs
crates/family-identity-core/tests/contract/account_identity_authority_issuer_transport.rs
crates/parent-runtime-core/tests/integration/account_issuer_owner.rs
```

They currently cover typed codecs, cryptographic and fail-closed boundaries;
they do not prove a positive owner admission. The current-v2 Cloudflare test
belongs to Cloudflare WP06.

## State and exit conditions

WP05 is reopened for the exact owner-admission composition above. Its
WP02/WP03/WP04 edges remain implementation-independent only for code shape;
normal completion still requires those operational owners. Account WP09 must
remain implementation-blocked until this seam is independently reviewed.

Exit requires:

1. the authoritative external Account enrollment binding;
2. preserved/revalidated broker transcript through Account dispatch;
3. opaque request-scoped Account admission and exact currentness binding;
4. restart/revocation/drift behavior in the real test roots;
5. focused execution, retained proof, checklist acceptance, pre-commit, CI,
   review, and normal merge.

No READY or DONE claim follows from source or test-file presence.
