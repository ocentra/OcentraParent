# Source Snapshot

- Branch: `codex/app-plan-evidence-control-continuation`
- Base commit: `8e1de427b8802abe6f3055767ed949128c1a4764`
- Lane: `codex-c`
- Cross-recorded from shared app/game WP29.

## Source Files Inspected

- `packages/activity-domain/src/app-game-primitives.ts`
- `packages/activity-domain/src/app-game-identity-primitives.ts`
- `packages/activity-domain/src/app-game.ts`
- `crates/agent-protocol/src/app_game.rs`
- `crates/agent-protocol/src/app_game_evidence_identity_tests.rs`
- `crates/agent-protocol/src/app_game_tests.rs`
- `crates/agent-protocol/src/lib.rs`
- `docs/plans/app-plan/current-app-snapshot.md`
- `docs/plans/app-plan/implementation-checklist.md`

## Native App Boundary

The native app plan continues to use the shared `AppGame*` evidence spine. No
duplicate native-app-only Rust protocol truth was created. The WP29 Rust tests
were split into a dedicated module to keep the existing app-game protocol test
module within source-shape limits.
