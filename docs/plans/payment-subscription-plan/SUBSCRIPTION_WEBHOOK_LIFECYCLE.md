# Subscription Webhook Lifecycle

Purpose: define how provider events move the billing system through state changes.

## Lifecycle rules

- Every provider event must pass signature validation before parsing.
- Every provider event must be deduplicated using a stable provider event ID or equivalent idempotency key.
- Every accepted provider event must produce an app-owned ledger entry.
- Out-of-order provider events must normalize to the same final ledger state.
- Stripe, Razorpay, PayPal, Apple, and Google events all normalize into the same billing truth.

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

## Required processing steps

- Verify signature.
- Parse event.
- Resolve the billing account and payment attempt.
- Check dedupe marker.
- Append a ledger entry.
- Transition entitlement state if and only if the ledger transition is valid.
- Enqueue reconciliation or retry work if the event needs follow-up.

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
