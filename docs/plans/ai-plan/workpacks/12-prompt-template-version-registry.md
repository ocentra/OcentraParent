# 12 - Prompt Template Version Registry

<!-- agent-capsule -->

> Agent Capsule
> Plan: `ai-plan`
> Doc: `12 - Prompt Template Version Registry`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

Prompts and templates are versioned, minimized, contract-owned, and auditable.

## Where We Are

Prompt/template version is required by expectations. The implementation must
avoid hidden prompt behavior becoming policy.

## Checklist

- [ ] Define prompt/template version contract.
- [ ] Add task-specific prompt ids.
- [ ] Include input minimization rules.
- [ ] Record prompt version in AI result.
- [ ] Add migration/deprecation policy.

## Proof

- Prompt version parser tests.
- Prompt minimization security test.
- Result journal includes prompt/template version.
