# Billing API Boundary

Purpose: define which callers may request billing state and hosted billing actions, while keeping provider/backend/runtime ownership outside the browser and outside this plan's thin TypeScript edge contracts.

## Live WP02 contract surfaces

- API route contract: `packages/schema-domain/src/endpoint-billing-account.ts`
- Hosted checkout and portal request/response contract: `packages/schema-domain/src/billing-checkout-portal-boundary.ts`
- Hosted checkout and portal value guards: `packages/schema-domain/src/billing-checkout-portal-boundary-values.ts`
- Endpoint proof: `scripts/test/billing-account-endpoint-contract-proof.mjs`
- Hosted checkout proof: `scripts/test/payment-checkout-boundary-proof.mjs`

These TypeScript files are thin edge-validation and proof-consumer surfaces only. They do not re-own Cloudflare Worker runtime, provider backend runtime, account backend runtime, entitlement signing runtime, or portal UI runtime.

The route groups below are consumed from `docs/plans/cloudflare-control-plane-plan/ROUTE_MANIFEST_MODEL.md`; payment owns the billing meaning attached to them, not the shared Worker shell.

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
| Parent web or mobile surface | Read pricing, request hosted checkout, request hosted portal, read billing status, preview entitlement state | Parent auth, plus device trust for sensitive actions |
| Support/admin surface | Search accounts, issue refunds, apply manual credits, inspect timelines | Admin auth and audit logging |
| Provider webhook | Deliver payment, subscription, refund, or dispute events | Provider signature validation |
| Queue / cron / reconciliation | Retry, reconcile, and finalize ledger state | Internal worker boundary |
| Other product surfaces | Read signed entitlement snapshot only | Snapshot signature and device binding |

## Hosted session request invariants

- Browser-originated hosted billing requests must carry `same-origin-verified`.
- Browser-originated hosted billing requests must carry `csrf-token-verified`.
- Browser-originated hosted billing requests must keep provider secret custody at `not-present`.
- Hosted checkout requests may use only billable plan ids currently allowlisted by the contract.
- Hosted return routes are allowlisted and carry explicit resolution state:
  - `family-billing-checkout-success` -> `/family/billing/checkout/success` -> `awaiting-provider-webhook`
  - `family-billing-checkout-cancel` -> `/family/billing/checkout/cancel` -> `cancelled-before-provider-confirmation`
  - `family-billing-portal-return` -> `/family/billing/manage` -> `portal-management-only`

## Boundary rules

- Browser callers must never receive provider secrets or webhook secrets.
- Hosted checkout and hosted portal URLs must reject `client_secret=` leakage.
- Checkout success URLs are not proof of payment.
- Portal returns are management-only and not entitlement truth.
- Provider event payloads are inputs, not truth.
- Billing writes must pass through the shared Worker and the per-account or per-household serialization boundary.
- Sensitive actions may require device-trust proof, but the device-trust flow itself belongs to the device-trust plan.
- Dashboard actions must return redacted, audit-friendly payloads.

## Explicit non-claims

WP02 proof does not claim:

- Cloudflare Worker runtime readiness
- billing provider backend runtime
- account backend runtime
- entitlement signing runtime
- portal UI runtime
- child activity custody
- paid access from redirect alone

## Failure conditions

- Do not expose raw provider event objects to UI consumers.
- Do not let support actions bypass audit logging.
- Do not let a provider event change entitlement without a ledger entry.
