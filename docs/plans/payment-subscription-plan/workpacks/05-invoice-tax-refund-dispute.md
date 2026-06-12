# Workpack 05: Invoice Tax Refund Dispute

Goal: define billing back-office states before launch.

Expected shape:

- Invoices and receipts are visible through Stripe-hosted surfaces or app-supported links.
- Tax handling is explicit and region-sensitive.
- Refunds, disputes, chargebacks, cancellation, and failed payment recovery have state transitions.
- Support/admin actions are authorized and audited.

Expected proof:

- Invoice/customer portal proof.
- Refund/dispute state proof.
- Admin role proof.
- Entitlement revoke/restore proof.

Failure: payment support docs that cannot explain what happens after cancellation, failed renewal, refund, or dispute.

## Execution Detail

Minimum context:

- Official Stripe Billing, Customer Portal, invoice, refund, and dispute docs.
- `E:\ocentra-games\infra\cloudflare\src\handlers\payments.ts`
- `E:\ocentra-games\infra\cloudflare\src\logic\payment-state-machine.ts`

Research required:

- Decide with Sujan which countries/currencies/taxes are supported at launch.
- Decide whether refunds are self-service, support-mediated, or both.
- Decide grace period and safety behavior after failed renewal or chargeback.

Required states:

- active.
- trialing.
- past_due.
- canceled.
- unpaid.
- refunded.
- disputed.
- dispute_won.
- dispute_lost.
- grace.
- support_required.

Expected tests/proof names:

- `billing.invoice-visible`
- `billing.tax-mode-decision`
- `billing.refund-state`
- `billing.dispute-state`
- `billing.failed-renewal-grace`
- `billing.support-admin-audited`

Proof artifact expectations:

- Billing lifecycle matrix.
- Support/admin action matrix.
- Entitlement side-effect proof.
- Customer-visible copy expectations.
