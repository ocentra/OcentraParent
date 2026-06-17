# Workpack 08: Provider Adapter Portability

## Goal

Define the normalized adapter contract across Stripe, Razorpay, PayPal, store billing, and manual invoice.

## First-touch surface

- `packages/billing-domain/src/billing-account-runtime-boundary.ts`
- `packages/billing-domain/src/billing-checkout-portal-boundary.ts`

## Read inputs

- [PLAN_STATE.md](../PLAN_STATE.md)
- [PLAN_EXECUTION_BLUEPRINT.md](../PLAN_EXECUTION_BLUEPRINT.md)
- [PAYMENT_PROVIDER_STRATEGY.md](../PAYMENT_PROVIDER_STRATEGY.md)
- [MOBILE_STORE_BILLING_ADAPTERS.md](../MOBILE_STORE_BILLING_ADAPTERS.md)
- [BILLING_API_BOUNDARY.md](../BILLING_API_BOUNDARY.md)
- [REGIONAL_PAYMENT_MARKET_MATRIX.md](../REGIONAL_PAYMENT_MARKET_MATRIX.md)

## Output files

- [PAYMENT_PROVIDER_STRATEGY.md](../PAYMENT_PROVIDER_STRATEGY.md)
- [MOBILE_STORE_BILLING_ADAPTERS.md](../MOBILE_STORE_BILLING_ADAPTERS.md)
- [BILLING_API_BOUNDARY.md](../BILLING_API_BOUNDARY.md)
- `output/payment-subscription-plan-proof/08-provider-adapter-portability/`

## Acceptance

- Provider-specific events normalize to one app-owned ledger model.
- Provider config missing cases fail safely.
- One provider adapter does not leak into another.
- Manual invoice stays a supported adapter, not a separate billing authority.
- Provider selection remains server-side policy.

## Proof IDs

- `payment-provider.stripe-normalized`
- `payment-provider.razorpay-normalized`
- `payment-provider.paypal-normalized`
- `payment-provider.store-normalized`
- `payment-provider.manual-invoice-normalized`
- `payment-provider.cross-provider-parity`

## Validation

- Docs validation: `npm run format:check`; `npm run lint:schema-boundaries`
- Required proof families: `payment-provider.adapter-interface`, `payment-provider.stripe-adapter-contract`, `payment-provider.razorpay-adapter-contract`, `payment-provider.paypal-adapter-contract`, `payment-provider.apple-store-adapter-contract`, `payment-provider.google-play-adapter-contract`, `payment-provider.manual-invoice-adapter-contract`, `payment-provider.normalized-event-contract`, `payment-provider.provider-lock-escape`, `payment-provider.no-direct-product-provider-reads`
- Proof bundle: `output/payment-subscription-plan-proof/08-provider-adapter-portability/08-provider-adapter-contract-proof.md`, `output/payment-subscription-plan-proof/08-provider-adapter-portability/08-normalized-event-proof.md`, `output/payment-subscription-plan-proof/08-provider-adapter-portability/08-provider-lock-escape-proof.md`

## Negative cases

- Reject provider-specific code paths that own product access.
- Reject missing config that silently falls back to the wrong adapter.
- Reject adapter outputs that do not normalize to the same ledger model.
- Reject any adapter that leaks secrets to the browser.

## Failure conditions

- Do not let provider-specific code paths own product access.
- Do not let missing config produce a silent fallback.
- Do not let adapter outputs fail to normalize.
