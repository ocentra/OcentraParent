# Rollback Proof

Rollback is code-only for this workpack:

- revert `crates/agent-core/src/activity_store_app_game/app_game_sessionization.rs`;
- revert `crates/agent-core/src/activity_store_app_game/app_game_sessionization_tests.rs`;
- revert the `ActivityStore::app_game_session_report` delegation change;
- revert new protocol fields/constants in `crates/agent-protocol/src/app_game.rs`;
- revert TypeScript session summary and daily rollup schema additions;
- remove the direct `chrono` dependency from `crates/agent-core/Cargo.toml`;
- rerun the focused activity-domain, agent-protocol, and agent-core tests.

No runtime migration, device setup, OS policy, block, suspend, shield, or
allowlist cleanup is needed because this workpack does not execute enforcement
or write platform policy.
