# 25 - Minimal Graph Edges For Safety Context

<!-- agent-capsule -->

> Agent Capsule
> Plan: `ai-plan`
> Doc: `25 - Minimal Graph Edges For Safety Context`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

The first graph supports safety context and explanation only. It does not become
a broad autonomous memory system.

## Where We Are

Graph modules exist, but the minimal product graph needs explicit edge limits
and source-citation tests.

## Checklist

- [ ] Add child/device/evidence edges.
- [ ] Add evidence/app/site/category candidate edges.
- [ ] Add policy rule/target edges.
- [ ] Add AI result/evidence edges.
- [ ] Add policy decision/rule/result edges.
- [ ] Add parent action/decision edges.

## Proof

- Minimal graph construction tests.
- Graph edge source refs.
- Context builder uses only allowed minimal edges.
