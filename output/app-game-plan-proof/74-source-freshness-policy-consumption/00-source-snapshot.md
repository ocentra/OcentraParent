# Source Snapshot

Branch: codex/app-game-source-freshness-policy-consumption-v2
Commit: bcccf90bdc882117e30fc810a88ac9f6e642c17f

Inspected source inputs:

- `packages/activity-domain/src/activity-surface.ts` for `sourceStatusRows` shape.
- `packages/parent-domain/src/app-game-policy-target-compiler.ts` for existing policy compiler boundaries.
- `docs/features/app-game-control.md` and app/app-game plan WP47/WP72 docs for remaining source freshness gaps.

Touched implementation:

- `packages/parent-domain/src/app-game-source-freshness-policy-consumption*.ts`
- `packages/parent-domain/tests/app-game-source-freshness-policy-consumption.test.ts`
- `scripts/test/app-game-source-freshness-policy-consumption-proof.mjs`
