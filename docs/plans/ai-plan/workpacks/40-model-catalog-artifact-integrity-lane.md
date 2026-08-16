# 40 - Model Catalog Artifact Integrity Lane

<!-- agent-capsule -->

> Agent Capsule
> Plan: `ai-plan`
> Doc: `40 - Model Catalog Artifact Integrity Lane`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

Local model artifacts have ids, versions, checksums, licenses, source, download
state, install path, cache status, corruption state, and removal behavior.

## Where We Are

Model artifact contracts and runtime cache status exist. Product-grade artifact
integrity still needs proof.

## Checklist

- [ ] Define model catalog entry.
- [ ] Add artifact checksum and version.
- [ ] Add license/source fields.
- [ ] Add download/resume/corruption states.
- [ ] Keep model cache separate from evidence storage.

## Proof

- Artifact parser tests.
- Checksum/corruption tests.
- Cache separation test.
