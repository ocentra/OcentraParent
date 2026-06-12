# 05 Capability Status Contract

<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-plan`
> Doc: `05 Capability Status Contract`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

Disabled, unsupported, permission-required, protected-surface, screen-locked, model-unavailable, queue-unavailable, degraded, and ready states are represented.

## Current State

Capability direction exists in docs and partial contracts, but visible product proof is open.

## Checklist

- [ ] Define platform enum.
- [ ] Define status enum.
- [ ] Define capture scopes available/unavailable.
- [ ] Define degraded reasons.
- [ ] Define model status.
- [ ] Define queue status.
- [ ] Define proof tier and proof refs.
- [ ] Surface state in portal.

## Proof

- Contract tests.
- Service read-model proof.
- Portal screenshots for unavailable and ready states.
