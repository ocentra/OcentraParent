# 03 - Contract Boundary And Effect Schemas

<!-- agent-capsule -->

> Agent Capsule
> Plan: `ai-plan`
> Doc: `03 - Contract Boundary And Effect Schemas`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

All AI input, output, runtime, queue, route, memory, graph, explanation, and
remote assistant shapes are Effect Schema backed.

## Where We Are

`packages/parent-domain` has local AI, runtime, context, scheduler, memory, and
parent assistant contracts. The contract family needs consolidation and gap
closure before new runtime work.

## Checklist

- [ ] Complete local AI input/result contracts.
- [ ] Complete runtime/provider/queue/route contracts.
- [ ] Complete context-builder request/result contracts.
- [ ] Complete memory and graph reference contracts.
- [ ] Complete AI result journal and explanation contracts.
- [ ] Keep remote assistant contracts separate from child safety.

## Proof

- Focused `packages/parent-domain` tests cover valid/invalid decode.
- Invalid confidence, unsourced memory, missing rule refs, and direct
  enforcement are rejected.
