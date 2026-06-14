# Billing API Boundary

Purpose: define which callers can touch billing state and what they are allowed to ask for.

## Public API groups

- `/public/pricing`

## Authenticated billing API groups

- `/auth/billing/status`
- `/auth/billing/checkout`
- `/auth/billing/portal`
- `/auth/billing/invoices`
- `/auth/billing/change-plan`
- `/auth/billing/cancel`
- `/auth/billing/referrals`
- `/auth/billing/referral-invite`
- `/auth/billing/entitlement-snapshot`
- `/auth/billing/license-check`
- `/auth/billing/manual-invoice`

## Webhook API groups

- `/webhooks/stripe`
- `/webhooks/razorpay`
- `/webhooks/paypal`
- `/webhooks/apple`
- `/webhooks/google`

## Admin API groups

- `/admin/billing/accounts`
- `/admin/billing/invoices`
- `/admin/billing/refunds`
- `/admin/billing/disputes`
- `/admin/billing/referrals`
- `/admin/billing/reconciliation`
- `/admin/billing/audit`

## Caller rules

| Caller | Allowed operations | Authentication |
| --- | --- | --- |
| Parent web or mobile surface | Read pricing, start checkout, open portal, read billing status, preview entitlement state | Parent auth, plus device trust for sensitive actions |
| Support/admin surface | Search accounts, issue refunds, apply manual credits, inspect timelines | Admin auth and audit logging |
| Provider webhook | Deliver payment, subscription, refund, or dispute events | Provider signature validation |
| Queue / cron / reconciliation | Retry, reconcile, and finalize ledger state | Internal worker boundary |
| Other product surfaces | Read signed entitlement snapshot only | Snapshot signature and device binding |

## Boundary rules

- Browser callers must never receive provider secrets or webhook secrets.
- Checkout success URLs are not proof of payment.
- Provider event payloads are inputs, not truth.
- Billing writes must pass through the Worker and the per-account or per-household serialization boundary.
- Sensitive actions may require device-trust proof, but the device-trust flow itself belongs to the device-trust plan.
- Dashboard actions must return redacted, audit-friendly payloads.

## Failure conditions

- Do not expose raw provider event objects to UI consumers.
- Do not let support actions bypass audit logging.
- Do not let a provider event change entitlement without a ledger entry.
