# 12 Android MediaProjection Adapter Plan Proof

## Target State

Consent, foreground service, app-window/full-display, status indicator, stop callback, and Android version constraints are proved before Android capture is claimed.

## Current State

Android capture is possible only through consent/session-heavy platform behavior. Silent background capture is not a claim.

## Checklist

- [ ] Verify current Android MediaProjection docs.
- [ ] Define consent/session state.
- [ ] Define foreground service requirements.
- [ ] Define app-window/full-display availability.
- [ ] Define stop callback behavior.
- [ ] Prove no silent background capture.
- [ ] Prove local OCR and deletion.

## Proof

- `output/screen-plan-proof/android/`.
- Physical/manual Android proof.
