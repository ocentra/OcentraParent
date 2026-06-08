# WP170 App/Game Adapter Dispatch Execution Audit

## Scope

Extend the WP169 scoped dispatch command-result read model with a
service-local execution audit seam. This records that the single scoped Windows
owned-process app/game time-limit row reached the local execution-audit boundary
without claiming actual adapter execution, platform enforcement, provider
delivery, child delivery, or broad installed-app blocking.

## Implementation

- Extend the app/game adapter dispatch result contract with execution audit
  state, decision, id, refs, and aggregate counts.
- Extend Rust protocol structs/constants with the same execution audit fields.
- Derive execution audit rows in the agent-service dispatch-result read model:
  the accepted scoped Windows owned-process time-limit row records a
  service-local audit, while all broad, degraded, unavailable, unsupported, and
  manual-required rows stay blocked before execution audit.
- Render the execution audit status and refs in the existing portal-domain
  app/game adapter dispatch result panel.
- Keep `adapterDispatchExecutedClaimed` and all platform/provider/child/private
  claim booleans false.

## Proof

- `scripts/test/app-game-adapter-dispatch-execution-audit-proof.mjs`
- `test-results/app-game-adapter-dispatch-execution-audit-proof/proof.json`

## Non-Claims

- Actual adapter execution remains unclaimed.
- Broad installed-app blocking execution remains unclaimed.
- Platform enforcement outside scoped Windows owned-process time-limit remains
  unclaimed.
- Provider delivery and provider receipt ingestion remain unclaimed.
- Child-device runtime delivery remains unclaimed.
- Raw private source rows, raw target values, and private diagnostics remain
  unclaimed.

## Product Doc Decision

`docs/features/app-game-control.md`,
`docs/plans/app-game-plan/implementation-checklist.md`, and this workpack index
record the execution-audit seam. The central product capability checklist
remains intentionally untouched because another lane owns checklist churn.
