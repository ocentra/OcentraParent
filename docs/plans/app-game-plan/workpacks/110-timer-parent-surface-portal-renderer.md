# WP110 Timer Parent-Surface Portal Renderer

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP110 Timer Parent-Surface Portal Renderer`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Scope

- Render the WP109 service-backed timer parent-surface read model on the
  App/Game Sessions route.
- Reuse the existing live activity event resolver and portal overlay pattern.
- Keep native apps and native games in one read-model surface while preserving
  `targetDomain` in each row.
- Keep the UI honest: no timer runtime, scheduler persistence, durable storage,
  audit/rollback runtime, adapter dispatch, child delivery, platform
  enforcement, or raw private source row claim.

## Implementation

- `apps/portal/src/live-activity-state.ts` now parses the timer parent-surface
  service event.
- `apps/portal/src/AppGameTimerParentSurfaceRoutePanel.tsx` renders summary
  and row cards on App/Game Sessions and exposes the service refresh command.
- `packages/portal-domain/src/app-game-timer-parent-surface-panel.ts` owns the
  panel intent and display-safe row mapping.
- `packages/text-domain/src/portal-dev.ts` owns the user-facing text tokens.

## Validation

- `cmd /c npx prettier --check packages/agent-protocol-domain/src/app-game-timer-parent-surface-read-model.ts packages/portal-domain/src/app-game-timer-parent-surface-panel.ts apps/portal/src/app-game-timer-parent-surface-panel.ts apps/portal/src/AppGameTimerParentSurfaceRoutePanel.tsx apps/portal/tests/app-game-timer-parent-surface-panel.test.ts apps/portal/src/live-activity-state.ts packages/portal-domain/src/command-results.ts apps/portal/src/ParentPortalRoute.tsx packages/text-domain/src/portal-dev.ts packages/portal-domain/src/contracts.ts`
- `cmd /c "npm run build --workspace @ocentra-parent/agent-protocol-domain && npm run build --workspace @ocentra-parent/text-domain && npm run build --workspace @ocentra-parent/portal-domain"`
- `cmd /c npm run test --workspace @ocentra-parent/portal -- app-game-timer-parent-surface-panel`

## Remaining Gaps

- Runtime persistence and live timer scheduling remain unimplemented.
- Scheduler persistence, durable scheduler storage, audit runtime, durable audit
  log storage, and rollback execution remain unimplemented.
- Adapter dispatch, child delivery, platform enforcement, and raw private
  source row access remain unclaimed.
