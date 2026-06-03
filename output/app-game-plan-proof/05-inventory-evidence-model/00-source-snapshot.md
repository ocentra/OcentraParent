# WP05 Source Snapshot

Workpack: `05-inventory-evidence-model`
Lane: `codex-c`
Branch: `codex/app-game-inventory-evidence-model`
Base head before WP05 edits: `522129f`

## Source Docs Read

- `docs/plans/app-game-plan/workpacks/05-inventory-evidence-model.md`
- `docs/plans/app-game-plan/implementation-checklist.md`
- `docs/plans/app-game-plan/source-index.md`
- `docs/plans/app-game-plan/current-app-game-snapshot.md`
- `docs/features/app-game-control.md`
- `docs/expectations/app-game-evidence.md`
- `packages/activity-domain/README.md`

## Scope Boundary

This workpack adds TypeScript inventory evidence contracts and parser proof only.
It does not add platform inventory adapters, Rust protocol parity, journal or
SQLite ingest, portal inventory rows, runtime use evidence, or product checklist
status movement. Inventory row schemas live in
`packages/activity-domain/src/app-game-inventory.ts` so the aggregate
`app-game.ts` module can keep re-exporting the public contract without growing
past source-shape advisory bands.
