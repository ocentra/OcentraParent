# Storage Binding Model

Purpose: define the Parent Cloudflare bindings stripped down from the games module.

## Required bindings

| Binding | Type | Ownership |
| --- | --- | --- |
| `BILLING_D1` | D1 | Queryable ledgers, invoice state, support/admin views, reconciliation state |
| `BILLING_DO` | Durable Object | Serialized billing-state and idempotency coordination |
| `REFERRAL_DO` | Durable Object | Referral qualification, abuse-review coordination, credit lifecycle |
| `ENTITLEMENT_SNAPSHOT_DO` | Durable Object | Snapshot issuance coordination and replay-safe signing workflow |
| `BILLING_RECONCILIATION_QUEUE` | Queue | Retry, provider polling, reconciliation jobs |
| `BILLING_DEAD_LETTER_QUEUE` | Queue | Dead-letter capture and operator replay workflow |
| `BILLING_RATE_LIMIT_KV` | KV | Rate limits and lightweight abuse counters |
| `BILLING_CONFIG_KV` | KV | Low-risk runtime flags and rollout config |
| `BILLING_AUDIT_R2` | Optional R2 | Support-safe audit/export bundles only |
| `ANALYTICS` | Optional analytics dataset | Redacted operational metrics and audit events |

## Secret names

- `STRIPE_SECRET_KEY`
- `STRIPE_WEBHOOK_SECRET`
- `RAZORPAY_KEY_ID`
- `RAZORPAY_KEY_SECRET`
- `PAYPAL_CLIENT_ID`
- `PAYPAL_CLIENT_SECRET`
- `APPLE_STORE_KEY_REF`
- `GOOGLE_PLAY_SERVICE_ACCOUNT_REF`
- `ENTITLEMENT_SIGNING_KEY_REF`

Every secret is server-only; never browser, portal bundle, desktop, or mobile.

## Hard boundaries

- Do not store child telemetry or raw child data in D1, KV, or R2.
- Do not let optional R2 become a general-purpose storage escape hatch.
- Do not let provider payloads become product authority without ledger or DO mediation.
