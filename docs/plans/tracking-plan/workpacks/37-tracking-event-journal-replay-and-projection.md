# WP37 Tracking Event Journal Replay And Projection

## Purpose

Prove tracking event chains are recoverable and parent UI read models come from
journal/projected service state.

## Source Inputs

- `docs/plans/tracking-plan/workpacks/34-tracking-event-contracts-and-protocol-constants.md`
- `docs/plans/tracking-plan/workpacks/36-tracking-detection-cascade-event-flow.md`
- `docs/plans/tracking-plan/workpacks/32-journal-sqlite-and-read-model-proof.md`
- `docs/plans/eventing-plan/05-implementation-workpacks.md`

## Target State

Selected tracking events are journaled and replayed into read models. Replay is
projection-only: it must not resend notifications, restart live tracking, or
execute child-agent commands.

## Required Source Behavior

- Journal selected tracking events.
- Replay tracking events into location, live-mode, notification, escalation,
  audit, and portal read models.
- Preserve retention delete/export behavior after replay.
- Carry hash/cursor/correlation/audit metadata where the shared eventing
  journal supports it.
- Make corrupt/missing events visible as degraded/manual-required read-model
  state instead of disappearing.

## Tests After Code

- Journal stores selected tracking events.
- Replay rebuilds latest location/tracking state.
- Replay rebuilds notification/live-mode/audit read models.
- Replay does not dispatch notification provider calls.
- Replay does not publish child-agent live tracking commands.
- Replay does not reapply tracking config commands.
- Corrupt or missing event produces degraded/manual-required state.
- Tombstoned/deleted retention rows stay hidden after replay.
- Retention delete/export rules are preserved after replay.

## Proof After Tests Pass

Proof root:

```text
output/tracking-plan-proof/37-tracking-event-journal-replay-projection/
```

Proof must include source files, tests, commands, journal artifacts, replay
artifacts, read-model artifacts, claims proven, claims not proven, and
manual-required gaps.
