<!-- agent-capsule -->

> Agent Capsule
> Plan: `payment-subscription-plan`
> Doc: `Payment Subscription Proof Index`
> Kind: proof artifact router.
> Read when: selected workpack needs proof paths or PR_READY/DONE proof validation.
> Stop rule: use only the proof root for the selected workpack.
> Proves: proof location routing only.
> Does not prove: implementation completion by itself.
> Proof rule: proof artifacts are valid only after focused commands run or precise blockers are recorded.

<!-- /agent-capsule -->

# Payment Subscription Proof Index

## Proof roots

```text
output/payment-subscription-plan-proof/00-cloudflare-control-plane-handoff/
output/payment-subscription-plan-proof/01-product-pricing-entitlement/
output/payment-subscription-plan-proof/02-checkout-billing-portal/
output/payment-subscription-plan-proof/03-subscription-webhook-lifecycle/
output/payment-subscription-plan-proof/04-entitlement-delivery-gates/
output/payment-subscription-plan-proof/05-invoice-tax-refund-dispute/
output/payment-subscription-plan-proof/06-security-privacy-observability/
output/payment-subscription-plan-proof/08-provider-adapter-portability/
output/payment-subscription-plan-proof/09-regional-payment-rollout/
output/payment-subscription-plan-proof/10-referral-growth-entitlement/
output/payment-subscription-plan-proof/11-parent-website-billing-dashboard/
output/payment-subscription-plan-proof/12-support-admin-billing-ops/
output/payment-subscription-plan-proof/07-rollout-proof-and-route-gate/
```

## Required universal proof files

Every proof root needs:

```text
00-scope-summary.md
01-negative-case-proof.md
02-rollback-or-teardown-proof.md
16-validation-commands.log
```

Workpack-specific proof ids stay in the selected workpack and `REQUIRED_TEST_ASSERTION_MATRIX.md`.

## Command log format

```text
command: <exact command>
exit: <code>
result: pass | fail | blocked
artifact: <path or n/a>
notes: <short note>
```

If blocked:

```text
blocker:
required environment:
why this does not prove completion:
next command:
```

## No-claim language

Do not claim:

```text
payment runtime ready
checkout ready
webhook ready
entitlement ready
regional provider ready
referral credit ready
billing dashboard ready
support/admin billing ready
PR_READY
```

unless the selected proof root proves the claim and WP07 aggregates it when broad readiness is claimed.

Cloudflare prerequisite proof must exist before any runtime payment workpack claims readiness.
