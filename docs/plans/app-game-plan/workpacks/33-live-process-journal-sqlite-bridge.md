# 33 Live Process Journal SQLite Bridge

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `33 Live Process Journal SQLite Bridge`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

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
