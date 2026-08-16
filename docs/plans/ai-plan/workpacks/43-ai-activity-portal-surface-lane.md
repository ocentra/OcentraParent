# 43 - AI Activity Portal Surface Lane

<!-- agent-capsule -->

> Agent Capsule
> Plan: `ai-plan`
> Doc: `43 - AI Activity Portal Surface Lane`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

The portal shows AI runtime, jobs, decisions, explanations, memory refs, and
remote boundary status without exposing unnecessary raw child data.

## Where We Are

Portal has local AI runtime details and activity memory graph panels. A coherent
AI activity surface is still needed.

## Checklist

- [ ] Add AI runtime panel.
- [ ] Add AI job/activity list.
- [ ] Add decision explanation panel.
- [ ] Add memory/graph source refs.
- [ ] Add remote boundary status.
- [ ] Add degraded/unavailable states.

## Proof

- Playwright AI runtime status.
- Playwright AI decision explanation.
- UI screenshots for normal/degraded/remote-disabled.
