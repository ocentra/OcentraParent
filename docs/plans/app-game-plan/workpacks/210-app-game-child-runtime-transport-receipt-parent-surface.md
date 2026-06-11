# WP210 App/Game Child Runtime Transport Receipt Parent Surface

## Scope

Render the service-backed app/game child runtime transport and receipt read
model in the parent portal App/Game Sessions surface.

This moves WP209 from service-visible to parent-visible without claiming child
runtime delivery execution.

## Implementation

- Added
  `packages/portal-domain/src/app-game-child-runtime-transport-receipt-panel.ts`.
- Added
  `packages/portal-domain/tests/app-game-child-runtime-transport-receipt-panel.test.ts`.
- Added
  `apps/portal/src/AppGameChildRuntimeTransportReceiptRoutePanel.tsx`.
- Added
  `apps/portal/tests/app-game-child-runtime-transport-receipt-route-panel.test.ts`.
- Updated `apps/portal/src/live-activity-state.ts` to parse
  `agent.activity.app-game.child-runtime-transport-receipt.read-model.reported`.
- Updated portal command lists and text tokens so the parent surface can request
  `agent.activity.app-game.child-runtime-transport-receipt.read-model.get`.

## Validation

Focused validation for this workpack:

```powershell
cmd /c npm run test --workspace @ocentra-parent/portal-domain -- app-game-child-runtime-transport-receipt-panel
cmd /c npm run test --workspace @ocentra-parent/portal -- app-game-child-runtime-transport-receipt-route-panel
```

## Proof

Proof artifacts are expected under:

- `test-results/app-game-child-runtime-transport-receipt-parent-surface-proof/proof.json`
- `output/app-game-plan-proof/210-app-game-child-runtime-transport-receipt-parent-surface/proof.json`
- `output/app-game-plan-proof/210-app-game-child-runtime-transport-receipt-parent-surface/00-source-snapshot.md`
- `output/app-game-plan-proof/210-app-game-child-runtime-transport-receipt-parent-surface/10-validation-commands.log`

## Boundaries

Proved:

- The parent portal can request the child runtime transport receipt read model.
- The portal live-activity state parses the service event into a typed result.
- The App/Game Sessions surface renders transport-required, manual-required,
  and unavailable rows with parent-safe transport refs, receipt refs, open
  gaps, and no-claim status values.

Not proved:

- Child runtime transport execution.
- Child runtime receipt ingestion.
- Provider delivery execution.
- Platform delivery channel execution.
- Adapter dispatch or platform enforcement.
- Raw private source row custody.
