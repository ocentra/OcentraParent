# WP113 Timer Parent-Surface Audit Rollback Active-State Visibility

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP113 Timer Parent-Surface Audit Rollback Active-State Visibility`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Scope

- Bridge existing active enforcement timer audit sequence and rollback token
  references into the unified app/game timer parent-surface read model.
- Keep native app and native game rows on the same shared parent-surface
  contract.
- Let the parent portal display audit and rollback active-state visibility when
  the service reports those references.
- Keep live scheduling execution, durable audit log read-models, rollback
  execution, adapter dispatch, child delivery, broad blocking, platform
  enforcement, and raw private source rows unclaimed.

## Implementation

- `crates/agent-service/src/activity_api/app_game_timer_parent_surface_payload.rs`
  now derives `auditRuntimeClaimed` from the active timer state's persisted
  audit journal sequence.
- The same service payload derives `rollbackRuntimeClaimed` from rollback token
  references already present on active timer action, result, or timer event
  state.
- `packages/agent-protocol-domain/src/app-game-timer-parent-surface-read-model.ts`
  now accepts service-reported audit and rollback visibility booleans while
  keeping adapter, child, platform, and raw private source flags literal false.
- `packages/portal-domain/src/app-game-timer-parent-surface-panel.ts` keeps the
  parent-facing product claim precise: active-state audit/rollback visibility
  can be shown, but execution and delivery are still not claimed.
- `packages/text-domain/src/portal-dev.ts` keeps the inactive fallback copy on
  the same boundary.

## Validation

- `cargo fmt --package ocentra-parent-agent-service`
- `cargo test -p ocentra-parent-agent-service app_game_timer_parent_surface`
- `cmd /c "npm run build --workspace @ocentra-parent/agent-protocol-domain && npm run build --workspace @ocentra-parent/text-domain && npm run build --workspace @ocentra-parent/portal-domain"`
- `cmd /c npm run test --workspace @ocentra-parent/agent-protocol-domain -- app-game-timer-parent-surface-read-model`
- `cmd /c npm run test --workspace @ocentra-parent/portal -- app-game-timer-parent-surface-panel`
- `cmd /c npx prettier --check packages/agent-protocol-domain/src/app-game-timer-parent-surface-read-model.ts packages/agent-protocol-domain/tests/app-game-timer-parent-surface-read-model.test.ts packages/portal-domain/src/app-game-timer-parent-surface-panel.ts packages/text-domain/src/portal-dev.ts apps/portal/tests/app-game-timer-parent-surface-panel.test.ts docs/features/app-game-control.md docs/plans/app-game-plan/implementation-checklist.md docs/plans/app-game-plan/workpacks/README.md docs/plans/app-game-plan/workpacks/113-timer-parent-surface-audit-rollback-active-state-visibility.md output/app-game-plan-proof/113-timer-parent-surface-audit-rollback-active-state-visibility/00-summary.md output/app-game-plan-proof/113-timer-parent-surface-audit-rollback-active-state-visibility/10-validation-commands.log test-results/app-game-timer-parent-surface-audit-rollback-active-state-visibility/handoff.json`

## Remaining Gaps

- Live scheduling execution remains unimplemented.
- Durable audit log read-models and rollback execution remain unimplemented.
- Adapter dispatch, child delivery, broad blocking, platform enforcement, and
  raw private source row access remain unclaimed.
