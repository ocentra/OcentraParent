# WP37 Tracking Event Journal Replay And Projection

## Purpose

Prove tracking event chains are recoverable and parent UI read models come from journal/projected service state.

## Central schema boundary

```text
schema-domain owns canonical replayable payload and read-model DTO shapes.
eventing owns generic journal/replay mechanics.
tracking-core consumes canonical contracts for projection behavior.
tracking-domain may provide projection helpers and proof adapters only.
```

## Source Inputs

- `docs/plans/tracking-plan/workpacks/34-tracking-event-contracts-and-protocol-constants.md`
- `docs/plans/tracking-plan/workpacks/36-tracking-detection-cascade-event-flow.md`
- `docs/plans/tracking-plan/workpacks/32-journal-sqlite-and-read-model-proof.md`
- `docs/plans/eventing-plan/05-implementation-workpacks.md`

## Target State

Selected tracking events are journaled and replayed into read models. Replay is projection-only: it must not resend notifications, restart live tracking, or execute runtime commands.

## Required proof fields

```text
canonical_schema_owner_state
journal_payload_state
replay_payload_state
read_model_dto_state
correlation_cursor_state
retention_tombstone_state
corrupt_event_state
missing_event_state
side_effect_replay_state
provider_dispatch_replay_state
runtime_command_replay_state
no_product_ready_claim
no_claim
```

## Required Source Behavior

- Journal selected tracking events.
- Replay tracking events into location, live-mode, notification, escalation, audit, and portal read models.
- Preserve retention delete/export behavior after replay.
- Carry hash/cursor/correlation/audit metadata where the shared eventing journal supports it.
- Make corrupt/missing events visible as degraded/manual-required read-model state instead of disappearing.

## Tests After Code

- Journal stores selected tracking events.
- Replay rebuilds latest tracking state.
- Replay rebuilds notification/live-mode/audit read models.
- Replay does not dispatch notification provider calls.
- Replay does not publish runtime commands.
- Replay does not reapply tracking config commands.
- Corrupt or missing event produces degraded/manual-required state.
- Tombstoned/deleted retention rows stay hidden after replay.
- Retention delete/export rules are preserved after replay.

## Matrix Categories / Target Test Locations

Matrix categories: contract, Rust unit/integration, replay/idempotency, security/AuthZ, service transport, and Playwright/service-backed UI where this workpack touches portal rendering.

Target Rust tests must follow the crate boundary: `crates/agent-protocol/tests` for protocol constants/payloads, `crates/agent-core/tests` for runtime and projection behavior, and `crates/agent-service/tests` for real transport after service seams are importable. Private module tests are only for internal helper invariants.

## Proof After Tests Pass

Proof root:

```text
output/tracking-plan-proof/37-tracking-event-journal-replay-projection/
```

Proof must include source files, tests, commands, journal artifacts, replay artifacts, read-model artifacts, schema owner state, claims proven, claims not proven, and manual-required gaps.
