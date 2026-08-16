# 13 iOS ReplayKit Adapter Plan Proof

<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-plan`
> Doc: `13 iOS ReplayKit Adapter Plan Proof`
> Kind: proof reference; read only when validating matching claim.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

ReplayKit/broadcast mode, explicit user/session capture, and no arbitrary background other-app capture claim are documented and proved.

## Current State

iOS/iPadOS is treated as explicit ReplayKit session/broadcast-extension work or
not claimed. The current proof verifies Apple ReplayKit source-doc boundaries
and blocks product readiness until physical-device ReplayKit execution and
deletion proof exist.

## Checklist

- [ ] Verify current Apple ReplayKit docs.
- [ ] Define explicit session capture state.
- [ ] Define broadcast extension state if used.
- [ ] Define in-app-only state if used.
- [ ] Record not-claimed state for arbitrary background capture.
- [ ] Prove deletion if any capture is supported.
- [ ] Run physical-device ReplayKit capture or broadcast-extension proof before
      claiming iOS screen capture support.

## Proof

- `output/screen-plan-proof/ios/proof-summary.json` records the source-doc
  readiness and no-overclaim gate.
- Physical iOS ReplayKit proof remains required before product readiness.
