# Workpack 04: Entitlement Delivery Gates

## Goal

Define the ledger-to-device gate that turns billing and referral truth into signed entitlement snapshots and device-bound access checks.

## First-touch surface

- `packages/billing-domain/src/billing-entitlement-runtime-proof.ts`
- `crates/entitlement-core/tests/unit/capability_gate.rs`
- Handoff: device-trust-bootstrap-plan owns device enrollment and binding; this slice only consumes the signed trust result.

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
- `docs/proof/payment-subscription-plan/wp04-entitlement-delivery-gates/`

## Acceptance

- Effective entitlement is derived from the billing, referral, and entitlement ledgers.
- Signed snapshots contain the fixed fields in the snapshot model.
- Snapshot verification fails closed for the wrong household or wrong device.
- Local sealed device trust is required where the snapshot model says so.
- Stale, revoked, or expired snapshots are rejected.

## Proof IDs

- `payment-entitlement.billing-ledger-source`
- `payment-entitlement.referral-ledger-source`
- `payment-entitlement.signed-snapshot-issued`
- `payment-entitlement.local-device-trust-required`
- `payment-entitlement.wrong-household-rejected`
- `payment-entitlement.wrong-device-rejected`

## Validation

- Docs validation: `npm run format:check`; `npm run lint:schema-boundaries`
- Required proof families: `payment-entitlement.billing-ledger-source`, `payment-entitlement.referral-ledger-source`, `payment-entitlement.entitlement-ledger-source`, `payment-entitlement.signed-snapshot-issued`, `payment-entitlement.snapshot-signature-invalid-rejected`, `payment-entitlement.local-device-trust-required`, `payment-entitlement.wrong-household-rejected`, `payment-entitlement.wrong-device-rejected`, `payment-entitlement.offline-stale-degraded`, `payment-entitlement.grace-period`, `payment-entitlement.cancel-revokes-paid-feature`, `payment-entitlement.referral-loss-revokes-earned-feature`, `payment-entitlement.safety-feature-not-silently-disabled`
- Proof bundle: `docs/proof/payment-subscription-plan/04-entitlement-ledger-proof.md`, `docs/proof/payment-subscription-plan/04-signed-snapshot-proof.md`, `docs/proof/payment-subscription-plan/04-local-device-trust-required-proof.md`, `docs/proof/payment-subscription-plan/04-referral-loss-recalculation-proof.md`

## Negative cases

- Reject provider state as the root of entitlement.
- Reject any snapshot that is not signed or does not bind the household and device.
- Reject stale, revoked, expired, or wrong-device snapshots.
- Reject any entitlement path that exposes child telemetry.

## Failure conditions

- Do not treat the snapshot as the root of trust.
- Do not let provider state bypass the entitlement ledger.
- Do not reuse a snapshot across households or devices.
