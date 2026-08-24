# 63 Source Freshness Source Panel Polish

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `63 Source Freshness Source Panel Polish`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

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

## Live Code/Test Re-Audit - 2026-08-24

- `app-game-source-panel-intent.ts` is tracked and production-reachable from the
  dashboard intent. It groups App use/Game rows and computes fresh,
  manual-required, evidence, capability, and last-observed presentation state.
- `ParentPortalSvgSurface.tsx` now renders those source-panel metrics in the
  mounted App/Game Sessions dashboard; the old "not yet rendered" wording is
  stale.
- The named `apps/portal/tests/activity-ui-app-game-dashboard-intent.test.ts`
  is absent. The shallow route scaffold does not prove grouping, degraded or
  manual-required semantics, or hostile/long metadata.
- Production source is present; expected-test writing, focused execution,
  proof, checklist acceptance, READY, and DONE remain open.
