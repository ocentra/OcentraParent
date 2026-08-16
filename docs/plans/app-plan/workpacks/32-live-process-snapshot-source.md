# 32 Live Process Snapshot Source

## Target State

The native app side cross-records shared app/game WP32: the Rust core can read
real local process snapshots into runtime-only native app/game records without
claiming foreground, content, policy, or adapter execution.

## Scope

- Reuse the shared app/game process snapshot source.
- Preserve native-app-safe opaque executable-path refs.
- Keep unknown/native app candidates separate from foreground and content
  evidence.
- Keep journal, service, portal, policy, and adapter work as follow-up gaps.

## Tests And Proof

- `cargo test -p ocentra-parent-agent-core app_game_windows_process`
- `output/app-plan-proof/32-live-process-snapshot-source`

## Done Signal

Native app runtime source proof exists at the core boundary, while product
status remains in progress until service ingestion, parent-visible source
freshness, policy consumption, and platform action proof are added.

## Execution Detail

Minimum context:

- `docs/features/child-agent-local-service.md`
- `docs/features/app-game-control.md`
- `docs/plans/app-game-plan/workpacks/32-live-process-snapshot-source.md`

Owner boundary:

- Core snapshot source proves local process evidence only.
- App-plan owns native runtime/service handoff.
- App-game-plan owns product interpretation, app/game category, and game-specific launcher semantics.

Required output:

- Process snapshot fields allowed for native app use.
- Opaque path/privacy handling.
- Missing bridges: journal, service event, portal freshness, policy, adapter action.

Expected tests/proof names:

- `app-plan.wp32.live-process-source`
- `app-plan.wp32.opaque-path-privacy`
- `app-plan.wp32.no-foreground-claim`
- `app-plan.wp32.no-policy-claim`

Failure conditions:

- Process exists is treated as foreground usage, content knowledge, app category, or enforceable policy.
