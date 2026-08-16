# Security Privacy Observability

Purpose: capture the shared Cloudflare guardrails Parent keeps from the games module and the ones it intentionally strips.

## Required runtime guards

| Guard | Required expectation | Primary proof surface |
| --- | --- | --- |
| Environment validation before dispatch | Required env keys and binding names fail closed when missing or malformed. | `UT-ENV-*` |
| CORS fail-fast before protected work | Disallowed origins are rejected before protected work and wildcard misconfigurations fail closed. | `SEC-CORS-*` |
| Request-size rejection before body processing | Oversize or ambiguous envelopes fail with redacted `413` behavior before dispatch. | `UT-LIMIT-*`, `SEC-SMUGGLE-*` |
| Emergency kill switch for state-changing routes | Write paths and webhook processing stop while read-only health and pricing remain available. | `UT-KILL-*` |
| Provider webhook signature verification | Missing, invalid, malformed, or wrong-provider signatures fail closed before business processing. | `UT-AUTH-06`, `IT-WEBHOOK-*`, `FUZZ-WEBHOOK-*` |
| Redacted error and operational logs | Secret names, child-data markers, recovery markers, and local device secrets are removed while correlation remains support-safe. | `UT-REDACT-*`, `SEC-REDACT-*` |
| Audit events for admin/support actions | Privileged rejection and access paths preserve audit-safe state without leaking raw auth internals. | `UT-AUTH-09`, `IT-ADMIN-*` |
| Queue-only or cron-only routes for internal jobs | Internal-only routes reject public or parent-facing callers. | `UT-AUTH-07`, `PROP-ROUTE-04` |

## Privacy boundaries

- No child telemetry or raw child data in D1, KV, Queue payloads, or optional R2.
- No provider secrets or signing-key references in client-visible responses.
- Support/admin outputs must be minimized and auditable.

### Required redaction markers

- Provider secret names and values:
  - `STRIPE_SECRET_KEY`
  - `STRIPE_WEBHOOK_SECRET`
  - `RAZORPAY_KEY_ID`
  - `RAZORPAY_KEY_SECRET`
  - `PAYPAL_CLIENT_ID`
  - `PAYPAL_CLIENT_SECRET`
  - `APPLE_STORE_KEY_REF`
  - `GOOGLE_PLAY_SERVICE_ACCOUNT_REF`
  - `ENTITLEMENT_SIGNING_KEY_REF`
- Child-data markers and raw evidence references.
- Recovery-bundle markers and support-bundle private-path references.
- Local device secret markers, auth headers, cookies, and tokens.

## Observability baseline

| Requirement | Meaning |
| --- | --- |
| Redacted structured logs | Logs keep route, request correlation, and outcome class but never raw secret-bearing or child-data payloads. |
| Route-level audit events | Private, admin, support, webhook, and queue boundaries preserve audit-safe event ownership. |
| Reconciliation and dead-letter counters | Retry and dead-letter state may be visible operationally, but not through secret-bearing payloads. |
| Explicit test/live separation in logs and proof | Manual-required, scaffold-only, degraded, stale, offline, and runtime-ready states stay explicit in proof and consumer-safe status surfaces. |

## Required negative cases

- untrusted origin rejected
- missing auth rejected
- admin/support role rejected
- invalid provider signature rejected
- request smuggling rejected
- provider secrets absent from client payloads
- malformed webhook payload rejected before business processing
- queue-only or cron-only routes rejected for non-internal callers

## First-slice non-goals

- Do not invent a standalone observability suite before the boundary tests above
  exist.
- Do not treat runtime counters, analytics, or log sinks as child-data custody.
- Do not mark secrets or redaction as proven from placeholder test files alone.
