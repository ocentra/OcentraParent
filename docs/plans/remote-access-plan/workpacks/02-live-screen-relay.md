# Workpack 02: Live Screen Relay

Goal: define remote live screen viewing without confusing it with local screenshot capture.

Expected shape:

- Screen-plan provides capture primitives and protected-surface rules.
- Remote access plan owns relay session, standing paired viewing authority, unavailable/degraded states, and remote artifact custody boundary.
- Raw frames are not retained unless an explicit screenshot/recording mode is authorized by data custody and screen settings.

Expected proof:

- Paired-view and protected-surface proof.
- Relay unavailable and reconnect proof.
- No unintended retention proof.
- Parent UI and child disclosure handoff proof.

Failure: remote live view claimed from local screenshot summary proof.

## Execution Detail

Minimum context:

- `docs/features/screen-visibility-live-view.md`
- `docs/plans/screen-plan/workpacks/28-live-view-optional-mode.md`
- `docs/plans/screen-plan/workpacks/39-redacted-summary-only-remote-boundary.md`
- `docs/plans/data-custody-storage-plan/AGENTS.md`

Required model:

- Capture source.
- Relay route.
- Standing paired view session.
- Child-visible state.
- Retention mode: no raw retention, screenshot opt-in, recording disabled unless separately approved.
- Protected-surface behavior.

Agent decision tree:

- If the work changes capture permission/custody, route to `screen-plan`.
- If it changes relay/session authorization, stay here.
- If it stores frames, route to `data-custody-storage-plan` before claiming privacy safety.

Expected tests/proof names:

- `live-screen.paired-view`
- `live-screen.view-only`
- `live-screen.protected-surface-blocked`
- `live-screen.relay-unavailable-degraded`
- `live-screen.no-raw-retention-default`
- `live-screen.child-disclosure-visible`

Proof artifact expectations:

- Paired-view proof.
- Relay reconnect/unavailable proof.
- Screenshot or UI artifact for active/degraded/stopped.
- Retention proof note.
