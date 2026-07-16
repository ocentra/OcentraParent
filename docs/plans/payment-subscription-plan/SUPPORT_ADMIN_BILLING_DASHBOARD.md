# Support/Admin Billing Dashboard

Purpose: define the support and admin billing surface for refunds, disputes, adjustments, reconciliation, and audit.

## Required surfaces

- Billing account search.
- Provider customer refs.
- Subscription status.
- Invoice, refund, and dispute state.
- Invoice search.
- Manual invoice state.
- Referral abuse state.
- Entitlement calculation.
- Over-limit households.
- Webhook failures.
- Reconciliation drift.
- Admin actions.
- Support-role minimized view.
- Support-role limited access.
- Audit trail.

## Field boundary

### Allowed fields

- Redacted billing account search results and provider customer refs.
- Subscription status, invoice state, refund state, and dispute state.
- Manual invoice state, referral abuse state, entitlement calculation, and over-limit households.
- Webhook failures, reconciliation drift, admin actions, and audit trail entries.
- Support-role minimized and support-role limited views.

### Denied fields

- Child activity, screenshots, browser history, app history, and network history.
- Location, policy details, custody keys, recovery bundles, and local device secrets.
- Raw child telemetry and private child data.
- Provider secrets and any support view that bypasses audit logging.

## Rules

- Every support action must be audited.
- Every support view must be redacted by default.
- The dashboard must not expose child telemetry or child content.
- Support can change billing state, but not device trust or account ownership.
- Support cannot use the dashboard to bypass provider or entitlement provenance.

## Must not expose

- Child activity.
- Screenshots.
- Browser, app, or network history.
- Location.
- Policy details.
- Data custody keys.
- Recovery bundles.
- Local device secrets.

## Required behaviors

| Action | Expected result |
| --- | --- |
| Search account | Return a redacted billing timeline. |
| Search invoice | Return invoice state, not child data. |
| Issue refund | Write a ledger entry and update entitlement impact. |
| Open dispute review | Surface provider refs and evidence pointers. |
| Apply manual credit | Record the adjustment and actor. |
| Retry reconciliation | Re-run the queued or dead-lettered billing work. |
| Inspect over-limit household | Show grace or over-limit state without leaking child data. |
| Review abuse | Show referral or provider abuse signals without exposing child data. |

## Proof hooks

- `payment-admin.billing-account-search`
- `payment-admin.invoice-search`
- `payment-admin.refund-action-audited`
- `payment-admin.dispute-state-visible`
- `payment-admin.manual-invoice-state`
- `payment-admin.referral-abuse-visible`
- `payment-admin.reconciliation-drift-visible`
- `payment-admin.webhook-failure-visible`
- `payment-admin.admin-role-required`
- `payment-admin.support-role-limited`
- `payment-admin.no-child-private-data`
- `payment-admin.audit-event-required`

## Failure conditions

- Do not let support bypass audit logging.
- Do not let support read child data.
- Do not let support mutate provider history without a ledger entry.
- Do not let support views reveal private child or custody data.
