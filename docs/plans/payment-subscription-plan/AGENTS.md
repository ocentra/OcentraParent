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

Task: define the billing, subscription, referral, and entitlement architecture for the parent product on top of the shared Cloudflare control-plane prerequisite. This plan is the single owner for monetization architecture; do not split a second subscription plan.

Context: `cloudflare-control-plane-plan` owns the repo-local `infra/cloudflare/` module, its bindings, auth boundary, route manifest, local dev loop, test runner, and deployment model. This plan consumes that module and owns monetization semantics: checkout meaning, referral meaning, webhook-to-ledger meaning, entitlement meaning, dashboard meaning, and support/admin meaning. Parent should reuse the shared control-plane pattern, not the game economy model.

Scope: subscription plans, starter bundle, child-device seats, referral credits, provider adapters, checkout and portal sessions, billing dashboard surfaces, support/admin billing ops, invoices/tax/refunds/disputes, webhook lifecycle, app-owned ledgers, signed entitlement snapshots, and proof.

Out of scope: account identity policy, device trust implementation, install/provisioning mechanics, custody/retention policy, policy enforcement, and child telemetry.

## High-Density Execution Contract

- Route first from `PLAN_STATE.md`; this plan owns subscription finance authority and must not absorb unrelated policy/runtime/custody work.
- Work only the selected workpack plus its proof rows and route-sync docs; prefer concrete billing/entitlement proof over implementation prose.
- For each claimed row, capture failure state, evidence path, negative test, rollback path, and adjacent handoff before marking done.
- Stop condition: no DONE/PR_READY for subscriptions without webhook idempotency proof, entitlement correctness, provider/region closure, and billing artifact traceability.
- Use `WORKPACK_FAMILIES.md` only when the selected workpack owner/proof family is unclear.

## Ownership, Import, And Boundary Contract

This plan owns monetization semantics and payment-domain proof. It does not own the shared Cloudflare runtime scaffold, account identity, device trust, data custody, setup/install, policy decisions, enforcement execution, child telemetry, or generic portal shell UX.

Module roles:

```text
billing-domain: TypeScript billing account, pricing, entitlement, referral, dashboard, support-admin, provider-facing contract, and proof-consumer surfaces. Public exports remain authoritative only when they exist.
billing-core: Rust webhook intake, provider lifecycle classification, event/idempotency, dispute/manual-review, and downstream entitlement update requirement helpers.
cloudflare-control-plane-plan: shared Worker/API runtime module, bindings, auth states, route manifest, local dev/test shape, deployment promotion, and payment handoff proof.
infra/cloudflare: Cloudflare runtime surface owned by cloudflare-control-plane-plan; payment consumes only approved billing route groups and handoff contracts.
crates/schema: canonical shared billing/payment/entitlement shapes when contracts cross package, app, crate, or plan boundaries.
account-identity-family-plan and family-domain: account, household, role, session, and parent authority owners.
device-trust-bootstrap-plan: trusted-device binding and local sealed trust owner for signed entitlement consumption.
data-custody-storage-plan: billing-record retention, export, deletion, privacy, and custody owner.
portal-domain/apps/portal/parent-domain: projection, dashboard, and parent-surface proof only when selected; they do not own billing truth.
policy-control-plane-plan and enforcement-control plans: policy decisions and enforcement behavior that consume entitlement state only after account, payment, and device-trust authority are proven.
```

Direct imports are allowed only for explicit public helper surfaces:

```text
billing-domain public contracts/helpers when selected by the workpack
billing-core public Rust lifecycle/eventing helpers when selected by the workpack
crates/schema canonical billing/payment/entitlement shapes
Cloudflare route/handoff contracts produced by cloudflare-control-plane-plan
account/device-trust/data-custody/portal public handoff contracts only when the selected workpack names them
neutral event/evidence/logging/protocol helpers that do not own product behavior
```

Forbidden direct imports and claims:

```text
Cloudflare runtime internals imported to bypass the shared control-plane handoff
provider checkout redirect upgraded into paid access
provider event payload upgraded into app entitlement without server verification and ledger write
provider truth upgraded into product authority
entitlement snapshot upgraded into root trust
billing-domain tests upgraded into parent dashboard proof
support/admin search upgraded into unrestricted support authority
regional provider docs upgraded into launch readiness
referral invite upgraded into referral credit without qualification proof
assertion matrix completeness upgraded into runtime proof
client code receiving provider or webhook secrets
provider metadata containing private child activity or child telemetry
```

If payment work needs account identity, device trust, data custody, setup, portal UX, policy, enforcement, Cloudflare runtime behavior, or child telemetry behavior, it must use typed handoffs, proof roots, and explicit no-claim boundaries. Do not solve cross-plan behavior by importing another feature owner's runtime internals.

## Research Gate

Before implementation, DONE, or PR_READY, the assigned agent must inspect the owning parent code, this plan's payment docs, official Cloudflare/Stripe/Razorpay/PayPal/Apple/Google docs for the touched slice, and the unresolved pricing/referral/dashboard choices with Sujan. Do not treat this route as final until those sources are reconciled.

## Decision Tree

| If the task is about...                                           | Open                                               |
| ----------------------------------------------------------------- | -------------------------------------------------- |
| Cloudflare prerequisite handoff and payment dependency gate       | `workpacks/00-cloudflare-control-plane-handoff.md` |
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

- Cloudflare Worker/API remains the billing runtime boundary, but `cloudflare-control-plane-plan` owns the shared module scaffold, bindings, auth, and test runner that this plan consumes.
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
- `cloudflare-control-plane-plan` owns `infra/cloudflare/`, shared auth states, bindings, local dev/test shape, and payment handoff proof.
- `setup-install-provisioning-plan` owns family site pricing/register/install entry points.
- `portal-ux-household-surfaces-plan` owns the non-billing household shell; this plan owns the billing content and state model.
- `policy-control-plane-plan` and domain plans consume entitlements only after account, payment, and device-trust authority are proven.

## Failure Conditions

- Do not claim paid access from Checkout redirect success alone.
- Do not expose provider secrets or webhook secrets to client code.
- Do not send child data to provider metadata.
- Do not claim subscription correctness without the Cloudflare handoff, webhook signature, idempotency, lifecycle, refund/dispute, regional provider, and entitlement proof.
