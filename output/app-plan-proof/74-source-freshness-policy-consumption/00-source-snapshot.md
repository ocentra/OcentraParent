# Source Snapshot

Branch: codex/app-game-source-freshness-policy-consumption-v2
Commit: 4b3d5474dd78bfd6a13425f7b9f43b86e75e426b

Inspected source inputs:

- `packages/activity-domain/src/activity-surface.ts` for `sourceStatusRows` shape.
- `packages/parent-domain/src/app-game-policy-target-compiler.ts` for existing policy compiler boundaries.
- `docs/features/app-game-control.md` and app/app-game plan WP47/WP72 docs for remaining source freshness gaps.

Touched implementation:

- `packages/parent-domain/src/app-game-source-freshness-policy-consumption*.ts`
- `packages/parent-domain/tests/app-game-source-freshness-policy-consumption.test.ts`
- `scripts/test/app-game-source-freshness-policy-consumption-proof.mjs`
