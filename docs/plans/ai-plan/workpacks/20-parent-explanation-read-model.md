# 20 - Parent Explanation Read Model

<!-- agent-capsule -->

> Agent Capsule
> Plan: `ai-plan`
> Doc: `20 - Parent Explanation Read Model`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

Parents can see what was decided, why, what evidence and rules were used, and
what AI/model state was involved.

## Where We Are

Parent assistant and portal runtime details exist. AI explanation needs a
dedicated read model that cites evidence and parent rules without exposing raw
child data unnecessarily.

## Checklist

- [ ] Define explanation read model.
- [ ] Include policy action and AI evidence result.
- [ ] Include evidence refs and parent-rule refs.
- [ ] Include confidence/degraded state.
- [ ] Include model/runtime and prompt refs.
- [ ] Render portal explanation UI.

## Proof

- Explanation contract tests.
- Portal Playwright proof.
- UI screenshots for normal, degraded, and unavailable states.
