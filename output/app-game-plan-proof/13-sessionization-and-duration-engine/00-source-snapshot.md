# Source Snapshot

Workpack: app-game WP13 sessionization and duration engine

Branch: `codex/app-game-sessionization-duration`

Starting head while preparing proof: `8fda8c3`

Before-state gap:

- `ActivityStore::app_game_session_report` grouped stored app/game rows by
  process identity but did not derive deterministic running, foreground,
  background, stale-gap, process-exit, replay, or daily rollup duration values.
- TypeScript and Rust protocol session summaries did not carry end reasons,
  last foreground/background evidence timestamps, or observation gap values.

Locked source/proof paths:

- `crates/agent-core/src/activity_store_app_game.rs`
- `crates/agent-core/src/activity_store_app_game_observation.rs`
- `crates/agent-core/src/activity_store_app_game/app_game_sessionization.rs`
- `crates/agent-core/src/activity_store_app_game/app_game_session_rollups.rs`
- `crates/agent-core/src/activity_store_app_game/app_game_session_time.rs`
- `crates/agent-core/src/activity_store_app_game/app_game_sessionization_tests.rs`
- `crates/agent-protocol/src/app_game.rs`
- `crates/agent-protocol/src/app_game_tests.rs`
- `packages/activity-domain/src/app-game-primitives.ts`
- `packages/activity-domain/src/app-game-session-primitives.ts`
- `packages/activity-domain/src/app-game.ts`
- `packages/activity-domain/tests/app-game.test.ts`
- `docs/features/app-game-control.md`
- `docs/product-capability-checklist.md`
- `docs/plans/app-game-plan/*`
- `docs/plans/app-plan/*`
- `output/app-game-plan-proof/13-sessionization-and-duration-engine`
- `output/app-plan-proof/12-app-sessionization-and-duration-engine`

Source files inspected:

- `docs/feature-list.md`
- `docs/features/app-game-control.md`
- `docs/architecture/app-game-evidence-sessions.md`
- `docs/plans/app-game-plan/workpacks/13-sessionization-and-duration-engine.md`
- `docs/plans/app-plan/workpacks/12-app-sessionization-and-duration-engine.md`
- `packages/activity-domain/README.md`
- `crates/agent-core/README.md`
- `crates/agent-protocol/README.md`

Current `git status --short` included the implementation and docs files above
plus pre-existing untracked `.codex/` and `.playwright-cli/` proof artifacts
from earlier C-lane UI work. Those unrelated artifacts were preserved and are
not part of this proof.
