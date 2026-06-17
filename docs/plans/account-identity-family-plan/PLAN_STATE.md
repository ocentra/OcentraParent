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
Execution-grade workpacks: WP01 has a docs-only proof pack on disk; WP02-WP05 each have partial proof roots on disk; WP07 and WP06 remain open
Implementation: partial contract implementation exists in family-domain, setup-domain, family-identity-core, and provisioning-core, but account-identity adapter/runtime, custody schema proof, and first-run setup UI remain open
Proof artifacts: `output/account-identity-family-plan-proof/01-auth-provider-decision/` is populated; `02` through `05` roots exist with partial artifact sets; `06`, `07`, and `test-results/account-identity-family-plan-*` roots remain absent
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
- `packages/setup-domain/src/family-setup-bridge.ts` and `packages/setup-domain/src/registration-entry.ts` already consume the household/invite/recovery contracts.
- `crates/family-identity-core` and `crates/provisioning-core` already carry Rust parity and downstream provisioning consumers for the same authority/session/setup surfaces.

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
- WP02 root currently contains only `03-cross-family-negative-proof.md` and `16-validation-commands.log`; entity-model, role-matrix, membership-state, observer, support-boundary, and audit proof slices are still missing.
- WP03 root currently contains only `02-token-expiry-replay-proof.md` and `16-validation-commands.log`; credential-matrix, lifecycle, refresh, freshness, request-safety, and redaction proof slices are still missing.
- WP04 root currently contains only `01-invite-negative-proof.md`, `02-recovery-state-machine-proof.md`, and `16-validation-commands.log`; invite-state, abuse, delete/export-handoff, and support-audit proof slices are still missing.
- WP05 root currently contains only `00-device-authority-matrix.md` and `16-validation-commands.log`; revoked-device, wrong-household, controller-lease, remote-capability, export/delete-owner, and billing-owner proof slices are still missing.
- `packages/family-domain/tests/unit/setup-lifecycle.test.ts` was repaired so the direct invite/recovery suite now matches the live schema; broader WP02-WP05 proof reconciliation remains open.
- No runtime implementation for account identity adapter boundary.
- No D1/DO/KV account-family schema or migration proof exists.
- No first-run family setup UI proof exists.
- No cross-plan route gate proof exists for setup, Cloudflare, payment, policy, data custody, device trust, LAN, or remote access.
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
- Do not treat partial proof roots as completed workpacks.
- Do not add setup UI before WP02/WP03 contract shapes exist or are explicitly stubbed with blockers.
- Do not let setup, payment, policy, remote, or device-trust plans own account-family authority.
- Do not use Firebase custom claims for household membership/product data.
- Do not put child activity evidence into account/identity state.
- Do not mark rows checked without exact proof artifact names and command logs.
