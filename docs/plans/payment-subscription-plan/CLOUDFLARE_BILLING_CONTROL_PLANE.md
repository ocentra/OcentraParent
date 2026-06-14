# Cloudflare Billing Control Plane

Purpose: define the billing overlay that runs on top of the shared Parent Cloudflare control-plane module. The shared `infra/cloudflare/` worker shell, bindings, auth adapter, route manifest, local dev loop, and test runner are owned by `cloudflare-control-plane-plan`; this doc only defines the monetization role mapping on that surface.

## Named components

| Component | Responsibilities |
| --- | --- |
| `BillingWorker` | Billing route group running inside the shared `infra/cloudflare/` worker entrypoint. |
| `BillingDurableObject` | Per-account or per-household idempotency and state coordination for billing-owned writes. |
| `BillingD1` | Queryable ledgers for dashboard, invoices, support/admin, referrals, and entitlements. |
| `BillingQueue` | Webhook processing, retry, reconciliation, dead-letter, and provider polling. |
| `BillingSigner` | signs `EntitlementSnapshot` |
| `BillingAdmin` | support/admin billing dashboard API |

## Boundary rules

- Billing routes consume the shared route manifest owned by `cloudflare-control-plane-plan`; payment does not define a second worker surface.
- The shared worker shell owns env validation, CORS fail-fast, request-size limits, kill switch, safe errors, auth-state dispatch, and scheduled reconciliation hooks.
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
- No dashboard write bypasses the shared Worker/DO boundary.
- No reconciliation job mutates state without a ledger entry.
