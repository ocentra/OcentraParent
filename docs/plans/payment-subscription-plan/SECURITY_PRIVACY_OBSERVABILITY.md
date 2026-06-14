# Security, Privacy, and Observability

Purpose: define the non-negotiable controls around monetization data.

## Security rules

- Provider secrets and webhook secrets stay server-side.
- Webhook signatures must be verified before event handling.
- Checkout and portal routes must rate-limit or abuse-gate repeated attempts.
- Support/admin actions must be authenticated and audited.
- Test/live mode separation must be visible in the ledger and the dashboard.
- Provider metadata must be the minimum needed for reconciliation and support.

## Privacy rules

- Do not send child names, child activity, screenshots, or policy details to billing providers.
- Use the minimum provider metadata needed for reconciliation and support.
- Redact support logs, webhook logs, and analytics payloads by default.
- Keep billing data separate from custody/export/delete decisions owned by other plans.
- Billing telemetry must not be a back door to child data or custody state.

## Observability rules

- Every state change must be attributable to a billing event, a referral event, or a support/admin action.
- Queue retries and dead-letter events must be visible to support/admin operators.
- Metrics should track checkout creation, webhook replay, refund rate, dispute rate, and reconciliation failures.
- Observability must distinguish provider state, app-owned ledger state, and support actions.

## Required proof hooks

- `payment-security.no-child-data-metadata`
- `payment-security.secret-scan`
- `payment-security.support-view-minimized`
- `payment-security.test-live-separated`
- `payment-security.redacted-logs`

## Failure conditions

- Do not log raw provider secrets.
- Do not log child data in payment telemetry.
- Do not expose admin-only timelines to parent surfaces.
- Do not let provider metadata carry child content, screenshots, or policy details.
