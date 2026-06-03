# 24 - Knowledge Graph Reference Contract

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
