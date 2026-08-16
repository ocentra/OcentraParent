# 14 Protected Surface Detector

<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-plan`
> Doc: `14 Protected Surface Detector`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

Lock screen, secure desktop, credential prompt, password field, DRM/protected media, OS-protected surface, and unsupported states are skipped or redacted.

## Current State

Expectation docs prohibit sensitive capture. Runtime proof is open.

## Checklist

- [ ] Define protected surface categories.
- [ ] Define skip result state.
- [ ] Define redaction result state.
- [ ] Add capture-side guard where possible.
- [ ] Add OCR-side redaction where needed.
- [ ] Add portal labels for skipped/redacted evidence.

## Proof

- Security tests.
- Manual protected-surface proof.
