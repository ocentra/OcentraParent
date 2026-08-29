# WP09 Account Issuer Key Custody And Cloudflare Handoff

> **Plan:** Account Identity Family
> **Workpack:** WP09
> **Status:** BLOCKED / NO-CHANGE until Protected Custody WP05 supplies the
> owner-issued Account admission consumed by the current-v2 family lifecycle.

## Agent capsule

- Route: Account Identity Family WP09 only.
- Consume the sealed current-v2 Account authority and one reviewed Protected
  owner admission; do not manufacture either.
- Do not revive the retired legacy signer/delivery/runtime adapter files.
- Do not edit Cloudflare, Protected Custody, Device Trust, Account WP02, or
  Account WP05A source from this workpack.

## Goal

Compose the existing family-owned current-v2 issuer lifecycle with the real
Protected Account owner after Protected WP05 has authenticated the broker
request. Then hand the already signed bounded wire to the Cloudflare WP06
consumer without transferring Account authority or signing-key selection to
Cloudflare.

## Current production truth - 2026-08-29

The modern v2 family lifecycle is already present. The key API is
`AccountIdentityAuthorityIssuerClient::issue_current_authority_with_account_owner_admission`;
it coordinates the family-owned authority/currentness repository with a
request-scoped signer callback. The real P-256 signer is in
`crates/account-issuer-owner/src/signing.rs`, and the protected broker has a
typed Account issuer RPC path.

The call path is deliberately unavailable today:

- `AccountIssuerOwner::authorize_protected_request` fails closed because no
  OS-enrolled Account admission exists;
- the fixed Account issuer mount returns unavailable;
- Protected core mints and revalidates `BrokerAuthorizedClientTranscript`, but
  the broker peer drops it before Account execution;
- the enrollment record binds OS peer/process/token/image/service/TPM state but
  no Account/service/current-key lineage;
- the family admission has private fields and no public constructor, and a
  family-to-Protected dependency would create a crate cycle;
- no production delivery owner or `deliver_next_pending` caller exists.

These are owner/dependency blockers, not missing family DTOs.

## Current-v2 owned roots

The graph maps the actual family lifecycle rather than the retired legacy
facade names:

```text
crates/family-identity-core/src/account_identity_authority_issuer_client.rs
crates/family-identity-core/src/account_identity_authority_issuer_client_api.rs
crates/family-identity-core/src/account_identity_authority_issuer_client_api_issue_signer_flow.rs
crates/family-identity-core/src/account_identity_authority_issuer_client_currentness.rs
crates/family-identity-core/src/account_identity_authority_issuer_client_key.rs
crates/family-identity-core/src/account_identity_authority_issuer_client_owner_admission.rs
crates/family-identity-core/src/account_identity_authority_issuer_client_startup.rs
crates/family-identity-core/src/account_identity_authority_issuer_client_transaction.rs
crates/family-identity-core/src/account_identity_authority_issuer_client_transaction_outbox.rs
crates/family-identity-core/src/account_identity_authority_issuer_client_transaction_receipt.rs
crates/family-identity-core/src/account_identity_authority_issuer_client_transaction_recovery.rs
crates/family-identity-core/src/account_identity_authority_issuer_client_types.rs
crates/family-identity-core/src/account_identity_authority_issuer_outbox_claim.rs
crates/family-identity-core/src/account_identity_authority_issuer_outbox_reconcile.rs
```

The current real family contract test root is:

```text
crates/family-identity-core/tests/contract/account_identity_authority_issuer_transport.rs
```

It proves typed and fail-closed family behavior only. It cannot prove positive
Protected admission, operational signing, broker composition, Cloudflare
delivery, restart recovery across both owners, or runtime reachability.

## Retired legacy route

The old planned files
`account_identity_authority_issuer_protected_signer.rs`,
`account_identity_authority_issuer_cloudflare_delivery.rs`, and
`account_identity_authority_issuer_runtime.rs`, plus their seven legacy test
paths, are retired. The rejected `d496f08a` packet wrapped a caller-supplied
signer and added unimplemented owner ports with no production caller. Recreating
those names would add dead scaffolding, not close the current-v2 owner seam.

## Dependency and source order

```text
Protected WP01 foundation
-> Protected WP02 external Enrollment/SCM/TPM plus Account service/key binding
-> Protected WP03 hardware monotonic currentness
-> Protected WP04 retained broker peer/session transcript
-> Protected WP05 private transcript-to-Account admission and owner consumption
-> Account WP08 sealed authority/currentness contract
-> Account WP09 family lifecycle composition
-> Cloudflare WP06 authenticated delivery/current-key/D1 consumer
```

Protected WP05 is a hard implementation prerequisite, not merely a completion
dependency. WP09 remains blocked until that seam is independently reviewed.
Cloudflare WP06 remains downstream of WP09 and owns its current-v2 consumer,
current-key registry/D1 custody, migration, route composition, and tests.

## Non-negotiable boundary

WP09 must not add public raw fields or constructors, booleans, closures,
environment/header authority, static keys, caller-selected Account/key/generation
values, a second Account database, a family-to-Protected crate cycle, a mock
owner, or an in-memory signer. Missing or stale admission remains unavailable.

## Exit conditions

WP09 may resume only after Protected WP05 exposes the reviewed opaque
request-scoped Account owner capability without widening its trust boundary.
Exit then requires a shipped lifecycle caller, exact admission/currentness/key
binding, restart and revocation behavior, real focused tests, retained proof,
checklist acceptance, pre-commit, CI, review, and normal merge. No READY or DONE
claim follows from current source-file presence.

Expected retained proof root:
`docs/proof/account-identity-family-plan/09-account-issuer-key-custody-and-cloudflare-handoff/`.
