<!-- agent-capsule -->

> Agent Capsule
> Plan: `account-identity-family-plan`
> Doc: `Account Identity Family Plan Agent Route`
> Kind: plan route and local agent contract.
> Read when: first file inside this plan after a global route selects it.
> Stop rule: choose one workpack; do not inspect setup, portal, data, policy, payment, Cloudflare, device-trust, LAN, or remote plans unless the selected workpack names a handoff.
> Proves: local routing and ownership only.
> Does not prove: auth implementation, production security, payment readiness, device trust, family setup readiness, or PR readiness.
> Proof rule: if this file changes status or claims, update the assigned workpack, checklist row, proof route, and PLAN_STATE.

<!-- /agent-capsule -->

# Account Identity Family Plan Agent Route

## Mission

This plan defines the account, household, role, invite, recovery, session, and device-authority foundation that other plans consume.

The core rule:

```text
authentication proves who the actor is
household membership proves where the actor belongs
role proves what the actor may do
device trust proves which physical/local endpoint may act
session freshness proves whether the current request is still allowed
```

Do not collapse those boundaries.

## Scope

This plan owns:

```text
provider decision and identity adapter boundary
user/account identity references
household membership and role model
child profile identity, separate from child device trust
session, token, refresh, logout, revoke, replay, and freshness model
invites, recovery, transfer, deletion handoff, and co-parent/observer lifecycle
device ownership authorization matrix
first-run account/family setup UI contracts and states
account/family security proof gate
```

Out of scope:

```text
public marketing/download site content -> setup-install-provisioning-plan
installer/package mechanics -> setup-install-provisioning-plan
shared Cloudflare worker scaffold/auth middleware -> cloudflare-control-plane-plan
billing semantics and entitlement math -> payment-subscription-plan
policy rule authoring/evaluation -> policy-control-plane-plan
child-device local trust bootstrap/key sealing -> device-trust-bootstrap-plan
child activity data storage/export/delete -> data-custody-storage-plan
LAN transport and remote transport -> lan-plan / remote-access-plan
```

## Ownership, Import, And Boundary Contract

The account/family authority surface is shared by many features, but this plan must not become a hidden cross-feature runtime dependency.

Module roles:

```text
crates/schema or the owning Rust crate: canonical shared account/family schema, brand, parser, route/action/read-model DTO, and encoded-shape owner when account/family shapes cross package, crate, app, or plan boundaries.
schema-domain: temporary generated-validation or edge-decoder surface only where TypeScript still needs one during migration.
family-domain: TypeScript helper/projection package that consumes Rust-owned/generated account/family contracts and exposes only approved account/family helper surfaces.
family-identity-core: Rust parity/runtime authority boundary for account, household, role, device, session, and invite/recovery primitives.
setup-domain and provisioning-core: setup/provisioning consumers; they do not own account/family authority.
portal-domain and apps/portal: parent-visible projections/renderers; they do not own account runtime, household truth, device trust, or child activity state.
Cloudflare control-plane runtime: account-family persistence/adapter implementation target after provider/schema decisions; it must not move family truth into an IdP.
payment, policy, data-custody, device-trust, LAN, and remote plans: handoff consumers only; they must not re-own or duplicate account/family authority.
```

Direct imports are allowed only for neutral/shared infrastructure or explicit public authority contracts:

```text
Rust-owned canonical shapes, generated DTOs, temporary edge decoders, brands, and literals
neutral protocol/event/evidence/logging/capability primitives
approved public family-domain helper exports for account/family authority consumption
approved Rust parity crates when the selected workpack names Rust parity proof
pure common helpers that do not own feature behavior or side effects
```

Forbidden direct imports:

```text
sibling feature owner packages or crates for runtime behavior
private source files from another plan's owning package/crate
peer feature contracts when the shape should live in crates/schema or another neutral Rust-owned boundary
setup, payment, policy, data-custody, device-trust, LAN, remote, or portal internals to satisfy account/family authority
```

If two feature owners need the same shape, promote or consume it through the neutral shared schema/protocol/evidence/event/read-model boundary. If behavior crosses ownership, use a typed command, event, request, read model, or proof handoff. Do not solve cross-plan behavior by importing another feature's runtime internals.

## Required read order

1. `PLAN_STATE.md`
2. `NEXT_ACTIONS.md`
3. `WORKPACK_INDEX.md`
4. one selected workpack under `workpacks/`
5. `CHECKLIST_INDEX.md` only for that workpack rows
6. `TEST_PROOF_EXPECTATIONS.md` only for that workpack command/proof set
7. `PROOF_INDEX.md` only when writing or validating proof artifacts
8. `PLAN_EXECUTION_BLUEPRINT.md` only when execution order or DONE/PR_READY criteria are unclear
9. `RESEARCH_AND_DECISIONS.md` only when provider/security/session/recovery decisions are touched

Do not read all workpacks. Do not read sibling plans by default.

## Decision tree

| If the task is about... | Open |
| --- | --- |
| Cloudflare/Firebase/Auth.js/provider choice | `workpacks/01-auth-provider-decision.md` |
| Users, households, roles, child profiles, devices | `workpacks/02-identity-household-role-model.md` |
| Sessions, tokens, refresh, revocation, replay | `workpacks/03-session-token-lifecycle.md` |
| Invites, recovery, co-parent, transfer, deletion | `workpacks/04-invites-recovery-lifecycle.md` |
| Cross-family authZ and device ownership | `workpacks/05-device-ownership-authz.md` |
| Security, abuse proof, route sync, rollout gate | `workpacks/06-security-proof-and-route-gate.md` |
| Parent account and family setup UI | `workpacks/07-parent-account-family-setup-ui.md` |
| Rust-owned canonical account/family schema and account-authority parity | `workpacks/08-rust-schema-workers-d1-runtime-migration.md` |

## Non-negotiable authority rules

- Third-party identity providers may prove user identity; they do not own household membership, child profiles, devices, invites, recovery, policy authority, child evidence, or product readiness.
- Firebase custom claims, if used, are minimal access hints only; product data and household truth stay in Ocentra-owned storage.
- Auth.js, if used, is a session/auth adapter; it does not become the family authority model.
- D1 is the default relational owner for account/household/member/device/session/invite metadata when Cloudflare runtime is selected.
- Durable Objects coordinate short-lived setup rooms, rate limits, invite/recovery/session coordination, and serialized state transitions where needed.
- KV is non-authoritative cache/rate-limit hint state only.
- R2 is not used for family authority; if later approved, it stores only explicitly encrypted artifacts outside child activity default custody.
- Authentication is not authorization. Every action checks household, role, device, session freshness, revocation, and action capability.
- Child profile is not child device trust.
- Parent login is not parent trusted-device proof.
- Support/admin is a separate audited actor class, never hidden parent ownership.

## Local work loop

1. Select exactly one workpack.
2. Fill the pre-edit note from the workpack.
3. Change only owned docs/source paths named by that workpack.
4. Add/update the required tests or record the exact missing test location.
5. Run the focused commands in `TEST_PROOF_EXPECTATIONS.md` through `npm run agent:run --` when possible.
6. Write proof artifacts to the workpack proof root from `PROOF_INDEX.md`.
7. Update `CHECKLIST_INDEX.md`, the selected workpack completion section, and `PLAN_STATE.md` only for proven rows.
8. Report no-claim boundaries.

## Failure conditions

Do not claim DONE or PR_READY if any of these are true:

- Provider decision is missing, stale, or stores family product data in an IdP.
- A valid login token is treated as household, device, policy, export, billing, or remote authority.
- Cross-family id guessing has no denial proof.
- Observer/co-parent/child/support permissions are not separated.
- Session expiry, refresh, logout, revocation, replay, CSRF/origin, and stale-token cases are untested.
- Invite/recovery flows lack expiry, single-use, revocation, enumeration resistance, and support-audit proof.
- UI implies “logged in” means “trusted household/device ready.”
- Child private activity data is routed into account/identity systems.
- Proof artifacts or command logs are missing.
