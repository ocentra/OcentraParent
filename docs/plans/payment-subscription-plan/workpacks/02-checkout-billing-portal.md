# Workpack 02: Checkout and Billing Portal

## Goal

Define the hosted checkout and billing portal boundary so payment actions stay server-side and the browser never owns provider truth.

## First-touch surface

- `packages/billing-domain/src/billing-checkout-portal-boundary.ts`
- `packages/billing-domain/tests/unit/billing-checkout-portal-boundary.test.ts`

## Read inputs

- [PLAN_STATE.md](../PLAN_STATE.md)
- [PLAN_EXECUTION_BLUEPRINT.md](../PLAN_EXECUTION_BLUEPRINT.md)
- [BILLING_API_BOUNDARY.md](../BILLING_API_BOUNDARY.md)
- [CHECKOUT_BILLING_PORTAL_MODEL.md](../CHECKOUT_BILLING_PORTAL_MODEL.md)
- [PAYMENT_PROVIDER_STRATEGY.md](../PAYMENT_PROVIDER_STRATEGY.md)
- [CLOUDFLARE_BILLING_CONTROL_PLANE.md](../CLOUDFLARE_BILLING_CONTROL_PLANE.md)

## Output files

- [BILLING_API_BOUNDARY.md](../BILLING_API_BOUNDARY.md)
- [CHECKOUT_BILLING_PORTAL_MODEL.md](../CHECKOUT_BILLING_PORTAL_MODEL.md)
- [PAYMENT_PROVIDER_STRATEGY.md](../PAYMENT_PROVIDER_STRATEGY.md)
- `output/payment-subscription-plan-proof/02-checkout-billing-portal/`

## Acceptance

- Checkout sessions are created server-side by the Worker.
- Portal sessions cover payment method updates, cancellations, invoices, and plan changes.
- Redirect success is never used as proof of payment.
- The browser never receives provider secrets or webhook secrets.
- Referral enrollment stays separate from billing checkout.

## Proof IDs

- `payment-checkout.cloudflare-billing-api-required`
- `payment-checkout.hosted-checkout-created`
- `payment-checkout.billing-portal-created`
- `payment-checkout.no-client-secret-exposure`

## Validation

- Docs validation: `npm run format:check`; `npm run lint:schema-boundaries`
- Required proof families: `payment-checkout.auth-required`, `payment-checkout.household-role-required`, `payment-checkout.invalid-product-rejected`, `payment-checkout.redirect-allowlist`, `payment-checkout.origin-csrf-negative`, `payment-checkout.bot-abuse-gate`, `payment-checkout.no-desktop-secrets`, `payment-checkout.no-client-secret-exposure`, `payment-checkout.return-success-not-entitlement`, `payment-checkout.cancel-state`
- Proof bundle: `output/payment-subscription-plan-proof/02-checkout-billing-portal/02-cloudflare-billing-api-boundary-proof.md`, `output/payment-subscription-plan-proof/02-checkout-billing-portal/02-hosted-checkout-proof.md`, `output/payment-subscription-plan-proof/02-checkout-billing-portal/02-billing-portal-proof.md`, `output/payment-subscription-plan-proof/02-checkout-billing-portal/02-no-client-secret-proof.md`, `output/payment-subscription-plan-proof/02-checkout-billing-portal/02-redirect-origin-negative-proof.md`

## Negative cases

- Reject any checkout flow that exposes a provider secret to the browser.
- Reject redirect success as a payment-complete signal.
- Reject portal flows that bypass the hosted or server-managed billing boundary.
- Reject any flow that merges referral enrollment into checkout.

## Failure conditions

- Do not claim payment completion from a redirect alone.
- Do not make the portal a secret-bearing client flow.
- Do not collapse checkout and referral enrollment into one action.
