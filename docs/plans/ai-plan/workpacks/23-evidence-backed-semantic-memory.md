# 23 - Evidence-Backed Semantic Memory

<!-- agent-capsule -->

> Agent Capsule
> Plan: `ai-plan`
> Doc: `23 - Evidence-Backed Semantic Memory`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

Semantic memory can group topics, intent, and risk locally, but every derived
fact cites source evidence and remains rebuildable.

## Where We Are

The repo has memory graph proof pieces. Semantic memory needs stronger source,
confidence, invalidation, and rebuild rules before policy can use it.

## Checklist

- [ ] Define semantic memory entry shape.
- [ ] Add source evidence refs.
- [ ] Add embedding/index version.
- [ ] Add confidence and freshness.
- [ ] Add rebuild/invalidation behavior.
- [ ] Keep policy impact gated by source refs.

## Proof

- Semantic memory parser tests.
- Rebuild proof.
- Source guard tests.
