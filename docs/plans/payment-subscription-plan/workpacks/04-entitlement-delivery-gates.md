# Workpack 04: Entitlement Delivery Gates

## Goal

Define the ledger-to-device gate that turns billing and referral truth into derived entitlement snapshots and device-bound access checks; the current source remains unsigned/manual-required until owner composition exists.

Status: `blocked / source reviewed / unsigned projection only / issuer, verifier/currentness composition, non-test consumer, expected tests, and proof open`

## Ownership boundary

```text
payment-subscription-plan owns billing/referral/entitlement ledger semantics and the derived entitlement snapshot model.
`crates/entitlement-core` owns the Rust derivation/access contracts and fail-closed snapshot boundary; no signed issuer/verifier bridge or public owner composition is present for this packet.
device-trust-bootstrap-plan owns device enrollment, local sealed trust, and trusted-device binding.
account-identity-family-plan owns account/household/role authority.
policy-control-plane-plan consumes proven entitlement state but does not define payment authority.
provider state is input only and is never the root of entitlement.
```

## First-touch surface

- `crates/entitlement-core/src/entitlement_access.rs`
- `crates/entitlement-core/src/entitlement_snapshot_derivation.rs`
- `crates/entitlement-core/src/entitlement_snapshot_issuer.rs`
- `crates/entitlement-core/src/entitlement_snapshot_authority.rs`
- `crates/entitlement-core/src/entitlement_snapshot_authority_ports.rs`
- `crates/entitlement-core/src/entitlement_snapshot_authority_verifier.rs`
- `crates/child-runtime/src/runtime_gate.rs`
- `crates/entitlement-core/tests/contract/signed_snapshot_delivery.rs`
- Supporting rejection coverage: `crates/entitlement-core/tests/unit/capability_gate.rs`, `crates/entitlement-core/tests/unit/capability_access.rs`
- Handoff: device-trust-bootstrap-plan owns device enrollment and binding; this slice only consumes the signed trust result.

Route drift resolved during execution:

- The older first-touch path `packages/billing-domain/src/billing-entitlement-runtime-proof.ts` does not exist in the live tree.
- WP04 closure is therefore Rust-first on `crates/entitlement-core`, with TypeScript left as proof-consumer surface only.

## Reviewed production truth - 2026-08-25

This is a source-and-routing truth update. It does not add completion evidence,
tests, proof, CI, PR, READY, or DONE state.

- `derive_unsigned_entitlement_snapshot` derives an explicitly unsigned
  projection from billing, referral, entitlement, and provider-input state.
  Provider state remains input-only and no production signer is composed.
- `entitlement_snapshot_issuer` is crate-private. Its trusted issuance input
  has no public constructor, and the manual-required signing provider returns
  `SigningUnavailable`.
- Snapshot authority `open` and
  `verify_current_account_and_device` are crate-private owner-composition
  entry points. The key, installed-package, and currentness ports remain
  manual-required/fail-closed, so no public unlock or capability handoff is
  available.
- `crates/child-runtime/src/runtime_gate.rs` is a non-test consumer of the
  entitlement decision function, but no non-test caller of
  `evaluate_child_runtime_preflight` or its decision recorder was found. Its
  generic input does not compose the missing issuer/verifier/currentness and
  Account/Device Trust owners; deserialization also forces unavailable
  snapshot context.
- The mapped `signed_snapshot_delivery.rs` contract still imports removed
  signed-derivation/context APIs, and the child-runtime unit helper attempts
  to construct crate-private snapshot context. The schema and TypeScript
  surfaces are contract/proof-consumer topology, not a production entitlement
  caller. The expected WP04 assertion matrix, focused tests, and proof root are
  open; `output/payment-subscription-plan-proof/04-entitlement-delivery-gates/`
  is absent in this checkout.

## Read inputs

- [PLAN_STATE.md](../PLAN_STATE.md)
- [PLAN_EXECUTION_BLUEPRINT.md](../PLAN_EXECUTION_BLUEPRINT.md)
- [APP_OWNED_BILLING_LEDGER.md](../APP_OWNED_BILLING_LEDGER.md)
- [APP_OWNED_REFERRAL_LEDGER.md](../APP_OWNED_REFERRAL_LEDGER.md)
- [APP_OWNED_ENTITLEMENT_LEDGER.md](../APP_OWNED_ENTITLEMENT_LEDGER.md)
- [SIGNED_ENTITLEMENT_SNAPSHOT_MODEL.md](../SIGNED_ENTITLEMENT_SNAPSHOT_MODEL.md)
- [BILLING_API_BOUNDARY.md](../BILLING_API_BOUNDARY.md)

## Expected output files (not produced by this packet)

- [APP_OWNED_BILLING_LEDGER.md](../APP_OWNED_BILLING_LEDGER.md)
- [APP_OWNED_REFERRAL_LEDGER.md](../APP_OWNED_REFERRAL_LEDGER.md)
- [APP_OWNED_ENTITLEMENT_LEDGER.md](../APP_OWNED_ENTITLEMENT_LEDGER.md)
- [SIGNED_ENTITLEMENT_SNAPSHOT_MODEL.md](../SIGNED_ENTITLEMENT_SNAPSHOT_MODEL.md)
- `output/payment-subscription-plan-proof/04-entitlement-delivery-gates/`
- `output/payment-subscription-plan-proof/04-entitlement-delivery-gates/04-entitlement-ledger-proof.md`
- `output/payment-subscription-plan-proof/04-entitlement-delivery-gates/04-signed-snapshot-proof.md`
- `output/payment-subscription-plan-proof/04-entitlement-delivery-gates/04-local-device-trust-required-proof.md`
- `output/payment-subscription-plan-proof/04-entitlement-delivery-gates/04-referral-loss-recalculation-proof.md`

## Target acceptance (open)

- Effective entitlement is derived from the billing, referral, and entitlement ledgers.
- Signed snapshots contain the fixed fields in the snapshot model.
- Snapshot verification fails closed for the wrong household or wrong device.
- Local sealed device trust is required where the snapshot model says so.
- Stale, revoked, or expired snapshots are rejected.

## Required proof fields

The selected proof must name, at minimum:

```text
billing_ledger_state
referral_ledger_state
entitlement_ledger_state
provider_state_boundary
snapshot_signature_state
snapshot_freshness_state
household_binding_state
device_binding_state
device_trust_handoff_state
account_authority_state
cancel_revoke_state
referral_loss_recalculation_state
safety_feature_state
no_claim
```

These are proof-routing fields, not implementation code prescriptions.

## Expected proof IDs (open)

- `payment-entitlement.billing-ledger-source`
- `payment-entitlement.referral-ledger-source`
- `payment-entitlement.signed-snapshot-issued`
- `payment-entitlement.local-device-trust-required`
- `payment-entitlement.wrong-household-rejected`
- `payment-entitlement.wrong-device-rejected`

## Deferred validation

No cargo, test, proof, CI, or precommit command was run in this docs/graph
truth packet. A future source/test packet must replace the stale signed API,
compose the real owner boundary, and run the focused entitlement-core
contract/unit checks, the architecture gate on the touched Rust boundary, and
the required proof/validation commands.

Expected proof bundle (absent in the current checkout):

- `output/payment-subscription-plan-proof/04-entitlement-delivery-gates/00-scope-summary.md`
- `output/payment-subscription-plan-proof/04-entitlement-delivery-gates/04-entitlement-ledger-proof.md`
- `output/payment-subscription-plan-proof/04-entitlement-delivery-gates/04-signed-snapshot-proof.md`
- `output/payment-subscription-plan-proof/04-entitlement-delivery-gates/04-local-device-trust-required-proof.md`
- `output/payment-subscription-plan-proof/04-entitlement-delivery-gates/04-referral-loss-recalculation-proof.md`
- `output/payment-subscription-plan-proof/04-entitlement-delivery-gates/05-no-claim-boundary.md`
- `output/payment-subscription-plan-proof/04-entitlement-delivery-gates/16-validation-commands.log`

## Negative cases

- Reject provider state as the root of entitlement.
- Reject any snapshot that is not signed or does not bind the household and device.
- Reject stale, revoked, expired, or wrong-device snapshots.
- Reject any entitlement path that exposes child telemetry.
- Reject entitlement unlock without account and device-trust handoffs.

## Failure conditions

- Do not treat the snapshot as the root of trust.
- Do not let provider state bypass the entitlement ledger.
- Do not reuse a snapshot across households or devices.
- Do not let entitlement proof claim policy/enforcement behavior.
