# 34 Service Capture App/Game Live Process Bridge

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `34 Service Capture App/Game Live Process Bridge`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

The existing agent-service activity capture path records bounded live app/game
runtime process rows into the encrypted journal and ActivityStore so existing
app-use/games read models can query service-captured runtime evidence without
claiming foreground, policy, or adapter authority.

## Scope

- Export the core live process app/game journal-event helper with a bounded
  event count for service capture.
- Include app/game runtime journal events in `agent-service` activity capture on
  Windows.
- Prove service capture appends those rows to the encrypted journal and
  ActivityStore.
- Prove the app/game service read model returns a runtime-only running row.
- Preserve opaque executable path refs, unknown-process classification,
  no-foreground state, no-content state, no-policy-consumer state, and no
  adapter execution.

## Tests And Proof

- `cargo test -p ocentra-parent-agent-core app_game_windows_process_source`
- `cargo test -p ocentra-parent-agent-service activity_capture`
- `output/app-game-plan-proof/34-service-capture-app-game-live-process-bridge`

## Done Signal

Service capture can produce a bounded live app/game runtime row in the existing
journal/query-store/read-model path. Recurring service polling or freshness
status, foreground capture, portal UI changes, policy consumption, and adapter
execution remain follow-up work.
