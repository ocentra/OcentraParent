# App-Owned Billing Ledger

Purpose: define the canonical billing history owned by the app, not by the provider.

## Ledger entries

| Entry | Meaning | Required fields |
| --- | --- | --- |
| `BillingAccount` | Canonical billing identity for a household or parent account. | `billingAccountId`, `householdRef`, `parentAccountRef`, `status`, `createdAt` |
| `BillingCustomer` | App-owned customer record for a payment provider. | `billingCustomerId`, `provider`, `providerCustomerRef`, `createdAt` |
| `ProviderCustomerRef` | External provider customer mapping. | `billingAccountId`, `provider`, `providerCustomerRef`, `createdAt` |
| `ProviderSubscriptionRef` | External provider subscription mapping. | `billingAccountId`, `provider`, `providerSubscriptionRef`, `status`, `createdAt` |
| `ProviderInvoiceRef` | External provider invoice mapping. | `billingAccountId`, `provider`, `providerInvoiceRef`, `status`, `createdAt` |
| `ProviderRefundRef` | External provider refund mapping. | `billingAccountId`, `provider`, `providerRefundRef`, `status`, `createdAt` |
| `ProviderDisputeRef` | External provider dispute mapping. | `billingAccountId`, `provider`, `providerDisputeRef`, `status`, `createdAt` |
| `BillingEvent` | Provider, checkout, refund, or support event normalized into app truth. | `billingEventId`, `billingAccountId`, `eventType`, `provider`, `providerEventId`, `receivedAt` |
| `BillingStateProjection` | Queryable billing state for dashboard and API consumers. | `billingAccountId`, `planTier`, `status`, `graceState`, `updatedAt` |
| `ManualInvoice` | App-owned manual invoice record. | `manualInvoiceId`, `billingAccountId`, `amountMinor`, `currency`, `status`, `createdAt` |
| `BillingAuditEvent` | Audited support/admin or operator action. | `auditEventId`, `billingAccountId`, `actorRef`, `action`, `createdAt` |

## Rules

- The ledger is append-only for history and queryable for dashboards.
- Durable Objects serialize live writes; D1 materializes query views.
- Provider event IDs are idempotency markers, not user-visible state.
- Billing records may reference household and parent identifiers, but never child telemetry or child content.

## Failure conditions

- Do not mutate provider history in place.
- Do not let dashboard consumers infer entitlements without the entitlement ledger.
- Do not store provider secrets, webhook secrets, or child data in the ledger.
