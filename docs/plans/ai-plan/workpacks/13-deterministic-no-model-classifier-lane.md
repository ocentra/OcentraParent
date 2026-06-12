# 13 - Deterministic No-Model Classifier Lane

<!-- agent-capsule -->

> Agent Capsule
> Plan: `ai-plan`
> Doc: `13 - Deterministic No-Model Classifier Lane`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

Known structured signals classify without a model before local AI jobs spend
resources.

## Where We Are

Browser, app/game, tracking, policy, and LAN slices already have structured
evidence opportunities. They need a shared deterministic lane before LLM/OCR/VLM.

## Checklist

- [ ] Use URL/domain/platform parsers.
- [ ] Use known app/game catalogs.
- [ ] Use parent rules and schedules.
- [ ] Use local metadata and category maps.
- [ ] Return typed classifier evidence with source refs.
- [ ] Escalate only ambiguous cases to model route.

## Proof

- Deterministic classifier tests.
- No-model path proof.
- Ambiguous path routes to AI queue safely.
