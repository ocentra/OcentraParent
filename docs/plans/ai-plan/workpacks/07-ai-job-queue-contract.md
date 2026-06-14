# 07 - AI Job Queue Contract

<!-- agent-capsule -->

> Agent Capsule
> Plan: `ai-plan`
> Doc: `07 - AI Job Queue Contract`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

AI jobs are event-driven, bounded, prioritized, cancellable, auditable,
source-referenced, lease-aware, deduplicated, replayable, and safe under local
and household mesh backpressure.

## Where We Are

Provider scheduler proof exists and now proves child-safety priority,
same-device parent/child sharing, queued/degraded/unavailable provider states,
and one independent runtime access lane per physical device. A broader
cross-slice AI job contract still needs to own task scope, evidence refs, parent
rule refs, provider route, timeout, result journal refs, claim/lease state,
idempotency, and child-agent authority.

## Checklist

- [ ] Define `AiWorkItem` contract.
- [ ] Define `AiWorkState` state machine.
- [ ] Define deterministic `dedupeKey` rules.
- [ ] Define aggregate key rules for ordered work transitions.
- [ ] Define idempotency key rules for duplicate jobs, claims, and results.
- [ ] Define provider scheduler queue state and child-safety priority for the
      local runtime lane.
- [ ] Add timeout, cancellation, retry, TTL, deadline, and max attempts.
- [ ] Define payload mode and custody policy.
- [ ] Require evidence refs, parent-rule refs, and child-agent authority refs.
- [ ] Journal queue, claim, lease, start, complete, fail, validate, accept,
      reject, requeue, and dead-letter states.
- [ ] Prove no direct capture-to-worker call path.

## Proof

- Queue parser tests.
- Backpressure/cancel tests.
- Invalid job rejection tests.
- Duplicate dedupe key tests.
- Replay rebuilds work state without duplicate execution.
- Job expiry/dead-letter tests.
