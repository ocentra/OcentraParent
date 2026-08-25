# 16 Parent Portal App/Game Dashboard Surfaces

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `16 Parent Portal App/Game Dashboard Surfaces`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

The parent portal renders a product-grade app/game dashboard from service-backed
read models or explicit fixtures.

## Scope

- App/game overview.
- Installed apps and games.
- Running and foreground states.
- Launcher-only and launcher-game candidate rows.
- Unknown/new app and game approvals.
- Risk app and game-risk candidates.
- Game budgets.
- Capability/platform matrix.
- Evidence drawer and audit timeline.

## Tests And Proof

- Playwright proves dashboard rows.
- Long/malicious names do not break layout.
- Inventory, running, foreground, launcher, and manual-required states look
  distinct.
- Browser console is clean for covered routes.

## Done Signal

Parents can inspect app/game state without confusing evidence types or hidden
capability gaps.

## Completion Notes - 2026-06-03

- Branch: `codex/app-game-read-model-service-events`.
- Proof root:
  `output/app-game-plan-proof/16-parent-portal-app-game-dashboard-surfaces`.
- Source/docs read: feature doc, app/game expectation docs, app-game plan
  README/source index/current snapshot/checklist, native app plan README/source
  index/current snapshot/checklist, this workpack, and the app-plan WP15
  required source docs.
- Implementation: `app-game-dashboard-intent.ts` maps service-backed app-use and
  games read-model rows into dashboard rows/metrics without scanning,
  classifying, timing, or enforcing in the portal.
- UI: App/Game Sessions renders service rows, capability matrix, evidence
  drawer, game-budget gap, and separate inventory/running/foreground/launcher
  counts.
- Product checklist: not updated because app/game control remains in progress;
  unknown approval, policy/game-budget authoring, live source adapters, platform
  authority proof, and broad blocking remain later work.

## Live Code/Test Re-Audit - 2026-08-24

- The dashboard is current production source, not a removed-package promise.
  `apps/portal/src/ParentPortalRoute.tsx` mounts the checked-in vendor surface
  through `apps/portal/src/vendor-parent-portal-surface.js`.
- `activity-ui-intent.ts` builds the dashboard from the service-backed activity
  state, and `ParentPortalSvgSurface.tsx` renders service rows, source freshness,
  capability limitations, evidence, counts, unknown/manual-required states, and
  the App/Game Sessions route.
- `apps/portal/tests/e2e/portal-route-scaffold-product-activity.ts` proves only
  the shallow route headings/count families. The focused intent/state matrix,
  hostile/long-metadata layout, and clean-console coverage required above are
  not checked in.
- The Game budgets tile deliberately reports `policy proof pending` (or no game
  rows). The Rust evaluator exists, but no service budget read-model row feeds
  this surface. Do not replace that honest gap with fixture-only or caller-minted
  data.
- Therefore the production surface source gap is closed, while expected-test
  writing, service-backed budget integration, focused execution, proof, and DONE
  remain open.

Use the standard checklist in [workpacks README](README.md).
