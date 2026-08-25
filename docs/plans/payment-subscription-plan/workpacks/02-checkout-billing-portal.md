# Workpack 02: Checkout and Billing Portal

Status: `blocked / route contracts present / provider execution owner, Account authority, boundary adapter, and expected tests open`

## Goal

Define the hosted checkout and billing portal boundary so payment actions stay server-side and the browser never owns provider truth.

## Execution truth

- The actual Cloudflare route manifest owns `POST /auth/billing/checkout` and
  `POST /auth/billing/portal` as parent-session-required billing writes. Their
  request/response codecs are bound to
  `infra/cloudflare/src/generated/billing-contracts.ts`, but both route
  execution bindings are `manual-required` with
  `payment-provider-execution-owner-missing`.
- Both Worker handlers return the shared 501 manual-required response. No
  Stripe/other-provider checkout or portal API client, session creator, or
  real production caller exists. Environment provider-key declarations,
  webhook signature verification, and fixture hosted URLs do not establish
  provider-session custody.
- Account authority is composed for browser-session/status and provider-webhook
  seams through current-authority and provider-mapping reads. The
  `ACCOUNT_IDENTITY_D1` binding remains manual-required, migration `0004`
  requires the Account current-authority schema, and no checkout/portal handler
  consumes that authority to create a provider session.
- Rust-owned canonical templates and generated schema-domain files exist. The
  planned `packages/schema-domain/src/billing-checkout-portal-boundary.ts`
  adapter is absent, and the `packages/billing-domain` package is absent.
- The existing Cloudflare hosted-session tests are local-fixture tests using
  `createTestHarness` and accepted-session assertions; they do not prove a
  live provider caller and are stale against the current manual-required
  source. The planned WP02 package tests and named proof scripts are absent.
- This packet does not move runtime ownership back into TypeScript. Cloudflare
  Worker runtime, provider runtime, Account backend readiness, entitlement
  runtime, and portal UI runtime remain outside the legal WP02 source packet.

## First-touch surface

- `infra/cloudflare/src/routes.ts`
- `infra/cloudflare/src/index.ts`
- `infra/cloudflare/src/auth/verifier.ts`
- `infra/cloudflare/src/storage/account-identity-billing-store.ts`
- `infra/cloudflare/src/generated/billing-contracts.ts`
- `packages/schema-domain/src/billing-checkout-portal-boundary.ts` (missing
  planned adapter)
- `packages/schema-domain/src/billing-checkout-portal-boundary-values.ts`
- `packages/billing-domain/tests/unit/billing-checkout-portal-boundary.test.ts`
  (missing expected test)
- `packages/billing-domain/tests/unit/billing-account-runtime-boundary.test.ts`
  (missing expected test)

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

## Source-level contract facts (not runtime proof)

- Rust/schema-domain contract sources encode interactive actor, allowlisted
  plan, return-route, origin/csrf, surface-secret, and no-client-secret
  boundaries.
- Cloudflare route metadata and generated request/response codecs expose the
  intended hosted-session models, but the execution side is manual-required.
- Local fixture tests contain accepted and rejected hosted-session assertions;
  they do not establish provider API execution, Account authority composition,
  durable provider-session custody, or payment completion.
- Redirect success remains explicitly distinct from paid access, and provider
  webhook/account-ledger ownership remains an open dependency.

## Acceptance intent and current result

- Checkout and portal sessions must be created server-side by an owned provider
  adapter; current Worker execution is manual-required, so this is not met.
- Portal sessions must cover payment method updates, cancellations, invoices,
  and plan changes; no live provider session caller exists.
- Redirect success is never valid proof of payment; this remains a preserved
  boundary fact.
- The browser must never receive provider secrets or webhook secrets; schema
  and route boundary sources encode this, but focused expected tests remain
  open.
- Referral enrollment stays separate from billing checkout.

## Manual-required / blocked

- The Cloudflare checkout and portal execution bindings remain manual-required
  because `payment-provider-execution-owner-missing`; both handlers return
  501 and do not call a provider API.
- Account current-authority/D1 readiness remains manual-required, and the
  Account migration dependency must land before the provider mapping migration
  can be treated as executable runtime custody.
- The schema-domain boundary adapter, the two planned WP02 package tests, and
  the named proof scripts are absent. Existing Cloudflare local-fixture tests
  are not a substitute for those expected tests or a real caller.
- This workpack remains blocked. No completion evidence, READY, DONE, proof,
  deployment, CI, PR, or live payment claim is made.

## Proof IDs

- `payment-checkout.cloudflare-billing-api-required`
- `payment-checkout.hosted-checkout-created`
- `payment-checkout.billing-portal-created`
- `payment-checkout.no-client-secret-exposure`

## Validation

- This docs/graph truth packet did not run WP02 tests, proof, CI, or a
  provider request.
- The future source/test wave must first supply the missing provider owner,
  schema-domain adapter, and expected tests, then run the focused Cloudflare
  route/authority tests, Rust/schema parity tests, and architecture/source-shape
  checks for the actual touched files.
- Whole-plan format/precommit and broad validation remain deferred; no prior
  local-fixture assertion is promoted to runtime evidence here.
- Required proof families: `payment-checkout.auth-required`, `payment-checkout.household-role-required`, `payment-checkout.invalid-product-rejected`, `payment-checkout.redirect-allowlist`, `payment-checkout.origin-csrf-negative`, `payment-checkout.bot-abuse-gate`, `payment-checkout.no-desktop-secrets`, `payment-checkout.no-client-secret-exposure`, `payment-checkout.return-success-not-entitlement`, `payment-checkout.cancel-state`
- Expected proof bundle (currently absent):
  `output/payment-subscription-plan-proof/02-checkout-billing-portal/`

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
