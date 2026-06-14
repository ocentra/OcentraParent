# Cloudflare Billing Control Plane

Purpose: define the server boundary for monetization, not the provider-specific product logic.

## Named components

| Component | Responsibilities |
| --- | --- |
| `BillingWorker` | public/auth/admin/webhook API boundary |
| `BillingDurableObject` | per account/household idempotency and state coordination |
| `BillingD1` | queryable ledgers for dashboard, invoices, support/admin, referrals, entitlements |
| `BillingQueue` | webhook processing, retry, reconciliation, dead-letter, provider polling |
| `BillingSigner` | signs `EntitlementSnapshot` |
| `BillingAdmin` | support/admin billing dashboard API |

## Boundary rules

- Worker routes handle checkout creation, portal sessions, billing status, webhook ingress, and admin actions.
- Durable Objects serialize per-household or per-account billing writes and idempotency markers.
- D1 stores queryable ledgers and dashboard views.
- Queues handle retries, dead-letter processing, reconciliation jobs, and provider polling.
- The signer owns snapshot signing only; it does not decide entitlement.
- The admin API is redacted, audited, and support-role minimized.
- Desktop/mobile/browser clients never own provider secrets or webhook truth.

## Why this shape

- Billing traffic is low-volume and coordination-heavy.
- Per-object serialization is a better fit than broad shared state.
- Hosted provider flows reduce payment surface area in the browser.
- Queue-backed retries and reconciliation are a clean fit for delayed provider events.

## Failure conditions

- No client-side provider secrets.
- No provider event accepted without signature verification.
- No dashboard write bypasses the Worker/DO boundary.
- No reconciliation job mutates state without a ledger entry.
