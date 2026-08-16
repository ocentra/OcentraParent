# WP111 Timer Parent-Surface Active State-Store Bridge

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP111 Timer Parent-Surface Active State-Store Bridge`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

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
