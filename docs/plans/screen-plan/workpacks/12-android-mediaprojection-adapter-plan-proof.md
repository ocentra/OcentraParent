# 12 Android MediaProjection Adapter Plan Proof

<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-plan`
> Doc: `12 Android MediaProjection Adapter Plan Proof`
> Kind: proof reference; read only when validating matching claim.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

Consent, foreground service, app-window/full-display, status indicator, stop callback, and Android version constraints are proved before Android capture is claimed.

## Current State

Android capture is possible only through consent/session-heavy platform behavior. Silent background capture is not a claim.
The current branch has emulator MediaProjection proof with explicit consent,
foreground-service capture, frame digest, and raw-temp deletion. The Android
readiness gate consumes that proof and blocks physical-device product readiness
until physical-device capture/deletion proof exists.
The readiness contract now also defines stop-callback-on-user-stop behavior for
MediaProjection rows and rejects capture-ready rows that drop the callback
requirement. This is a contract/proof gate, not physical-device callback runtime
execution.

## Checklist

- [ ] Verify current Android MediaProjection docs.
- [ ] Define consent/session state.
- [ ] Define foreground service requirements.
- [ ] Define app-window/full-display availability.
- [ ] Define stop callback behavior.
- [ ] Prove no silent background capture.
- [ ] Prove emulator capture deletion.
- [ ] Prove physical-device capture deletion.
- [ ] Prove local OCR on physical-device Android capture.

## Proof

- `output/screen-plan-proof/android-mediaprojection/proof-summary.json` records
  the existing emulator MediaProjection proof.
- `output/screen-plan-proof/android/proof-summary.json` records the
  source-doc/readiness gate, stop-callback requirement, and physical-device
  non-claim.
- Physical/manual Android proof remains required before product readiness.
