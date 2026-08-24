# WP09 Account Issuer Key Custody And Cloudflare Handoff

> **Plan:** Account Identity Family
> **Workpack:** WP09
> **Status:** independently reviewed durable issuer core is integrated at canonical `4f6245e51`; protected signer, authenticated binding/delivery adapters, production caller, Cloudflare consumer, expected tests, proof, and normal completion remain open.

## Agent capsule

- Route: Account Identity Family WP09 only.
- Own the Account-side issuer, signing-key custody, public-key registry, authenticated producer binding, and startup recovery boundary.
- Consume the sealed wire contract owned by WP08; do not duplicate its schema or wire authority.
- Do not edit or claim Cloudflare, Device Trust, Account WP02, or Account WP05A source.

## Goal

Provide the missing Account-owned durable issuer and authenticated producer handoff needed by Cloudflare WP06 without transferring Account authority to Cloudflare or allowing callers to select signing keys.

## Ownership

WP09 owns durable issuer and signing-key custody, monotonic versioned public-key registration and revocation, the authenticated Account producer service-binding adapter, Account-side startup reload and recovery, and the typed handoff over WP08's existing sealed wire contract.

WP08 remains the owner of the canonical sealed Account authority and wire contract. Cloudflare WP06 owns its private consumer, D1/DO/KV persistence, migration, and Cloudflare-side storage proof. WP09 does not own Cloudflare files, Worker bindings, migrations, provider verification, Device Trust, Account WP02 authority, or Account WP05A effect fencing.

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

## Dependency route

WP09 has exactly one direct prerequisite: Account WP08. The WP08 sealed contract is a reviewed-implementation prerequisite, so the implementation ordering edge is reviewed-implementation; the normal completion gate remains WP08 DONE. WP09 has no dependency on Account WP02, Account WP05A, Device Trust WP01 or WP03, or Cloudflare source.

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
