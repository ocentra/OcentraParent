# 12 Android MediaProjection Adapter Plan Proof

## Target State

Consent, foreground service, app-window/full-display, status indicator, stop callback, and Android version constraints are proved before Android capture is claimed.

## Current State

Android capture is possible only through consent/session-heavy platform behavior. Silent background capture is not a claim.
The current branch has emulator MediaProjection proof with explicit consent,
foreground-service capture, frame digest, and raw-temp deletion. The Android
readiness gate consumes that proof and blocks physical-device product readiness
until physical-device capture/deletion proof exists.

## Checklist

- [x] Verify current Android MediaProjection docs.
- [x] Define consent/session state.
- [x] Define foreground service requirements.
- [x] Define app-window/full-display availability.
- [ ] Define stop callback behavior.
- [x] Prove no silent background capture.
- [x] Prove emulator capture deletion.
- [ ] Prove physical-device capture deletion.
- [ ] Prove local OCR on physical-device Android capture.

## Proof

- `output/screen-plan-proof/android-mediaprojection/proof-summary.json` records
  the existing emulator MediaProjection proof.
- `output/screen-plan-proof/android/proof-summary.json` records the
  source-doc/readiness gate and physical-device non-claim.
- Physical/manual Android proof remains required before product readiness.
