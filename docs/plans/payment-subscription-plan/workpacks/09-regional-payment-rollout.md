# Workpack 09: Regional Payment Rollout

## Goal

Define the region matrix, provider availability, currency and tax launch gating, and fallback behavior.

## First-touch surface

- `packages/billing-domain/src/billing-pricing-matrix.ts`
- `packages/billing-domain/src/billing-checkout-portal-boundary.ts`

## Reviewed production truth - 2026-08-25

This workpack is `blocked / source reviewed`; this checkpoint records source
and caller truth only. It adds no completion evidence, tests, proof, CI, PR,
READY, or DONE state.

- The planned `packages/billing-domain` pricing and checkout boundary files
  are absent. The regional strategy and market matrix are explicit policy
  inputs, not a runtime selector or verified provider-availability source.
- No non-test Cloudflare caller selects a provider by region or composes a
  currency/tax launch gate. The Worker accepts a free-form region for the
  support/admin manual-invoice path, while billing status projects only
  `stripe-hosted` and `manual-invoice` provider modes.
- No regional provider API owner, currentness binding, or live fallback caller
  exists. Pakistan and China remain manual/support paths in the matrix; UAE and
  other regional fallbacks remain strategy-only until their provider and
  currency/tax owners exist.
- The mapped Cloudflare pricing and worker-runtime tests use local fixture data
  and do not prove live regional provider execution or launch readiness. The
  WP09 unit-test roots from the proof inventory and the WP09 proof root are
  absent. Regional matrix, currency/tax, manual-required-gap, and
  unsupported-region negative assertions remain open.

## Read inputs

- [PLAN_STATE.md](../PLAN_STATE.md)
- [PLAN_EXECUTION_BLUEPRINT.md](../PLAN_EXECUTION_BLUEPRINT.md)
- [REGIONAL_PAYMENT_MARKET_MATRIX.md](../REGIONAL_PAYMENT_MARKET_MATRIX.md)
- [PAYMENT_PROVIDER_STRATEGY.md](../PAYMENT_PROVIDER_STRATEGY.md)
- [BILLING_API_BOUNDARY.md](../BILLING_API_BOUNDARY.md)

## Output files

- [REGIONAL_PAYMENT_MARKET_MATRIX.md](../REGIONAL_PAYMENT_MARKET_MATRIX.md)
- [PAYMENT_PROVIDER_STRATEGY.md](../PAYMENT_PROVIDER_STRATEGY.md)
- `output/payment-subscription-plan-proof/09-regional-payment-rollout/`

## Acceptance

- Region-specific default provider selection is explicit.
- Unsupported regions degrade to a supported fallback.
- Currency and tax handling are checked before launch.
- Region-disabled behavior is visible and safe.
- The region matrix is closed before any region is marked live.

## Proof IDs

- `payment-region.india`
- `payment-region.pakistan`
- `payment-region.china`
- `payment-region.uae-dubai`

## Validation

- Docs validation: `npm run format:check`; `npm run lint:schema-boundaries`
- Required proof families: `payment-region.canada-us`, `payment-region.india`, `payment-region.pakistan`, `payment-region.china`, `payment-region.uae-dubai`, `payment-region.eu-uk`, `payment-region.southeast-asia`, `payment-region.manual-enterprise`, `payment-region.local-methods`, `payment-region.subscription-support`, `payment-region.manual-required-gaps`
- Proof bundle: `output/payment-subscription-plan-proof/09-regional-payment-rollout/09-regional-payment-matrix.md`, `output/payment-subscription-plan-proof/09-regional-payment-rollout/09-india-razorpay-proof.md`, `output/payment-subscription-plan-proof/09-regional-payment-rollout/09-pakistan-manual-required-proof.md`, `output/payment-subscription-plan-proof/09-regional-payment-rollout/09-china-wallet-proof.md`, `output/payment-subscription-plan-proof/09-regional-payment-rollout/09-uae-provider-proof.md`

## Negative cases

- Reject a region being marked live without a matrix entry.
- Reject unsupported regions silently creating payment claims.
- Reject region-specific currency or tax behavior that is untested.
- Reject client-side region override logic.

## Failure conditions

- Do not mark a region live without closing the matrix.
- Do not let unsupported regions silently create payment claims.
- Do not let tax or currency behavior remain untested for the region.
