# 42 - Inference Settings Template Governance Lane

<!-- agent-capsule -->

> Agent Capsule
> Plan: `ai-plan`
> Doc: `42 - Inference Settings Template Governance Lane`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

Inference settings and prompt/template changes are governed, versioned, tested,
and auditable.

## Where We Are

Runtime generation args exist. Prompt/template registry and inference governance
need product rules.

## Checklist

- [ ] Define allowed inference settings per task.
- [ ] Add max token/time/resource guards.
- [ ] Version prompt templates.
- [ ] Record settings in AI result.
- [ ] Add regression fixtures for task prompts.

## Proof

- Settings parser tests.
- Prompt version tests.
- Regression fixture output tests.
