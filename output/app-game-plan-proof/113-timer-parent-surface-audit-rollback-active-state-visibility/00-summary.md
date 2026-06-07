# WP113 Timer Parent-Surface Audit Rollback Active-State Visibility

## Scope

WP113 updates the app/game timer parent-surface service and portal intent so
existing active enforcement timer audit sequence and rollback token references
are visible to parents when the active timer state contains them.

## Implementation Evidence

- `crates/agent-service/src/activity_api/app_game_timer_parent_surface_payload.rs`
  derives audit visibility from the active timer state's audit journal sequence.
- The same service payload derives rollback visibility from rollback token
  references on the active timer action, result, or timer event state.
- `packages/agent-protocol-domain/src/app-game-timer-parent-surface-read-model.ts`
  accepts service-reported audit and rollback visibility while continuing to
  reject adapter, child, platform, and raw private source overclaims.
- `packages/portal-domain/src/app-game-timer-parent-surface-panel.ts` and
  `apps/portal/tests/app-game-timer-parent-surface-panel.test.ts` render the
  visibility as ready state without claiming execution.
- `packages/text-domain/src/portal-dev.ts` keeps inactive fallback copy aligned
  with the same no-execution boundary.
- `crates/agent-service/src/activity_api/app_game_timer_parent_surface_service_tests.rs`
  covers the real service path from enforcement audit output to timer
  parent-surface read-model reporting.

## Claim Boundary

- Visible when active timer state reports it: timer runtime state-store,
  scheduler persistence state-store, durable scheduler state-store, audit
  active-state sequence, rollback active-state token.
- Not claimed: live scheduling execution, durable audit log read-models,
  rollback execution, adapter dispatch, child delivery, broad blocking,
  platform enforcement, raw private source rows.

## Validation

- `cargo fmt --package ocentra-parent-agent-service`
- `cargo test -p ocentra-parent-agent-service app_game_timer_parent_surface`
- `cmd /c "npm run build --workspace @ocentra-parent/agent-protocol-domain && npm run build --workspace @ocentra-parent/text-domain && npm run build --workspace @ocentra-parent/portal-domain"`
- `cmd /c npm run test --workspace @ocentra-parent/agent-protocol-domain -- app-game-timer-parent-surface-read-model`
- `cmd /c npm run test --workspace @ocentra-parent/portal -- app-game-timer-parent-surface-panel`
- `cmd /c npx prettier --check ...`
- `cmd /c node scripts/check-no-test-doubles.mjs`
- `cmd /c node scripts/check-source-shape.mjs`
- `git diff --check`
- `cmd /c npm run lanes:guard`
- `cmd /c npm run hub:guard`
