<!-- agent-capsule -->

> Agent Capsule
> Plan: `account-identity-family-plan`
> Doc: AGENTS
> Kind: plan route and local agent contract.
> Read when: First file inside this plan after a global route selects it.
> Stop rule: Choose one workpack; do not inspect setup, portal, or data plans unless named.
> Proves: local routing and ownership only.
> Does not prove: auth implementation, production security, or PR readiness.

<!-- /agent-capsule -->

# Account Identity Family Plan Agent Route

Task: define the account and household authority system.
Context: the product needs a parent account, household, child profile, device membership, roles, invites, and safe session/token lifecycle. The architecture should be Cloudflare-first for app/data custody, with Firebase Auth considered only as an identity provider/token issuer if chosen.
Scope: identity model, provider decision, login/session lifecycle, role authorization, household/device ownership, invite/recovery, and abuse/security proof.
Out of scope: public site content, installer UX, package build, child data storage, policy authoring, and remote transport.

## High-Density Execution Contract

- Route first from `PLAN_STATE.md`; treat this plan as the single source for account-family authority and no read of sibling plans unless the assignment names a handoff path.
- Work only the selected workpack plus required checklist/proof rows; avoid broad historical docs and full checklist scans.
- Every accepted action must include: decision owner, boundary, concrete acceptance evidence path, failure condition, and remaining-risk note.
- Stop condition: do not claim DONE/PR_READY without proof rows updated for session/household/domain boundaries and unresolved risk list cleared or explicitly deferred with a valid reason.

## Research Gate

This plan is first-pass. Before implementation, DONE, or PR_READY, the assigned agent must inspect existing repo code/docs and the games-project Cloudflare/Firebase pattern for the touched slice, map what already exists versus missing, and discuss unresolved provider/security/product choices with Sujan. Do not treat this first-pass plan as final architecture.

## Decision Tree

| If the task is about...                           | Open                                            |
| ------------------------------------------------- | ----------------------------------------------- |
| Cloudflare vs Firebase/Auth.js/provider choice    | `workpacks/01-auth-provider-decision.md`        |
| Users, households, roles, child profiles, devices | `workpacks/02-identity-household-role-model.md` |
| Sessions, tokens, refresh, revocation, replay     | `workpacks/03-session-token-lifecycle.md`       |
| Invites, recovery, co-parent, transfer, deletion  | `workpacks/04-invites-recovery-lifecycle.md`    |
| Cross-family authZ and device ownership           | `workpacks/05-device-ownership-authz.md`        |
| Proof, threat model, rollout gate                 | `workpacks/06-security-proof-and-route-gate.md` |

## Ownership Boundaries

- `setup-install-provisioning-plan` owns the visible setup journey and routes users into account flows.
- `data-custody-storage-plan` owns storage, export, delete, encryption, and parent-owned sync.
- `policy-control-plane-plan` owns policy authority after authenticated household context exists.
- `remote-access-plan` owns remote session grants after identity/device authority exists.

## State

- State remains first-pass research and route-gate until provider decision, auth/session proof, and abuse controls are completed and mirrored in `PLAN_STATE.md`.
- Do not move to DONE/PR_READY unless workpack completion, proof artifacts, and cross-plan handoff proof are explicitly synchronized.

## Failure Conditions

- Do not store product data in Firebase/third-party identity by default.
- Do not treat authentication as authorization; every action needs household, role, and device authority.
- Do not allow child profile creation to imply a real child device is trusted.
- Do not claim secure login without replay, expiry, revocation, rate-limit, abuse, and recovery proof.
