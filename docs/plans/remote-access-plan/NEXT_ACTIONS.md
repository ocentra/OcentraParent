# Next Actions

## Scope and ownership

- Plan owner: `remote-access-plan/AGENTS.md` with security/product coordination and platform runtime dependencies.
- Ownership boundary: remote capability grants, standing access, preview/control split, pairing, and network posture.
- Scope boundary: define runtime proof and threat-model gates before launching implementation.

## Decision routes and failure conditions

- Decision path:
  - If remote capability grant model is unresolved -> stay in execution lane.
  - If screen/view versus deferred control separation is undefined -> split lanes and delay control claims.
  - If abuse/threat proof matrix is incomplete -> block rollout claims.
- Failure modes:
  - Undefined abuse/safe-off states for deferred remote-control attempts.
  - Missing pairing, disclosure, or revoke/remove-device contract for parent/child participants.
  - No explicit degraded/unavailable behavior under relay/session failure.

## Actioned completion tracker

- [ ] Define pairing-based remote capability grants and standing-access lifecycle.
- [ ] Separate live screen view from deferred remote input/control.
- [ ] Define relay fallback, unavailable, and degraded states.
- [ ] Define pairing/disclosure and child-visible state expectations.
- [ ] Define abuse/security proof matrix and route sync.
