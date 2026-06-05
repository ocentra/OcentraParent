# 63 Source Freshness Source Panel Polish

## Scope

Build the App/Game Sessions source-panel intent seam for the dedicated source
freshness panel that follows WP48.

This workpack groups existing service-backed `sourceStatusRows` into
parent-readable app-use and game source sections with fresh/manual/evidence
counts, row labels, capability labels, last-observed labels, and no-claim
boundaries. It prepares the rendering seam without editing the SVG surface while
another lane owns that file.

## Implementation

- Add a split source-panel intent helper beside the app/game dashboard intent.
- Expose `sourcePanelSections` from the existing dashboard intent.
- Prove app-use and game source sections with fresh, manual-required, row, and
  evidence metrics in the portal intent test.
- Do not edit `ParentPortalSvgSurface.tsx` or route E2E assertions while those
  paths are locked by E-A.

## Proof

- `cmd /c npm exec --workspace @ocentra-parent/portal -- vitest run tests/activity-ui-app-game-dashboard-intent.test.ts`
- `node scripts/test/app-game-source-panel-polish-proof.mjs`
- focused format/schema/source checks and hub/lane guards before PR-ready handoff

Proof artifacts live in:

```text
output/app-game-plan-proof/63-source-freshness-source-panel-polish
```

## No-Claim Boundaries

- Source-panel rows summarize already-stored service evidence only.
- This slice does not add backend source contracts, source subscriptions,
  policy evaluator consumption, adapter execution, broad blocking, provider
  delivery, or platform support.
- SVG source-panel rendering remains a follow-up until the surface and route
  assertion locks are available.

## Product Doc Decision

Feature docs and plan checklists are updated because the source panel data seam
now exists. `docs/product-capability-checklist.md` is not updated in this slice
because the dedicated source panel is not yet a rendered parent-visible surface.
