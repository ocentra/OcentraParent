# Payment Subscription Plan State

## Reviewed WP02 production truth checkpoint - 2026-08-25

This is a source-and-routing truth update from canonical `baa37f98e`. It does
not add completion evidence, tests, proof, CI, PR, READY, or DONE state.

The Cloudflare route manifest exposes `POST /auth/billing/checkout` and
`POST /auth/billing/portal` as `parent-session-required` billing writes. Their
request and response codecs are bound to the generated Worker billing-contract
registry, but both execution bindings remain `manual-required` with blocker
`payment-provider-execution-owner-missing`. The route handlers return the
shared manual-required response, and no checkout or portal provider API/session
caller exists. Provider key names in `infra/cloudflare/src/env.ts`, webhook
signature verification, and local fixture hosted URLs are not provider-session
execution or production caller evidence.

Account authority composition is real at the existing status/webhook seams:
browser sessions re-read current Account authority, and provider billing
mapping resolution joins the canonical provider mapping to
`ocentra_account_identity_current_authority` while rejecting missing, stale,
revoked, mismatched, or inactive ownership. The `ACCOUNT_IDENTITY_D1` binding
is still `manual-required`, migration `0004` requires the Account current-
authority schema, and no checkout or portal path composes this authority into a
provider session request.

Rust-owned canonical checkout/portal templates and generated schema-domain
files are present, and the Worker has a separate generated route codec surface.
The planned `packages/schema-domain/src/billing-checkout-portal-boundary.ts`
adapter is absent. The `packages/billing-domain` package is absent, so its
planned WP02 unit tests and the two named proof scripts are absent. Existing
Cloudflare hosted-session tests use `createTestHarness` with
`AUTH_ADAPTER_MODE: local-safe-fixture` and assert accepted hosted sessions;
they are mapped local-fixture assertions, not evidence of a live provider
caller, and remain stale against the current manual-required route source.

WP02 therefore remains `blocked`: route contracts and Account lookup seams are
present, while the provider execution owner, Account current-authority/D1
readiness, schema-domain adapter, focused expected tests, proof, deployment,
CI, PR, and completion remain open.

## Reviewed WP03 production truth checkpoint - 2026-08-25

This is a source-and-routing truth update from the WP02-integrated canonical
base `d337a7e5d`. It does not add completion evidence, tests, proof, CI, PR,
READY, or DONE state.

The Rust lifecycle surface is real: `crates/billing-core` defines provider,
test/live, signature, payload-parse, idempotency, replay/order, retry,
dead-letter, reconciliation, and entitlement-transition classifications. Its
public classifier and projector accept those states from the caller, and no
non-test caller of that API was found. The mapped Rust unit tests construct
those trust/state enums directly; they do not establish a production ingress
or provider owner.

The reachable Cloudflare webhook ingress is also real but incomplete. The
route manifest exposes Stripe, Razorpay, PayPal, Apple, and Google webhook
routes, while request/response provider-event contracts remain unbound and
manual-required. Only Stripe raw HMAC verification is implemented. Razorpay,
PayPal, Apple, and Google fail closed as unavailable/manual-required before
lifecycle processing.

Cloudflare has durable D1/Durable Object receipt, cursor, queue, lease,
dead-letter, reconciliation, and mutation-outbox custody. The receipt schema
does not persist normalized signature state, payload parse state, or provider
mode, and the reachable TypeScript queue/mutation path does not invoke the
Rust billing-core classifier/projector. Account/provider mapping is rechecked
at the Worker custody seam, but the Account binding remains manual-required.

The mapped Cloudflare integration/fuzz tests use the local-safe fixture
harness, and some non-Stripe acceptance expectations are stale against the
current manual-required source. No tests were run in this packet. The WP03
proof root is absent in this checkout. WP03 therefore remains
`blocked / source reviewed`: runtime provider composition, normalized receipt
truth, focused expected tests, proof, CI, PR, and completion remain open.

## Reviewed WP04 production truth checkpoint - 2026-08-25

This is a source-and-routing truth update from canonical `643c137dc`. It does
not add completion evidence, tests, proof, CI, PR, READY, or DONE state.

`crates/entitlement-core/src/entitlement_snapshot_derivation.rs` derives an
explicitly unsigned projection from billing, referral, entitlement, and
provider-input state. The provider boundary remains input-only. The issuer
module is crate-private, its trusted issuance projection has no public
constructor, and its manual-required signer returns `SigningUnavailable`.
The snapshot authority's `open` and
`verify_current_account_and_device` entry points are crate-private; its key,
installed-package, and currentness ports remain manual-required/fail-closed,
so no public owner-composed unlock path exists.

`crates/child-runtime/src/runtime_gate.rs` is a non-test consumer of the
entitlement decision function, but no non-test caller of
`evaluate_child_runtime_preflight` or its decision recorder was found. The
generic input does not compose the missing issuer/verifier/currentness and
Account/Device Trust owners, and deserialization forces unavailable snapshot
context. The mapped entitlement contract imports removed signed-derivation and
context APIs; schema/TypeScript roots remain proof-consumer topology. The WP04
expected assertion matrix, focused tests, proof root, and validation evidence
are open, so WP04 remains `blocked / source reviewed`.

## Accepted production-source checkpoint - 2026-08-17

The independently reviewed Payment source wave at `63305016f` is now on the
root integration line. This is a **source-only** checkpoint: expected tests,
focused test execution, proof, precommit, CI, and PR promotion remain open.

Live source now provides:

- Account-composite provider identity and exact provider-object binding rather
  than provider subject text as standalone product authority;
- durable Worker/Durable Object idempotency with versioned leases, bounded
  retry/backoff, collision-safe ownership, and terminal manual-required state;
- monotonic receipt/outbox/provider-cursor handling that rejects stale or
  conflicting replay instead of re-running an accepted mutation;
- a real forward D1 migration from legacy provider mappings that aborts on
  ambiguous account/household ownership or uniqueness collisions; and
- an unsigned entitlement projection that cannot be upgraded into access by
  caller-supplied signature/key/status text.

The source wave does **not** close the Payment plan. The Account WP02 current
authority migration must land before Payment migration `0003`; ambiguous
legacy rows require manual backfill; and no genuine provider-owned entitlement
issuer/verifier bridge exists. Paid access therefore remains manual-required,
and old text describing WP04 as a completed signed-snapshot runtime is
superseded by this fail-closed source truth. Existing tests that construct the
removed signed API are intentionally deferred to the expected-test wave.

## Production reachability audit - 2026-08-16

The payment workpacks were checked against non-test callers and owned runtime
boundaries. No payment-owned production slice is authorized from this pass.

- **WP00:** no implementation source; the Cloudflare handoff remains an
  upstream blocked dependency, not payment runtime.
- **WP01:** Rust schema and generated/edge TypeScript surfaces exist, but no
  billing production caller consumes the pricing/seat model. The package
  surfaces remain proof/contract boundaries, not shipped entitlement runtime.
- **WP02:** checkout and portal route contracts exist, while the actual Worker
  path is owned by `cloudflare-control-plane-plan`. The reachable POST routes
  bind generated request/response codecs but keep execution
  `manual-required` because the provider execution owner is missing; their
  handlers do not create provider sessions. Local fixture seeding is gated to
  local/test/development and production read-model access requires durable
  bindings, so fixture URLs are not provider execution. Account-composite
  provider mapping is consumed by status/webhook seams, not by a live
  checkout/portal caller. The missing provider owner, Account D1 readiness,
  schema-domain adapter, and focused expected tests remain blockers outside a
  legal Payment-only runtime edit.
- **WP03:** `crates/billing-core` contains the lifecycle classifier and
  idempotency/projector helpers, but no non-test caller outside the crate was
  found and its public API accepts caller-supplied trust/state enums. The
  Cloudflare webhook route is the reachable ingress: only Stripe raw HMAC is
  implemented, the other providers are unavailable/manual-required, and the
  route parses/queues generic payload data without invoking the Rust lifecycle
  owner. Durable receipt custody exists, but normalized signature, parse, and
  provider-mode fields are absent from the receipt schema.
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

Status: engineering-grade monetization spec is complete, and the recorded WP00-WP04 plan references preserve their historical narrow source or blocked-state results. WP01's pricing reference, WP03's Rust webhook-lifecycle source/tests, and WP04's Rust entitlement-delivery source/tests do not establish production-code or proof completion: this audit found no production caller for the WP01 model, no non-test consumer of the WP03 lifecycle owner, and no non-test downstream consumer of the WP04 entitlement owner. WP02 remains blocked with route-contract source present but provider execution, Account authority readiness, focused expected tests, and proof open; broader payment runtime execution remains blocked behind the exact upstream Cloudflare and Account dependency set.

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
WP02 has no current checked-in checkout or billing portal proof bundle; the
expected proof root remains open until a real provider caller, focused tests,
and the required validation evidence exist.
WP03 has no checked-in proof bundle at `output/payment-subscription-plan-proof/03-subscription-webhook-lifecycle/` in this checkout. Its mapped Rust and Cloudflare tests are not live-provider proof, and no test or proof command was run in the reviewed source packet. WP04's historical proof reference is superseded by the reviewed WP04 source truth checkpoint above.
WP04 has no checked-in proof bundle at `output/payment-subscription-plan-proof/04-entitlement-delivery-gates/` in this checkout. Its mapped signed-snapshot contract imports removed APIs, and its schema/TypeScript surfaces are proof-consumer topology rather than a production entitlement caller. No test or proof command was run in the reviewed WP04 source packet.
All runtime payment rows after WP00 remain blocked until selected code, tests, negative cases, rollback/teardown notes, validation logs, and proof bundles exist.
```

Current Parent direction:

- `cloudflare-control-plane-plan` owns `infra/cloudflare/`, Wrangler envs, auth states, route manifest, local dev loop, test runner, deployment promotion, and the shared payment handoff gate.
- WP00 is closed only as `blocked / proof-present`: payment consumed the Cloudflare module/auth/route/testing truth, but the upstream handoff still carries missing Cloudflare WP03/WP05/WP09/WP11 proof roots, missing billing-domain runtime boundary modules, unresolved account-auth/trusted-device authority, and blocked portal-smoke/deployment proof.
- WP01 retains its historical `done / proof-present` pricing result: the shared payment proof surface makes the one-parent-plus-one-child starter bundle explicit, keeps extra parent access separate from child-seat math, derives the effective child-device limit from base plus referral plus paid seats, carries visible over-limit grace, preserves safety-critical local behavior under degradation, and rejects game-economy pricing references. It is not production-code complete because no billing production caller consumes the model.
- WP01 does not restore TS ownership: `crates/schema` remains the Rust-first canonical contract target, while the `packages/schema-domain/src/*` edits in this packet are transitional thin edge validation and proof data only.
- WP02 remains `blocked`: the Rust/schema and Worker route codec surfaces
  describe hosted checkout/portal states and redirect/secret boundaries, but
  the reachable Worker handlers are manual-required and no provider session
  caller exists. The schema-domain adapter, Account current-authority
  readiness, focused expected tests, proof, and broader validation remain open;
  no hosted-session or payment-completion claim follows from local fixtures.
- WP03 current truth is `blocked / source reviewed`, not production-complete. `crates/billing-core` carries provider channel, payload-parse, idempotency, replay/order, retry, dead-letter, reconciliation, and test/live boundary classifications with mapped unit coverage under `crates/billing-core/tests/unit/**`, but its public entry points accept caller-supplied trust/state enums and have no non-test production consumer. Cloudflare owns the reachable ingress and durable receipt/queue custody, yet does not compose normalized signature/parse/mode receipt fields or invoke the Rust lifecycle owner. TypeScript therefore supplies the current runtime mutation path while the Rust contract remains unbound.
- WP04 current truth is `blocked / source reviewed`: `crates/entitlement-core/src/entitlement_snapshot_derivation.rs` owns an unsigned projection from billing/referral/entitlement/provider inputs, while the crate-private issuer and verifier/currentness ports remain manual-required and no public owner-composed unlock path exists. `crates/child-runtime/src/runtime_gate.rs` consumes the generic entitlement decision function, but no non-test caller of its preflight/decision path was found. The mapped signed-snapshot contract imports removed APIs, generated schema/TypeScript remains proof-consumer topology, and the focused expected tests and proof root remain open.
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

This plan is architecture-route ready, not implementation complete. WP00 proves the upstream dependency boundary and exact blocker set only; WP03 and WP04 have bounded Rust source/test contract surfaces, but neither establishes production runtime completion from those surfaces alone. Broader runtime correctness remains unproven until the selected workpack's code, tests, negative cases, proof bundle, rollback/teardown notes, and validation log exist under `output/payment-subscription-plan-proof/` and the relevant scoped validation actually passes or is carried as an exact blocker.
