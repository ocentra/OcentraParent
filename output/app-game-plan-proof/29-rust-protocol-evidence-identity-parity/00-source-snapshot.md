# Source Snapshot

- Branch: `codex/app-plan-evidence-control-continuation`
- Base commit: `8e1de427b8802abe6f3055767ed949128c1a4764`
- Lane: `codex-c`
- Hub lock: `crates/agent-protocol/src/app_game.rs`, `crates/agent-protocol/src/app_game_tests.rs`, `crates/agent-protocol/README.md`, app-game/app-plan docs, workpack docs, and WP29 proof folders.

## Source Files Inspected

- `packages/activity-domain/src/app-game-primitives.ts`
- `packages/activity-domain/src/app-game-identity-primitives.ts`
- `packages/activity-domain/src/app-game.ts`
- `packages/activity-domain/tests/app-game-identity.test.ts`
- `packages/activity-domain/tests/app-game-evidence-claim.test.ts`
- `crates/agent-protocol/src/app_game.rs`
- `crates/agent-protocol/src/app_game_evidence_identity_tests.rs`
- `crates/agent-protocol/src/app_game_tests.rs`
- `crates/agent-protocol/src/lib.rs`
- `docs/features/app-game-control.md`
- `docs/plans/app-game-plan/current-app-game-snapshot.md`
- `docs/plans/app-plan/current-app-snapshot.md`

## Changed Scope

Rust protocol parity was added for existing activity-domain app/game evidence
claim, AI digest reference/classification digest, identity, and identity-merge
shapes. New parity tests live in a dedicated Rust test module so the existing
app-game session/read-model test module stays within source-shape limits. No
service, journal, portal, policy, adapter, or platform behavior was added.
