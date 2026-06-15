# Workpack 12: Support/Admin Billing Ops

Purpose: define the support and admin billing surface for refunds, disputes, adjustments, and reconciliation.

## Owns

- `SUPPORT_ADMIN_BILLING_DASHBOARD.md`
- the support/admin parts of `RESEARCH_AND_UI_GUIDANCE.md`

## Must prove

- Admin-authenticated users can inspect a redacted billing timeline.
- Refund, dispute, and manual adjustment actions are audited.
- Reconciliation and dead-letter handling are visible.
- Child data is not exposed to support.

## Proof path

- Use `docs/proof/payment-subscription-plan/wp12/` or the owning crate's local proof directory.

## Failure conditions

- The workpack fails if support mutations are not audited.
- The workpack fails if support can see child telemetry.
- The workpack fails if provider history changes without a ledger entry.
