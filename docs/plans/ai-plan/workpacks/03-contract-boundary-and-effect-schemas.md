# 03 - Contract Boundary And Effect Schemas

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
