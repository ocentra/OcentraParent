# 10 MacOS Capture Adapter Plan Proof

## Target State

ScreenCaptureKit, Screen Recording permission, display/window capture, and PPPC/MDM manual proof requirements are documented and proved before macOS support is claimed.

## Current State

The shared Rust desktop adapter path now targets macOS through `xcap`.
GitHub-hosted macOS CI can prove compile/build behavior, but live capture still
requires a real macOS session with Screen Recording permission evidence before
capture support is claimed.

## Checklist

- [ ] Verify current Apple official capture docs.
- [~] Add macOS capability probe.
- [ ] Prove Screen Recording permission state.
- [ ] Prove display capture.
- [ ] Prove window/app capture.
- [ ] Record PPPC/MDM manual-required state unless proved.
- [ ] Prove local OCR and deletion.

## Proof

- `output/screen-plan-proof/macos/`.
- Manual macOS permission/capture proof.
