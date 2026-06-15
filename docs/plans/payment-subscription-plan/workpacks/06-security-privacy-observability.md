# Workpack 06: Security, Privacy, and Observability

Purpose: define secrets handling, redaction, audit, metrics, abuse controls, and test/live separation.

## Owns

- `SECURITY_PRIVACY_OBSERVABILITY.md`
- PSP-002, PSP-006, and the audit/test-live parts of the route

## Must prove

- Provider and webhook secrets never leave the server boundary.
- Logs and support surfaces are redacted.
- Billing provider metadata is privacy-safe.
- Test/live mode separation is visible.
- Abuse controls or rate limits exist at checkout and portal entry points.

## Proof path

- Use `docs/proof/payment-subscription-plan/wp06/` or the owning crate's local proof directory.

## Failure conditions

- The workpack fails if child data appears in telemetry, logs, or provider metadata.
- The workpack fails if admin actions are not auditable.
- The workpack fails if test and live mode are indistinguishable.
