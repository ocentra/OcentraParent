# 36 - App Game Unknown Classifier Lane

<!-- agent-capsule -->

> Agent Capsule
> Plan: `ai-plan`
> Doc: `36 - App Game Unknown Classifier Lane`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

Unknown apps and games classify from stored app/game evidence, catalog refs,
launcher refs, session summaries, and optional screen summaries.

## Where We Are

App/game evidence and plans exist. AI must not scan processes or infer duration;
it reads agent-generated summaries and typed evidence.

## Checklist

- [ ] Consume app/game evidence refs.
- [ ] Use deterministic catalog first.
- [ ] Include launcher-only and unknown states.
- [ ] Include session duration summaries from agent evidence.
- [ ] Use screen summary only when approved.
- [ ] Return category/risk evidence with confidence.

## Proof

- Unknown app AI dry-run test.
- Unknown game AI dry-run test.
- Duration-not-model-output test.
