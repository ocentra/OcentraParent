# 28 Live View Optional Mode

## Target State

Separate live view mode exists only with transport, relay/cache, viewer audit, retention, platform proof, and remote access boundary.

## Current State

Live view is a separate planned/gap feature and not part of default local summary evidence.
`ScreenEvidenceRemoteBoundarySettingSchema` now records the current default as
`liveViewMode: disabled`, and the retention proof rejects relay/live-view mode
values from the local screen-summary boundary.

## Checklist

- [x] Record product decision.
- [ ] Define LAN-only vs relay-backed mode.
- [ ] Define consent/disclosure.
- [ ] Define transport/custody.
- [ ] Define retention/no-retention behavior.
- [ ] Define viewer audit.
- [ ] Add separate platform proof.

## Proof

- Updated live-view feature doc.
- Tests proving local-summary opt-in does not enable live view.
- `output/screen-plan-proof/remote-retention-boundary/proof-summary.json`.
