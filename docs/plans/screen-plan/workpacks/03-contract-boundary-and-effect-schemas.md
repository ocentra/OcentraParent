# 03 Contract Boundary And Effect Schemas

<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-plan`
> Doc: `03 Contract Boundary And Effect Schemas`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

Settings, capability, queue, capture job, analysis result, model status, deletion, policy target, and read-model contracts are schema-backed.

## Current State

Partial foundation exists in `packages/activity-domain/src/screen-evidence*.ts` and `crates/agent-protocol/src/screen_evidence.rs`.

## Checklist

- [ ] Reconcile existing schemas with expectation docs.
- [ ] Add missing settings contract.
- [ ] Add missing capability/status contract.
- [ ] Add missing capture queue job contract.
- [ ] Add missing analysis result contract.
- [ ] Add missing deletion/custody contract.
- [ ] Add policy evidence ref contract.
- [ ] Add strict malformed payload tests.

## Proof

- Activity-domain tests.
- Agent-protocol tests.
- No raw app/runtime strings or manual brands.
