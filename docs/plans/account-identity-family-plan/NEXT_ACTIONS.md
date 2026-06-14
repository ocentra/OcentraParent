# Next Actions

## Scope And Ownership

- Plan owner: `account-identity-family-plan/AGENTS.md` plus the selected workpack and proof inventory.
- Ownership boundary: identity, household, role, invitation, session, recovery, device authority, and family setup UI contracts.
- Scope boundary: define contract and security posture before implementation or account-flow claims are marked complete.

## Decision Routes And Failure Conditions

- Decision path:
  - If provider boundary is unresolved, pause runtime implementation and keep the provider decision workpack open.
  - If ownership or role assignment is ambiguous, keep auth contracts blocking.
  - If session, recovery, or route-sync proof is missing, defer rollout-facing claims.
- Failure modes:
  - Role leakage or cross-family authorization gaps.
  - Missing session expiry, revocation, or replay coverage.
  - Invite misuse without explicit expiry/revocation proof.
  - UI that implies login equals household trust.

## Action Tracker

- [ ] Lock the auth provider decision record and rejected options.
- [ ] Define user, household, child profile, device, invite, and role ownership shapes.
- [ ] Define session/token lifecycle, freshness, and revocation requirements.
- [ ] Define invite, recovery, deletion, and household-transfer flows.
- [ ] Define cross-family and wrong-role authorization tests.
- [ ] Define the first-run parent family setup UI and source/custody labels.
- [ ] Sync setup, policy, remote, and data-custody handoffs with explicit ownership in each plan state update.
