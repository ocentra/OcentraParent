# 06 Capture Scope Model

<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-plan`
> Doc: `06 Capture Scope Model`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

Full screen, display, active window, selected app window, managed browser/window, Android app-window, and unsupported scope are modeled.

## Current State

Scope appears in architecture direction. Implementation proof is open.

## Checklist

- [ ] Define capture scope contract.
- [ ] Platform-gate each scope.
- [ ] Add unsupported scope state.
- [ ] Link selected app/window scope to source refs.
- [ ] Link managed browser/window scope to browser evidence refs.
- [ ] Add portal labels.

## Proof

- Contract tests.
- Platform capability matrix.
- Portal state screenshots.
