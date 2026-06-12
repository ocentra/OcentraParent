# 23 Policy Compiler For Screen Derived Evidence

<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-plan`
> Doc: `23 Policy Compiler For Screen Derived Evidence`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

Visible category/risk targets compile only from validated summaries and parent rules.

## Current State

Screen policy catalog inputs exist. Summary-to-policy proof is open.

## Checklist

- [ ] Define screen evidence policy targets.
- [ ] Compile category/risk rules.
- [ ] Include confidence threshold.
- [ ] Include unknown/low-confidence behavior.
- [ ] Reject raw image/raw AI text inputs.
- [ ] Add dry-run proof.

## Proof

- Policy compiler tests.
- Dry-run output proof.
