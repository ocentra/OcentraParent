# Subscription Webhook Lifecycle

Purpose: define how provider events move the billing system through state changes.

## Lifecycle rules

- Stripe, Razorpay, PayPal, Apple, and Google webhook channels normalize into one Rust-owned lifecycle contract in `crates/billing-core/src/billing_subscription.rs`.
- Every provider event must present a verified signature state and a parsed payload state before it can be accepted into billing truth.
- Every provider event must be deduplicated using a stable provider event ID plus explicit app-owned idempotency, replay, and ordering state.
- Every accepted provider event must produce an app-owned ledger entry.
- Replayed or out-of-order provider events must normalize to the same final ledger state without double-granting access.
- Retry, dead-letter, reconciliation, and test/live boundary state must remain explicit in the app-owned webhook decision; they are not implicit Cloudflare behavior.

## Normalized states

| State | Meaning |
| --- | --- |
| `session_created` | Checkout or billing session was created. |
| `payment_pending` | Provider acknowledged the purchase but settlement is not final. |
| `payment_succeeded` | Payment is confirmed and can advance entitlement. |
| `payment_failed` | Payment failed and may enter grace. |
| `subscription_active` | Recurring billing is active. |
| `subscription_past_due` | A renewal failed and the household may be in grace. |
| `subscription_canceled` | Subscription ended per policy. |
| `refund_pending` / `refunded` | Reversal is being processed or has completed. |
| `dispute_open` / `dispute_closed` | A dispute or chargeback is active or resolved. |

## Normalized ingress boundary fields

| Field | Meaning |
| --- | --- |
| `provider` | Canonical webhook source channel: Stripe, Razorpay, PayPal, Apple, or Google. |
| `mode` | Test or live provider mode. |
| `signature_state` | Whether the provider signature boundary verified, was missing, or was invalid. |
| `payload_parse_state` | Whether the payload parsed, was malformed, or was unsupported. |
| `idempotency_state` | Whether the provider event ID is fresh or duplicate. |
| `replay_state` | Whether the webhook was fresh or replayed. |
| `ordering_state` | Whether the webhook arrived in-order or out-of-order. |
| `retry_state` | Whether retry follow-up must be queued. |
| `dead_letter_state` | Whether the event must be carried as a manual-required dead-letter outcome. |
| `reconciliation_state` | Whether reconciliation follow-up must be queued to repair drift. |
| `test_live_boundary_state` | Whether test/live traffic stayed isolated or mixed and therefore blocked. |

## Required processing steps

- Verify signature.
- Parse event into the normalized Rust-owned webhook contract.
- Resolve the billing account and payment attempt.
- Check idempotency, replay, and ordering markers.
- Append a ledger entry for every accepted lifecycle change; rejected/no-write cases remain explicit decision artifacts.
- Transition entitlement state if and only if the ledger transition is valid.
- Enqueue retry or reconciliation work when a payment failure, replay-safe repair, refund/dispute, or ordering gap needs follow-up.
- Reject mixed test/live traffic and malformed or unsigned payloads into manual-required dead-letter state.

## Proof hooks

- `payment-webhook.stripe-signature-valid`
- `payment-webhook.razorpay-signature-valid`
- `payment-webhook.paypal-webhook-verified`
- `payment-webhook.duplicate-event-idempotent`
- `payment-webhook.replayed-event-rejected`
- `payment-webhook.reconciliation-repairs-drift`

## Failure conditions

- Do not double-grant entitlement on duplicate events.
- Do not accept payloads from a provider channel without signature validation.
- Do not let a webhook alter access without a ledger entry.
- Do not let replayed or out-of-order events silently change access without reconciliation state.
- Do not let test and live provider traffic mix into one accepted lifecycle path.
