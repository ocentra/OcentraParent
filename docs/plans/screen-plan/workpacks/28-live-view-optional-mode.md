# 28 Live View Optional Mode

## Target State

Separate live view mode exists only with transport, relay/cache, viewer audit, retention, platform proof, and remote access boundary.

## Current State

Live view is a separate planned/gap feature and not part of default local summary evidence.
`ScreenEvidenceRemoteBoundarySettingSchema` now records the current default as
`liveViewMode: disabled`, and the retention proof rejects relay/live-view mode
values from the local screen-summary boundary.
`ScreenLiveViewOptInSettingSchema` and
`scripts/test/screen-optional-retention-live-preflight-proof.mjs` now define and
prove the separate optional live-view preflight contract for disabled,
LAN-only, and relay-backed modes. The contract requires explicit parent
approval, viewer audit, platform proof ref, LAN mutual-auth or relay
end-to-end-encrypted transport label, custody label, no frame retention, no
session recording, and no remote input control. This is contract/preflight
proof only; real live transport, relay/cache execution, platform permission
prompts, service persistence, parent UI, and privacy/legal approval remain
separate gates.
Android MediaProjection proof records explicit OS capture consent for the
Android child-agent capture adapter, but it is not live-view permission-prompt
proof and does not close this workpack's live-view runtime gate.
`ScreenLiveViewPlatformPermissionGateSchema` and
`scripts/test/screen-live-view-platform-permission-proof.mjs` now add a
fail-closed platform-permission gate. The proof consumes the existing real
Android MediaProjection capture-consent artifact and records it as
`screen-capture-only`, proving it cannot mark live view product-ready without a
live-view permission prompt proof, viewer audit, live transport proof, no frame
retention, and no remote input. This closes the missing gate-artifact slot but
does not implement or claim real live transport, relay/cache execution, service
live-view sessions, platform screenshots, or privacy/legal approval.
`ScreenLiveViewParentUiPersistenceProofSchema` and
`scripts/test/screen-live-view-parent-ui-persistence-proof.mjs` now prove parent
Settings command/readiness evidence can be carried as persisted live-view
opt-in state into the service-session and Rust runtime decision proofs while
product live view stays false. This closes parent UI persistence as a proof
input only; production worker startup, real live-view prompt screenshots,
relay/cache execution, physical-device parity, and privacy/legal approval remain
open.
`scripts/test/screen-live-view-worker-startup-proof.mjs` now proves the Rust
service worker startup gate exists behind the runtime decision boundary and
stays stopped unless runtime readiness, a real live-view prompt artifact,
relay/cache execution when needed, physical-device parity, and privacy/legal
approval are all proved. This closes the worker-startup gate artifact only;
actual production worker start, real platform prompt screenshots, relay/cache
execution, physical-device parity, and privacy/legal approval remain open.

## Checklist

- [x] Record product decision.
- [x] Define LAN-only vs relay-backed mode.
- [x] Define consent/disclosure.
- [x] Define transport/custody.
- [x] Define retention/no-retention behavior.
- [x] Define viewer audit.
- [x] Add separate contract/preflight proof.
- [x] Add fail-closed platform permission gate proof.
- [x] Add parent UI persistence carry-forward proof.
- [x] Add fail-closed Rust service worker startup gate proof.
- [ ] Add real live-view platform prompt and transport proof.

## Proof

- Updated live-view feature doc.
- Tests proving local-summary opt-in does not enable live view.
- `output/screen-plan-proof/remote-retention-boundary/proof-summary.json`.
- `output/screen-plan-proof/27-28-optional-retention-live-preflight/proof-summary.json`.
- `output/screen-plan-proof/live-view-platform-permission/proof-summary.json`.
- `output/screen-plan-proof/live-view-parent-ui-persistence/proof-summary.json`.
- `output/screen-plan-proof/live-view-worker-startup/proof-summary.json`.
- Capture-adapter platform consent reference:
  `output/screen-plan-proof/android-mediaprojection/proof-summary.json`.
