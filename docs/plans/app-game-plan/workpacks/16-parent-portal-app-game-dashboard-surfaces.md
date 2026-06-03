# 16 Parent Portal App/Game Dashboard Surfaces

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

Use the standard checklist in [workpacks README](README.md).
