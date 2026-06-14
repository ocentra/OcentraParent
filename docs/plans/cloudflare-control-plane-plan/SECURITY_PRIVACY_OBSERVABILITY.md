# Security Privacy Observability

Purpose: capture the shared Cloudflare guardrails Parent keeps from the games module and the ones it intentionally strips.

## Required runtime guards

- environment validation before dispatch
- CORS fail-fast before protected work
- request-size rejection before body processing
- emergency kill switch for state-changing routes
- provider webhook signature verification
- redacted error and operational logs
- audit events for admin/support actions
- queue-only or cron-only routes for internal jobs

## Privacy boundaries

- No child telemetry or raw child data in D1, KV, Queue payloads, or optional R2.
- No provider secrets or signing-key references in client-visible responses.
- Support/admin outputs must be minimized and auditable.

## Observability baseline

- redacted structured logs
- route-level audit events
- reconciliation and dead-letter counters
- explicit test/live separation in logs and proof

## Required negative cases

- untrusted origin rejected
- missing auth rejected
- admin/support role rejected
- invalid provider signature rejected
- request smuggling rejected
- provider secrets absent from client payloads
