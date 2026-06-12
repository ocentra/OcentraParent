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

## Execution Detail

Minimum context:

- `docs/plans/eventing-plan/AGENTS.md`
- `docs/plans/data-custody-storage-plan/AGENTS.md`
- `docs/plans/app-game-plan/workpacks/33-live-process-journal-sqlite-bridge.md`

Owner boundary:

- This workpack proves local persistence/query bridge only.
- Data custody owns retention/delete/export rules for stored evidence.
- Eventing owns reusable event transport semantics.

Required output:

- Journal/write/read-model boundary.
- Retention and delete handoff note.
- Query proof location.
- Explicit gaps for service events and parent UI.

Expected tests/proof names:

- `app-plan.wp33.journal-ingest`
- `app-plan.wp33.sqlite-query`
- `app-plan.wp33.retention-handoff`
- `app-plan.wp33.no-portal-claim`

Failure conditions:

- Local query proof is used to claim parent report readiness, data custody completion, or policy enforcement.
