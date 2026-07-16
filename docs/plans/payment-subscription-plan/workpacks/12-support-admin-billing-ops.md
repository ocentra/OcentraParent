# Workpack 12: Support/Admin Billing Ops

## Goal

Define the support and admin billing surface for refunds, disputes, adjustments, and reconciliation.

## First-touch surface

- `packages/parent-domain/src/billing-support-admin-boundary.ts`
- `packages/parent-domain/src/billing-support-admin-status-proof.ts`

## Read inputs

- [PLAN_STATE.md](../PLAN_STATE.md)
- [PLAN_EXECUTION_BLUEPRINT.md](../PLAN_EXECUTION_BLUEPRINT.md)
- [SUPPORT_ADMIN_BILLING_DASHBOARD.md](../SUPPORT_ADMIN_BILLING_DASHBOARD.md)
- [RESEARCH_AND_UI_GUIDANCE.md](../RESEARCH_AND_UI_GUIDANCE.md)
- [SECURITY_PRIVACY_OBSERVABILITY.md](../SECURITY_PRIVACY_OBSERVABILITY.md)
- [APP_OWNED_BILLING_LEDGER.md](../APP_OWNED_BILLING_LEDGER.md)

## Output files

- [SUPPORT_ADMIN_BILLING_DASHBOARD.md](../SUPPORT_ADMIN_BILLING_DASHBOARD.md)
- [RESEARCH_AND_UI_GUIDANCE.md](../RESEARCH_AND_UI_GUIDANCE.md)
- [SECURITY_PRIVACY_OBSERVABILITY.md](../SECURITY_PRIVACY_OBSERVABILITY.md)
- [APP_OWNED_BILLING_LEDGER.md](../APP_OWNED_BILLING_LEDGER.md)
- `output/payment-subscription-plan-proof/12-support-admin-billing-ops/`

## Acceptance

- Admin-authenticated users can inspect a redacted billing timeline.
- Refund, dispute, manual adjustment, and reconciliation actions are audited.
- Reconciliation and dead-letter handling are visible.
- Child data is not exposed to support.
- Support can search billing accounts without seeing private child data.

## Proof IDs

- `payment-admin.billing-account-search`
- `payment-admin.refund-action-audited`
- `payment-admin.dispute-state-visible`
- `payment-admin.referral-abuse-visible`
- `payment-admin.reconciliation-drift-visible`
- `payment-admin.no-child-private-data`

## Validation

- Docs validation: `npm run format:check`; `npm run lint:schema-boundaries`
- Required proof families: `payment-admin.billing-account-search`, `payment-admin.invoice-search`, `payment-admin.refund-action-audited`, `payment-admin.dispute-state-visible`, `payment-admin.manual-invoice-state`, `payment-admin.referral-abuse-visible`, `payment-admin.reconciliation-drift-visible`, `payment-admin.webhook-failure-visible`, `payment-admin.admin-role-required`, `payment-admin.support-role-limited`, `payment-admin.no-child-private-data`, `payment-admin.audit-event-required`
- Proof bundle: `output/payment-subscription-plan-proof/12-support-admin-billing-ops/12-support-admin-ops-proof.md`, `output/payment-subscription-plan-proof/12-support-admin-billing-ops/12-admin-role-negative-proof.md`, `output/payment-subscription-plan-proof/12-support-admin-billing-ops/12-support-data-minimization-proof.md`, `output/payment-subscription-plan-proof/12-support-admin-billing-ops/12-reconciliation-admin-proof.md`

## Negative cases

- Reject support mutations that are not audited.
- Reject support access to child telemetry.
- Reject provider history changes without a ledger entry.
- Reject support views that expose private child or custody data.

## Failure conditions

- Do not let support mutations bypass audit logging.
- Do not let support see child telemetry.
- Do not let provider history change without a ledger entry.
