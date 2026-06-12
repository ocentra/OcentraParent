<!-- agent-capsule -->

> Agent Capsule
> Plan: `payment-subscription-plan`
> Doc: AGENTS
> Kind: plan route and local agent contract.
> Read when: First file inside this plan after a global route selects it.
> Stop rule: Choose one workpack; do not inspect account/data/setup plans unless the selected workpack names a handoff.
> Proves: local routing and ownership only.

<!-- /agent-capsule -->

# Payment Subscription Plan Agent Route

Task: define payment and subscription architecture for the parent product.
Context: the games project uses Cloudflare Workers, Stripe Checkout, Stripe Billing/Portal, signed Stripe webhooks, Durable Object payment state, Turnstile, Firebase-backed auth/admin checks, and redacted logs. Parent should reuse the architectural lessons, not the game economy model.
Scope: products/prices, household subscriptions, entitlement gates, checkout session creation, billing portal, webhook ingestion, refunds/disputes, invoices/tax, cancellation, grace period, abuse controls, and proof.
Out of scope: account login provider choice, public site content, package install mechanics, child data custody, and domain enforcement.

## High-Density Execution Contract

- Route first from `PLAN_STATE.md`; this plan owns subscription finance authority and must not absorb unrelated policy/app runtime work.
- Work only the selected workpack plus required checklist/proof rows; prefer concrete billing/entitlement proof over implementation prose.
- For each claimed row, capture failure state, evidence path, negative test/rollback path, and cross-plan dependency closure before marking done.
- Stop condition: no DONE/PR_READY for subscriptions without webhook idempotency proof, entitlement correctness, and billing artifact traceability.

## Research Gate

This plan is first-pass. Before implementation, DONE, or PR_READY, the assigned agent must inspect existing repo account/portal/domain code, `E:\ocentra-games` Cloudflare/Firebase/Stripe payment patterns, current Stripe docs for the touched slice, and unresolved pricing/entitlement/privacy choices with Sujan. Do not treat this first-pass plan as final architecture.

## Decision Tree

| If the task is about...                                           | Open                                             |
| ----------------------------------------------------------------- | ------------------------------------------------ |
| Pricing, household tiers, device seats, trials                    | `workpacks/01-product-pricing-entitlement.md`    |
| Stripe Checkout, Customer Portal, direct-vs-worker decision       | `workpacks/02-checkout-billing-portal.md`        |
| Webhooks, subscription lifecycle, event idempotency               | `workpacks/03-subscription-webhook-lifecycle.md` |
| Entitlement delivery to account, household, device, feature gates | `workpacks/04-entitlement-delivery-gates.md`     |
| Invoices, tax, refunds, disputes, cancellation, grace periods     | `workpacks/05-invoice-tax-refund-dispute.md`     |
| Security, abuse, privacy, PCI, observability                      | `workpacks/06-security-privacy-observability.md` |
| Proof, route sync, rollout gate                                   | `workpacks/07-rollout-proof-and-route-gate.md`   |

## Architecture Decision

- Do not put Stripe secret operations in the browser.
- Use a server boundary, preferably Cloudflare Worker/API, to create Checkout Sessions, Customer Portal Sessions, and webhook handlers.
- Use Stripe-hosted Checkout for subscriptions unless a later proof justifies a custom embedded flow.
- Use Stripe Billing for recurring subscriptions; do not build manual renewal loops with raw PaymentIntents.
- Use Stripe Customer Portal for self-service plan management unless product/legal requirements force custom flows.
- Store payment provider references and entitlement state in an app-owned ledger; never treat the browser redirect alone as proof of payment.
- Keep Stripe metadata privacy-safe: billing account/household/customer ids are allowed; child names, child activity, location, screenshots, policy details, and sensitive evidence are not.

## Handoffs

- `account-identity-family-plan` owns account, household, role, and session authority.
- `data-custody-storage-plan` owns privacy, export/delete, and retention boundaries for billing-related app records.
- `setup-install-provisioning-plan` owns family site pricing/register/install entry points.
- `policy-control-plane-plan` and domain plans consume entitlements only after account and payment authority are proven.

## Failure Conditions

- Do not claim paid access from Checkout redirect success alone.
- Do not expose Stripe secrets or webhook secrets to client code.
- Do not send child data to Stripe metadata.
- Do not claim subscription correctness without webhook signature, idempotency, lifecycle, refund/dispute, and entitlement proof.
