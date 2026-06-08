# WP145 - Timer parent preference setup child-runtime delivery receipt-required seam

## Scope

WP145 adds a service-local child-runtime delivery receipt-required seam to the
accepted app/game parent preference setup request result after dispatch-ready.

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
- `docs/plans/app-game-plan/workpacks/145-timer-parent-preference-setup-child-runtime-delivery-receipt-required.md`
- `test-results/app-game-timer-parent-preference-setup-child-runtime-delivery-receipt-required/handoff.json`

## Result

- The accepted parent preference setup request result now includes
  child-runtime delivery receipt-required refs/status.
- The Rust protocol mirrors the TypeScript contract.
- The service persists a sixth local audit event after dispatch-ready to record
  that a future child-runtime receipt is required before delivery can be
  claimed.
- `docs/product-capability-checklist.md` is intentionally untouched.

## No-Claim Boundaries

- No actual child runtime delivery or receipt is claimed.
- No provider delivery or provider receipt ingestion is claimed.
- No durable production outbox runtime is claimed.
- No adapter dispatch, broad blocking, or platform enforcement is claimed.
- No raw private source rows, raw target values, or private diagnostics are
  exposed.
