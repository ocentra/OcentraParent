# Storage Bindings

Bindings modeled in this scaffold:

- `BILLING_D1`
- `BILLING_DO`
- `REFERRAL_DO`
- `ENTITLEMENT_SNAPSHOT_DO`
- `BILLING_RECONCILIATION_QUEUE`
- `BILLING_DEAD_LETTER_QUEUE`
- `BILLING_RATE_LIMIT_KV`
- `BILLING_CONFIG_KV`
- `BILLING_AUDIT_R2`
- `ANALYTICS`

Hard boundary:

- do not store child telemetry or raw child data in this module
