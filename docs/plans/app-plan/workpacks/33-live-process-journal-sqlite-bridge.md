# 33 Live Process Journal SQLite Bridge

## Target State

The native app side cross-records shared app/game WP33: live process snapshots
can be stored through the local app/game journal and SQLite read-model path
without product-complete service, portal, policy, or adapter claims.

## Scope

- Reuse the shared app/game live process journal bridge.
- Prove current native process evidence reaches the local query store.
- Keep foreground, service subscription, portal freshness, policy, and adapter
  execution as explicit gaps.

## Tests And Proof

- `cargo test -p ocentra-parent-agent-core app_game_windows_process`
- `output/app-plan-proof/33-live-process-journal-sqlite-bridge`

## Done Signal

Native app runtime evidence can be locally journaled and queried from a real
process snapshot, while product status remains in progress until service
events, portal source freshness, foreground capture, policy consumption, and
platform action proof are added.
