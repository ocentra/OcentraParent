# Payment Subscription Plan State

## Production reachability audit - 2026-08-16

The payment workpacks were checked against non-test callers and owned runtime
boundaries. No payment-owned production slice is authorized from this pass.

- **WP00:** no implementation source; the Cloudflare handoff remains an
  upstream blocked dependency, not payment runtime.
- **WP01:** Rust schema and generated/edge TypeScript surfaces exist, but no
  billing production caller consumes the pricing/seat model. The package
  surfaces remain proof/contract boundaries, not shipped entitlement runtime.
- **WP02:** checkout and portal contracts exist, while the actual Worker path
  is owned by `cloudflare-control-plane-plan`. Its reachable handlers call
  `infra/cloudflare/src/billing-binding-read-model.ts`, which currently
  auto-seeds and falls back to fixture-built billing data when durable rows are
  absent. Hardening that seam is a Cloudflare-owned production task, not a
  legal Payment-plan edit.
- **WP03:** `crates/billing-core` contains the lifecycle classifier and
  idempotency helpers, but no non-test caller outside the crate was found. The
  Cloudflare webhook route parses/queues generic payload data and does not
  invoke the Rust lifecycle owner or write the app ledger.
- **WP04:** `crates/entitlement-core` owns a fail-closed derivation contract,
  but no non-test downstream consumer was found in the payment/runtime
  surfaces. Device-trust binding remains adjacent-plan owned.
- **WP05:** invoice/tax/refund/dispute semantics exist as contract/model code,
  but the reachable Cloudflare mutation path still uses fixture builders and
  lacks provider-owned ledger authority. No production slice is legal here.
- **WP06:** billing security/privacy contracts exist; shared Worker auth,
  binding, redaction, and observability remain Cloudflare-owned. No payment
  runtime caller or provider-secret authority was found.
- **WP07:** rollout/route gate is proof-only and has no production source.
- **WP08/WP09:** provider portability and regional routing are strategy and
  contract surfaces; no live provider adapter, credential owner, or verified
  regional runtime input is present.
- **WP10:** referral/entitlement model code and local seed fixtures exist, but
  no non-test production qualification/credit caller was found.
- **WP11:** the graph maps a parent dashboard to `packages/parent-domain`,
  which is absent in this checkout; actual portal source has no billing summary
  consumer. This is stale topology plus a missing portal owner, not a legal
  payment edit.
- **WP12:** support/admin contracts and Worker route handlers exist, but their
  read model can fall back to fixtures and provider/account role authority is
  unresolved. Runtime support authority remains unproven.

The rejected candidate was a fail-closed production guard around the Worker
read-model fallback. It has a real caller, but its owning files are under
`cloudflare-control-plane-plan`; no cross-plan code change was retained. The
next legal production owners are Cloudflare for fixture-seed/fallback removal,
an actual runtime consumer for `billing-core`, and an actual portal consumer
for WP11. No tests, builds, proof, CI, or graph edits were run.

Status: engineering-grade monetization spec is complete, WP00 now has a real `blocked / proof-present` handoff bundle, WP01 now has a real `done / proof-present` pricing bundle, WP02 now has a real `blocked / proof-present` checkout and billing-portal bundle, WP03 now has a real Rust-owned `done` webhook lifecycle bundle, WP04 now has a real Rust-owned `done / proof-present` entitlement-delivery bundle, and broader runtime execution remains blocked behind the exact upstream Cloudflare blocker set plus the current broader-workspace validation blockers carried by WP02.

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
WP00 now has a real blocked-state proof bundle at `output/payment-subscription-plan-proof/00-cloudflare-control-plane-handoff/`.
That bundle consumes `output/cloudflare-control-plane-plan-proof/12-payment-plan-handoff-gate/payment-handoff-proof.md` and carries its exact blocker set forward into payment.
WP01 now has a real pricing and entitlement proof bundle at `output/payment-subscription-plan-proof/01-product-pricing-entitlement/`.
WP02 now has a real checkout and billing portal proof bundle at `output/payment-subscription-plan-proof/02-checkout-billing-portal/`.
WP03 now has a real Rust-owned webhook lifecycle proof bundle at `output/payment-subscription-plan-proof/03-subscription-webhook-lifecycle/`.
WP04 now has a real Rust-owned entitlement-delivery proof bundle at `output/payment-subscription-plan-proof/04-entitlement-delivery-gates/`.
All runtime payment rows after WP00 remain blocked until selected code, tests, negative cases, rollback/teardown notes, validation logs, and proof bundles exist.
```

Current Parent direction:

- `cloudflare-control-plane-plan` owns `infra/cloudflare/`, Wrangler envs, auth states, route manifest, local dev loop, test runner, deployment promotion, and the shared payment handoff gate.
- WP00 is closed only as `blocked / proof-present`: payment consumed the Cloudflare module/auth/route/testing truth, but the upstream handoff still carries missing Cloudflare WP03/WP05/WP09/WP11 proof roots, missing billing-domain runtime boundary modules, unresolved account-auth/trusted-device authority, and blocked portal-smoke/deployment proof.
- WP01 is closed as `done / proof-present`: the shared payment proof surface now makes the one-parent-plus-one-child starter bundle explicit, keeps extra parent access separate from child-seat math, derives the effective child-device limit from base plus referral plus paid seats, carries visible over-limit grace, preserves safety-critical local behavior under degradation, and rejects game-economy pricing references.
- WP01 does not restore TS ownership: `crates/schema` remains the Rust-first canonical contract target, while the `packages/schema-domain/src/*` edits in this packet are transitional thin edge validation and proof data only.
- WP02 is closed only as `blocked / proof-present`: the live schema-domain edge contracts now make hosted checkout/portal return states, redirect/origin/csrf/secret boundaries, invalid-plan rejection, and no-client-secret leakage explicit, while keeping Cloudflare Worker runtime, provider/backend runtime, account backend runtime, portal UI runtime, and entitlement completion as non-claims. WP02 remains blocked by the current broader validation failures in `npm run format:check` and `npm run lint:schema-boundaries`.
- WP03 is closed as `done`: `crates/billing-core` now carries provider channel, payload-parse, idempotency, replay/order, retry, dead-letter, reconciliation, and test/live boundary truth with real unit coverage under `crates/billing-core/tests/unit/**`. TypeScript does not own this webhook lifecycle contract.
- WP04 is closed as `done / proof-present`: `crates/entitlement-core` now owns signed entitlement snapshot derivation, provider input-only boundary, referral seat recalculation, fixed snapshot-model fields, and the snapshot-to-device gate bridge with real unit coverage under `crates/entitlement-core/tests/unit/**`. TypeScript remains proof-consumer only for this slice.
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
| PSP-017 | architecture-closed / spec-complete / wp00-proof-present / runtime-blocked | Proof matrix, route sync, and exact assertion scope are documented, and WP00 now has a real payment proof root, but runtime remains blocked by the carried upstream Cloudflare handoff blockers. | Proof paths, assertion matrix, validation logs, route docs, WP00 payment proof, and the upstream Cloudflare handoff all agree; upstream blockers are either cleared or carried honestly. |

## HID Execution Guard

- Scope and completion source:
  - follow `PLAN_EXECUTION_BLUEPRINT.md`, then this plan's assigned `WORKPACK_INDEX.md`, `NEXT_ACTIONS.md`, `WORKPACK_FAMILIES.md` when needed, and `SOURCE_SURFACE_STATUS_MATRIX.md`.
  - do not mark this plan complete from checklist deltas, assertion matrix completeness, scaffold existence, or provider docs alone.
- Before any checked update, attach:
  - a real test run log or explicit blocker from the assigned implementation boundary,
  - a proof path under `output/payment-subscription-plan-proof/`.
- Failure rule: no PR-ready claim until the Cloudflare handoff, replay/idempotency, authZ/replay, and rollback/teardown proofs are present for the assigned slice.

## Overclaim boundary

This plan is architecture-route ready, not implementation complete. WP00 proves the upstream dependency boundary and exact blocker set only, while WP03 and WP04 now prove their Rust-owned lifecycle and entitlement-delivery boundaries on their own surfaces. Broader runtime correctness remains unproven until the selected workpack's code, tests, negative cases, proof bundle, rollback/teardown notes, and validation log exist under `output/payment-subscription-plan-proof/` and the relevant scoped validation actually passes or is carried as an exact blocker.
