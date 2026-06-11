# WP111 Timer Parent-Surface Active State-Store Bridge

## Scope

- Reuse the existing enforcement active timer state file while building the
  app/game timer parent-surface read model.
- Report timer runtime, scheduler persistence, and durable scheduler
  state-store visibility only when that real active state exists.
- Keep the app/game read model unified for native apps and native games.
- Do not claim live scheduling execution, audit read-models, rollback
  execution, adapter dispatch, child delivery, broad blocking, platform
  enforcement, or raw private source rows.

## Implementation

- `crates/agent-service/src/activity_api/app_game_timer_parent_surface_payload.rs`
  now reads `AGENT_ENFORCEMENT_TIMER_STATE_PATH` through the existing
  enforcement timer state helper before building the parent-surface payload.
- The pure service-model builder remains available and defaults to no active
  timer state.
- The read model flips only `timerRuntimeClaimed`,
  `schedulerPersistenceClaimed`, and `durableSchedulerStorageClaimed` when a
  parsed active timer state exists.

## Validation

- `cargo fmt --package ocentra-parent-agent-service`
- `cargo test -p ocentra-parent-agent-service app_game_timer_parent_surface`

## Remaining Gaps

- Live scheduling execution remains unimplemented.
- Audit runtime/read-model storage and rollback execution remain unimplemented.
- Adapter dispatch, child delivery, broad blocking, platform enforcement, and
  raw private source row access remain unclaimed.
