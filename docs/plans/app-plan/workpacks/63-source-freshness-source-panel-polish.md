# WP63 - Source Freshness Source Panel Polish

## Scope

Cross-record the shared app/game WP63 source freshness source-panel intent seam
for the native app plan.

The shared App/Game Sessions dashboard intent now exposes grouped source-panel
sections for native app/game `sourceStatusRows`, separating app-use and game
source freshness with fresh/manual/evidence counts.

## Implementation

- Add a split source-panel helper for app/game source status rows.
- Expose `sourcePanelSections` from the portal dashboard intent.
- Render the typed source-panel sections in the existing app/game dashboard
  side-panel stack alongside capability and evidence panels.
- Prove app-use and game source sections in the portal intent test.
- Keep SVG rendering and route E2E assertions as a follow-up because those files
  are now production-drafted here; route/E2E validation remains deferred.

Code-pass status: code-drafted; unvalidated; tests/proof/checklist deferred.

## Proof

- `cmd /c npm exec --workspace @ocentra-parent/portal -- vitest run tests/activity-ui-app-game-dashboard-intent.test.ts`
- `node scripts/test/app-game-source-panel-polish-proof.mjs`
- focused format/schema/source checks and hub/lane guards before PR-ready handoff

Proof artifacts live in:

```text
output/app-plan-proof/63-source-freshness-source-panel-polish
```

## No-Claim Boundaries

- This is portal intent/data proof for existing read-model data.
- It does not prove native app policy runtime consumption, provider execution,
  app blocking, installer/store integration, rendered source-panel UI, or
  cross-platform adapter support.

## Product Doc Decision

The shared feature docs and plan checklists are updated. The product capability
checklist is unchanged until the source panel is rendered in the parent portal
or otherwise changes a product row proof/gap.
