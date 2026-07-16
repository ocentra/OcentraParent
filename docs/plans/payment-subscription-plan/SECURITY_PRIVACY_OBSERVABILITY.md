# Security, Privacy, and Observability

Purpose: define the non-negotiable controls around monetization data.

## Security rules

| Rule | Required expectation | Primary proof hooks |
| --- | --- | --- |
| Server-only secrets | Provider secrets, webhook secrets, and billing tokens stay server-side. | `payment-security.secret-scan`, `payment-security.redacted-logs` |
| Verified ingress | Webhook signatures and boundary verification happen before lifecycle processing. | `payment-security.webhook-smuggling-negative`, `payment-security.webhook-replay` |
| Abuse gating | Checkout, portal, invite, and webhook abuse paths are rate-limited or review-gated. | `payment-security.rate-limit`, `payment-security.bot-abuse-gate`, `payment-security.referral-abuse-signals` |
| Redirect safety | Success and cancel returns stay inside the allow-list boundary. | `payment-security.open-redirect-negative` |
| Privileged audit | Support/admin actions are authenticated, role-limited, and audited. | `payment-security.admin-audit-required`, `payment-security.support-view-minimized` |
| Test/live separation | Test and live billing states remain visibly distinct in ledger, proof, and operator-facing surfaces. | `payment-security.test-live-separated` |

## Provider metadata allow list

Allowed examples:

- plan reference
- price reference
- family or account reference
- referral code or referral reference
- invoice or receipt reference
- session or checkout reference
- idempotency reference
- region, currency, and test/live marker

Denied examples:

- child names
- child activity
- screenshots
- URL history
- policy details
- support bundle contents
- local device secret material

## Privacy rules

- Do not send child names, child activity, screenshots, or policy details to
  billing providers.
- Use the minimum provider metadata needed for reconciliation and support.
- Redact support logs, webhook logs, and analytics payloads by default.
- Keep billing data separate from custody/export/delete decisions owned by
  other plans.
- Billing telemetry must not be a back door to child data or custody state.

## Observability rules

- Every state change must be attributable to a billing event, a referral event,
  or a support/admin action.
- Queue retries and dead-letter events must be visible to support/admin
  operators.
- Metrics should track checkout creation, webhook replay, refund rate, dispute
  rate, and reconciliation failures.
- Observability must distinguish provider state, app-owned ledger state, and
  support actions.
- Redacted observability must still preserve request correlation and outcome
  class.

## Required proof hooks

- `payment-security.provider-metadata-allow-deny`
- `payment-security.no-child-data-metadata`
- `payment-security.secret-scan`
- `payment-security.support-view-minimized`
- `payment-security.test-live-separated`
- `payment-security.redacted-logs`

## Failure conditions

- Do not log raw provider secrets.
- Do not log child data in payment telemetry.
- Do not expose admin-only timelines to parent surfaces.
- Do not let provider metadata carry child content, screenshots, or policy
  details.
