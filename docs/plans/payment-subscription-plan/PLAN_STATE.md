# Payment Subscription Plan State

Status: the engineering-grade monetization spec is complete, but runtime and launch readiness are not. WP00 remains `blocked / proof-required`. WP03 and WP04 have implemented Rust-owned source plus focused test evidence, but their canonical proof directories are physically absent in this checkout, so those workpacks remain `blocked / proof-required / WP00-blocked` rather than `done` or `proof-present`. Payment also remains blocked by account and trusted-device authority, deployment and data-custody proof, PSP-013 provider/region/setup decisions, PSP-014 dashboard/support runtime proof, PSP-015 referral and anti-abuse approval, and PSP-016 legal/tax/refund/dispute/grace closure.

Research status: aligned against the current Parent codebase, billing-domain and parent-domain surfaces, the reusable games Cloudflare deep dive summarized in `docs/plans/cloudflare-control-plane-plan/GAMES_INFRA_PARITY_MAP.md`, and the new `cloudflare-control-plane-plan` that now owns the shared Worker/module scaffold. This plan remains the single monetization owner; the shared Cloudflare module itself is not owned here.

## Current ownership interpretation

```text
billing-domain:
  TypeScript billing account, entitlement, support-admin, and payment-facing contract/proof-consumer surfaces. Public package exports are authoritative only where they exist; internal first-touch files are implementation targets, not public API by themselves.

billing-core:
  Rust provider webhook intake, subscription lifecycle classification, event/idempotency, dispute/manual-review state, and downstream entitlement update helper logic.

cloudflare-control-plane-plan:
  Owner of infra/cloudflare, shared Worker/API scaffold, auth, bindings, local dev/test, deploy promotion, and payment handoff proof.

crates/schema:
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
WP00's expected proof path is `output/payment-subscription-plan-proof/00-cloudflare-control-plane-handoff/`, but this checkout does not yet track a generated proof bundle there.
WP00 remains blocked until the Cloudflare handoff proof is generated, validated, and consumed downstream instead of being treated as already consumed.
WP01 has a declared pricing and entitlement proof path at `output/payment-subscription-plan-proof/01-product-pricing-entitlement/`.
WP02 has a declared checkout and billing portal proof path at `output/payment-subscription-plan-proof/02-checkout-billing-portal/`.
WP03 has a declared Rust-owned webhook lifecycle proof path at `output/payment-subscription-plan-proof/03-subscription-webhook-lifecycle/`, but that directory is physically absent in this checkout.
WP04 has a declared Rust-owned entitlement-delivery proof path at `output/payment-subscription-plan-proof/04-entitlement-delivery-gates/`, but that directory is physically absent in this checkout.
Existing source and focused tests are implementation evidence; they do not substitute for the missing negative-case, rollback/teardown, validation-log, and custody artifacts or clear the WP00 dependency.
All runtime payment rows after WP00 remain proof-required until selected code, tests, negative cases, rollback/teardown notes, validation logs, and proof bundles exist.
```

Current Parent direction:

- `cloudflare-control-plane-plan` owns `infra/cloudflare/`, Wrangler envs, auth states, route manifest, local dev loop, test runner, deployment promotion, and the shared payment handoff gate.
- WP00 remains blocked / proof-required: payment cannot claim the Cloudflare module/auth/route/testing truth as consumed until the upstream handoff proof is generated, validated, and downstream-consumed; the current blockers are the absent handoff proof, unresolved account-auth/trusted-device authority, and absent portal-smoke/deployment proof bundles.
- WP01 is closed as `done / proof-required`: the shared payment proof surface now makes the one-parent-plus-one-child starter bundle explicit, keeps extra parent access separate from child-seat math, derives the effective child-device limit from base plus referral plus paid seats, carries visible over-limit grace, preserves safety-critical local behavior under degradation, and rejects game-economy pricing references.
- WP01 does not restore TS ownership: `crates/schema` remains the Rust-first canonical contract target, while the `packages/schema-domain/src/*` edits in this packet are transitional thin edge validation and proof data only.
- WP02 is closed only as `blocked / proof-required`: the schema-domain edge contracts make hosted checkout/portal return states, redirect/origin/csrf/secret boundaries, invalid-plan rejection, and no-client-secret leakage explicit, while keeping Cloudflare Worker runtime, provider/backend runtime, account backend runtime, portal UI runtime, and entitlement completion as non-claims. WP02 remains blocked by the current broader validation failures in `npm run format:check` and `npm run lint:schema-boundaries`.
- WP03 source and focused-test evidence is present: `crates/billing-core` carries provider channel, payload-parse, idempotency, replay/order, retry, dead-letter, reconciliation, and test/live boundary truth with unit coverage under `crates/billing-core/tests/unit/**`. Workpack closure remains `blocked / proof-required / WP00-blocked` because its physical proof bundle is absent. TypeScript does not own this webhook lifecycle contract.
- WP04 source and focused-test evidence is present: `crates/entitlement-core` owns signed entitlement snapshot derivation, provider input-only boundary, referral seat recalculation, fixed snapshot-model fields, and the snapshot-to-device gate bridge with unit coverage under `crates/entitlement-core/tests/unit/**`. Workpack closure remains `blocked / proof-required / WP00-blocked` because its physical proof bundle is absent. TypeScript remains proof-consumer only for this slice.
- This plan owns billing semantics on top of that module: pricing, referral qualification, provider strategy, checkout meaning, webhook-to-ledger meaning, entitlement meaning, dashboard meaning, and support/admin meaning.
- Stripe Checkout, Billing, Portal, invoices, entitlements, and webhooks remain the default web control-plane path.
- Razorpay remains the India-native adapter; PayPal remains the secondary wallet/subscription adapter; Apple and Google remain channel adapters, not the root billing authority.
- App-owned billing, referral, and entitlement ledgers remain the access authority.
- Signed entitlement snapshots remain derived artifacts consumed by trusted devices, not the root of trust.
- The payment spec now includes an exhaustive assertion matrix keyed to every required workpack test and proof ID, plus explicit spec-versus-runtime boundaries for dashboard, support/admin, regional, referral, and lifecycle slices.
- `packages/parent-domain/tests/unit/billing-entitlement-proof.test.ts` exists, but the targeted parent-surface proof remains blocked until `@ocentra-parent/parent-domain` builds cleanly enough to run it; do not overclaim parent-surface proof from billing-domain tests.
- The canonical payment proof root is `output/payment-subscription-plan-proof/`; any proof reference outside that root is legacy drift, and WP00's upstream handoff gate is an expected path only until the proof is actually produced.

## Decision records

| Record | Status | Gap | Closure criteria |
| --- | --- | --- | --- |
| PSP-013 | architecture-closed / implementation-open / sujan-decision-required / provider-setup-required | Final launch order for regional providers and store billing adapters. | Region matrix, provider order, and setup blockers are mirrored in proof and route docs. |
| PSP-014 | architecture-closed / implementation-open / manual-required | Parent dashboard and support/admin field boundaries are documented but not runtime-proven. | Allow/deny field lists and parent/support proof remain synchronized. |
| PSP-015 | architecture-closed / implementation-open / sujan-decision-required | Referral qualification and anti-abuse thresholds are documented but not business-approved or proven. | Qualification thresholds, abuse reviews, and referral grace are approved and proven. |
| PSP-016 | architecture-closed / implementation-open / legal-tax-required | Mixed-provider invoice, tax, refund, dispute, and grace behavior is documented but not legal/tax-closed. | Billing-grace, refund, dispute, cancellation, and tax policy are approved and proven. |
| PSP-017 | architecture-closed / spec-complete / wp00-proof-required / selected-source-tests-present | Proof matrix, route sync, and exact assertion scope are documented, but WP00 is still missing the Cloudflare handoff proof and WP03/WP04 lack physical proof bundles in this checkout. | Proof paths, assertion matrix, validation logs, route docs, generated WP00/WP03/WP04 proof bundles, and the upstream Cloudflare handoff all agree; upstream blockers are either cleared or explicitly accepted. |

## HID Execution Guard

- Scope and completion source:
  - follow `PLAN_EXECUTION_BLUEPRINT.md`, then this plan's assigned `WORKPACK_INDEX.md`, `NEXT_ACTIONS.md`, `WORKPACK_FAMILIES.md` when needed, and `SOURCE_SURFACE_STATUS_MATRIX.md`.
  - do not mark this plan complete from checklist deltas, assertion matrix completeness, scaffold existence, or provider docs alone.
- Before any checked update, attach:
  - a real test run log or explicit blocker from the assigned implementation boundary,
  - a proof path under `output/payment-subscription-plan-proof/`.
- Failure rule: no PR-ready claim until the Cloudflare handoff, replay/idempotency, authZ/replay, and rollback/teardown proofs are present for the assigned slice.

## Overclaim boundary

This plan is architecture-route ready, not implementation complete. WP00 records the upstream dependency boundary and exact blocker set only. WP03 and WP04 retain independently inspectable Rust source and focused tests, but their workpack completion is not proven while their physical proof bundles are absent and WP00 is unaccepted. Broader runtime correctness stays proof-required until the selected workpack's code, tests, negative cases, proof bundle, rollback/teardown notes, and validation log exist under `output/payment-subscription-plan-proof/` and the relevant scoped validation is captured or explicitly blocked.
