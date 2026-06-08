# WP152 provider delivery manual-required handoff proof

## Summary

WP152 extends the accepted app/game timer parent preference setup request result
with provider-delivery attempt refs/status and keeps that attempt explicitly
manual-required. The service persists the attempt audit row after the
provider-delivery readiness audit row, and the parent portal command-result
panel renders the provider-delivery attempt refs/status.

## Touched Scope

- `packages/agent-protocol-domain`: setup request result schema and parser test.
- `crates/agent-protocol`: Rust result mirror and provider-delivery attempt
  constants.
- `crates/agent-service`: persisted setup result, ActivityStore audit event, and
  service tests.
- `packages/portal-domain` and `apps/portal`: parent command-result labels,
  readable status, and portal surface tests.
- App/game feature doc, implementation checklist, workpack README, and WP152
  workpack.

## No-Claim Boundaries

- Provider-delivery attempt status is manual-required; it is not provider
  delivery execution.
- Provider receipt ingestion, adapter dispatch, broad blocking, platform
  enforcement, raw private source rows, raw target values, and private
  diagnostics remain unclaimed.
- `docs/product-capability-checklist.md` is intentionally untouched.
