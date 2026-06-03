# 07 - AI Job Queue Contract

## Target State

AI jobs are bounded, prioritized, cancellable, auditable, source-referenced, and
safe under backpressure.

## Where We Are

Provider scheduler proof exists, but a cross-slice AI job contract needs to own
task scope, evidence refs, parent rule refs, provider route, timeout, and result
journal refs.

## Checklist

- [ ] Define AI job input contract.
- [ ] Define queue state and priority.
- [ ] Add timeout, cancellation, retry, and resource class.
- [ ] Require evidence refs and custody labels.
- [ ] Journal queue start/finish/fail states.

## Proof

- Queue parser tests.
- Backpressure/cancel tests.
- Invalid job rejection tests.
