# 07 - AI Job Queue Contract

## Target State

AI jobs are bounded, prioritized, cancellable, auditable, source-referenced, and
safe under backpressure.

## Where We Are

Provider scheduler proof exists and now proves child-safety priority,
same-device parent/child sharing, queued/degraded/unavailable provider states,
and one independent runtime access lane per physical device. A broader
cross-slice AI job contract still needs to own task scope, evidence refs, parent
rule refs, provider route, timeout, and result journal refs.

## Checklist

- [ ] Define AI job input contract.
- [x] Define provider scheduler queue state and child-safety priority for the
      local runtime lane.
- [ ] Add timeout, cancellation, retry, and resource class.
- [ ] Require evidence refs and custody labels.
- [ ] Journal queue start/finish/fail states.

## Proof

- Queue parser tests.
- Backpressure/cancel tests.
- Invalid job rejection tests.
