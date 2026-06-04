# WP47 - Backend Source Freshness Read-Model Rows

## Scope

Expose backend app/game source freshness and status rows through the existing
app-use and games activity-surface read-model payloads.

This workpack proves that service-backed app/game rows can carry source-kind
status for inventory, runtime, foreground, and launcher evidence without the
portal parsing raw evidence vectors.

It does not add portal UI, policy consumption, adapter execution, broad
blocking, or platform support claims.

## Implementation

- Add a typed `ActivityAppGameSourceStatusRow` contract in Rust protocol and
  Effect Schema activity-domain read-model rows.
- Add `sourceStatusRows` to app-use and games activity-surface read-model rows.
- Group service app/game rows by source kind, row count, latest observed time,
  capability state, and evidence refs.
- Keep launcher source status rows in the games read-model only.

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
output/app-game-plan-proof/47-backend-source-freshness-read-model
```

## No-Claim Boundaries

- Source status rows summarize already-stored evidence only.
- Source status rows do not prove live portal rendering, policy decisions,
  adapter execution, broad app/game blocking, platform support, or content
  knowledge.
- Inventory source status remains inventory-only and cannot become app/game
  runtime or foreground use.
- Launcher source status remains launcher evidence unless child-game proof is
  separately present.

## Product Doc Decision

`docs/product-capability-checklist.md` is intentionally unchanged. WP47 exposes
backend source freshness/status rows, but product status should not move until
portal rendering, policy consumption, adapter execution, and platform proof are
finished.
