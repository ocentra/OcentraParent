# WP47 - Backend Source Freshness Read-Model Rows

## Scope

Cross-record the shared app/game WP47 backend source freshness read-model rows
for the native app plan.

This workpack proves that native app read-model rows can carry source-kind
status for inventory, runtime, and foreground evidence through the existing
activity-surface payload.

It does not add portal UI, policy consumption, adapter execution, broad app
blocking, or platform support claims.

## Implementation

- Reuse the shared app/game `ActivityAppGameSourceStatusRow` contract.
- Add `sourceStatusRows` to the native app-use activity read-model row.
- Group native app sources by source kind, row count, latest observed time,
  capability state, and evidence refs.
- Keep launcher source status scoped to the games read model, not app-use.

## Proof

- `cargo test -p ocentra-parent-agent-protocol activity_surface -- --nocapture`
- `cargo test -p ocentra-parent-agent-service app_game_source_status -- --nocapture`
- `cmd /c npm exec --workspace @ocentra-parent/activity-domain -- vitest run tests/activity-surface.test.ts`
- `cargo fmt --all --check`
- `cmd /c npm run format:check`
- `cmd /c npm run lint:schema-boundaries`
- `git diff --check`
- `cmd /c npm run lanes:guard`
- `cmd /c npm run hub:guard`

Proof artifacts live in:

```text
output/app-plan-proof/47-backend-source-freshness-read-model
```

## No-Claim Boundaries

- Native app source status rows summarize already-stored evidence only.
- Source status rows do not prove live portal rendering, policy decisions,
  adapter execution, broad app blocking, platform support, or content
  knowledge.
- Inventory source status remains inventory-only and cannot become app runtime
  or foreground use.

## Product Doc Decision

`docs/product-capability-checklist.md` is intentionally unchanged. WP47 exposes
backend source freshness/status rows, but product status should not move until
portal rendering, policy consumption, adapter execution, and platform proof are
finished.
