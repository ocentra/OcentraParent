# 44 - Provider API Authorization Custody Lane

## Target State

External/API providers are parent-authorized, custody-labeled, retention-labeled,
and unavailable for normal child safety.

## Where We Are

API AI provider authorization proof exists. The route must remain separate from
child-device safety and visible in UI.

## Checklist

- [ ] Define API provider authorization state.
- [ ] Require parent action ref.
- [ ] Require data custody and retention state.
- [ ] Reject safety-path remote provider use.
- [ ] Add provider unavailable/degraded states.

## Proof

- API authorization proof.
- Remote disabled-by-default test.
- Custody label guard test.
