# App-Owned Billing Ledger

Purpose: define the canonical billing history owned by the app, not by the provider.

## Ledger entries

| Entry         | Meaning                                       | Required fields                                                                                       |
| ------------- | --------------------------------------------- | ----------------------------------------------------------------------------------------------------- |
| CheckoutInit  | A payment session was created.                | `billingAccountId`, `paymentId`, `provider`, `amountMinor`, `currency`, `productId`, `region`, `mode` |
| ProviderEvent | An external event was normalized.             | `providerEventId`, `providerObjectId`, `eventType`, `receivedAt`, `mode`                              |
| Invoice       | An invoice or renewal entry was materialized. | `invoiceId`, `periodStart`, `periodEnd`, `taxMinor`, `status`                                         |
| Refund        | A refund or partial reversal was recorded.    | `refundId`, `reason`, `amountMinor`, `providerRefs`, `actor`                                          |
| Dispute       | A dispute or chargeback was recorded.         | `disputeId`, `state`, `evidenceDeadline`, `providerRefs`                                              |
| Adjustment    | A support/admin change was applied.           | `adjustmentId`, `actor`, `reason`, `deltaSeats`, `deltaAmountMinor`                                   |

## Rules

- The ledger is append-only for history and queryable for dashboards.
- Durable Objects serialize live writes; D1 materializes query views.
- Provider event IDs are idempotency markers, not user-visible state.
- Billing records may reference household and parent identifiers, but never child telemetry or child content.

## Failure conditions

- Do not mutate provider history in place.
- Do not let dashboard consumers infer entitlements without the entitlement ledger.
- Do not store provider secrets, webhook secrets, or child data in the ledger.
