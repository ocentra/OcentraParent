# Payment Subscription Plan - Full Proof, Test, And Ops Inventory

## Purpose

This document defines required proof and tests for the corrected payment, subscription, referral, billing dashboard, and support-ops plan.

Exact assertion scope lives in
[`REQUIRED_TEST_ASSERTION_MATRIX.md`](REQUIRED_TEST_ASSERTION_MATRIX.md). This
document owns harnesses, commands, artifact paths, and proof bundle structure;
the assertion matrix owns the exact meaning of each test or proof ID.

Use canonical proof root:

```text
output/payment-subscription-plan-proof/
```

Any payment proof reference outside `output/payment-subscription-plan-proof/` is legacy drift. The upstream Cloudflare handoff gate for WP00 is `output/cloudflare-control-plane-plan-proof/12-payment-plan-handoff-gate/payment-handoff-proof.md`, and payment runtime stays blocked until that artifact exists or records an exact blocker.

No payment, referral, entitlement, invoice, dashboard, admin, provider, or regional claim is DONE or PR_READY without positive proof, negative proof, command output, artifact paths, and no-claim boundaries.

## Harness map

| Workpack | First-touch harness / test files | Notes |
| --- | --- | --- |
| WP00 | `docs/plans/cloudflare-control-plane-plan/PLAN_STATE.md`, `docs/plans/cloudflare-control-plane-plan/ROUTE_MANIFEST_MODEL.md`, `docs/plans/cloudflare-control-plane-plan/TESTING_STRATEGY.md`, `docs/plans/cloudflare-control-plane-plan/REQUIRED_TEST_ASSERTION_MATRIX.md` | Cloudflare handoff is a proof-gate workpack; payment runtime remains blocked until this bundle exists or records an exact blocker. |
| WP01 | `packages/billing-domain/tests/unit/billing-pricing-matrix.test.ts`, `packages/billing-domain/tests/unit/billing-entitlement.test.ts` | Covers starter bundle math, seat expansion, and over-limit behavior. |
| WP02 | `packages/billing-domain/tests/unit/billing-checkout-portal-boundary.test.ts`, `packages/billing-domain/tests/unit/billing-account-runtime-boundary.test.ts`, `scripts/test/billing-account-endpoint-contract-proof.mjs` | Covers hosted checkout, portal handoff, and boundary negatives. |
| WP03 | `crates/billing-core/tests/unit/provider_webhook.rs`, `crates/billing-core/tests/unit/subscription_lifecycle.rs` | Covers webhook signatures, replay rejection, and lifecycle convergence. |
| WP04 | `packages/billing-domain/tests/unit/billing-entitlement-runtime-proof.test.ts`, `crates/entitlement-core/tests/unit/capability_gate.rs`, `crates/entitlement-core/tests/unit/capability_access.rs` | Covers signed snapshots, device gating, and entitlement source-of-truth checks. |
| WP05 | `packages/billing-domain/tests/unit/billing-invoice-tax-refund-dispute.test.ts`, `crates/billing-core/tests/unit/subscription_lifecycle.rs` | Covers invoice, refund, dispute, and grace transitions. |
| WP06 | `packages/billing-domain/tests/unit/billing-security-privacy-observability.test.ts`, `scripts/test/support-bundle-redaction-proof.mjs`, `scripts/test/billing-support-admin-boundary-proof.mjs` | Covers secret handling, redaction, and support-view minimization. |
| WP07 | `scripts/test/real-evidence-proof-checkpoint.mjs`, `output/payment-subscription-plan-proof/07-rollout-proof-and-route-gate/07-validation-command-log.md`, `output/payment-subscription-plan-proof/07-rollout-proof-and-route-gate/` | Covers route closure, proof-path sync, and the close-gate log. Route documents now point at the canonical output root, but the proof bundles themselves are still absent on disk. |
| WP08 | `packages/billing-domain/tests/unit/billing-checkout-portal-boundary.test.ts`, `packages/billing-domain/tests/unit/billing-account-runtime-boundary.test.ts` | Covers provider portability entrypoints and normalization boundaries. |
| WP09 | `packages/billing-domain/tests/unit/billing-pricing-matrix.test.ts`, `packages/billing-domain/tests/unit/billing-checkout-portal-boundary.test.ts` | Covers region/provider choice and rollout-gating inputs. |
| WP10 | `packages/billing-domain/tests/unit/billing-entitlement.test.ts`, `packages/billing-domain/tests/unit/billing-entitlement-runtime-proof.test.ts` | Covers referral credits, entitlement impact, and grace behavior. |
| WP11 | `packages/parent-domain/src/billing-entitlement.ts`, `packages/parent-domain/src/billing-entitlement-proof.ts`, `packages/parent-domain/tests/unit/billing-entitlement-proof.test.ts`, `packages/billing-domain/tests/unit/billing-support-admin-status-proof.test.ts` | Covers the parent billing surface and its proof model. `packages/parent-domain/tests/unit/billing-entitlement-proof.test.ts` exists, but execution is currently blocked by parent-domain build/import failures before the targeted file can run. |
| WP12 | `scripts/test/billing-support-admin-status-proof.mjs`, `scripts/test/billing-support-admin-boundary-proof.mjs`, `packages/billing-domain/tests/unit/billing-support-admin-boundary.test.ts` | Covers support/admin search, redaction, and audit gating. |

## Required proof manifest fields

Every proof file must include:

```text
# Proof: <proof name>

## Scope
## Workpack
## Provider
## Region
## Billing state
## Referral state
## Entitlement state
## Household / device scope
## Dashboard/admin surface
## Commands run
## Positive cases
## Negative cases
## Artifacts
## Redacted logs
## No-claim boundaries
## Manual-required gaps
## Adjacent plan handoffs
## Remaining risks
```

## Global no-claim flags

Every proof must state whether it claims:

```text
Cloudflare Worker runtime
Durable Object runtime
D1 ledger runtime
Queue/retry runtime
Stripe runtime
Razorpay runtime
PayPal runtime
Apple/Google store runtime
manual invoice runtime
parent website dashboard
support/admin dashboard
signed EntitlementSnapshot runtime
device-bound license runtime
test-mode readiness
live-mode readiness
tax readiness
production support readiness
```

Default is false unless proof exists.

## Proof minimum contents

Every proof bundle for WP00 through WP12 must record:

- the exact workpack assertion IDs executed from
  `REQUIRED_TEST_ASSERTION_MATRIX.md`;
- the exact command used or the exact blocker when no command could run;
- at least one negative case;
- at least one rollback or teardown note;
- any manual-required provider, legal, or Cloudflare dependency that remains
  open;
- a no-claim boundary stating what runtime remains unproven.

## Spec-only completion rule

- This plan may be `engineering-spec complete` while runtime remains blocked.
- Runtime completion still requires code, tests, commands, and proof artifacts.
- Open provider setup, legal/tax closure, missing test files, or Cloudflare
  runtime proof are execution blockers, not reasons to under-spec the plan.

## Validation matrix

| Workpack | Exact validation commands | Run order | Proof locations |
| --- | --- | --- | --- |
| WP00 | `npm run format:check`; `npm run lint:schema-boundaries` | 0 | `output/payment-subscription-plan-proof/00-cloudflare-control-plane-handoff/cloudflare-handoff-proof.md` |
| WP01 | `npm run format:check`; `npm run lint:schema-boundaries` | 1 | `output/payment-subscription-plan-proof/01-product-pricing-entitlement/01-free-starter-bundle-proof.md`, `output/payment-subscription-plan-proof/01-product-pricing-entitlement/01-effective-child-device-limit-proof.md`, `output/payment-subscription-plan-proof/01-product-pricing-entitlement/01-safety-critical-grace-proof.md`, `output/payment-subscription-plan-proof/01-product-pricing-entitlement/01-rejected-game-economy-proof.md` |
| WP02 | `npm run format:check`; `npm run lint:schema-boundaries` | 2 | `output/payment-subscription-plan-proof/02-checkout-billing-portal/02-cloudflare-billing-api-boundary-proof.md`, `output/payment-subscription-plan-proof/02-checkout-billing-portal/02-hosted-checkout-proof.md`, `output/payment-subscription-plan-proof/02-checkout-billing-portal/02-billing-portal-proof.md`, `output/payment-subscription-plan-proof/02-checkout-billing-portal/02-no-client-secret-proof.md`, `output/payment-subscription-plan-proof/02-checkout-billing-portal/02-redirect-origin-negative-proof.md` |
| WP03 | `npm run format:check`; `npm run lint:schema-boundaries` | 3 | `output/payment-subscription-plan-proof/03-subscription-webhook-lifecycle/03-provider-webhook-proof.md`, `output/payment-subscription-plan-proof/03-subscription-webhook-lifecycle/03-idempotency-replay-proof.md`, `output/payment-subscription-plan-proof/03-subscription-webhook-lifecycle/03-dead-letter-proof.md`, `output/payment-subscription-plan-proof/03-subscription-webhook-lifecycle/03-reconciliation-proof.md`, `output/payment-subscription-plan-proof/03-subscription-webhook-lifecycle/03-test-live-boundary-proof.md` |
| WP04 | `npm run format:check`; `npm run lint:schema-boundaries` | 4 | `output/payment-subscription-plan-proof/04-entitlement-delivery-gates/04-entitlement-ledger-proof.md`, `output/payment-subscription-plan-proof/04-entitlement-delivery-gates/04-signed-snapshot-proof.md`, `output/payment-subscription-plan-proof/04-entitlement-delivery-gates/04-local-device-trust-required-proof.md`, `output/payment-subscription-plan-proof/04-entitlement-delivery-gates/04-referral-loss-recalculation-proof.md` |
| WP05 | `npm run format:check`; `npm run lint:schema-boundaries` | 5 | `output/payment-subscription-plan-proof/05-invoice-tax-refund-dispute/05-invoice-tax-refund-dispute-matrix.md`, `output/payment-subscription-plan-proof/05-invoice-tax-refund-dispute/05-invoice-dashboard-proof.md`, `output/payment-subscription-plan-proof/05-invoice-tax-refund-dispute/05-refund-dispute-entitlement-proof.md`, `output/payment-subscription-plan-proof/05-invoice-tax-refund-dispute/05-support-admin-audit-proof.md` |
| WP06 | `npm run format:check`; `npm run lint:schema-boundaries` | 6 | `output/payment-subscription-plan-proof/06-security-privacy-observability/06-metadata-privacy-proof.md`, `output/payment-subscription-plan-proof/06-security-privacy-observability/06-secret-scan-proof.md`, `output/payment-subscription-plan-proof/06-security-privacy-observability/06-referral-abuse-proof.md`, `output/payment-subscription-plan-proof/06-security-privacy-observability/06-support-view-minimized-proof.md`, `output/payment-subscription-plan-proof/06-security-privacy-observability/06-pci-hosted-boundary-proof.md` |
| WP07 | `npm run format:check`; `npm run lint:schema-boundaries` | 7 | `output/payment-subscription-plan-proof/07-rollout-proof-and-route-gate/07-route-sync-proof.md`, `output/payment-subscription-plan-proof/07-rollout-proof-and-route-gate/07-proof-path-proof.md`, `output/payment-subscription-plan-proof/07-rollout-proof-and-route-gate/07-validation-command-log.md` |
| WP08 | `npm run format:check`; `npm run lint:schema-boundaries` | 8 | `output/payment-subscription-plan-proof/08-provider-adapter-portability/08-provider-adapter-contract-proof.md`, `output/payment-subscription-plan-proof/08-provider-adapter-portability/08-normalized-event-proof.md`, `output/payment-subscription-plan-proof/08-provider-adapter-portability/08-provider-lock-escape-proof.md` |
| WP09 | `npm run format:check`; `npm run lint:schema-boundaries` | 9 | `output/payment-subscription-plan-proof/09-regional-payment-rollout/09-regional-payment-matrix.md`, `output/payment-subscription-plan-proof/09-regional-payment-rollout/09-india-razorpay-proof.md`, `output/payment-subscription-plan-proof/09-regional-payment-rollout/09-pakistan-manual-required-proof.md`, `output/payment-subscription-plan-proof/09-regional-payment-rollout/09-china-wallet-proof.md`, `output/payment-subscription-plan-proof/09-regional-payment-rollout/09-uae-provider-proof.md` |
| WP10 | `npm run format:check`; `npm run lint:schema-boundaries` | 10 | `output/payment-subscription-plan-proof/10-referral-growth-entitlement/10-referral-state-machine-proof.md`, `output/payment-subscription-plan-proof/10-referral-growth-entitlement/10-referral-qualification-proof.md`, `output/payment-subscription-plan-proof/10-referral-growth-entitlement/10-referral-abuse-negative-proof.md`, `output/payment-subscription-plan-proof/10-referral-growth-entitlement/10-referral-loss-entitlement-proof.md`, `output/payment-subscription-plan-proof/10-referral-growth-entitlement/10-over-limit-grace-proof.md` |
| WP11 | `npm run format:check`; `npm run lint:schema-boundaries` | 11 | `output/payment-subscription-plan-proof/11-parent-website-billing-dashboard/11-parent-website-dashboard-proof.md`, `output/payment-subscription-plan-proof/11-parent-website-billing-dashboard/11-dashboard-wrong-household-negative-proof.md`, `output/payment-subscription-plan-proof/11-parent-website-billing-dashboard/11-dashboard-no-child-private-data-proof.md` |
| WP12 | `npm run format:check`; `npm run lint:schema-boundaries` | 12 | `output/payment-subscription-plan-proof/12-support-admin-billing-ops/12-support-admin-ops-proof.md`, `output/payment-subscription-plan-proof/12-support-admin-billing-ops/12-admin-role-negative-proof.md`, `output/payment-subscription-plan-proof/12-support-admin-billing-ops/12-support-data-minimization-proof.md`, `output/payment-subscription-plan-proof/12-support-admin-billing-ops/12-reconciliation-admin-proof.md` |

## WP00 - Cloudflare Control Plane Handoff

Required checks:

```text
payment-route.cloudflare-plan-exists
payment-route.cloudflare-module-spec-exists
payment-route.cloudflare-auth-boundary-consumed
payment-route.cloudflare-route-manifest-consumed
payment-route.cloudflare-test-shape-consumed
payment-route.cloudflare-portal-smoke-blocker-visible
payment-route.payment-remains-blocked-without-handoff
```

Proof artifacts:

```text
output/payment-subscription-plan-proof/00-cloudflare-control-plane-handoff/cloudflare-handoff-proof.md
```

## WP01 - Product Pricing Entitlement

Required tests:

```text
payment-pricing.free-starter-bundle
payment-pricing.base-one-parent-one-child
payment-pricing.paid-extra-child-device
payment-pricing.extra-parent-slot
payment-pricing.effective-child-device-limit
payment-pricing.over-limit-grace
payment-pricing.safety-critical-grace
payment-pricing.rejected-game-economy-model
```

Proof artifacts:

```text
output/payment-subscription-plan-proof/01-product-pricing-entitlement/01-free-starter-bundle-proof.md
output/payment-subscription-plan-proof/01-product-pricing-entitlement/01-effective-child-device-limit-proof.md
output/payment-subscription-plan-proof/01-product-pricing-entitlement/01-safety-critical-grace-proof.md
output/payment-subscription-plan-proof/01-product-pricing-entitlement/01-rejected-game-economy-proof.md
```

## WP02 - Checkout Billing Portal

Required tests:

```text
payment-checkout.cloudflare-billing-api-required
payment-checkout.hosted-checkout-created
payment-checkout.billing-portal-created
payment-checkout.auth-required
payment-checkout.household-role-required
payment-checkout.invalid-product-rejected
payment-checkout.redirect-allowlist
payment-checkout.origin-csrf-negative
payment-checkout.bot-abuse-gate
payment-checkout.no-desktop-secrets
payment-checkout.no-client-secret-exposure
payment-checkout.return-success-not-entitlement
payment-checkout.cancel-state
```

Proof artifacts:

```text
output/payment-subscription-plan-proof/02-checkout-billing-portal/02-cloudflare-billing-api-boundary-proof.md
output/payment-subscription-plan-proof/02-checkout-billing-portal/02-hosted-checkout-proof.md
output/payment-subscription-plan-proof/02-checkout-billing-portal/02-billing-portal-proof.md
output/payment-subscription-plan-proof/02-checkout-billing-portal/02-no-client-secret-proof.md
output/payment-subscription-plan-proof/02-checkout-billing-portal/02-redirect-origin-negative-proof.md
```

## WP03 - Subscription Webhook Lifecycle

Required tests:

```text
payment-webhook.stripe-signature-valid
payment-webhook.stripe-signature-invalid
payment-webhook.razorpay-signature-valid
payment-webhook.paypal-webhook-verified
payment-webhook.duplicate-event-idempotent
payment-webhook.replayed-event-rejected
payment-webhook.out-of-order-event-safe
payment-webhook.unknown-event-safe
payment-webhook.retry-no-double-grant
payment-webhook.dead-letter-manual-required
payment-webhook.reconciliation-repairs-drift
payment-webhook.test-live-separated
```

Proof artifacts:

```text
output/payment-subscription-plan-proof/03-subscription-webhook-lifecycle/03-provider-webhook-proof.md
output/payment-subscription-plan-proof/03-subscription-webhook-lifecycle/03-idempotency-replay-proof.md
output/payment-subscription-plan-proof/03-subscription-webhook-lifecycle/03-dead-letter-proof.md
output/payment-subscription-plan-proof/03-subscription-webhook-lifecycle/03-reconciliation-proof.md
output/payment-subscription-plan-proof/03-subscription-webhook-lifecycle/03-test-live-boundary-proof.md
```

## WP04 - Entitlement Delivery Gates

Required tests:

```text
payment-entitlement.billing-ledger-source
payment-entitlement.referral-ledger-source
payment-entitlement.entitlement-ledger-source
payment-entitlement.signed-snapshot-issued
payment-entitlement.snapshot-signature-invalid-rejected
payment-entitlement.local-device-trust-required
payment-entitlement.wrong-household-rejected
payment-entitlement.wrong-device-rejected
payment-entitlement.offline-stale-degraded
payment-entitlement.grace-period
payment-entitlement.cancel-revokes-paid-feature
payment-entitlement.referral-loss-revokes-earned-feature
payment-entitlement.safety-feature-not-silently-disabled
```

Proof artifacts:

```text
output/payment-subscription-plan-proof/04-entitlement-delivery-gates/04-entitlement-ledger-proof.md
output/payment-subscription-plan-proof/04-entitlement-delivery-gates/04-signed-snapshot-proof.md
output/payment-subscription-plan-proof/04-entitlement-delivery-gates/04-local-device-trust-required-proof.md
output/payment-subscription-plan-proof/04-entitlement-delivery-gates/04-referral-loss-recalculation-proof.md
```

## WP05 - Invoice Tax Refund Dispute

Required tests:

```text
payment-lifecycle.invoice-visible
payment-lifecycle.receipt-visible
payment-lifecycle.tax-mode-decision
payment-lifecycle.refund-state
payment-lifecycle.partial-refund-state
payment-lifecycle.refund-failed-state
payment-lifecycle.dispute-opened
payment-lifecycle.dispute-won
payment-lifecycle.dispute-lost
payment-lifecycle.chargeback-state
payment-lifecycle.failed-renewal-grace
payment-lifecycle.cancel-immediate
payment-lifecycle.cancel-period-end
payment-lifecycle.resume-after-past-due
payment-lifecycle.support-admin-audited
payment-lifecycle.no-data-delete-on-refund
```

Proof artifacts:

```text
output/payment-subscription-plan-proof/05-invoice-tax-refund-dispute/05-invoice-tax-refund-dispute-matrix.md
output/payment-subscription-plan-proof/05-invoice-tax-refund-dispute/05-invoice-dashboard-proof.md
output/payment-subscription-plan-proof/05-invoice-tax-refund-dispute/05-refund-dispute-entitlement-proof.md
output/payment-subscription-plan-proof/05-invoice-tax-refund-dispute/05-support-admin-audit-proof.md
```

## WP06 - Security Privacy Observability

Required tests:

```text
payment-security.provider-metadata-allow-deny
payment-security.no-child-data-metadata
payment-security.secret-scan
payment-security.webhook-smuggling-negative
payment-security.webhook-replay
payment-security.rate-limit
payment-security.bot-abuse-gate
payment-security.open-redirect-negative
payment-security.redacted-logs
payment-security.support-view-minimized
payment-security.pci-hosted-checkout-boundary
payment-security.referral-abuse-signals
payment-security.admin-audit-required
```

Proof artifacts:

```text
output/payment-subscription-plan-proof/06-security-privacy-observability/06-metadata-privacy-proof.md
output/payment-subscription-plan-proof/06-security-privacy-observability/06-secret-scan-proof.md
output/payment-subscription-plan-proof/06-security-privacy-observability/06-referral-abuse-proof.md
output/payment-subscription-plan-proof/06-security-privacy-observability/06-support-view-minimized-proof.md
output/payment-subscription-plan-proof/06-security-privacy-observability/06-pci-hosted-boundary-proof.md
```

## WP07 - Rollout Proof and Route Gate

Required checks:

```text
payment-route.plan-sync
payment-route.workpack-proof-manifest
payment-route.validation-log
payment-route.negative-gate
payment-route.rollback-path
```

Proof artifacts:

```text
output/payment-subscription-plan-proof/07-rollout-proof-and-route-gate/07-route-sync-proof.md
output/payment-subscription-plan-proof/07-rollout-proof-and-route-gate/07-proof-path-proof.md
output/payment-subscription-plan-proof/07-rollout-proof-and-route-gate/07-validation-command-log.md
```

## WP08 - Provider Adapter Portability

Required tests:

```text
payment-provider.adapter-interface
payment-provider.stripe-adapter-contract
payment-provider.razorpay-adapter-contract
payment-provider.paypal-adapter-contract
payment-provider.apple-store-adapter-contract
payment-provider.google-play-adapter-contract
payment-provider.manual-invoice-adapter-contract
payment-provider.normalized-event-contract
payment-provider.provider-lock-escape
payment-provider.no-direct-product-provider-reads
```

Proof artifacts:

```text
output/payment-subscription-plan-proof/08-provider-adapter-portability/08-provider-adapter-contract-proof.md
output/payment-subscription-plan-proof/08-provider-adapter-portability/08-normalized-event-proof.md
output/payment-subscription-plan-proof/08-provider-adapter-portability/08-provider-lock-escape-proof.md
```

## WP09 - Regional Payment Rollout

Required tests:

```text
payment-region.canada-us
payment-region.india
payment-region.pakistan
payment-region.china
payment-region.uae-dubai
payment-region.eu-uk
payment-region.southeast-asia
payment-region.manual-enterprise
payment-region.local-methods
payment-region.subscription-support
payment-region.manual-required-gaps
```

Proof artifacts:

```text
output/payment-subscription-plan-proof/09-regional-payment-rollout/09-regional-payment-matrix.md
output/payment-subscription-plan-proof/09-regional-payment-rollout/09-india-razorpay-proof.md
output/payment-subscription-plan-proof/09-regional-payment-rollout/09-pakistan-manual-required-proof.md
output/payment-subscription-plan-proof/09-regional-payment-rollout/09-china-wallet-proof.md
output/payment-subscription-plan-proof/09-regional-payment-rollout/09-uae-provider-proof.md
```

## WP10 - Referral Growth Entitlement

Required tests:

```text
payment-referral.invite-created
payment-referral.invite-opened
payment-referral.signup-started
payment-referral.account-created
payment-referral.household-created
payment-referral.setup-activated
payment-referral.qualified-credit-granted
payment-referral.active-referred-parent-required
payment-referral.lost-referral-credit-removed
payment-referral.referral-grace
payment-referral.self-referral-rejected
payment-referral.same-household-rejected
payment-referral.same-device-farm-rejected
payment-referral.same-payment-method-manual-review
payment-referral.fraud-review
payment-referral.entitlement-recalculated
payment-referral.over-limit-grace-visible
payment-referral.no-data-delete-on-lost-referral
```

Proof artifacts:

```text
output/payment-subscription-plan-proof/10-referral-growth-entitlement/10-referral-state-machine-proof.md
output/payment-subscription-plan-proof/10-referral-growth-entitlement/10-referral-qualification-proof.md
output/payment-subscription-plan-proof/10-referral-growth-entitlement/10-referral-abuse-negative-proof.md
output/payment-subscription-plan-proof/10-referral-growth-entitlement/10-referral-loss-entitlement-proof.md
output/payment-subscription-plan-proof/10-referral-growth-entitlement/10-over-limit-grace-proof.md
```

## WP11 - Parent Website Billing Dashboard

Required tests:

```text
payment-dashboard.parent-account-visible
payment-dashboard.current-plan-visible
payment-dashboard.child-device-usage-visible
payment-dashboard.referral-credit-visible
payment-dashboard.paid-seat-visible
payment-dashboard.invoice-visible
payment-dashboard.change-plan-visible
payment-dashboard.cancel-visible
payment-dashboard.billing-portal-link
payment-dashboard.license-snapshot-visible
payment-dashboard.wrong-household-denied
payment-dashboard.no-child-private-data
payment-dashboard.targeted-parent-proof-file
```

Proof artifacts:

```text
output/payment-subscription-plan-proof/11-parent-website-billing-dashboard/11-parent-website-dashboard-proof.md
output/payment-subscription-plan-proof/11-parent-website-billing-dashboard/11-dashboard-wrong-household-negative-proof.md
output/payment-subscription-plan-proof/11-parent-website-billing-dashboard/11-dashboard-no-child-private-data-proof.md
```

## WP12 - Support Admin Billing Ops

Required tests:

```text
payment-admin.billing-account-search
payment-admin.invoice-search
payment-admin.refund-action-audited
payment-admin.dispute-state-visible
payment-admin.manual-invoice-state
payment-admin.referral-abuse-visible
payment-admin.reconciliation-drift-visible
payment-admin.webhook-failure-visible
payment-admin.admin-role-required
payment-admin.support-role-limited
payment-admin.no-child-private-data
payment-admin.audit-event-required
```

Proof artifacts:

```text
output/payment-subscription-plan-proof/12-support-admin-billing-ops/12-support-admin-ops-proof.md
output/payment-subscription-plan-proof/12-support-admin-billing-ops/12-admin-role-negative-proof.md
output/payment-subscription-plan-proof/12-support-admin-billing-ops/12-support-data-minimization-proof.md
output/payment-subscription-plan-proof/12-support-admin-billing-ops/12-reconciliation-admin-proof.md
```

## Suggested proof scripts

Add only when matching runtime or contracts exist:

```text
scripts/test/payment-cloudflare-boundary-proof.mjs
scripts/test/payment-pricing-referral-proof.mjs
scripts/test/payment-checkout-boundary-proof.mjs
scripts/test/payment-webhook-lifecycle-proof.mjs
scripts/test/payment-entitlement-snapshot-proof.mjs
scripts/test/payment-referral-entitlement-proof.mjs
scripts/test/payment-dashboard-proof.mjs
scripts/test/payment-admin-ops-proof.mjs
scripts/test/payment-security-privacy-proof.mjs
scripts/test/payment-regional-matrix-proof.mjs
scripts/test/payment-rollout-proof.mjs
```

## Hard blockers

Do not mark DONE or PR_READY if any are missing:

```text
Cloudflare/API boundary
website billing dashboard
support/admin billing ops
invoice/refund/dispute tracking
free starter model
referral entitlement model
paid add-on child-device model
effective child-device entitlement calculation
lost referral behavior
referral anti-abuse proof
provider adapter boundary
regional payment matrix
no-desktop-secret proof
webhook idempotency proof
signed EntitlementSnapshot proof
local device trust requirement
metadata no-child-data proof
support/admin data minimization proof
targeted parent billing proof file
test/live mode separation
route/index sync
commands run
proof artifact paths
```
