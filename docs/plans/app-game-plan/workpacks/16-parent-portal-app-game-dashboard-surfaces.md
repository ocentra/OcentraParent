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

Use the standard checklist in [workpacks README](README.md).
