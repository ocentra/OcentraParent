# Next Actions

## Scope and ownership

- Plan owner: `remote-access-plan/AGENTS.md` with security/product coordination and platform runtime dependencies.
- Ownership boundary: remote capability grants, control/control-vs-preview split, session consent, and network posture.
- Scope boundary: define runtime proof and threat-model gates before launching implementation.

## Decision routes and failure conditions

- Decision path:
  - If remote capability grant model is unresolved -> stay in research/open decision lane.
  - If screen/control separation is undefined -> split lanes and delay control claims.
  - If abuse/threat proof matrix is incomplete -> block rollout claims.
- Failure modes:
  - Undefined abuse/safe-off states for remote-control attempts.
  - Missing consent or disclosure contract for parent/child participants.
  - No explicit degraded/unavailable behavior under relay/session failure.

## Actioned completion tracker

- [ ] Define remote capability grants and session lifecycle.
- [ ] Separate live screen view from remote input/control.
- [ ] Define relay fallback, unavailable, and degraded states.
- [ ] Define consent/disclosure and child-visible state expectations.
- [ ] Define abuse/security proof matrix and route sync.
