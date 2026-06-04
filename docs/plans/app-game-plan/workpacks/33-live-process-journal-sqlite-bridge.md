# 33 Live Process Journal SQLite Bridge

## Target State

Live process snapshot evidence can move from the real local process table into
the existing app/game encrypted journal and SQLite read-model path without
claiming service polling, foreground state, policy decisions, or adapter
execution.

## Scope

- Add bridge helpers from live `sysinfo` process snapshots to app/game runtime
  journal events.
- Prove one current-process event appends to the real encrypted journal.
- Prove the real ActivityStore journal ingest path replays it into the
  app/game service read-model query.
- Preserve runtime-only, unknown-process, no-foreground, no-content, and
  no-adapter boundaries.

## Tests And Proof

- `cargo test -p ocentra-parent-agent-core app_game_windows_process`
- Current-process source emits a runtime journal event.
- Encrypted journal append and SQLite ingest store exactly one event.
- Read model reports the current process in `running_now_rows` and no
  foreground rows.

## Done Signal

The live process source has a proof-backed route into local journal/query-store
state. Service scheduling/events, portal source freshness, foreground capture,
policy consumption, and adapter execution remain follow-up work.
