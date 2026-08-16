# Storage Binding Model

Purpose: define the Parent Cloudflare bindings stripped down from the games module.

## Required bindings

| Binding | Type | Owner | Purpose | Privacy boundary | State |
| --- | --- | --- | --- | --- | --- |
| `BILLING_D1` | D1 | `billing-ledger-read-model` | Queryable billing ledgers, support/admin views, and reconciliation read models | Billing/support/reconciliation records only; no child telemetry or raw child data | required |
| `BILLING_DO` | Durable Object | `billing-control-do` | Serialized billing-state writes and idempotency coordination | Billing write coordination only; no child telemetry or raw child data | required |
| `REFERRAL_DO` | Durable Object | `referral-control-do` | Serialized referral qualification, abuse review, and credit lifecycle coordination | Referral coordination only; no child telemetry or raw child data | required |
| `ENTITLEMENT_SNAPSHOT_DO` | Durable Object | `entitlement-snapshot-do` | Snapshot issuance coordination and replay-safe signing workflow | Snapshot coordination only; no child telemetry or raw child data | required |
| `BILLING_RECONCILIATION_QUEUE` | Queue | `billing-reconciliation-jobs` | Retry, provider polling, and reconciliation jobs | Redacted reconciliation payloads only; no child telemetry or raw child data | required; dead-letters into `BILLING_DEAD_LETTER_QUEUE` |
| `BILLING_DEAD_LETTER_QUEUE` | Queue | `billing-dead-letter-ops` | Dead-letter capture and operator replay workflow for reconciliation failures | Redacted dead-letter payloads only; no child telemetry or raw child data | required; receives failures from `BILLING_RECONCILIATION_QUEUE` |
| `BILLING_RATE_LIMIT_KV` | KV | `billing-rate-limit-guard` | Rate limits and lightweight abuse counters | Rate-limit counters and low-risk abuse state only; no child telemetry or raw child data | required |
| `BILLING_CONFIG_KV` | KV | `billing-runtime-config` | Low-risk runtime flags and rollout config | Rollout flags and non-secret config only; no child telemetry, raw child data, or provider secrets | required |
| `BILLING_AUDIT_R2` | Optional R2 | `support-audit-export` | Support-safe audit and export bundles only | Support-safe audit/export artifacts only; no child telemetry or raw child data | manual-required; not runtime-ready by default |
| `ANALYTICS` | Optional analytics dataset | `redacted-ops-analytics` | Redacted operational metrics and audit event counters | Redacted operational metrics only; no child telemetry, raw child data, or provider secrets | optional |

## Account-identity binding prerequisite

WP06 now declares the narrow account-identity D1 binding in `src/env.ts` and
both Wrangler configs, with an isolated `migrations/account-identity` directory.
The bindings above remain billing-owned and cannot be repurposed for account
authority. Account DO and KV are intentionally not declared in this slice and
remain manual-required until the Account WP08 handoff defines a need and owner:

| Required binding role | Type | Owner | Purpose | Privacy boundary | Current state |
| --- | --- | --- | --- | --- | --- |
| Account-identity authority store | D1 | Account WP08 canonical Rust authority; Cloudflare WP06 migrated-schema consumer only | Persist the selected narrow provider-subject mapping and migration-compatible read/write state | Account identity mapping only; no child telemetry, raw child data, billing ledger, or provider secrets | declared; migration/proof deferred |
| Account-identity coordination | Durable Object | Account WP08 authority boundary; Cloudflare consumer only | Serialize selected account/session authority coordination if later required | Account/session coordination only; no child telemetry, raw child data, billing state, or provider secrets | absent; manual-required |
| Account-identity low-risk operational state | KV | Account WP08 authority boundary; Cloudflare consumer only | Store selected low-risk, non-authoritative rollout or freshness state if later required | Non-secret low-risk operational state only; no child telemetry, raw child data, authority ledger, or provider secrets | absent; manual-required |

### Account D1 migration isolation

`BILLING_D1` remains billing/support/reconciliation storage and its migrations
must never be applied to the account database. Before WP06 may run the account
migration command, the selected account D1 entry in `wrangler.toml` must name a
binding-specific `migrations_dir`, or an equivalent configuration-backed mapping
that proves the account migration directory is selected for that database only.
The retained proof must name the account database, selected binding, and
migration directory/mapping; a generic `wrangler d1 migrations apply` result or
a `BILLING_D1` result cannot satisfy this gate.

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
- Do not treat placeholder Wrangler IDs, binding names, or local example refs as runtime success.
- Queue and dead-letter ownership must stay paired and explicit; green enqueue paths must not hide dead-letter responsibility.
- Do not borrow `BILLING_D1`, billing Durable Objects, or billing KV for account authority, and do not apply account migrations through a billing migration directory.
