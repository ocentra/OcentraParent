# Workpack 05 - Account Issuer Key and Store Custody

<!-- agent-capsule -->

> Plan: `protected-capability-custody-foundation-plan`
> Workpack: `05-account-issuer-key-and-store-custody`
> Kind: Account-owned protected issuer v2 source-order route.
> Proves: ownership, versioned P-256 v2 contract, and dependency boundaries only.
> Does not prove: provider execution, implementation, tests, proof, READY, or DONE.

<!-- /agent-capsule -->

## Purpose

Close the real boundary between Protected Custody and Account WP09. WP01's
current protocol actions (`Seal`, `Rotate`, `Revoke`, and `Recover`) and opaque
prepared token are not signer/store authority. The selected owner is the new
Account-owned `crates/account-issuer-owner` crate, statically linked into the
existing Protected broker. It absorbs issuer, key-registry, outbox, delivery,
startup, recovery, signing, and typed RPC mechanics without making the broker
depend directly on `family-identity-core`.

Family-core remains the owner of `VerifiedAccountIdentityAuthority`, the
authority repository/source of truth, and one opaque `BEGIN IMMEDIATE`
transaction/currentness host. Its existing family-owned handoff contract
remains a separate historical/input boundary and is never embedded, re-signed,
or duplicated inside P-256 v2. The broker mounts the owner for service lifetime
and retains protected signer custody. The owner receives narrow opaque
Account-specific transaction and protected-signer capabilities; it must not
receive raw path, SQL, generic signer, private material, open a second Account
connection, or merge with protected `custody.sqlite`.

The runtime composition is one-way: Protected broker -> account-issuer-owner.
The owner crate may depend on family-identity-core only through the narrow
Account transaction/currentness facade; the broker itself must not directly
import family-identity-core. Family-core keeps its existing authority DTO
within account_identity_authority.rs and its existing v1 historical
parse/verifier path within account_identity_authority_producer_parse.rs and
account_identity_authority_producer_envelope.rs. No speculative DTO or v1
verifier split is part of this packet.

## Source packet 1 — v2 contract freeze

This is the first WP05 source packet and must land before parallel owner, FFI,
facade, broker, parent, and Cloudflare implementation. It freezes the shared
wire and receipt shape; it does not claim provider feasibility, runtime
composition, tests, proof, READY, or DONE.

The frozen contract is:

- the producer is `ocentra.account-authority-producer.v2`, the inner signing
  domain is `ocentra.account-authority-producer.signing.v2\0`, the audience is
  `ocentra.account.authority.v2`, the algorithm is
  `ecdsa-p256-sha256-p1363`, the outer transport domain is
  `ocentra.account-issuer.transport.v2\0`, and the service is
  `ocentra.account-authority-producer.cloudflare.v2`;
- v2 is self-contained P-256 inner plus outer wire. It must not wrap a freshly
  signed Ed25519-v1 inner wire. Ed25519 v1 is historical parse/verify only;
  there is no new v1 signing path or fallback;
- every Account issuer request and receipt carries an explicit v2 wire/version
  tag and an algorithm-aware key identifier. The key identifier is
  `sha256:ecdsa-p256:<hex>` derived with a v2 derivation domain; a caller cannot
  mint the algorithm tag, key generation, enrollment generation, or authority
  binding;
- the public key is canonical 65-byte SEC1 and the signature is exactly 64-byte
  P1363 `r || s`; the verifier rejects wrong lengths, wrong algorithm tags,
  high-S values, and any silent downgrade to Ed25519 or another algorithm;
- the signed message is the original canonical Account payload bytes. The
  protected signer applies SHA-256 exactly once, and Cloudflare verifies those
  original bytes with ECDSA/SHA-256 rather than accepting a caller-supplied
  digest or double-hashing;
- protocol envelope message kinds 6 and 7 are AccountIssuerRequest and
  AccountIssuerResponse; they carry the initial inner operations
  IssueCurrentAuthority and AcknowledgeReceipt. Verify remains owner-local and
  is not a protected protocol message. Each receipt binds the operation result
  to the algorithm-aware key id, authority identity, enrolled generation, key
  generation, service binding, and currentness observed by the single Account
  transaction host; a boolean, opaque lifecycle token, path, handle, or raw
  operation bytes is not a receipt;
- enrollment metadata is broker/provider-owned and includes the enrolled
  authority and generation observations needed to reject stale, revoked,
  rebound, or cross-service results. The Account owner may consume only the
  narrow opaque Account-specific transaction and signer capabilities;
- family-core retains VerifiedAccountIdentityAuthority, its authority DTO and
  repository/source of truth, the v1 historical verifier, and one opaque BEGIN
  IMMEDIATE transaction/currentness host. The existing family-owned handoff
  contract remains a separate historical/input boundary and is never embedded,
  re-signed, or duplicated inside P-256 v2. The owner must not open a second
  connection or merge with protected custody.sqlite.

Only shared contract/module wiring is serialized in this packet. After this
freeze, the owner/repository, Windows CNG P-256, protected admission, distinct
protocol/client/broker `account_issuer` RPC, family v2 facade, Parent Runtime,
and Cloudflare v2/D1 lanes may proceed in parallel against the same shape.

## Selected TPM-native issuer v2 contract

The selected issuer is deliberate TPM-native ECDSA P-256 v2. Runtime must use
`NCryptIsAlgSupported`/`EnumAlgorithms`, then create a unique non-exportable
signing-only Microsoft Platform Crypto Provider key with a service-specific
ACL. Export must convert `BCRYPT_ECCPUBLIC_BLOB` to canonical 65-byte SEC1.
Signing must use `NCryptSignHash` with SHA-256 and produce the exact 64-byte
P1363 `r||s` form; the FFI canonicalizes low-S, while Rust and Cloudflare reject
high-S. Cloudflare verifies the original canonical message bytes with
ECDSA/SHA-256 and never digest- or double-hashes. Ed25519 v1 remains
verification-only for historical migration; it is not a v2 inner-wire wrapper,
signing fallback, or downgrade. Algorithm-aware key IDs and schema/D1 v2 fields
distinguish the versions. Pinned Windows preflight confirms windows-sys 0.61.2
already exposes the required NCrypt and P-256/ECC constants under existing
features, so no FFI dependency change is needed; this is implementation-
authorization evidence only, not provider, ACL, provisioning, runtime, test,
proof, READY, or DONE evidence.

Service-specific key custody is currently REJECT/runtime-blocked. The existing
key ACL is SYSTEM GenericAll; SCM exposes only the service SID type; token
observation lacks TokenGroups; and no LookupAccountNameW service-SID resolver
is present. External provisioning must create and set the service-specific ACL.
The broker may only open and revalidate the resulting security descriptor and
token/service observations. Caller-supplied SDDL/SID, broad SYSTEM/BA grants,
or a caller-minted service identity are forbidden. The shared WP04 anchor
packet maps the exact FFI/core service-SID and TokenGroups observation roots;
WP05 maps CNG security-descriptor revalidation.

Rust v2 verification uses the locked ring 0.17.14
ECDSA_P256_SHA256_FIXED path over original bytes after an explicit low-S
precheck. sha2 is a direct dependency only for v2 algorithm-aware key-ID
hashing; no new p256 or ecdsa dependency is legal.

Unsupported TPM capability or manual enrollment remains fail-closed. Missing
attestation, rotation, recovery, provider binding, and cross-binding lineage
are explicit gaps. Existing supersede/newer-row queries omit
`service_binding_id` and require a P1 repair.
The cross-process live SQLite lease design is rejected: a broker-side fs2 lock
is advisory, a write-sharing handle does not protect the store, a no-write-share
handle prevents Account/rusqlite from opening it, and no custom VFS/handle
design exists. WAL/journal custody also cannot be made correct through that
lease shape. A one-shot token, path, boolean, caller-selected key, or SQLite
row cannot stand in for a signer or repository owner.

The expected typed operation family is:

```text
IssueCurrentAuthority
AcknowledgeReceipt
Verify (owner-local only; not a protected protocol message)
```

The protected envelope kinds are 6 (AccountIssuerRequest) and 7
(AccountIssuerResponse); the operation names in the preceding list are not
message-kind values.

The selected repository direction is: the broker derives the fixed enrolled
path, mounts `crates/account-issuer-owner` for service lifetime, retains the
protected signer, and passes only opaque Account-specific transaction and
signer capabilities. Family-core keeps the single Account repository/
`BEGIN IMMEDIATE` host for authority/key/outbox atomicity; the owner must not
open a second Account connection or touch protected `custody.sqlite`. No caller
may supply key material, private material, store path, SQL, handle, lease
result, generic signer, or signing authority. Account SQLite owns the
authoritative key, receipt, and outbox state. D1 owns only public verifier
currentness/CAS and the inbound idempotency receipt; it does not duplicate the
Account outbox.

## Dependencies and expected route

WP05 depends on reviewed WP01 implementation source plus implementation-
independent WP02, WP03, and WP04 operational boundaries for source ordering.
Account WP09 must depend on reviewed WP05 source as its protected issuer/
repository owner while retaining its Account WP08 dependency. WP04 alone does
not unblock Account. The base lifecycle is planned/source-authorable, but
normal derived state remains blocked until operational owner completion and the
remaining provider, binding, attestation, rotation, recovery, test, proof, and
runtime gates close.

## Expected source and evidence

Expected Account owner, protected protocol/client/broker, family facade, and
Parent Runtime composition roots are:

```text
crates/account-issuer-owner/src/contract.rs
crates/account-issuer-owner/src/repository.rs
crates/account-issuer-owner/src/currentness.rs
crates/account-issuer-owner/src/key_registry.rs
crates/account-issuer-owner/src/outbox.rs
crates/account-issuer-owner/src/delivery.rs
crates/account-issuer-owner/src/startup.rs
crates/account-issuer-owner/src/recovery.rs
crates/account-issuer-owner/src/signing.rs
crates/account-issuer-owner/src/rpc.rs
crates/protected-capability-custody-protocol/src/account_issuer.rs
crates/protected-capability-custody-protocol/src/account_issuer_contract.rs
crates/protected-capability-custody-protocol/src/account_issuer_v2_codec.rs
crates/protected-capability-custody-client/src/account_issuer.rs
crates/protected-capability-custody-client/src/account_issuer_rpc.rs
crates/protected-capability-custody-broker/src/account_issuer.rs
crates/protected-capability-custody-broker/src/account_issuer_rpc.rs
crates/ocentra-protected-capability-custody-windows-ffi/src/windows/cng_account_issuer_p256_capability.rs
crates/ocentra-protected-capability-custody-windows-ffi/src/windows/cng_account_issuer_p256_lifecycle.rs
crates/ocentra-protected-capability-custody-windows-ffi/src/windows/cng_account_issuer_p256_export.rs
crates/ocentra-protected-capability-custody-windows-ffi/src/windows/cng_account_issuer_p256_sign.rs
crates/ocentra-protected-capability-custody-windows-ffi/src/windows/cng_account_issuer_p256_security.rs
crates/ocentra-protected-capability-custody-provisioner/src/provisioning/account_issuer_acl.rs
crates/family-identity-core/src/account_identity_authority_repository.rs
crates/family-identity-core/src/account_identity_authority_producer_parse.rs
crates/family-identity-core/src/account_identity_authority_producer_envelope.rs
crates/family-identity-core/src/account_identity_authority_issuer_transport.rs
crates/family-identity-core/src/account_identity_authority_issuer_transport_codec.rs
crates/family-identity-core/src/account_identity_authority_producer_v2.rs
crates/family-identity-core/src/account_identity_authority_parser_v2.rs
crates/family-identity-core/src/account_identity_authority_envelope_v2.rs
crates/schema/src/account_identity_authority_producer_v2.rs
crates/parent-runtime-core/src/account_issuer_owner.rs
infra/cloudflare/src/auth/account-identity-authority-producer-v2-contract.ts
infra/cloudflare/src/auth/account-identity-authority-producer-v2-transport.ts
```

The Rust leaves require their existing registration and runtime integration
hosts in the same source packet. `crates/schema/src/lib.rs` registers the v2
schema module. `crates/family-identity-core/src/lib.rs` registers the Account
issuer client and v2 producer/parser/envelope modules; the existing
`account_identity_authority_repository.rs` remains the one Account repository
and `BEGIN IMMEDIATE` host, while the chosen narrow public opaque
transaction/currentness facade host is the new
`account_identity_authority_issuer_client.rs`. It must expose Account-specific
capabilities without a second connection or raw SQL/path.

`crates/protected-capability-custody-protocol/src/lib.rs` registers the
AccountIssuer request/response and `account_issuer_v2_codec.rs` modules. The
client and broker `src/lib.rs` files register their `account_issuer` and RPC
modules. The broker's `Cargo.toml` is a workspace/package root only; its
`src/custody.rs` and `src/custody/runtime.rs` hosts retain the owner lifetime
and dispatch composition, and its existing `src/windows_ipc.rs` path host
registers `service.rs` and `peer.rs`. Those startup/request hosts must dispatch
authenticated AccountIssuer messages to the broker-owned owner crate for the
service lifetime; no caller or generic lifecycle operation bytes can bypass
that boundary.

The manifest dependency hosts are explicit: the root `Cargo.toml` registers
the new `crates/account-issuer-owner` workspace member; the broker
`Cargo.toml` adds the `ocentra-account-issuer-owner` dependency; and
`crates/parent-runtime-core/Cargo.toml` adds the protected-client dependency
used by the Parent Runtime facade. These Cargo manifests are roots and
workspace-contract evidence only, never `plannedImplementationRoots`.

Windows FFI uses both `src/lib.rs` and `src/windows.rs` for the dedicated CNG
P-256 leaves. The provisioner uses `src/main.rs`,
`src/provisioning/mod.rs`, and `src/provisioning/ceremony.rs`; ceremony must
invoke the service-ACL leaf or the mapped ACL source is unreachable. Finally,
`crates/parent-runtime-core/src/lib.rs` registers the Parent Runtime facade,
but there is no real upstream production caller yet; that remains Account
WP09/runtime integration debt and is not a caller claim.

Mapping these hosts is required to prevent orphan AI files from appearing as
implementation; it does not claim that any host or leaf exists.

Cloudflare v2 consumer/currentness roots are:

```text
infra/cloudflare/src/auth/account-identity-authority-issuer-v2.ts
infra/cloudflare/src/storage/account-identity-authority-issuer-v2.ts
infra/cloudflare/migrations/account-identity/0008_account_identity_authority_issuer_v2.sql
```

The v2 Cloudflare leaves must be imported through the existing composition
hosts `auth/account-identity-authority-caller.ts`,
`auth/account-identity-authority-runtime.ts`,
`storage/account-identity-authority-writer.ts`, and
`storage/account-identity-authority-store.ts`. No public index or route is a
binding authority. The D1 migration remains explicit apply debt; it uses the
existing `infra/cloudflare/wrangler.toml` `migration_dir` discovery and needs
no new registry entry.

Expected tests remain absent:

```text
crates/account-issuer-owner/tests/account_issuer_owner.rs
crates/protected-capability-custody-protocol/tests/account_issuer.rs
crates/protected-capability-custody-client/tests/account_issuer.rs
crates/protected-capability-custody-broker/tests/account_issuer.rs
crates/ocentra-protected-capability-custody-windows-ffi/tests/account_issuer_p256.rs
crates/family-identity-core/tests/contract/account_identity_authority_issuer_transport.rs
crates/parent-runtime-core/tests/integration/account_issuer_owner.rs
infra/cloudflare/tests/account-identity-authority-issuer-v2.test.ts
```

No software/wrapped fallback, silent downgrade, generic lifecycle operation
bytes, caller-minted authority, path-only lease, direct SQLite sharing, mock,
or no-op provider may be added. Tests, proof, READY, and DONE remain open.

The workspace contract must register the active
crates/account-issuer-owner/Cargo.toml lib target in the root Cargo.toml and
declare direct ring = 0.17.14 and sha2 = 0.10.9 dependencies for the owner.
ring is the only v2 verifier dependency; sha2 is limited to key-ID hashing. No
new p256 or ecdsa dependency is part of this route.
