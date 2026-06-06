# 25 Parent Portal Summary UI

## Target State

Settings, capability, queue health, summaries, confidence, source refs, deletion state, model status, and audit are visible.

## Current State

Portal route/read-model plumbing exists. Settings now has read-only catalog
proof plus writable local screen-summary intent draft proof. Complete parent UI
is still open, but the dedicated Screen Analysis route now has a typed summary
panel intent and renderer proof at
`output/screen-plan-proof/screen-parent-portal-summary-ui/proof-summary.json`.
The proof uses service-backed Activity Screen read-model rows and shows
capability, queue job, summary/category, confidence, source refs, deleted-image
custody, model/runtime status, policy, audit, and not-claimed enforcement state.
It does not yet provide fresh desktop/mobile screenshots for this exact route.

## Checklist

- [x] Build disabled state.
- [x] Build capability state.
- [x] Build queue health state.
- [x] Build summary cards.
- [x] Build confidence/unknown UI.
- [x] Build deletion/custody labels.
- [x] Build policy/audit details.
- [~] Capture screenshots.

## Proof

- `packages/portal-domain/tests/screen-summary-panel.test.ts`.
- `apps/portal/tests/screen-summary-route-panel.test.ts`.
- `scripts/test/screen-parent-portal-summary-ui-proof.mjs`.
- Desktop and mobile screenshots for the dedicated summary route remain open.
- `output/screen-plan-proof/settings-writable-controls/proof-summary.json`.
- `output/screen-plan-proof/screen-parent-portal-summary-ui/proof-summary.json`.
