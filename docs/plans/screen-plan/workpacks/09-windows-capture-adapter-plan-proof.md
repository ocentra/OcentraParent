# 09 Windows Capture Adapter Plan Proof

<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-plan`
> Doc: `09 Windows Capture Adapter Plan Proof`
> Kind: proof reference; read only when validating matching claim.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

Windows.Graphics.Capture path, picker/consent/border, display/window capture, and protected/degraded states are proved before Windows support is claimed.

## Current State

Windows is the preferred first desktop proof target. Implementation proof is open.

## Checklist

- [ ] Verify current Microsoft official capture docs.
- [ ] Add Windows capability probe.
- [ ] Prove display capture.
- [ ] Prove app/window capture.
- [ ] Prove managed browser window capture.
- [ ] Prove protected-surface skip/degraded state.
- [ ] Prove queue write and deletion.

## Proof

- `output/screen-plan-proof/windows/`.
- Local Windows capture logs/screenshots.
