# 25 Parent Portal Summary UI

## Target State

Settings, capability, queue health, summaries, confidence, source refs, deletion state, model status, and audit are visible.

## Current State

Portal route/read-model plumbing exists. Settings now has read-only catalog
proof plus writable local screen-summary intent draft proof. Complete parent UI
is still open, but the dedicated Screen Analysis route now has a typed summary
panel intent and renderer proof at
`output/screen-plan-proof/screen-parent-portal-summary-ui/proof-summary.json`.
The proof launches the real portal/agent path, clicks the Activity Screen
read-model command, renders `#/screen-analysis`, and shows capability, queue
job, summary/category, confidence, source refs, deleted-image custody,
model/runtime status, policy, audit, and not-claimed enforcement state.
Desktop and mobile screenshots are captured under
`output/screen-plan-proof/screen-parent-portal-summary-ui/screenshots/`.

## Checklist

- [x] Build disabled state.
- [x] Build capability state.
- [x] Build queue health state.
- [x] Build summary cards.
- [x] Build confidence/unknown UI.
- [x] Build deletion/custody labels.
- [x] Build policy/audit details.
- [x] Capture screenshots.

## Proof

- `packages/portal-domain/tests/screen-summary-panel.test.ts`.
- `apps/portal/tests/screen-summary-route-panel.test.ts`.
- `apps/portal/e2e/screen-summary-ui-proof.spec.ts`.
- `scripts/test/screen-parent-portal-summary-ui-proof.mjs`.
- `output/screen-plan-proof/screen-parent-portal-summary-ui/screenshots/screen-analysis-route-desktop.png`.
- `output/screen-plan-proof/screen-parent-portal-summary-ui/screenshots/screen-analysis-route-mobile.png`.
- `output/screen-plan-proof/settings-writable-controls/proof-summary.json`.
- `output/screen-plan-proof/screen-parent-portal-summary-ui/proof-summary.json`.
