# Rollback Proof

Status: no runtime setup was changed.

Rollback for this slice is code-only:

- remove `packages/activity-domain/src/app-game-foreground.ts` and its test;
- remove `AppGameForegroundEvidenceRow` and foreground constants from
  `crates/agent-protocol/src/app_game.rs`;
- remove `app_game_windows_foreground` parser/test registration from
  `crates/agent-core/src/activity_store_app_game.rs`;
- remove the WP09 proof/checklist/doc updates.

No service, OS permission, policy, enforcement, AppLocker/App Control, child UI,
or parent portal state needs cleanup because none was installed or executed.
