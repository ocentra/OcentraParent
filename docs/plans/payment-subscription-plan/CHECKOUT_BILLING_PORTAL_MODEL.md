# Checkout and Billing Portal Model

Purpose: define the hosted payment and self-service flows the parent product should use.

## Checkout model

- Checkout sessions are created server-side by the Worker.
- The browser may choose a provider that the server has enabled, but the server decides the actual route.
- The browser must not hold provider secrets or webhook secrets.
- Checkout success is only a step in the flow; the webhook or receipt is what closes the loop.

## Portal model

- The portal is the self-service path for payment method updates, cancellations, invoices, and plan changes.
- The portal should reflect the current household seats, referral credits, and billing status.
- The portal must not expose child telemetry or support-only data.
- If a provider has a hosted portal, use it rather than building a custom browser flow.

## Required behavior

| Action               | Expected result                                                                     |
| -------------------- | ----------------------------------------------------------------------------------- |
| Create checkout      | Return a session that is tied to the app-owned payment record.                      |
| Return from checkout | Show a pending/processing state until the webhook or receipt lands.                 |
| Open portal          | Launch the hosted or server-managed billing portal for the current billing account. |
| Cancel subscription  | Reflect the cancellation in the ledger and entitlement model.                       |

## Failure conditions

- Do not claim payment completion from a redirect alone.
- Do not make the portal a secret-bearing client flow.
- Do not collapse checkout and referral enrollment into one action.
