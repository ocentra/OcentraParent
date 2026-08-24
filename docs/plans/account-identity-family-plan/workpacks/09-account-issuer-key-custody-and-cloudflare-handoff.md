# WP09 Account Issuer Key Custody And Cloudflare Handoff

> **Plan:** Account Identity Family
> **Workpack:** WP09
> **Status:** planned Account issuer and key-custody route; normal completion remains gated while tests, startup recovery, authenticated service binding, Cloudflare composition, and proof remain open.

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

## Planned source and test boundary

The existing shared integration root crates/family-identity-core/src/lib.rs is retained in the graph roots union only. It is not a planned implementation root for WP09.

Planned implementation roots:

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

All listed planned and expected paths remain in the graph roots union. They are currently absent; no source, test, proof, READY, or DONE claim is made by this route.

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
