# WP04 Source Snapshot

Workpack: `04-app-game-identity-model`
Lane: `codex-c`
Branch: `codex/app-game-identity-contracts`
Base head before WP04 edits: `044e25e`

## Source Docs Read

- `docs/plans/app-game-plan/workpacks/04-app-game-identity-model.md`
- `docs/plans/app-game-plan/v0-5-app-game-test-blueprint.md`
- `docs/plans/app-game-plan/source-index.md`
- `docs/plans/app-game-plan/current-app-game-snapshot.md`
- `packages/activity-domain/README.md`

## Scope Boundary

This workpack adds TypeScript identity contracts and parser proof only. It does
not add Rust protocol parity, runtime identity merging, inventory adapters,
portal identity rows, or product checklist status. Identity-only primitive
exports live in `packages/activity-domain/src/app-game-identity-primitives.ts`
so the existing app/game primitive file stays under the source-shape export
budget.
