# 10 MacOS Capture Adapter Plan Proof

<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-plan`
> Doc: `10 MacOS Capture Adapter Plan Proof`
> Kind: proof reference; read only when validating matching claim.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

ScreenCaptureKit, Screen Recording permission, display/window capture, and PPPC/MDM manual proof requirements are documented and proved before macOS support is claimed.

## Current State

The shared Rust desktop adapter path now targets macOS through `xcap`.
GitHub-hosted macOS CI can prove compile/build behavior, but live capture still
requires a real macOS session with Screen Recording permission evidence before
capture support is claimed.
`ScreenMacosCaptureCapabilityProofSchema` records the current
ScreenCaptureKit/Screen Recording source-doc boundary and keeps macOS product
readiness false until live macOS display/window pixels, permission proof,
deletion proof, and PPPC/MDM deployment review exist.

## Checklist

- [ ] Verify current Apple official capture docs.
- [ ] Add macOS capability probe.
- [ ] Prove Screen Recording permission state.
- [ ] Prove display capture.
- [ ] Prove window/app capture.
- [ ] Record PPPC/MDM manual-required state unless proved.
- [ ] Prove local OCR and deletion.

## Proof

- `output/screen-plan-proof/macos/proof-summary.json`.
- Manual macOS permission/capture proof remains required before product
  readiness.
