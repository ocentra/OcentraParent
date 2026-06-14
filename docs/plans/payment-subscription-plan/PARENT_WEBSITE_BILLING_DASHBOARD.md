# Parent Website Billing Dashboard

Purpose: define the parent-facing billing surface for status, self-service, and entitlement visibility.

## Required surfaces

- Account profile.
- Household summary.
- Current plan.
- Child devices used and allowed.
- Parent and co-parent invite usage.
- Referral invite links.
- Referral status.
- Earned child-device credits.
- Paid extra device seats.
- Billing status.
- Payment method and hosted portal entry.
- Invoices and receipts.
- Refund and support contact.
- Cancel plan.
- Change plan.
- Manual invoice state.
- License and device entitlement state.

## Field boundary

### Allowed fields

- Account profile and household summary.
- Current plan, billing status, and change/cancel state.
- Child-device counts and seat usage, but not child telemetry.
- Parent and co-parent invite usage.
- Referral links, referral status, and earned child-device credits.
- Payment method summary, invoice history, receipts, and hosted portal entry.
- Manual invoice state and license/device entitlement state.

### Denied fields

- Child telemetry, child screenshots, and raw child content.
- Support-only fields and support audit internals.
- Provider secrets, provider-only actions, and raw provider tokens.
- Local device secrets, recovery bundles, and custody keys.
- Policy details that are not needed to understand billing state.

## Rules

- The dashboard must be readable on web and mobile.
- The dashboard must not show child telemetry, child screenshots, or policy details.
- The dashboard may show support-safe account and household identifiers.
- The dashboard must reflect the app-owned ledger, not the provider UI alone.
- The dashboard must not imply access before the entitlement ledger does.

## Required behaviors

| Action | Expected result |
| --- | --- |
| View status | Show the current billing and entitlement state. |
| Open portal | Hand off to the hosted portal for payment actions. |
| Review invoices | Show invoice history and amounts. |
| Review referral credits | Show whether credits are active, pending, or revoked. |
| Change plan | Update the app-owned ledger after the provider confirms the change. |
| Cancel plan | Show cancellation status, grace state, and entitlement impact. |
| Review seats | Show used, allowed, paid, and referral-derived child-device seats. |

## Proof hooks

- `payment-dashboard.parent-account-visible`
- `payment-dashboard.current-plan-visible`
- `payment-dashboard.child-device-usage-visible`
- `payment-dashboard.referral-credit-visible`
- `payment-dashboard.invoice-visible`
- `payment-dashboard.change-plan-visible`
- `payment-dashboard.wrong-household-denied`

## Failure conditions

- Do not expose support-only fields.
- Do not expose child data.
- Do not let the dashboard claim entitlement before the ledger does.
- Do not expose provider-only actions without the hosted portal handoff.
