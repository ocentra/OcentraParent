# Workpack 05: Invoice, Tax, Refund, and Dispute

Purpose: define invoice, tax, refund, dispute, cancellation, and grace behavior as one coordinated contract.

## Owns

- `INVOICE_TAX_REFUND_DISPUTE_MODEL.md`
- PSP-005 and the invoice/grace parts of the billing policy

## Must prove

- Invoice creation and finalization are recorded.
- Tax is materialized in the app ledger.
- Full and partial refunds are distinguishable.
- Disputes freeze or limit entitlement per policy.
- Cancellation and grace transitions are explicit.

## Proof path

- Use `docs/proof/payment-subscription-plan/wp05/` or the owning crate's local proof directory.

## Failure conditions

- The workpack fails if refunds or disputes silently change access.
- The workpack fails if invoice or tax data includes child telemetry.
- The workpack fails if cancellation does not record its entitlement effect.
