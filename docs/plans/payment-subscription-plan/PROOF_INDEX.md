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

## Structured proof metadata

For new proof artifacts and command-log entries, include structured metadata when available:

```text
plan: payment-subscription-plan
workpack: <workpack id and name>
owner: billing-domain | billing-core | cloudflare-handoff | schema-domain | account-handoff | device-trust-handoff | data-custody-handoff | portal-parent-surface | policy-handoff | support-admin | docs-only
provider: stripe | razorpay | paypal | apple | google | manual | multi-provider | n/a
mode: test | live | mixed-blocked | n/a
region: <region/country/currency or n/a>
account_ref: <account/household ref or n/a>
provider_event_state: not-tested | verified | missing-signature | invalid-signature | duplicate | replayed | out-of-order | unknown | n/a
idempotency_state: not-tested | fresh | duplicate-rejected | replay-rejected | conflict | n/a
ledger_state: not-tested | app-owned-entry-written | provider-only-blocked | reconciled | drift-detected | n/a
entitlement_state: not-tested | granted | grace | limited | revoked | held-for-review | no-write | n/a
snapshot_state: not-tested | signed | invalid-signature-rejected | stale-rejected | wrong-household-rejected | wrong-device-rejected | n/a
device_trust_state: not-tested | required | accepted | missing | rejected | handoff-required | n/a
invoice_refund_dispute_state: not-tested | invoiced | refunded | disputed | cancelled | grace | tax-required | legal-required | n/a
referral_state: not-tested | qualified | rejected | review-required | credit-issued | credit-revoked | n/a
provider_availability_state: available | unavailable | manual-required | blocked | not-tested | n/a
dashboard_visibility_state: not-tested | billing-only | wrong-household-denied | child-data-blocked | parent-proof-blocked | n/a
support_admin_state: not-tested | authorized | denied | audited | redacted | blocked | n/a
rollback_teardown_state: not-tested | proved | blocked | manual-required | n/a
run_id: <wrapper run id or n/a>
command_id: <wrapper command id or n/a>
command: <exact command>
exit: <code>
result: pass | fail | blocked
artifact: <stdout/stderr artifact pointer, proof file, test result path, or n/a>
diagnostics_summary: <short unique failure or proof summary>
manual_required_note: <manual-required gap or n/a>
no_claim: <what this result does not prove>
```

The command log is a compact index, not a raw terminal transcript. Store long command output, provider fixtures, test reports, dashboard screenshots, or failure dumps under artifact paths and reference them by pointer. If no wrapper exists, write `run_id: n/a` and `command_id: n/a`; do not omit the proof row.

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
