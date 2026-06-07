# WP111 Timer Parent-Surface Active State-Store Bridge

## Scope

WP111 bridges the existing enforcement active timer state file into the
app/game timer parent-surface read model.

## Implementation Evidence

- `crates/agent-service/src/activity_api/app_game_timer_parent_surface_payload.rs`
  reads the active timer state path through the existing state-file helper and
  passes an optional active state into the read-model builder.
- `crates/agent-service/src/activity_api/app_game_timer_parent_surface_service_tests.rs`
  creates a real active timer state through the enforcement audit path, then
  verifies the app/game parent-surface command reports active timer
  state-store visibility.

## Claim Boundary

- Claimed when active state exists: timer runtime visibility, scheduler
  persistence visibility, durable scheduler state-store visibility.
- Not claimed: live scheduling execution, audit runtime/read-model storage,
  rollback execution, adapter dispatch, child delivery, broad blocking,
  platform enforcement, raw private source rows.

## Validation

- `cargo fmt --package ocentra-parent-agent-service`
- `cargo test -p ocentra-parent-agent-service app_game_timer_parent_surface`
