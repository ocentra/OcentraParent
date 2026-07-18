# Workpack 02: Checkout and Billing Portal

Status: `blocked / proof-required`

## Goal

Define the hosted checkout and billing portal boundary so payment actions stay server-side and the browser never owns provider truth.

## Execution truth

- Live first-touch edge contracts are `packages/schema-domain/src/billing-checkout-portal-boundary.ts` and `packages/schema-domain/src/billing-checkout-portal-boundary-values.ts`; the older `packages/billing-domain/src/...` references in plan text were stale.
- `packages/billing-domain/tests/unit/billing-checkout-portal-boundary.test.ts` and `scripts/test/payment-checkout-boundary-proof.mjs` now prove hosted checkout and billing-portal request/response boundary states, explicit return resolutions, redirect allowlist, origin/csrf rejection, plan rejection, and client-secret leak rejection.
- `packages/billing-domain/tests/unit/billing-account-runtime-boundary.test.ts` and `scripts/test/billing-account-endpoint-contract-proof.mjs` now prove the billing account endpoint contract and explicit runtime non-claims around provider/backend/portal ownership.
- This packet does not move runtime ownership back into TypeScript. The touched TypeScript files remain thin edge validation or proof-consumer surfaces; Cloudflare Worker runtime, provider runtime, account backend runtime, entitlement runtime, and portal UI runtime remain outside WP02.

## First-touch surface

- `packages/schema-domain/src/billing-checkout-portal-boundary.ts`
- `packages/schema-domain/src/billing-checkout-portal-boundary-values.ts`
- `packages/billing-domain/tests/unit/billing-checkout-portal-boundary.test.ts`
- `packages/billing-domain/tests/unit/billing-account-runtime-boundary.test.ts`
- `scripts/test/payment-checkout-boundary-proof.mjs`
- `scripts/test/billing-account-endpoint-contract-proof.mjs`

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

## Proof root

- `output/payment-subscription-plan-proof/02-checkout-billing-portal/`

## Proved now

- Hosted checkout requests accept only interactive `parent` or `guardian` actors.
- Hosted checkout requests accept only allowlisted billable plan ids.
- Hosted checkout and portal requests require `same-origin-verified`, `csrf-token-verified`, and `surfaceSecretCustody: not-present`.
- Hosted return routes are explicit and stable:
  - checkout success -> `awaiting-provider-webhook`
  - checkout cancel -> `cancelled-before-provider-confirmation`
  - portal return -> `portal-management-only`
- Checkout and portal URLs reject `client_secret=` leakage.
- Explicit rejected checkout states exist for `auth-required`, `invalid-plan`, and `abuse-gate-required` without implying paid success.
- The billing account boundary keeps Cloudflare runtime, provider backend runtime, account backend runtime, portal UI runtime, and child activity custody as explicit non-claims.

## Acceptance

- Checkout sessions are created server-side by the Worker.
- Portal sessions cover payment method updates, cancellations, invoices, and plan changes.
- Redirect success is never used as proof of payment.
- The browser never receives provider secrets or webhook secrets.
- Referral enrollment stays separate from billing checkout.

## Manual-required / blocked

- This workpack still does not claim Cloudflare Worker runtime, provider backend runtime, account backend runtime, entitlement signing runtime, portal UI runtime, or live payment completion.
- Separate household-membership authority beyond the request-role boundary remains an adjacent account-identity/runtime concern and is not claimed from WP02.
- The packet remains `blocked / proof-required` because the required broader validation gates fail outside WP02 scope and the physical proof bundle is still absent:
  - `cmd /c npm run format:check` -> `Code style issues found in 1015 files. Run Prettier with --write to fix.`
  - `cmd /c npm run lint:schema-boundaries` -> `Weak assertion guard failed` in `crates/agent-protocol/tests/contract/*.rs` and `crates/agent-service/tests/unit/lan_pairing.rs`

## Proof IDs

- `payment-checkout.cloudflare-billing-api-required`
- `payment-checkout.hosted-checkout-created`
- `payment-checkout.billing-portal-created`
- `payment-checkout.no-client-secret-exposure`

## Validation

- Docs validation: `npm run format:check`; `npm run lint:schema-boundaries`
- Focused pass commands:
  - `cmd /c npm run build --workspace @ocentra-parent/schema-domain`
  - `cmd /c npm run test --workspace @ocentra-parent/billing-domain -- tests/unit/billing-checkout-portal-boundary.test.ts`
  - `cmd /c npm run test --workspace @ocentra-parent/billing-domain -- tests/unit/billing-account-runtime-boundary.test.ts`
  - `node scripts/test/payment-checkout-boundary-proof.mjs`
  - `node scripts/test/billing-account-endpoint-contract-proof.mjs`
  - `cmd /c npm run lint:architecture -- --files packages/schema-domain/src/billing-checkout-portal-boundary.ts packages/schema-domain/src/billing-checkout-portal-boundary-values.ts packages/billing-domain/tests/unit/billing-checkout-portal-boundary.test.ts scripts/test/payment-checkout-boundary-proof.mjs`
- Required proof families: `payment-checkout.auth-required`, `payment-checkout.household-role-required`, `payment-checkout.invalid-product-rejected`, `payment-checkout.redirect-allowlist`, `payment-checkout.origin-csrf-negative`, `payment-checkout.bot-abuse-gate`, `payment-checkout.no-desktop-secrets`, `payment-checkout.no-client-secret-exposure`, `payment-checkout.return-success-not-entitlement`, `payment-checkout.cancel-state`
- Proof bundle: `output/payment-subscription-plan-proof/02-checkout-billing-portal/02-cloudflare-billing-api-boundary-proof.md`, `output/payment-subscription-plan-proof/02-checkout-billing-portal/02-hosted-checkout-proof.md`, `output/payment-subscription-plan-proof/02-checkout-billing-portal/02-billing-portal-proof.md`, `output/payment-subscription-plan-proof/02-checkout-billing-portal/02-no-client-secret-proof.md`, `output/payment-subscription-plan-proof/02-checkout-billing-portal/02-redirect-origin-negative-proof.md`

## No-claim boundary

- Do not claim payment completion from a redirect alone.
- Do not claim entitlement grant, revocation, or snapshot signing from this packet.
- Do not claim Cloudflare Worker runtime or billing provider backend runtime from this packet.
- Do not claim hosted portal UI runtime from this packet.

## Negative cases

- Reject any checkout flow that exposes a provider secret to the browser.
- Reject redirect success as a payment-complete signal.
- Reject portal flows that bypass the hosted or server-managed billing boundary.
- Reject any flow that merges referral enrollment into checkout.

## Failure conditions

- Do not claim payment completion from a redirect alone.
- Do not make the portal a secret-bearing client flow.
- Do not collapse checkout and referral enrollment into one action.
