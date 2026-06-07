# WP116 Timer Parent-Surface Child Reason/Status Refs

## Scope

WP116 updates the app/game timer parent-surface service and portal intent so
existing app/game SQLite replay approval action-result rows expose child-safe
reason and status reference ids.

## Implementation Evidence

- `crates/agent-service/src/activity_api/app_game_timer_parent_surface_action_results.rs`
  derives unique child-facing reason/status refs from
  `approval_action_result_rows[*].request`.
- `crates/agent-protocol/src/app_game_timer_parent_surface_read_model.rs`
  carries the refs through the Rust protocol mirror.
- `packages/agent-protocol-domain/src/app-game-timer-parent-surface-read-model.ts`
  accepts the ref arrays while continuing to reject adapter, child-delivery,
  platform, and raw private source overclaims.
- `packages/portal-domain/src/app-game-timer-parent-surface-panel.ts` and
  `apps/portal/tests/app-game-timer-parent-surface-panel.test.ts` render the
  refs without claiming child delivery.

## Claim Boundary

- Visible when replay rows report it: child-facing reason refs and status refs.
- Not included: private diagnostics, raw action request payloads, raw decision
  payloads, raw private source rows.
- Not claimed: live child UI, notification delivery, adapter dispatch, broad
  blocking, platform enforcement.
- Central `docs/product-capability-checklist.md` wording is deferred because
  another lane owns that shared checklist lock; this proof updates the owning
  feature doc and app-game checklist/workpack.

## Validation

- `cargo fmt --package ocentra-parent-agent-protocol --package ocentra-parent-agent-service`
- `cargo test -p ocentra-parent-agent-protocol app_game_timer_parent_surface_read_model`
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
