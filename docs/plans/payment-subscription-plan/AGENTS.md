<!-- agent-capsule -->

> Agent Capsule
> Plan: `payment-subscription-plan`
> Doc: AGENTS
> Kind: plan route and local agent contract.
> Read when: First file inside this plan after a global route selects it.
> Stop rule: Choose one workpack; do not inspect sibling plans unless the selected workpack names a handoff.
> Proves: local routing and ownership only.

<!-- /agent-capsule -->

# Payment Subscription Plan Agent Route

Task: define the Cloudflare-first billing, subscription, referral, and entitlement architecture for the parent product. This plan is the single owner for monetization architecture; do not split a second subscription plan.

Context: the games repo at `E:\ocentra-games\infra\cloudflare` already demonstrates the control-plane shape this plan should reuse: Worker handlers, Durable Objects, provider-normalized checkout flows, signed Stripe webhooks, provider adapters, queue-backed reconciliation, wrangler-based configuration, and a heavy test boundary. Parent should reuse that control-plane pattern, not the game economy model.

Scope: subscription plans, starter bundle, child-device seats, referral credits, provider adapters, checkout and portal sessions, billing dashboard surfaces, support/admin billing ops, invoices/tax/refunds/disputes, webhook lifecycle, app-owned ledgers, signed entitlement snapshots, and proof.

Out of scope: account identity policy, device trust implementation, install/provisioning mechanics, custody/retention policy, policy enforcement, and child telemetry.

## High-Density Execution Contract

- Route first from `PLAN_STATE.md`; this plan owns subscription finance authority and must not absorb unrelated policy/runtime/custody work.
- Work only the selected workpack plus its proof rows and route-sync docs; prefer concrete billing/entitlement proof over implementation prose.
- For each claimed row, capture failure state, evidence path, negative test, rollback path, and adjacent handoff before marking done.
- Stop condition: no DONE/PR_READY for subscriptions without webhook idempotency proof, entitlement correctness, provider/region closure, and billing artifact traceability.

## Research Gate

Before implementation, DONE, or PR_READY, the assigned agent must inspect the owning parent code, `E:\ocentra-games\infra\cloudflare` payment patterns, official Cloudflare/Stripe/Razorpay/PayPal/Apple/Google docs for the touched slice, and the unresolved pricing/referral/dashboard choices with Sujan. Do not treat this route as final until those sources are reconciled.

## Decision Tree

| If the task is about...                                           | Open                                               |
| ----------------------------------------------------------------- | -------------------------------------------------- |
| Pricing, starter bundle, child seats, referral credits            | `workpacks/01-product-pricing-entitlement.md`      |
| Worker/API boundary, checkout sessions, portal sessions           | `workpacks/02-checkout-billing-portal.md`          |
| Webhooks, subscription lifecycle, event idempotency               | `workpacks/03-subscription-webhook-lifecycle.md`   |
| Ledgers, entitlement delivery, signed snapshots, device gates     | `workpacks/04-entitlement-delivery-gates.md`       |
| Invoices, tax, refunds, disputes, cancellation, grace periods     | `workpacks/05-invoice-tax-refund-dispute.md`       |
| Security, abuse, privacy, PCI, observability                      | `workpacks/06-security-privacy-observability.md`   |
| Proof, route sync, rollout gate                                   | `workpacks/07-rollout-proof-and-route-gate.md`     |
| Provider portability across Stripe/Razorpay/PayPal/store adapters | `workpacks/08-provider-adapter-portability.md`     |
| Regional payment rollout and market gating                        | `workpacks/09-regional-payment-rollout.md`         |
| Referral qualification, anti-abuse, and credit lifecycle          | `workpacks/10-referral-growth-entitlement.md`      |
| Parent website billing dashboard                                  | `workpacks/11-parent-website-billing-dashboard.md` |
| Support/admin billing operations                                  | `workpacks/12-support-admin-billing-ops.md`        |

## Architecture Decisions

- Cloudflare Worker/API is the default billing control plane.
- Provider secret operations never leave the server boundary.
- Hosted checkout and hosted portal are the default user-facing billing surfaces.
- Provider is not product authority; app-owned ledgers decide entitlement.
- Billing metadata must stay privacy-safe.
- `device-trust-bootstrap-plan` owns device trust; this plan consumes its result when deriving signed entitlement snapshots.
- `account-identity-family-plan` owns household identity and roles.
- `data-custody-storage-plan` owns retention/export/delete constraints for billing records.
- `portal-ux-household-surfaces-plan` owns the generic household shell; this plan owns billing semantics and states.
- `policy-control-plane-plan` consumes entitlement state only after account, payment, and device-trust authority are proven.

## Handoffs

- `account-identity-family-plan` owns account, household, role, and session authority.
- `device-trust-bootstrap-plan` owns trusted-device bootstrap and the sealed trust required to unlock entitlements.
- `data-custody-storage-plan` owns privacy, export/delete, and retention boundaries for billing-related app records.
- `setup-install-provisioning-plan` owns family site pricing/register/install entry points.
- `portal-ux-household-surfaces-plan` owns the non-billing household shell; this plan owns the billing content and state model.
- `policy-control-plane-plan` and domain plans consume entitlements only after account, payment, and device-trust authority are proven.

## Failure Conditions

- Do not claim paid access from Checkout redirect success alone.
- Do not expose provider secrets or webhook secrets to client code.
- Do not send child data to provider metadata.
- Do not claim subscription correctness without webhook signature, idempotency, lifecycle, refund/dispute, regional provider, and entitlement proof.
