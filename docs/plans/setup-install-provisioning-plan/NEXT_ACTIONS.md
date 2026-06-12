<!-- agent-capsule -->

> Agent Capsule
> Plan: `setup-install-provisioning-plan`
> Doc: NEXT_ACTIONS
> Kind: short resume list.
> Read when: After PLAN_STATE.
> Stop rule: Choose one action; do not open all workpacks.

<!-- /agent-capsule -->

# Next Actions

## Scope and ownership

- Plan owner: `setup-install-provisioning-plan/AGENTS.md` with account/portal onboarding and install-readiness lanes.
- Ownership boundary: family website boundary, register/login handoff, install/setup proof states, and deployment surface constraints.
- Scope boundary: no production account flow changes before contract proof and route/index sync are completed.

## Decision routes and failure conditions

- Decision path:
  - If family landing surface privacy boundary remains ambiguous -> keep this plan in research lane.
  - If install/readiness states are undefined -> block rollout planning and keep plan claims at "first-pass".
  - If handoff to account-identity plan is incomplete -> reject first implementation draft.
- Failure modes:
  - Data collection over-collection beyond declared non-activity scope.
  - Missing recovery path for account pairing failure and installer fallback.
  - Undefined transition states for stale/blocked install conditions.

## Actioned completion tracker

- [ ] Define `family.ocentra.ca` as an informational Vite/Cloudflare surface with explicit no-child-data collection boundary.
- [ ] Define register/login entry as a handoff to `account-identity-family-plan`, not an ad hoc website form.
- [ ] Define parent/child install journey readiness states and degraded recovery labels.
- [ ] Define setup proof expectations: screenshots, logs, installer artifact refs, pairing state refs, and negative cases.
- [ ] Update route/index docs when workpacks become active.
