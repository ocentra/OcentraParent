# 29 - TabAgent Memory Graph Reuse Candidate

<!-- agent-capsule -->

> Agent Capsule
> Plan: `ai-plan`
> Doc: `29 - TabAgent Memory Graph Reuse Candidate`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

TabAgent memory/graph ideas are translated into Ocentra source-cited local memory
and graph references.

## Where We Are

TabAgent has IndexedDB and Rust knowledge graph files. Ocentra has activity
memory graph modules and local AI memory contracts.

## Checklist

- [ ] Compare graph node/edge models.
- [ ] Keep evidence journal as source truth.
- [ ] Add source refs to every derived graph item.
- [ ] Add expiry/invalidation.
- [ ] Add rebuild from Ocentra journal/read models.

## Proof

- Unsourced graph rejected.
- Rebuild proof.
- Parent explanation cites graph source refs.
