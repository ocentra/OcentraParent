# WP11 Source Snapshot

Branch: `codex/app-game-authority-matrix`

Before this slice, app/game control had approval authority and action-result
contracts, but no dedicated platform/action matrix that named authority tier,
setup state, proof state, and proof needed to move from manual-required to a
claim.

Touched contract paths:

- `packages/parent-domain/src/app-game-control-platform-authority.ts`
- `packages/parent-domain/src/app-game-control-platform-authority-rules.ts`
- `packages/parent-domain/src/app-game-control-authority.ts`
- `packages/parent-domain/tests/app-game-control-platform-authority.test.ts`

No Rust, service, portal, journal, or SQLite source was changed.
