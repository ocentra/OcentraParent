# WP210 App/Game Child Runtime Transport Receipt Parent Surface

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP210 App/Game Child Runtime Transport Receipt Parent Surface`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

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
