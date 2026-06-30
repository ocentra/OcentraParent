# Checkout and Billing Portal Model

Purpose: define the hosted payment and self-service flows the parent product should use without turning the browser into a provider-truth or entitlement-truth owner.

## Live WP02 edge model

- Hosted session request and response validation lives in `packages/schema-domain/src/billing-checkout-portal-boundary.ts`.
- Hosted session value guards live in `packages/schema-domain/src/billing-checkout-portal-boundary-values.ts`.
- Runtime and provider ownership do not move to TypeScript here; the live WP02 TypeScript surface is an edge contract and proof-consumer layer only.

## Checkout model

- Checkout sessions are created server-side by the Worker boundary; the browser only submits an allowlisted request shape.
- The browser may choose among server-enabled plans/providers, but the server decides the actual hosted route and provider session.
- The browser must not hold provider secrets or webhook secrets.
- Checkout success is only a return state; the webhook or later verified receipt is what closes the loop.
- The success return state is explicit: `awaiting-provider-webhook`.
- The cancel return state is explicit: `cancelled-before-provider-confirmation`.

## Portal model

- The portal is the self-service path for payment method updates, cancellations, invoices, and plan changes.
- The portal should reflect the current household seats, referral credits, and billing status once server-owned state is available.
- The portal must not expose child telemetry or support-only data.
- If a provider has a hosted portal, use it rather than building a custom browser flow.
- Portal returns are explicit and management-only: `portal-management-only`.

## Browser request boundary

- Interactive hosted session requests accept only `parent` or `guardian` actors.
- Hosted checkout currently accepts only the allowlisted billable plan ids:
  - `family-plus-monthly`
  - `family-monitor-core`
  - `family-monitor-plus`
- The browser request contract requires:
  - `originGateState: same-origin-verified`
  - `csrfState: csrf-token-verified`
  - `surfaceSecretCustody: not-present`
- Return routes must stay on the allowlist and cannot drift between success, cancel, and portal semantics.

## Required behavior

| Action | Expected result |
| --- | --- |
| Create checkout | Return a hosted session request/response contract tied to an app-owned payment record, not a client secret. |
| Return from checkout | Show a pending or processing state until the webhook or later verified receipt lands. |
| Open portal | Launch the hosted or server-managed billing portal for the current billing account. |
| Return from portal | Return to billing management only; do not imply entitlement change. |
| Cancel subscription | Reflect cancellation only after later server-owned ledger and entitlement workpacks prove it. |

## Explicit non-claims

WP02 does not claim:

- payment completion from redirect alone
- provider webhook processing
- provider/backend runtime
- account backend runtime
- entitlement grant or revocation
- portal UI runtime

## Failure conditions

- Do not claim payment completion from a redirect alone.
- Do not make the portal a secret-bearing client flow.
- Do not collapse checkout and referral enrollment into one action.
