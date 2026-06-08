# WP147 - Timer parent preference setup child-runtime delivery receipt-pending seam

## Scope

WP147 extends the accepted app/game timer parent preference setup request result
with service-local child-runtime delivery receipt-pending refs/status after the
receipt-required row.

## Touched Files

- `packages/agent-protocol-domain/src/app-game-timer-parent-preference-setup-request.ts`
- `packages/agent-protocol-domain/tests/app-game-timer-parent-preference-setup-request.test.ts`
- `crates/agent-protocol/src/app_game_timer_parent_preference_setup_request.rs`
- `crates/agent-protocol/src/constants/field.rs`
- `crates/agent-protocol/src/constants/value.rs`
- `crates/agent-service/src/activity_api/app_game_timer_parent_preference_setup_request.rs`
- `crates/agent-service/src/activity_api/app_game_timer_parent_preference_setup_request_persistence.rs`
- `crates/agent-service/src/activity_api/app_game_timer_parent_preference_setup_request_tests.rs`
- `apps/portal/tests/app-game-timer-parent-surface-panel.test.ts`
- `docs/features/app-game-control.md`
- `docs/plans/app-game-plan/implementation-checklist.md`
- `docs/plans/app-game-plan/workpacks/README.md`
- `docs/plans/app-game-plan/workpacks/147-timer-parent-preference-setup-child-runtime-delivery-receipt-pending.md`
- `test-results/app-game-timer-parent-preference-setup-child-runtime-delivery-receipt-pending/handoff.json`

## Result

- The accepted setup request result now carries receipt-pending ID, refs,
  status, and claimed-state fields.
- Rust protocol mirrors the new fields and constants.
- Agent service persists a seventh local audit row for receipt-pending after
  the receipt-required row.
- Focused TypeScript and Rust tests assert the new contract and persistence
  behavior.
- Portal fixture parsing remains strict, but receipt-pending UI rendering is a
  later slice.
- Product capability checklist is intentionally untouched.

## No-Claim Boundaries

- No actual child runtime delivery or receipt is claimed.
- No provider delivery or provider receipt ingestion is claimed.
- No durable production outbox runtime is claimed.
- No adapter dispatch, broad blocking, or platform enforcement is claimed.
- No raw private source rows, raw target values, or private diagnostics are
  exposed.
- Package/crate README updates are deferred because E-D owns those locks.
