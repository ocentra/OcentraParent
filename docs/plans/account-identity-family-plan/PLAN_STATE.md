<!-- agent-capsule -->

> Agent Capsule
> Plan: `account-identity-family-plan`
> Doc: `Account Identity Family Plan State`
> Kind: current state and open gaps.
> Read when: immediately after AGENTS.md.
> Stop rule: use this file to choose route state, then continue only to NEXT_ACTIONS.md and WORKPACK_INDEX.md.
> Proves: only current plan state and open-gap accounting.
> Does not prove: implementation completion, security readiness, or PR readiness.
> Proof rule: if state changes, update the assigned workpack, CHECKLIST_INDEX.md, and PROOF_INDEX.md proof path.

<!-- /agent-capsule -->

# Account Identity Family Plan State

## Current status

```text
Plan route: upgraded
Execution-grade workpacks: in progress
Implementation: not started by this plan route
Proof artifacts: none recorded by this plan route yet
PR-ready: false
```

## Current product direction

```text
Cloudflare-first custody for account/family authority.
D1 owns relational account, household, membership, child profile, device, invite, recovery, and session metadata when Cloudflare runtime is selected.
Durable Objects own short-lived coordination and serialized setup/session/recovery state where needed.
KV is non-authoritative cache, rate-limit, or lookup-hint state only.
R2 is excluded from account authority and may hold only explicitly encrypted artifacts if a later data-custody decision approves it.
Firebase Auth, if used, is an external IdP/token issuer only.
Auth.js or another app-owned auth layer may be used only as an adapter/session layer, not the family authority model.
```

## Current repo facts already read

- `docs/features/family-setup-device-roles.md` says family setup is product foundation and not portal polish. It also states the child-device agent remains authority for device role, controller lease, revocation, stale command rejection, and local capability status.
- `docs/expectations/family-setup.md` separates parent outcome, child-device outcome, data scope, contract families, validation gates, and non-goals.
- `docs/expectations/portal.md` says portal sends typed queries/intents to the agent and must not become child-device execution authority.
- `packages/family-domain/package.json` already exports `session-lifecycle`, `child-profile`, `household-authority`, `setup-lifecycle`, and reference primitives.

## External research anchors

- Cloudflare D1 is a managed serverless SQLite-compatible database for Workers/Pages and supports relational query/storage ownership.
- Cloudflare Durable Objects provide stateful serverless coordination with compute plus durable storage and are appropriate for serialized short-lived coordination.
- Firebase custom claims are delivered through ID tokens, must be validated server-side, are size-limited, and should be used for access control only, not as a product-data store.
- Auth.js supports JWT and database session strategies; either choice must be evaluated against revocation, token size, custody, and adapter constraints.
- OWASP requires deny-by-default authorization and permission validation on every request.
- OWASP session guidance requires meaningless, unpredictable session identifiers and server-side session state.
- OWASP recovery guidance requires consistent responses, side-channel reset delivery, random single-use expiring tokens, and rate limiting.
- NIST 800-63B requires risk-appropriate authentication assurance, step-up when higher assurance is required, replay resistance at higher assurance, and reauthentication/session timeout rules.

## Open gaps

```text
- Provider decision record is not locked.
- No runtime implementation for account identity adapter boundary.
- No D1/DO/KV account-family schema or migration proof exists.
- No household membership/role/device authority matrix proof exists.
- No session/token/refresh/logout/revocation/replay proof exists.
- No invite/recovery/transfer/deletion handoff proof exists.
- No first-run family setup UI proof exists.
- No cross-plan route gate proof exists for setup, Cloudflare, payment, policy, data custody, device trust, LAN, or remote access.
- No proof artifacts under `output/account-identity-family-plan-proof/` exist yet.
```

## No-claim boundaries

Do not claim:

```text
auth provider selected
family authority implemented
household setup implemented
secure login/session implemented
device authority implemented
invite/recovery implemented
first-run setup UI ready
payment/customer ownership ready
policy authorization ready
remote access authorization ready
device trust bootstrap ready
product-ready account/family flow
```

until the relevant workpack proof root and checklist rows prove the claim.

## Default execution order

```text
WP01 provider decision and custody boundary
WP02 account/household/role/device model
WP03 session/token lifecycle
WP04 invite/recovery lifecycle
WP05 device ownership authorization
WP07 parent account/family setup UI
WP06 security proof and route gate
```

WP06 is last because it consumes proof from every earlier workpack.

## Health rules

- Do not start runtime implementation if WP01 provider/custody decision is open.
- Do not add setup UI before WP02/WP03 contract shapes exist or are explicitly stubbed with blockers.
- Do not let setup, payment, policy, remote, or device-trust plans own account-family authority.
- Do not use Firebase custom claims for household membership/product data.
- Do not put child activity evidence into account/identity state.
- Do not mark rows checked without exact proof artifact names and command logs.
