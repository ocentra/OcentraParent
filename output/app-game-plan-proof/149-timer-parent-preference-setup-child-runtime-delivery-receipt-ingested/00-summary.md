# WP149 - Timer parent preference setup child-runtime delivery receipt-ingested boundary

## Scope

WP149 extends the accepted app/game timer parent preference setup request result
from receipt-pending to a service-local receipt-ingested boundary and renders
that boundary in the parent portal command-result details.

## Touched Files

- `packages/agent-protocol-domain/src/app-game-timer-parent-preference-setup-request.ts`
- `packages/agent-protocol-domain/tests/app-game-timer-parent-preference-setup-request.test.ts`
- `crates/agent-protocol/src/app_game_timer_parent_preference_setup_request.rs`
- `crates/agent-protocol/src/constants/value.rs`
- `crates/agent-service/src/activity_api/app_game_timer_parent_preference_setup_request.rs`
- `crates/agent-service/src/activity_api/app_game_timer_parent_preference_setup_request_persistence.rs`
- `crates/agent-service/src/activity_api/app_game_timer_parent_preference_setup_request_tests.rs`
- `packages/portal-domain/src/app-game-timer-parent-surface-panel.ts`
- `apps/portal/tests/app-game-timer-parent-surface-panel.test.ts`
- `packages/portal-domain/README.md`
- `apps/portal/README.md`
- `docs/features/app-game-control.md`
- `docs/plans/app-game-plan/implementation-checklist.md`
- `docs/plans/app-game-plan/workpacks/README.md`
- `docs/plans/app-game-plan/workpacks/149-timer-parent-preference-setup-child-runtime-delivery-receipt-ingested.md`
- `test-results/app-game-timer-parent-preference-setup-child-runtime-delivery-receipt-ingested/handoff.json`

## Result

- The TypeScript and Rust app-game parent preference setup result contracts now
  include receipt-ingested refs/status.
- The agent service creates receipt-ingested refs, marks them after local
  persistence succeeds, and stores one additional local audit event after
  receipt-pending.
- The portal command-result details render receipt-ingested refs/status beside
  action-result persistence, mutation receipt, handoff, queue, dispatch,
  receipt-required, and receipt-pending details.
- Focused tests cover protocol parsing, service persistence, and portal detail
  rendering.
- `docs/product-capability-checklist.md` is intentionally untouched.

## No-Claim Boundaries

- Service-local receipt ingestion is not provider receipt ingestion.
- No provider delivery is claimed.
- No durable production outbox runtime is claimed.
- No adapter dispatch, broad blocking, or platform enforcement is claimed.
- No raw private source rows, raw target values, or private diagnostics are
  exposed.
