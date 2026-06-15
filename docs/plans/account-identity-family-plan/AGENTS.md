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
Context: the product needs a parent account, household, child profile, device membership, roles, invites, recovery, setup UI, and safe session/token lifecycle. The architecture is Cloudflare-first for app/data custody, with Firebase Auth allowed only as an external identity provider/token issuer if the decision record keeps family data out of the IdP.
Scope: identity model, provider decision, login/session lifecycle, role authorization, household/device ownership, invite/recovery, parent family setup UI, and abuse/security proof.
Out of scope: public site content, installer/package build mechanics, child data storage, policy authoring, and remote transport.

## High-Density Execution Contract

- Route first from `PLAN_STATE.md`; treat this plan as the single source for account-family authority and do not read sibling plans unless the assignment names a handoff path.
- Work only the selected workpack plus the exact checklist/proof rows named by `CHECKLIST_INDEX.md` and `PROOF_AND_TEST_INVENTORY.md`.
- Every accepted action must include: decision owner, boundary, concrete acceptance evidence path, failure condition, and remaining-risk note.
- Stop condition: do not claim DONE/PR_READY without proof rows updated for session, household, device, and UI boundaries, and with unresolved risk either cleared or explicitly deferred with a valid reason.

## Decision And Proof Gate

- This plan is execution-grade on paper, but it is not product-ready until the workpack proofs and route sync gates are present.
- Before implementation claims, confirm the provider decision, identity authority model, session/token model, invite/recovery model, device authority matrix, and UI expectations all agree with the current repo contracts.
- Keep family data in Cloudflare-owned custody domains by default; Firebase Auth, if used, stays adapter-only as an external IdP/token issuer.

## Decision Tree

| If the task is about...                           | Open                                            |
| ------------------------------------------------- | ----------------------------------------------- |
| Cloudflare vs Firebase/Auth.js/provider choice    | `workpacks/01-auth-provider-decision.md`        |
| Users, households, roles, child profiles, devices | `workpacks/02-identity-household-role-model.md` |
| Sessions, tokens, refresh, revocation, replay     | `workpacks/03-session-token-lifecycle.md`       |
| Invites, recovery, co-parent, transfer, deletion  | `workpacks/04-invites-recovery-lifecycle.md`    |
| Cross-family authZ and device ownership           | `workpacks/05-device-ownership-authz.md`        |
| Proof, threat model, route gate                   | `workpacks/06-security-proof-and-route-gate.md` |
| Parent account and family setup UI               | `workpacks/07-parent-account-family-setup-ui.md` |

## Ownership Boundaries

- `setup-install-provisioning-plan` owns the installer/download route and public release mechanics that hand off into account flows.
- `data-custody-storage-plan` owns storage, export, delete, encryption, and parent-owned sync.
- `policy-control-plane-plan` owns policy authority after authenticated household context exists.
- `remote-access-plan` owns remote session grants after identity/device authority exists.
- `device-trust-bootstrap-plan` owns trusted-device bootstrap, parent step-up proof, and local sealed trust boundaries after identity exists.

## State

- State remains proof-open until provider decision, auth/session proof, invite/recovery proof, device authZ proof, setup UI proof, and cross-plan handoff proof are explicitly synchronized.
- Do not move to DONE/PR_READY unless workpack completion, proof artifacts, and route/index sync are explicitly synchronized.

## Failure Conditions

- Do not store product data in Firebase or any third-party identity system by default.
- Do not treat authentication as authorization; every action needs household, role, and device authority.
- Do not allow child profile creation to imply a real child device is trusted.
- Do not claim secure login without replay, expiry, revocation, rate-limit, abuse, and recovery proof.
