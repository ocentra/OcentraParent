# Workpack 03: Subscription Webhook Lifecycle

Goal: define webhook processing and subscription state lifecycle.

Expected shape:

- Webhooks use raw body signature verification.
- Events are idempotency checked before entitlement changes.
- Subscription state is an app-owned projection from Stripe events, not browser redirect state.
- Unknown events are safe and observable.
- Reconciliation path repairs drift without double-granting.

Expected proof:

- Signature valid/invalid proof.
- Duplicate/replayed/stale event proof.
- Subscription lifecycle matrix.
- Reconciliation proof.

Failure: granting or revoking access from unverified webhook payloads or client redirects.

## Execution Detail

Minimum context:

- `E:\ocentra-games\infra\cloudflare\src\handlers\webhooks-stripe.ts`
- `E:\ocentra-games\infra\cloudflare\src\flows\stripe-webhook-flow.ts`
- `E:\ocentra-games\infra\cloudflare\src\utils\stripe-webhook-signature.ts`
- Official Stripe webhook docs for current signature requirements.

Research required:

- Confirm Workers raw-body handling for Stripe signatures in Parent deployment shape.
- Decide the Parent subscription event set before coding.
- Discuss lifecycle edge cases with Sujan: failed renewal, cancellation, trial end, refund, dispute, grace.

Required event classes:

- checkout completed.
- subscription created/updated/deleted.
- invoice paid/payment failed.
- payment intent succeeded/failed when relevant.
- customer portal changes.
- dispute/refund events.

Expected tests/proof names:

- `stripe-webhook.signature-valid`
- `stripe-webhook.signature-invalid`
- `stripe-webhook.duplicate-event-idempotent`
- `stripe-webhook.unknown-event-safe`
- `stripe-webhook.retry-no-double-grant`
- `subscription.lifecycle-matrix`

Proof artifact expectations:

- Webhook event matrix.
- Idempotency proof.
- Entitlement transition proof.
- Reconciliation/drift repair note.
