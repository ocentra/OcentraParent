# Workpack 05: Invoice, Tax, Refund, and Dispute

## Goal

Define invoice, tax, refund, dispute, cancellation, and grace behavior as one coordinated contract.

## First-touch surface

- `packages/billing-domain/src/billing-invoice-tax-refund-dispute.ts`
- `packages/billing-domain/tests/unit/billing-invoice-tax-refund-dispute.test.ts`

## Read inputs

- [PLAN_STATE.md](../PLAN_STATE.md)
- [PLAN_EXECUTION_BLUEPRINT.md](../PLAN_EXECUTION_BLUEPRINT.md)
- [INVOICE_TAX_REFUND_DISPUTE_MODEL.md](../INVOICE_TAX_REFUND_DISPUTE_MODEL.md)
- [APP_OWNED_BILLING_LEDGER.md](../APP_OWNED_BILLING_LEDGER.md)
- [BILLING_API_BOUNDARY.md](../BILLING_API_BOUNDARY.md)
- [PAYMENT_PROVIDER_STRATEGY.md](../PAYMENT_PROVIDER_STRATEGY.md)

## Output files

- [INVOICE_TAX_REFUND_DISPUTE_MODEL.md](../INVOICE_TAX_REFUND_DISPUTE_MODEL.md)
- [APP_OWNED_BILLING_LEDGER.md](../APP_OWNED_BILLING_LEDGER.md)
- `output/payment-subscription-plan-proof/05-invoice-tax-refund-dispute/`

## Acceptance

- Invoice lifecycle records plan, period, currency, tax, provider reference, and invoice number.
- Refunds preserve history and record the entitlement impact.
- Disputes freeze or limit entitlement per policy and retain evidence pointers.
- Cancellation and grace are visible in the entitlement ledger and snapshot model.
- Manual invoice state remains auditable and support-owned.

## Proof IDs

- `payment-lifecycle.invoice-visible`
- `payment-lifecycle.refund-state`
- `payment-lifecycle.dispute-opened`
- `payment-lifecycle.failed-renewal-grace`

## Validation

- Docs validation: `npm run format:check`; `npm run lint:schema-boundaries`
- Required proof families: `payment-lifecycle.invoice-visible`, `payment-lifecycle.receipt-visible`, `payment-lifecycle.tax-mode-decision`, `payment-lifecycle.refund-state`, `payment-lifecycle.partial-refund-state`, `payment-lifecycle.refund-failed-state`, `payment-lifecycle.dispute-opened`, `payment-lifecycle.dispute-won`, `payment-lifecycle.dispute-lost`, `payment-lifecycle.chargeback-state`, `payment-lifecycle.failed-renewal-grace`, `payment-lifecycle.cancel-immediate`, `payment-lifecycle.cancel-period-end`, `payment-lifecycle.resume-after-past-due`, `payment-lifecycle.support-admin-audited`, `payment-lifecycle.no-data-delete-on-refund`
- Proof bundle: `output/payment-subscription-plan-proof/05-invoice-tax-refund-dispute/05-invoice-tax-refund-dispute-matrix.md`, `output/payment-subscription-plan-proof/05-invoice-tax-refund-dispute/05-invoice-dashboard-proof.md`, `output/payment-subscription-plan-proof/05-invoice-tax-refund-dispute/05-refund-dispute-entitlement-proof.md`, `output/payment-subscription-plan-proof/05-invoice-tax-refund-dispute/05-support-admin-audit-proof.md`

## Negative cases

- Reject any refund or dispute that changes access without a ledger entry.
- Reject any invoice or tax record that contains child telemetry.
- Reject provider invoice events as final until the app ledger records them.
- Reject silent cancellation or grace changes that are not visible in the ledger.

## Failure conditions

- Do not allow a refund or dispute to silently change access without a ledger entry.
- Do not let tax or invoice data contain child telemetry.
- Do not treat a provider invoice event as final until the app ledger records it.
