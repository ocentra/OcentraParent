# 37 - Tracking Location Safety Analysis Lane

## Target State

Tracking AI explains expected-place, nearby-place ambiguity, stale/offline
state, unusual movement, and parent acknowledgement using typed location
evidence and parent rules.

## Where We Are

Tracking plan exists separately. AI should support explanation and ambiguity
handling, not replace location policy.

## Checklist

- [ ] Consume tracking/location evidence refs.
- [ ] Include expected-place and schedule context.
- [ ] Include stale/offline states.
- [ ] Include nearby-place ambiguity.
- [ ] Return explanation support result.
- [ ] Feed deterministic tracking policy.

## Proof

- Location AI alert support test.
- Stale/offline degrade test.
- Parent acknowledgement explanation proof.
