<!-- agent-capsule -->

> Agent Capsule
> Plan: `payment-subscription-plan`
> Doc: `Payment Subscription Workpack Families`
> Kind: owner-path classifier for selected workpacks.
> Read when: only after `WORKPACK_INDEX.md` selects or names a workpack and owner/proof family is unclear.
> Stop rule: classify the selected workpack only; do not use this file as permission to scan every workpack.
> Proves: routing and owner-path classification only.
> Does not prove: payment runtime readiness, entitlement readiness, regional readiness, provider readiness, dashboard readiness, support/admin readiness, or PR readiness.
> Proof rule: if this file changes route/status claims, update `AGENTS.md`, `PLAN_STATE.md`, and any affected selected workpack route.

<!-- /agent-capsule -->

# Payment Subscription Workpack Families

Use this file to classify a selected workpack before opening source. This plan owns payment/subscription semantics and proof boundaries. It consumes Cloudflare, account, device-trust, data-custody, setup, portal, and policy handoffs; it does not own those sibling surfaces.

## Cloudflare handoff family

```text
Workpacks:
WP00 Cloudflare Control Plane Handoff

Owners:
cloudflare-control-plane-plan for shared Worker/API module, auth, bindings, route manifest, local dev/test shape, deployment promotion, and handoff proof
payment-subscription-plan for billing runtime remaining blocked until handoff proof is explicit

Rule:
Cloudflare route presence is not payment runtime readiness. Payment runtime work remains blocked until WP00 accepts or records the exact blocker.
```

## Pricing, seats, referral math, and product model family

```text
Workpacks:
WP01 Product Pricing Entitlement
WP10 Referral Growth Entitlement

Owners:
payment-subscription-plan for pricing, starter bundle, seats, referral qualification, referral credit lifecycle, and billing-to-entitlement math
account-identity-family-plan for household/account authority

Rule:
Pricing/referral proof must separate billing math from provider transport and account identity. Referral invite or household membership is not referral credit without qualification and anti-abuse proof.
```

## Checkout, billing portal, and provider session family

```text
Workpacks:
WP02 Checkout Billing Portal

Owners:
payment-subscription-plan for checkout/portal session meaning and provider-facing billing boundary
cloudflare-control-plane-plan for shared route/auth/runtime module
provider adapters only through server-side provider boundaries

Rule:
Hosted checkout or portal redirect proves session behavior only. It is not paid access and not entitlement proof.
```

## Webhook and lifecycle family

```text
Workpacks:
WP03 Subscription Webhook Lifecycle

Owners:
billing-core for Rust provider lifecycle classification and event/idempotency helpers when selected
payment-subscription-plan for app-owned webhook-to-ledger semantics
provider adapters for verified provider event formats only

Rule:
Provider events must be server-verified, idempotent, replay-safe, out-of-order safe, and ledger-backed before they can affect entitlement.
```

## App-owned ledgers and entitlement snapshot family

```text
Workpacks:
WP04 Entitlement Delivery Gates

Owners:
payment-subscription-plan for billing/referral/entitlement ledger semantics and signed entitlement snapshot model
device-trust-bootstrap-plan for trusted-device binding and local sealed trust
account-identity-family-plan for household/account authority
policy-control-plane-plan for consuming entitlement state after authority proof

Rule:
Provider state and signed snapshots are not the root of trust. Effective entitlement is derived from app-owned ledgers and consumed only after account and device-trust handoffs.
```

## Invoice, tax, refund, dispute, cancellation, and grace family

```text
Workpacks:
WP05 Invoice Tax Refund Dispute

Owners:
payment-subscription-plan for invoice/refund/dispute/cancellation/grace semantics and audit states
legal/tax/business decision records when policy is not settled
provider adapters only through normalized server-side events

Rule:
Refund, dispute, cancellation, tax, and grace behavior must remain replay-safe and audit-backed. Legal/tax-required states stay manual-required until approved.
```

## Security, privacy, observability, and test/live split family

```text
Workpacks:
WP06 Security Privacy Observability

Owners:
payment-subscription-plan for provider metadata minimization, payment logging boundaries, PCI boundary, test/live split, and observability proof
data-custody-storage-plan for billing record retention/export/delete/privacy constraints

Rule:
Provider secrets and webhook secrets stay server-side. Provider metadata must not include private child activity or child telemetry. Test and live modes must not mix.
```

## Provider portability and regional rollout family

```text
Workpacks:
WP08 Provider Adapter Portability
WP09 Regional Payment Rollout

Owners:
payment-subscription-plan for normalized provider states, provider ordering, regional rollout matrix, currency/tax availability, and manual-required blockers
provider setup owners only through server-side adapter proof

Rule:
Provider docs or regional availability rows are not launch readiness. Provider and region claims require configured adapter proof or explicit manual-required blockers.
```

## Parent dashboard and support/admin family

```text
Workpacks:
WP11 Parent Website Billing Dashboard
WP12 Support Admin Billing Ops

Owners:
payment-subscription-plan for billing states, dashboard fields, support/admin billing actions, redaction, and audit semantics
parent-domain/portal surfaces only when selected for projection proof
account-identity-family-plan for parent/session/role authority

Rule:
Billing-domain tests do not substitute for targeted parent/dashboard proof. Support/admin proof must include authorization, minimized fields, redaction, and audit trail.
```

## Rollout proof and route gate family

```text
Workpacks:
WP07 Rollout Proof And Route Gate

Owners:
selected proof roots under `output/payment-subscription-plan-proof/<workpack>/`
PLAN_STATE, WORKPACK_INDEX, NEXT_ACTIONS, PROOF_INDEX, TEST_PROOF_EXPECTATIONS, SOURCE_SURFACE_STATUS_MATRIX, and assertion matrix docs when state changes

Rule:
Rollout proof may aggregate only accepted proof roots or exact carried blockers. Assertion-matrix completeness, scaffold docs, or Cloudflare route presence cannot become payment runtime readiness.
```
