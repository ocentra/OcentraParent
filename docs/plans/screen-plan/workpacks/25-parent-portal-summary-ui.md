# 25 Parent Portal Summary UI

<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-plan`
> Doc: `25 Parent Portal Summary UI`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

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

- [ ] Build disabled state.
- [ ] Build capability state.
- [ ] Build queue health state.
- [ ] Build summary cards.
- [ ] Build confidence/unknown UI.
- [ ] Build deletion/custody labels.
- [ ] Build policy/audit details.
- [ ] Capture screenshots.

## Proof

- `packages/portal-domain/tests/screen-summary-panel.test.ts`.
- `apps/portal/tests/screen-summary-route-panel.test.ts`.
- `apps/portal/e2e/screen-summary-ui-proof.spec.ts`.
- `scripts/test/screen-parent-portal-summary-ui-proof.mjs`.
- `output/screen-plan-proof/screen-parent-portal-summary-ui/screenshots/screen-analysis-route-desktop.png`.
- `output/screen-plan-proof/screen-parent-portal-summary-ui/screenshots/screen-analysis-route-mobile.png`.
- `output/screen-plan-proof/settings-writable-controls/proof-summary.json`.
- `output/screen-plan-proof/screen-parent-portal-summary-ui/proof-summary.json`.
