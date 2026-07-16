# Workpack 03: Subscription Webhook Lifecycle

## Current status

- Verdict: `done`
- Proof root: `output/payment-subscription-plan-proof/03-subscription-webhook-lifecycle/`
- Rust owner: `crates/billing-core/src/billing_subscription.rs`
- No-claim boundary: this packet proves Rust-owned webhook lifecycle truth only; it does not claim Cloudflare shell ownership or entitlement delivery completion.

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

## Current execution truth

| Field | Current state | Evidence |
| --- | --- | --- |
| `provider` | Stripe, Razorpay, PayPal, Apple, and Google are explicit normalized channels in the Rust contract | `crates/billing-core/src/billing_subscription.rs` |
| `mode` | test/live provider mode is explicit in the Rust contract | `crates/billing-core/src/billing_subscription.rs`; `provider_webhook::mixed_test_live_boundary_is_rejected_before_any_ledger_write` |
| `signature_state` | verified signatures are required for acceptance; missing or invalid signatures reject before ledger writes | `provider_webhook::accepts_verified_fresh_account_matched_provider_event`; `provider_webhook::rejects_missing_signature_provider_event_without_entitlement_update`; `subscription_lifecycle::invalid_signature_blocks_entitlement_write_and_requires_manual_review` |
| `payload_parse_state` | malformed payloads reject before any entitlement write | `provider_webhook::malformed_provider_payload_is_dead_lettered_before_any_entitlement_write` |
| `idempotency_state` | duplicate events reject without new entitlement write | `provider_webhook::rejects_duplicate_provider_event_without_entitlement_update` |
| `replay_state` | replayed events reuse the same idempotency chain and do not double-grant | `provider_webhook::replayed_provider_event_reuses_idempotency_chain_and_blocks_double_grant` |
| `out_of_order_state` | out-of-order events queue reconciliation and do not double-grant | `provider_webhook::out_of_order_provider_event_requires_reconciliation_and_blocks_double_grant` |
| `ledger_write_state` | accepted lifecycle changes project app-owned entitlement writes; rejected states project `no-write` | `provider_webhook::active_subscription_projects_household_entitlement_grant`; `provider_webhook::rejected_provider_event_projects_no_entitlement_write` |
| `retry_state` | payment failures queue retry follow-up without skipping ledger projection | `provider_webhook::payment_failure_queues_retry_follow_up_without_skipping_projection` |
| `dead_letter_state` | malformed, unsigned, invalid, mismatched, or mixed-boundary events remain explicit manual-required outcomes | `provider_webhook::malformed_provider_payload_is_dead_lettered_before_any_entitlement_write`; `provider_webhook::mixed_test_live_boundary_is_rejected_before_any_ledger_write` |
| `reconciliation_state` | refund, dispute, replay-safe repair, and out-of-order states remain explicit follow-up outcomes | `provider_webhook::refund_event_is_safe_but_does_not_auto_update_entitlement`; `provider_webhook::out_of_order_provider_event_requires_reconciliation_and_blocks_double_grant` |
| `test_live_boundary_state` | mixed test/live traffic is rejected before any ledger write | `provider_webhook::mixed_test_live_boundary_is_rejected_before_any_ledger_write` |

## Acceptance

- Every provider event is signature-verified before parsing.
- Every accepted event produces an app-owned ledger entry.
- Stable provider event IDs dedupe replays and duplicates.
- Out-of-order events converge on the same final ledger state.
- Retry or reconciliation work is queued when a webhook needs follow-up.

## Acceptance status

- [x] Every accepted provider event requires verified signature state, parsed payload state, and isolated test/live mode.
- [x] Every accepted event projects an app-owned ledger transition through the Rust-owned contract.
- [x] Stable provider event IDs and explicit idempotency/replay state prevent duplicate or replayed double-grants.
- [x] Out-of-order events stay explicit and queue reconciliation rather than silently rewriting access.
- [x] Retry follow-up is explicit for payment failure lifecycles.
- [x] Dead-letter/manual-required outcomes are explicit for malformed, unsigned, mismatched, or mixed-boundary traffic.
- [x] WP03 does not claim entitlement delivery completion; it only proves webhook-to-ledger lifecycle truth.

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

- Focused validation: `cargo test -p ocentra-billing-core --test unit`; `cargo lint-architecture crates/billing-core/src/billing_subscription.rs crates/billing-core/tests/unit/provider_webhook.rs crates/billing-core/tests/unit/subscription_lifecycle.rs`; narrow Prettier check on touched WP03 docs/proof files
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
