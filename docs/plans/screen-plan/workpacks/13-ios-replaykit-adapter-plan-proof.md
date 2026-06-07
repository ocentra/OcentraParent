# 13 iOS ReplayKit Adapter Plan Proof

## Target State

ReplayKit/broadcast mode, explicit user/session capture, and no arbitrary background other-app capture claim are documented and proved.

## Current State

iOS/iPadOS is treated as explicit ReplayKit session/broadcast-extension work or
not claimed. The current proof verifies Apple ReplayKit source-doc boundaries
and blocks product readiness until physical-device ReplayKit execution and
deletion proof exist.

## Checklist

- [x] Verify current Apple ReplayKit docs.
- [x] Define explicit session capture state.
- [x] Define broadcast extension state if used.
- [x] Define in-app-only state if used.
- [x] Record not-claimed state for arbitrary background capture.
- [ ] Prove deletion if any capture is supported.
- [ ] Run physical-device ReplayKit capture or broadcast-extension proof before
      claiming iOS screen capture support.

## Proof

- `output/screen-plan-proof/ios/proof-summary.json` records the source-doc
  readiness and no-overclaim gate.
- Physical iOS ReplayKit proof remains required before product readiness.
