# 24 - Knowledge Graph Reference Contract

<!-- agent-capsule -->

> Agent Capsule
> Plan: `ai-plan`
> Doc: `24 - Knowledge Graph Reference Contract`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

Knowledge graph refs are typed, local, source-cited, and safe for AI context and
parent explanation.

## Where We Are

`crates/agent-core` has activity memory graph modules. TabAgent has graph
reference ideas. Ocentra needs its own graph contract and safety gate.

## Checklist

- [ ] Define graph entity refs.
- [ ] Define graph edge refs.
- [ ] Require source evidence/policy/action refs.
- [ ] Include confidence, generated time, and index version.
- [ ] Reject unsourced graph refs for decisioning.

## Proof

- Graph reference tests.
- Unsourced edge rejection tests.
- Explanation cites graph source refs.
