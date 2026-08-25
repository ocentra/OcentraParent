# Route Manifest Model

Purpose: define the shared Cloudflare route groups that consumer plans may rely on.

## Required route groups

```text
/health
/public/pricing
/auth/session/login
/auth/session/refresh
/auth/session/logout
/auth/session/revoke
/auth/billing/status
/auth/billing/checkout
/auth/billing/portal
/auth/billing/invoices
/auth/billing/change-plan
/auth/billing/cancel
/auth/billing/referrals
/auth/billing/referral-invite
/auth/billing/entitlement-snapshot
/auth/billing/license-check
/auth/billing/manual-invoice
/webhooks/stripe
/webhooks/razorpay
/webhooks/paypal
/webhooks/apple
/webhooks/google
/admin/billing/accounts
/admin/billing/invoices
/admin/billing/refunds
/admin/billing/disputes
/admin/billing/referrals
/admin/billing/reconciliation
/admin/billing/audit
```

## Ownership rules

- Route strings belong in the route manifest, not in handler-local literals.
- Request/response contracts must be owned by domain packages or explicit route docs before handler work starts.
- Payment may define billing semantics, but it must consume this route group shape rather than re-creating a second Cloudflare API surface.
- `/auth/billing/manual-invoice` remains in the shared manifest as a support-owned exception; do not split it into a second support-only route list just because its auth state is elevated.

## Manifest expectations

Each route entry must eventually own:

- path
- method
- auth state
- audit rule
- handler key
- request model
- response model
- audit event
- proof ID family

Admin and support routes must use audit rules that match their elevated auth state. A non-empty `auditEvent` string is not enough by itself.
`/auth/billing/manual-invoice` is the only `/auth/billing/*` route that may attach `support-required` plus `support-write`; all other elevated `/auth/billing/*` entries must be the trusted-device routes listed above.
