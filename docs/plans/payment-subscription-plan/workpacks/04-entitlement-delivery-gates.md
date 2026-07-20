# Workpack 04: Entitlement Delivery Gates

## Goal

Define the ledger-to-device gate that turns billing and referral truth into signed entitlement snapshots and device-bound access checks.

Status: `source-and-focused-tests-present / workpack-blocked / proof-required / WP00-blocked`

The Rust source and named focused tests remain independently inspectable implementation evidence. The canonical WP04 proof directory is physically absent in this checkout, so ledger/snapshot/device-gate closure is not proven and account/device-trust plus WP00 dependencies remain open.

## Ownership boundary

```text
payment-subscription-plan owns billing/referral/entitlement ledger semantics and signed entitlement snapshot model.
`crates/entitlement-core` owns the Rust runtime derivation contract and snapshot-to-gate bridge implemented for this packet.
device-trust-bootstrap-plan owns device enrollment, local sealed trust, and trusted-device binding.
account-identity-family-plan owns account/household/role authority.
policy-control-plane-plan consumes proven entitlement state but does not define payment authority.
provider state is input only and is never the root of entitlement.
```

## First-touch surface

- `crates/entitlement-core/src/entitlement_access.rs`
- `crates/entitlement-core/tests/unit/signed_snapshot_delivery.rs`
- Supporting rejection coverage: `crates/entitlement-core/tests/unit/capability_gate.rs`, `crates/entitlement-core/tests/unit/capability_access.rs`
- Handoff: device-trust-bootstrap-plan owns device enrollment and binding; this slice only consumes the signed trust result.

Route drift resolved during execution:

- The older first-touch path `packages/billing-domain/src/billing-entitlement-runtime-proof.ts` does not exist in the live tree.
- WP04 closure is therefore Rust-first on `crates/entitlement-core`, with TypeScript left as proof-consumer surface only.

## Read inputs

- [PLAN_STATE.md](../PLAN_STATE.md)
- [PLAN_EXECUTION_BLUEPRINT.md](../PLAN_EXECUTION_BLUEPRINT.md)
- [APP_OWNED_BILLING_LEDGER.md](../APP_OWNED_BILLING_LEDGER.md)
- [APP_OWNED_REFERRAL_LEDGER.md](../APP_OWNED_REFERRAL_LEDGER.md)
- [APP_OWNED_ENTITLEMENT_LEDGER.md](../APP_OWNED_ENTITLEMENT_LEDGER.md)
- [SIGNED_ENTITLEMENT_SNAPSHOT_MODEL.md](../SIGNED_ENTITLEMENT_SNAPSHOT_MODEL.md)
- [BILLING_API_BOUNDARY.md](../BILLING_API_BOUNDARY.md)

## Output files

- [APP_OWNED_BILLING_LEDGER.md](../APP_OWNED_BILLING_LEDGER.md)
- [APP_OWNED_REFERRAL_LEDGER.md](../APP_OWNED_REFERRAL_LEDGER.md)
- [APP_OWNED_ENTITLEMENT_LEDGER.md](../APP_OWNED_ENTITLEMENT_LEDGER.md)
- [SIGNED_ENTITLEMENT_SNAPSHOT_MODEL.md](../SIGNED_ENTITLEMENT_SNAPSHOT_MODEL.md)
- `output/payment-subscription-plan-proof/04-entitlement-delivery-gates/`
- `output/payment-subscription-plan-proof/04-entitlement-delivery-gates/04-entitlement-ledger-proof.md`
- `output/payment-subscription-plan-proof/04-entitlement-delivery-gates/04-signed-snapshot-proof.md`
- `output/payment-subscription-plan-proof/04-entitlement-delivery-gates/04-local-device-trust-required-proof.md`
- `output/payment-subscription-plan-proof/04-entitlement-delivery-gates/04-referral-loss-recalculation-proof.md`

## Acceptance

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

## Proof IDs

- `payment-entitlement.billing-ledger-source`
- `payment-entitlement.referral-ledger-source`
- `payment-entitlement.signed-snapshot-issued`
- `payment-entitlement.local-device-trust-required`
- `payment-entitlement.wrong-household-rejected`
- `payment-entitlement.wrong-device-rejected`

## Validation

Previously recorded focused source/test commands for this packet; their physical validation log is absent in this checkout and must be rerun and retained before closure:

- `cargo test -p ocentra-entitlement-core --test unit`
- `cargo lint-architecture crates/entitlement-core/src/entitlement_access.rs crates/entitlement-core/tests/unit.rs crates/entitlement-core/tests/unit/capability_gate.rs crates/entitlement-core/tests/unit/capability_access.rs crates/entitlement-core/tests/unit/signed_snapshot_delivery.rs`
- `cmd /c npm exec -- prettier --check docs/plans/payment-subscription-plan/APP_OWNED_BILLING_LEDGER.md docs/plans/payment-subscription-plan/APP_OWNED_REFERRAL_LEDGER.md docs/plans/payment-subscription-plan/APP_OWNED_ENTITLEMENT_LEDGER.md docs/plans/payment-subscription-plan/SIGNED_ENTITLEMENT_SNAPSHOT_MODEL.md output/payment-subscription-plan-proof/04-entitlement-delivery-gates/00-scope-summary.md output/payment-subscription-plan-proof/04-entitlement-delivery-gates/04-entitlement-ledger-proof.md output/payment-subscription-plan-proof/04-entitlement-delivery-gates/04-signed-snapshot-proof.md output/payment-subscription-plan-proof/04-entitlement-delivery-gates/04-local-device-trust-required-proof.md output/payment-subscription-plan-proof/04-entitlement-delivery-gates/04-referral-loss-recalculation-proof.md output/payment-subscription-plan-proof/04-entitlement-delivery-gates/05-no-claim-boundary.md`

Expected proof bundle, currently absent:

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
- Do not claim WP04 complete while WP00/account/device-trust handoffs are unaccepted or the canonical physical proof bundle is absent.
