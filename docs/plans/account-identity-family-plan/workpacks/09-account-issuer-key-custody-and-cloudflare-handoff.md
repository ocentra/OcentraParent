# WP09 Account Issuer Key Custody And Cloudflare Handoff

> **Plan:** Account Identity Family
> **Workpack:** WP09
> **Status:** planned implementation-only adapter and real test-source wave; the durable issuer core plus reviewed Protected Custody WP04/WP05 boundaries are present, while production composition, Cloudflare consumer, execution, proof, and normal completion remain open.

## Agent capsule

- Route: Account Identity Family WP09 only.
- Own the Account-side issuer, signing-key custody, public-key registry, authenticated producer binding, and startup recovery boundary.
- Consume the sealed wire contract owned by WP08; do not duplicate its schema or wire authority.
- Do not edit or claim Cloudflare, Device Trust, Account WP02, or Account WP05A source.

## Goal

Provide the missing Account-owned durable issuer and authenticated producer handoff needed by Cloudflare WP06 without transferring Account authority to Cloudflare or allowing callers to select signing keys.

## Ownership

WP09 owns durable Account issuer/key lineage and signing semantics, monotonic versioned public-key registration and revocation, the authenticated Account producer service-binding adapter, Account-side startup reload and recovery, and the typed handoff over WP08's existing sealed wire contract. Secret key material and protected signing admission remain in Protected Custody WP01.

WP08 remains the owner of the canonical sealed Account authority and wire contract. Protected Custody WP01 owns the isolated broker/client, authenticated OS IPC, and protected key/admission custody that the Account signer must consume; WP09 must not replace it with in-process DPAPI, caller-selected keys, mutex/file-lock custody, or a private parallel broker. Cloudflare WP06 owns its private consumer, D1/DO/KV persistence, migration, and Cloudflare-side storage proof. WP09 does not own Cloudflare files, Worker bindings, migrations, provider verification, Device Trust, Account WP02 authority, or Account WP05A effect fencing.

## Reviewed source and expected-test boundary

The existing shared integration root crates/family-identity-core/src/lib.rs is retained in the graph roots union only. It is not a planned implementation root for WP09.

The planned implementation roots now exist and were independently reviewed:

- crates/family-identity-core/src/account_identity_authority_issuer.rs
- crates/family-identity-core/src/account_identity_authority_issuer_key_custody.rs
- crates/family-identity-core/src/account_identity_authority_issuer_key_registry.rs
- crates/family-identity-core/src/account_identity_authority_issuer_service_binding.rs
- crates/family-identity-core/src/account_identity_authority_issuer_startup.rs

Expected test roots:

- crates/family-identity-core/tests/contract/account_identity_authority_issuer.rs
- crates/family-identity-core/tests/unit/account_identity_authority_issuer_key_custody.rs
- crates/family-identity-core/tests/unit/account_identity_authority_issuer_key_registry.rs
- crates/family-identity-core/tests/unit/account_identity_authority_issuer_startup.rs

The integrated packet also owns private helper modules for transactional currentness, delivery/outbox custody, registry lineage/receipts/row and schema validation, transport encoding, and durable startup reconciliation. The graph maps those actual helper roots rather than hiding them behind the five public module roots.

The coherent production packet still requires these planned roots:

- crates/family-identity-core/src/account_identity_authority_issuer_protected_signer.rs
- crates/family-identity-core/src/account_identity_authority_issuer_cloudflare_delivery.rs
- crates/family-identity-core/src/account_identity_authority_issuer_runtime.rs

The existing four expected tests plus these runtime/adapter tests remain absent:

- crates/family-identity-core/tests/unit/account_identity_authority_issuer_protected_signer.rs
- crates/family-identity-core/tests/contract/account_identity_authority_issuer_cloudflare_delivery.rs
- crates/family-identity-core/tests/integration/account_identity_authority_issuer_runtime.rs

No Cloudflare consumer/mount, test execution, proof, checklist acceptance, READY, or DONE claim is made by this route.

## Reviewed production result

Independent P0/P1 review accepted the source integrated through `4f6245e51`:

- issuer keys and public-key lineage are SQLite-owned, versioned, monotonic, and validated at startup;
- issue/claim/ack/reconcile operations bind the current household authority, service identity, key generation, outer wire metadata, inner sealed authority, receipt, and signature;
- currentness and mutation occur under `BEGIN IMMEDIATE`, while outbox expiry/supersession is scoped to the exact household;
- expired or superseded rows become terminal, and a claim or acknowledgement cannot silently cross an external-delivery gap without revalidation;
- the handoff remains an Account-owned outbox/wire boundary. A shipped Cloudflare private consumer and runtime mount are still missing.

Focused formatting, library compilation, architecture, Enforcer source-shape/no-test-doubles/validation-bypass, exact claim guard, and diff checks passed before canonical integration. This is implementation evidence only.

Live caller review after integration found no implementation of `AccountIdentityIssuerSignerAdapter`, `AccountIdentityIssuerServiceBindingAuthenticator`, or `AccountIdentityIssuerDeliveryOwnerAdapter`, and no production call to `deliver_next_pending`. All installation and delivery methods are crate-private, and the delivery attempt exposes the wire but not an authenticated current public-key registry record that a Cloudflare consumer can use. The accepted files are therefore a durable fail-closed core, not a complete producer adapter or runtime. The graph keeps WP09 implementation open until the protected signer, authenticated Cloudflare delivery packet/ack path, and production lifecycle caller exist.

The later source attempt at `d496f08a7f5feca35d5d1479e983566924e3801c`
does not close those gaps and is rejected from consolidation. Its
`AccountIdentityIssuerProtectedSigner` only wraps a caller-supplied signer
trait object; it does not call the Protected Custody broker/client. Its sealed
Cloudflare owner port has no shipped implementor or constructible owner
response, and the added runtime has no production caller. That is dormant
adapter scaffolding, not protected signing or authenticated delivery. The
current-key-record binding introduced in the same packet can be reconsidered
only with a real protected signer, Cloudflare consumer, and production
composition; the branch must not be merged or cherry-picked wholesale.

## Dependency route

WP09 has four direct source-order prerequisites: Account WP08, Protected
Custody WP01, Protected Custody WP04, and Protected Custody WP05. WP08 supplies
the reviewed sealed Account contract; WP01 supplies the neutral foundation;
WP04 supplies the reviewed retained fixed-pipe process/token transport; and
WP05 supplies the reviewed Account issuer-owner, protected signing, and broker
RPC boundary. These reviewed implementations authorize only the missing WP09
adapter/runtime and expected-test source wave. Normal completion still requires
their operational gates. WP09 has no dependency on Account WP02, Account WP05A,
Device Trust WP01 or WP03, or Cloudflare source.

Cloudflare WP06 retains its direct WP08 dependency and adds WP09 as an additional reviewed-implementation prerequisite for the durable issuer/key custody and authenticated producer binding. This does not transfer Account ownership or claim Cloudflare runtime readiness.

## Acceptance and non-claims

- Durable custody must be authoritative, recoverable, monotonic, and versioned; caller-selected keys are not acceptable.
- Public-key registration and revocation must be authenticated and tied to the durable issuer boundary.
- The producer adapter must authenticate the Account service binding and use WP08's existing sealed wire contract.
- Startup reload and recovery must be explicit and durable; mock, no-op, process-global, or in-memory custody is not an implementation.
- The mapped tests and retained proof must cover custody, registry/revocation, authenticated handoff, and startup recovery before normal completion.
- No Cloudflare source, duplicated schema/wire, provider readiness, runtime readiness, READY, or DONE is claimed here.

## Proof boundary

Expected retained proof root: docs/proof/account-identity-family-plan/09-account-issuer-key-custody-and-cloudflare-handoff/.
