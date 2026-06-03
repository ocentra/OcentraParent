# WP22 Source Snapshot

Date: 2026-06-03

Scope: app-game WP22, cross-recorded to app-plan WP21.

Files inspected before implementation:

- `docs/features/app-game-control.md`
- `docs/expectations/app-game-evidence.md`
- `docs/expectations/enforcement.md`
- `docs/expectations/policy.md`
- `docs/plans/app-game-plan/README.md`
- `docs/plans/app-game-plan/source-index.md`
- `docs/plans/app-game-plan/current-app-game-snapshot.md`
- `docs/plans/app-game-plan/v0-5-app-game-shared-evidence-spine-plan.md`
- `docs/plans/app-game-plan/v0-5-native-apps-product-slice-plan.md`
- `docs/plans/app-game-plan/v0-5-native-games-product-slice-plan.md`
- `docs/plans/app-game-plan/v0-5-app-game-platform-deep-dive.md`
- `docs/plans/app-game-plan/v0-5-app-game-test-blueprint.md`
- `docs/plans/app-game-plan/ui-ux-requirements-guide.md`
- `docs/plans/app-game-plan/implementation-checklist.md`
- `docs/plans/app-game-plan/workpacks/22-windows-owned-process-terminate-time-limit-proof.md`
- `docs/plans/app-plan/README.md`
- `docs/plans/app-plan/source-index.md`
- `docs/plans/app-plan/current-app-snapshot.md`
- `docs/plans/app-plan/v0-5-native-apps-full-scope-plan.md`
- `docs/plans/app-plan/v0-5-native-apps-platform-deep-dive.md`
- `docs/plans/app-plan/v0-5-native-apps-test-blueprint.md`
- `docs/plans/app-plan/ui-ux-requirements-guide.md`
- `crates/agent-core/src/enforcement_app_time_limit.rs`
- `crates/agent-core/src/enforcement_timer_state.rs`
- `crates/agent-core/src/enforcement_adapter.rs`
- `crates/agent-service/src/enforcement_timer_api.rs`
- `crates/agent-service/src/enforcement_timer_payload.rs`
- `scripts/test/v0-8-windows-app-time-limit-adapter-mvp.mjs`

Boundary found:

- `crates/agent-core` already rechecks PID/name before process termination.
- `crates/agent-service` validates the active timer action before calling the adapter.
- Broad app/game package blocking remains manual-required/no-claim.

Implementation boundary:

- Core/service files were read-only in this pass because codex-a held broad locks.
- The WP22 change extends the real-service proof harness and docs/proof packs only.
