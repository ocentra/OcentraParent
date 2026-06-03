# 13 iOS ReplayKit Adapter Plan Proof

## Target State

ReplayKit/broadcast mode, explicit user/session capture, and no arbitrary background other-app capture claim are documented and proved.

## Current State

iOS/iPadOS should be treated as explicit session-based or not claimed.

## Checklist

- [ ] Verify current Apple ReplayKit docs.
- [ ] Define explicit session capture state.
- [ ] Define broadcast extension state if used.
- [ ] Define in-app-only state if used.
- [ ] Record not-claimed state for arbitrary background capture.
- [ ] Prove deletion if any capture is supported.

## Proof

- `output/screen-plan-proof/ios/`.
- Manual/physical iOS proof or not-claimed doc.
