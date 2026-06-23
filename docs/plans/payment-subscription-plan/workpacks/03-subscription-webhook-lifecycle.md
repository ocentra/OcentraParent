# Workpack 03: Subscription Webhook Lifecycle

## Goal

Define how provider events become app-owned billing truth, including signature validation, dedupe, retries, and reconciliation.

## Ownership boundary

```text
billing-core owns Rust provider lifecycle classification and event/idempotency helper behavior when selected.
payment-subscription-plan owns webhook-to-app-ledger semantics.
cloudflare-control-plane-plan owns shared Worker route/auth/runtime shell.
provider adapters provide verified provider event formats only.
entitlement delivery happens through WP04 and must not be bypassed by provider events.
```

## First-touch surface

- `crates/billing-core/src/billing_subscription.rs`
- `crates/billing-core/tests/unit/provider_webhook.rs`

## Read inputs

- [PLAN_STATE.md](../PLAN_STATE.md)
- [PLAN_EXECUTION_BLUEPRINT.md](../PLAN_EXECUTION_BLUEPRINT.md)
- [SUBSCRIPTION_WEBHOOK_LIFECYCLE.md](../SUBSCRIPTION_WEBHOOK_LIFECYCLE.md)
- [BILLING_API_BOUNDARY.md](../BILLING_API_BOUNDARY.md)
- [APP_OWNED_BILLING_LEDGER.md](../APP_OWNED_BILLING_LEDGER.md)

## Output files

- [SUBSCRIPTION_WEBHOOK_LIFECYCLE.md](../SUBSCRIPTION_WEBHOOK_LIFECYCLE.md)
- [APP_OWNED_BILLING_LEDGER.md](../APP_OWNED_BILLING_LEDGER.md)
- `output/payment-subscription-plan-proof/03-subscription-webhook-lifecycle/`

## Acceptance

- Every provider event is signature-verified before parsing.
- Every accepted event produces an app-owned ledger entry.
- Stable provider event IDs dedupe replays and duplicates.
- Out-of-order events converge on the same final ledger state.
- Retry or reconciliation work is queued when a webhook needs follow-up.

## Required proof fields

The selected proof must name, at minimum:

```text
provider
mode
provider_event_id
signature_state
payload_parse_state
idempotency_state
replay_state
out_of_order_state
ledger_write_state
entitlement_update_requirement
retry_state
dead_letter_state
reconciliation_state
test_live_boundary_state
no_claim
```

These are proof-routing fields, not implementation code prescriptions.

## Proof IDs

- `payment-webhook.stripe-signature-valid`
- `payment-webhook.razorpay-signature-valid`
- `payment-webhook.paypal-webhook-verified`
- `payment-webhook.duplicate-event-idempotent`
- `payment-webhook.replayed-event-rejected`
- `payment-webhook.reconciliation-repairs-drift`

## Validation

- Docs validation: `npm run format:check`; `npm run lint:schema-boundaries`
- Required proof families: `payment-webhook.stripe-signature-valid`, `payment-webhook.stripe-signature-invalid`, `payment-webhook.razorpay-signature-valid`, `payment-webhook.paypal-webhook-verified`, `payment-webhook.duplicate-event-idempotent`, `payment-webhook.replayed-event-rejected`, `payment-webhook.out-of-order-event-safe`, `payment-webhook.unknown-event-safe`, `payment-webhook.retry-no-double-grant`, `payment-webhook.dead-letter-manual-required`, `payment-webhook.reconciliation-repairs-drift`, `payment-webhook.test-live-separated`
- Proof bundle: `output/payment-subscription-plan-proof/03-subscription-webhook-lifecycle/03-provider-webhook-proof.md`, `output/payment-subscription-plan-proof/03-subscription-webhook-lifecycle/03-idempotency-replay-proof.md`, `output/payment-subscription-plan-proof/03-subscription-webhook-lifecycle/03-dead-letter-proof.md`, `output/payment-subscription-plan-proof/03-subscription-webhook-lifecycle/03-reconciliation-proof.md`, `output/payment-subscription-plan-proof/03-subscription-webhook-lifecycle/03-test-live-boundary-proof.md`

## Negative cases

- Reject unsigned or malformed provider payloads.
- Reject duplicate events as new entitlement.
- Reject replayed events that would double-grant access.
- Reject any webhook path that changes access without a ledger entry.
- Reject test/live mode mixing.

## Failure conditions

- Do not trust provider payloads without signature validation.
- Do not let a webhook change access without the ledger.
- Do not double-grant entitlement on duplicate events.
- Do not claim entitlement delivery from webhook acceptance alone; WP04 owns entitlement delivery proof.
