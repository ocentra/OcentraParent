# Payment Subscription Plan State

Status: engineering-grade monetization spec is complete, but runtime execution remains blocked behind the shared Cloudflare control-plane handoff; implementation and proof remain open.

Research status: aligned against the current Parent codebase, billing-domain and parent-domain surfaces, the reusable games Cloudflare deep dive summarized in `docs/plans/cloudflare-control-plane-plan/GAMES_INFRA_PARITY_MAP.md`, and the new `cloudflare-control-plane-plan` that now owns the shared Worker/module scaffold. This plan remains the single monetization owner; the shared Cloudflare module itself is not owned here.

## Current ownership interpretation

```text
billing-domain:
  TypeScript billing account, entitlement, support-admin, and payment-facing contract/proof-consumer surfaces. Public package exports are authoritative only where they exist; internal first-touch files are implementation targets, not public API by themselves.

billing-core:
  Rust provider webhook intake, subscription lifecycle classification, event/idempotency, dispute/manual-review state, and downstream entitlement update helper logic.

cloudflare-control-plane-plan:
  Owner of infra/cloudflare, shared Worker/API scaffold, auth, bindings, local dev/test, deploy promotion, and payment handoff proof.

schema-domain:
  Canonical shared billing/payment/entitlement shapes when they cross package, app, crate, or plan boundaries.

account-identity-family-plan:
  Account, household, role, session, and parent authority owner.

device-trust-bootstrap-plan:
  Trusted-device binding and local sealed trust owner for signed entitlement consumption.

data-custody-storage-plan:
  Billing-record privacy, retention, export, deletion, and custody owner.

portal-domain/apps/portal/parent-domain:
  Parent dashboard and projection surfaces only when selected; they do not own billing truth.

policy-control-plane-plan:
  Consumer of proven entitlement state after account, payment, and device-trust authority are proven.
```

## Current coupling risks

```text
- Cloudflare route presence is not payment runtime readiness.
- Checkout redirect success is not paid access.
- Provider payloads are not app authority without server verification and ledger writes.
- Provider state is not the root of entitlement.
- Signed entitlement snapshots are derived artifacts, not the root of trust.
- Billing-domain tests are not parent dashboard proof unless the targeted parent-surface proof runs.
- Support/admin search is not support authority without role, redaction, and audit proof.
- Assertion matrix completeness is not runtime proof.
- Regional/provider documentation is not launch readiness.
```

## Current proof interpretation

```text
output/payment-subscription-plan-proof/<workpack>/ is the canonical proof root.
The upstream Cloudflare handoff gate is output/cloudflare-control-plane-plan-proof/12-payment-plan-handoff-gate/payment-handoff-proof.md.
All runtime payment rows remain open until selected code, tests, negative cases, rollback/teardown notes, validation logs, and proof bundles exist.
WP00 blocks all runtime payment work until the handoff proof or exact blocker is recorded.
```

Current Parent direction:

- `cloudflare-control-plane-plan` owns `infra/cloudflare/`, Wrangler envs, auth states, route manifest, local dev loop, test runner, deployment promotion, and the shared payment handoff gate.
- This plan owns billing semantics on top of that module: pricing, referral qualification, provider strategy, checkout meaning, webhook-to-ledger meaning, entitlement meaning, dashboard meaning, and support/admin meaning.
- Stripe Checkout, Billing, Portal, invoices, entitlements, and webhooks remain the default web control-plane path.
- Razorpay remains the India-native adapter; PayPal remains the secondary wallet/subscription adapter; Apple and Google remain channel adapters, not the root billing authority.
- App-owned billing, referral, and entitlement ledgers remain the access authority.
- Signed entitlement snapshots remain derived artifacts consumed by trusted devices, not the root of trust.
- The payment spec now includes an exhaustive assertion matrix keyed to every required workpack test and proof ID, plus explicit spec-versus-runtime boundaries for dashboard, support/admin, regional, referral, and lifecycle slices.
- `packages/parent-domain/tests/unit/billing-entitlement-proof.test.ts` exists, but the targeted parent-surface proof remains blocked until `@ocentra-parent/parent-domain` builds cleanly enough to run it; do not overclaim parent-surface proof from billing-domain tests.
- The canonical payment proof root is `output/payment-subscription-plan-proof/`; any proof reference outside that root is legacy drift, and the WP00 upstream handoff gate is `output/cloudflare-control-plane-plan-proof/12-payment-plan-handoff-gate/payment-handoff-proof.md`.

## Decision records

| Record | Status | Gap | Closure criteria |
| --- | --- | --- | --- |
| PSP-013 | architecture-closed / implementation-open / sujan-decision-required / provider-setup-required | Final launch order for regional providers and store billing adapters. | Region matrix, provider order, and setup blockers are mirrored in proof and route docs. |
| PSP-014 | architecture-closed / implementation-open / manual-required | Parent dashboard and support/admin field boundaries are documented but not runtime-proven. | Allow/deny field lists and parent/support proof remain synchronized. |
| PSP-015 | architecture-closed / implementation-open / sujan-decision-required | Referral qualification and anti-abuse thresholds are documented but not business-approved or proven. | Qualification thresholds, abuse reviews, and referral grace are approved and proven. |
| PSP-016 | architecture-closed / implementation-open / legal-tax-required | Mixed-provider invoice, tax, refund, dispute, and grace behavior is documented but not legal/tax-closed. | Billing-grace, refund, dispute, cancellation, and tax policy are approved and proven. |
| PSP-017 | architecture-closed / spec-complete / runtime-open | Proof matrix, route sync, and exact assertion scope are documented, but not fully backed by handoff and runtime proof. | Proof paths, assertion matrix, validation logs, route docs, and Cloudflare prerequisite handoff all agree. |

## HID Execution Guard

- Scope and completion source:
  - follow `PLAN_EXECUTION_BLUEPRINT.md`, then this plan's assigned `WORKPACK_INDEX.md`, `NEXT_ACTIONS.md`, `WORKPACK_FAMILIES.md` when needed, and `SOURCE_SURFACE_STATUS_MATRIX.md`.
  - do not mark this plan complete from checklist deltas, assertion matrix completeness, scaffold existence, or provider docs alone.
- Before any checked update, attach:
  - a real test run log or explicit blocker from the assigned implementation boundary,
  - a proof path under `output/payment-subscription-plan-proof/`.
- Failure rule: no PR-ready claim until the Cloudflare handoff, replay/idempotency, authZ/replay, and rollback/teardown proofs are present for the assigned slice.

## Overclaim boundary

This plan is architecture-route ready, not implementation complete. Runtime correctness remains unproven until the selected workpack's code, tests, negative cases, proof bundle, rollback/teardown notes, and validation log exist under `output/payment-subscription-plan-proof/` and the relevant scoped validation actually passes.
