# WP153 provider delivery preflight requirements

## Branch

- `codex/app-game-control-product-completion`

## Scope

The accepted app/game parent preference setup request now carries parent-safe
provider adapter and provider credential/manual-proof requirement refs/status.
After the provider-delivery attempt handoff persists, the service records local
manual-required audit rows for both preflight requirements and the portal
command-result panel renders those refs/status.

## Touched Areas

- `packages/agent-protocol-domain`: accepted setup result schema and contract
  fixture.
- `crates/agent-protocol`: Rust protocol fields and provider requirement
  constants.
- `crates/agent-service`: setup result construction, ActivityStore persistence,
  outbox/read-model assertions, and protocol tests.
- `packages/portal-domain`: parent command-result details and manual-required
  status mapping.
- `apps/portal`: panel test fixture and visible provider requirement rows.
- `docs/plans/app-game-plan`, `docs/features/app-game-control`,
  `apps/portal/README.md`, and `packages/portal-domain/README.md`: WP153 scope,
  no-claim boundaries, proof routing, and checklist status.

## No-Claim Boundaries

- Provider adapter requirement status is manual-required; it is not adapter
  dispatch.
- Provider credential/manual-proof requirement status is manual-required; it is
  not provider delivery execution.
- Provider receipt ingestion, broad blocking, and platform enforcement remain
  unclaimed.
- Raw private source rows, raw target values, and private diagnostics remain
  excluded.
- `docs/product-capability-checklist.md` is intentionally untouched.
