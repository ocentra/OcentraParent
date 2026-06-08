# WP150 - Timer parent preference setup durable local outbox runtime

## Scope

WP150 extends the accepted app/game timer parent preference setup request path
from service-local receipt ingestion to a durable local outbox JSONL record, and
renders that outbox status in the parent portal command-result details.

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
- `docs/plans/app-game-plan/workpacks/150-timer-parent-preference-setup-durable-local-outbox-runtime.md`
- `test-results/app-game-timer-parent-preference-setup-durable-local-outbox-runtime/handoff.json`

## Result

- The TypeScript and Rust app-game parent preference setup result contracts now
  include durable local outbox refs/status.
- The agent service writes a parent-safe JSONL outbox record after the setup
  ActivityStore rows persist.
- The Rust service test reads that JSONL record and verifies it carries setup
  refs while keeping provider, adapter, platform, and raw/private claims false.
- The portal command-result details render durable local outbox refs/status
  beside the receipt chain.
- `docs/product-capability-checklist.md` is intentionally untouched.

## No-Claim Boundaries

- Durable local outbox recording is not provider delivery.
- Provider receipt ingestion is not claimed.
- Adapter dispatch, broad blocking, and platform enforcement are not claimed.
- Raw private source rows, raw target values, and private diagnostics remain
  excluded.
