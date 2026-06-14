# Payment Subscription Plan State

Status: monetization architecture documented, but blocked behind the shared Cloudflare control-plane handoff; implementation and proof remain open.

Research status: aligned against the current Parent codebase, billing-domain and parent-domain surfaces, the direct `E:\ocentra-games\infra\cloudflare` deep dive, and the new `cloudflare-control-plane-plan` that now owns the shared Worker/module scaffold. This plan remains the single monetization owner; the shared Cloudflare module itself is not owned here.

Current Parent direction:

- `cloudflare-control-plane-plan` owns `infra/cloudflare/`, Wrangler envs, auth states, route manifest, local dev loop, test runner, deployment promotion, and the shared payment handoff gate.
- This plan owns billing semantics on top of that module: pricing, referral qualification, provider strategy, checkout meaning, webhook-to-ledger meaning, entitlement meaning, dashboard meaning, and support/admin meaning.
- Stripe Checkout, Billing, Portal, invoices, entitlements, and webhooks remain the default web control-plane path.
- Razorpay remains the India-native adapter; PayPal remains the secondary wallet/subscription adapter; Apple and Google remain channel adapters, not the root billing authority.
- App-owned billing, referral, and entitlement ledgers remain the access authority.
- Signed entitlement snapshots remain derived artifacts consumed by trusted devices, not the root of trust.
- `packages/parent-domain/tests/unit/billing-entitlement-proof.test.ts` is currently missing; do not overclaim parent-surface proof from billing-domain tests.

## Decision records

| Record | Status | Gap | Closure criteria |
| --- | --- | --- | --- |
| PSP-013 | architecture-closed / implementation-open / sujan-decision-required / provider-setup-required | Final launch order for regional providers and store billing adapters. | Region matrix, provider order, and setup blockers are mirrored in proof and route docs. |
| PSP-014 | architecture-closed / implementation-open / manual-required | Parent dashboard and support/admin field boundaries are documented but not runtime-proven. | Allow/deny field lists and parent/support proof remain synchronized. |
| PSP-015 | architecture-closed / implementation-open / sujan-decision-required | Referral qualification and anti-abuse thresholds are documented but not business-approved or proven. | Qualification thresholds, abuse reviews, and referral grace are approved and proven. |
| PSP-016 | architecture-closed / implementation-open / legal-tax-required | Mixed-provider invoice, tax, refund, dispute, and grace behavior is documented but not legal/tax-closed. | Billing-grace, refund, dispute, cancellation, and tax policy are approved and proven. |
| PSP-017 | architecture-closed / proof-open | Proof matrix and route sync are documented but not fully backed by handoff and runtime proof. | Proof paths, validation logs, route docs, and Cloudflare prerequisite handoff all agree. |

## HID Execution Guard

- Scope and completion source:
  - follow `PLAN_EXECUTION_BLUEPRINT.md`, then this plan's assigned `WORKPACK_INDEX.md`, `NEXT_ACTIONS.md`, and `SOURCE_SURFACE_STATUS_MATRIX.md`.
  - do not mark this plan complete from checklist deltas or scaffold existence alone.
- Before any checked update, attach:
  - a real test run log or explicit blocker from the assigned implementation boundary,
  - a proof path under `docs/proof/payment-subscription-plan/`.
- Failure rule: no PR-ready claim until the Cloudflare handoff, replay/idempotency, authZ/replay, and rollback/teardown proofs are present for the assigned slice.

## Overclaim boundary

This plan is architecture-route ready, not implementation complete. Runtime correctness remains unproven until the selected workpack's code, tests, negative cases, proof bundle, rollback/teardown notes, and validation log exist under `docs/proof/payment-subscription-plan/`.
