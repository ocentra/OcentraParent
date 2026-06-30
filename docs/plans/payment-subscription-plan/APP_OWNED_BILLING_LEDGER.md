# App-Owned Billing Ledger

Purpose: define the canonical billing history owned by the app, not by the provider.

Current Rust owner: billing lifecycle truth stays in Rust-owned billing and entitlement crates. TypeScript proof surfaces may consume this ledger shape, but they do not own entitlement derivation.

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
| `BillingEvent` | Provider, checkout, refund, or support event normalized into app truth. | `billingEventId`, `billingAccountId`, `eventType`, `provider`, `providerMode`, `providerEventId`, `signatureState`, `payloadParseState`, `idempotencyState`, `replayState`, `orderingState`, `retryState`, `deadLetterState`, `reconciliationState`, `receivedAt` |
| `BillingStateProjection` | Queryable billing state for dashboard and API consumers and the billing-side input to entitlement derivation. | `billingAccountId`, `planTier`, `status`, `graceState`, `baseChildDeviceLimit`, `paidExtraChildDeviceSeats`, `updatedAt` |
| `ManualInvoice` | App-owned manual invoice record. | `manualInvoiceId`, `billingAccountId`, `amountMinor`, `currency`, `status`, `createdAt` |
| `BillingAuditEvent` | Audited support/admin or operator action. | `auditEventId`, `billingAccountId`, `actorRef`, `action`, `createdAt` |

## Rules

- The ledger is append-only for history and queryable for dashboards.
- Durable Objects serialize live writes; D1 materializes query views.
- Provider event IDs, replay markers, and ordering markers are idempotency and repair inputs, not user-visible state.
- Retry, dead-letter, and reconciliation follow-up remain explicit app-owned fields rather than inferred Worker behavior.
- Billing records may reference household and parent identifiers, but never child telemetry or child content.
- Provider livemode and provider subscription echoes are inputs to downstream snapshot issuance, not entitlement authority by themselves.

## Failure conditions

- Do not mutate provider history in place.
- Do not let dashboard consumers infer entitlements without the entitlement ledger.
- Do not store provider secrets, webhook secrets, or child data in the ledger.
- Do not collapse mixed test/live traffic into one accepted ledger path.
