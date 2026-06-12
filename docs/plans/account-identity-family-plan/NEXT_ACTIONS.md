# Next Actions

## Scope and ownership

- Plan owner: `account-identity-family-plan/AGENTS.md` plus auth runtime and household data planes.
- Ownership boundary: identity, household, role, invitation, and ownership-state contracts and the auth-provider decision path.
- Scope boundary: define contract and security posture before implementation or account-flow claims are marked complete.

## Decision routes and failure conditions

- Decision path:
  - If provider boundary is unresolved -> pause implementation and keep plan in research state.
  - If ownership or role assignment model is ambiguous -> keep auth contracts as blocking work.
  - If session/revocation model is undefined -> defer rollout-facing claims.
- Failure modes:
  - Role leakage or cross-family authorization gaps.
  - Missing session expiry/revocation coverage.
  - Invite misuse without explicit expiry/revocation proof.

## Actioned completion tracker

- [ ] Write the auth provider decision record: Cloudflare-only, Firebase Auth as IdP, Auth.js/D1, or staged hybrid.
- [ ] Define user, household, child profile, device, invite, and role ownership shapes.
- [ ] Define session/token lifecycle and revocation requirements.
- [ ] Define cross-family and wrong-role authorization tests.
- [ ] Sync setup, policy, remote, and data-custody handoffs with explicit ownership in each PLAN_STATE.
