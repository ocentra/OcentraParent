# Source Snapshot

- Date: 2026-06-03
- Lane: codex-c
- Branch: `codex/app-game-category-risk-taxonomy`
- Rebased head before final WP12 commit: `dfd686f`
- Latest `origin/main` included before final commit: `26e3cdc`
- Latest hub instruction acknowledged:
  `codex-c-msg-20260603T033251038Z-187`
- Hub locks held for this slice:
  `packages/activity-domain/src/app-game.ts`,
  `packages/activity-domain/src/app-game-inventory.ts`,
  `packages/activity-domain/src/app-game-category-risk-primitives.ts`,
  `packages/activity-domain/src/app-game-category-risk.ts`,
  `packages/activity-domain/tests/app-game-category-risk.test.ts`,
  `docs/features/app-game-control.md`,
  app-plan/app-game-plan checklist, source-index, snapshot, workpack files, and
  both WP12/WP11 proof roots.

## Source Files Inspected

- `.ocentra-ai/rules/ocentra-parent-rules.mdc`
- `.ocentra-ai/rules/ocentra-parent-domain-boundaries.mdc`
- `.ocentra-ai/rules/ocentra-parent-source-shape.mdc`
- `.ocentra-ai/rules/ocentra-parent-test-rules.mdc`
- `docs/feature-list.md`
- `docs/features/app-game-control.md`
- `docs/plans/app-game-plan/workpacks/12-app-game-category-and-risk-taxonomy.md`
- `docs/plans/app-plan/workpacks/11-app-category-and-risk-taxonomy.md`
- `packages/activity-domain/README.md`
- `packages/activity-domain/src/app-game-primitives.ts`
- `packages/activity-domain/src/app-game-identity-primitives.ts`
- `packages/activity-domain/src/app-game-inventory-primitives.ts`
- `packages/activity-domain/src/app-game-inventory.ts`
- `packages/activity-domain/src/app-game.ts`
- `packages/activity-domain/tests/app-game-inventory.test.ts`

## Before-State Gap

The existing inventory candidate shape carried coarse app/game category hints,
but there was no first-class taxonomy contract for source/confidence-bearing
native app categories, native game categories, risk candidates, game context
signals, parent display overrides, AI digest refs, and no-direct-enforcement
proof.

## Local Status Note

Pre-existing untracked `.codex/` and `.playwright-cli/` proof artifacts were
preserved and intentionally excluded from this proof pack.
