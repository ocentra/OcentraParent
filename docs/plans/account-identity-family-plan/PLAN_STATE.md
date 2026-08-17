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
Execution-grade workpacks: WP01 has a provider/custody proof pack plus the retained narrow D1 storage-adapter proof at `docs/proof/account-identity-family-plan/01-auth-provider-decision/06-account-identity-storage-adapter-proof.md`; WP08 has a tracked durable Rust-authority manifest under `docs/proof/account-identity-family-plan/08-rust-schema-workers-d1-runtime-migration/`; WP02, WP03, WP04, WP05, and WP07 have prior complete proof roots on disk; WP06 is reopened for final aggregation after WP08 plus Cloudflare WP06/WP08 handoffs
Implementation: central-schema migration is in progress/current for shared account/family shapes; helper/projection implementation exists in family-domain, setup-domain, family-identity-core, and provisioning-core. Independent P0/P1 review accepts WP08's bounded `v0.7` source packet: the canonical household/child/device binding includes pairing, installation, selected route, lifecycle, revocation, bounded authority generation, guarded identifiers, active provider mapping, and exact account consistency. The family-owned repository/read port and trusted result are crate-private, so request or generated DTOs cannot mint authority. Focused tests and proof remain deferred to later phases. Cloudflare WP06 still owns the real durable adapter and production caller; external provider verification, account runtime routes, and D1/DO/KV migration proof remain open
Proof artifacts: `output/account-identity-family-plan-proof/01-auth-provider-decision/`, `02-identity-household-role-model/`, `03-session-token-lifecycle/`, `04-invites-recovery-lifecycle/`, `05-device-ownership-authz/`, `06-security-proof-and-route-gate/`, and `07-parent-account-family-setup-ui/` are populated; WP08 uses its tracked durable manifest rather than ignored raw output; WP03 and WP06 carry request-safety as an explicit blocker note instead of a fake-green proof; `test-results/account-identity-family-plan-*` roots remain absent unless a selected workpack explicitly requires them
PR-ready: false
```

## Closed PR disposition

PR #607 is closed without merge. Its TypeScript Cloudflare account-identity
persistence/D1-test-double slice is preserved as branch evidence only; it does
not establish Rust schema authority or any Cloudflare runtime/migration proof.

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

## 2026-08-17 provider handoff

Account WP01 selects Firebase Auth as the external identity provider for the
Cloudflare Worker adapter. The provider may prove only the external user
identity: the adapter must verify Firebase RS256 ID tokens against configured
issuer, audience, JWKS, time, and non-empty subject, then return only that
verified provider subject. D1/DO and the Rust family authority remain the sole
owners of account, household, membership, role, child, device, invite,
recovery, and session product truth. Auth.js is not selected for this Worker
path and cannot become family authority.

This is a narrow handoff to Cloudflare WP05's implementation-only packet. It
does not authorize account login/session routes, D1 migration, deployment,
tests, proof, or runtime readiness; unresolved provider configuration remains
fail-closed/manual-required.

## Current repo facts already read

- `docs/features/family-setup-device-roles.md` says family setup is product foundation and not portal polish. It also states the child-device agent remains authority for device role, controller lease, revocation, stale command rejection, and local capability status.
- `docs/expectations/family-setup.md` separates parent outcome, child-device outcome, data scope, contract families, validation gates, and non-goals.
- `docs/expectations/portal.md` says portal sends typed queries/intents to the agent and must not become child-device execution authority.
- `packages/family-domain/package.json` now describes family-domain as helpers that consume canonical `schema-domain` family contracts; do not move shared canonical shapes back into family-domain.
- `packages/setup-domain/src/family-setup-bridge.ts` and `packages/setup-domain/src/registration-entry.ts` already consume the household/invite/recovery contracts.
- `crates/family-identity-core` and `crates/provisioning-core` already carry Rust parity and downstream provisioning consumers for the same authority/session/setup surfaces.

## Module ownership and linkage

```text
crates/schema or the owning Rust crate:
  Canonical shared account/family/session/device-authority schemas, brands, parsers, route/action/read-model DTOs, literals, and encoded-shape parity when shapes cross package, crate, app, or plan boundaries.

schema-domain:
  Temporary generated-validation or edge-decoder surface only where TypeScript still needs one during migration.

family-domain:
  TypeScript helper/projection package for account/family authority. It consumes Rust-owned/generated contracts and exposes approved helper surfaces; it must not become a sibling-feature runtime dependency.

family-identity-core:
  Rust parity/runtime authority boundary for account, household, role, child profile, device, session, invite/recovery, and audit semantics.

setup-domain and provisioning-core:
  Setup/provisioning consumers of account/family authority, not authority owners.

portal-domain and apps/portal:
  UI projection/rendering consumers. They may prove honest state visibility but do not prove account runtime, Cloudflare persistence, device trust, LAN/remote transport, or child activity readiness.

Cloudflare control-plane runtime/schema:
  Cloudflare retains an isolated optional `ACCOUNT_IDENTITY_D1` store and migration configuration, but no provider verifier, runtime store caller, account/session routes, household authority, deployed/migrated D1 schema, Durable Object coordination, or production Worker readiness. Those runtime/persistence boundaries remain open here.

2026-08-16 production reachability audit:
  `infra/cloudflare/src/auth/verifier.ts` is invoked by the Worker route dispatcher, but both Wrangler configurations set `AUTH_ADAPTER_MODE` to `account-auth-adapter-manual-required`; the only non-blocked bearer path is the local-safe fixture mode and its token normalization is not cryptographic provider verification. The Worker route manifest contains billing/admin/webhook routes, not Account identity routes. Provider library, issuer, trust material, and runtime-owned account caller remain unresolved, so no Account auth or D1 persistence implementation slice is authorized.

Adjacent plans:
  Payment, policy, data custody, device trust, LAN, remote, setup-install, and broader portal UX consume account/family authority through handoff contracts, events, requests, read models, and proof routes. They must not re-own the authority model.
```

## Current proof interpretation

```text
Workpack proof roots prove local contract/proof slices only.
Absent `test-results/account-identity-family-plan-*` roots are not automatically fatal because current proof logs live under `output/account-identity-family-plan-proof/**/16-validation-commands.log` unless a selected workpack requires a test-results artifact.
WP01 10/10 means the provider/custody decision proof pack is filled, not that runtime auth/provider implementation is complete.
WP03 and WP06 request-safety artifacts are blockers because this plan still does not own a real browser request consumer.
WP07 proves the local setup route/projection slice; it does not prove physical device trust, Cloudflare account runtime, LAN/remote transport, or custody execution.
```

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
- WP02 root now contains `00-identity-entity-model-proof.md`, `01-role-action-resource-matrix.md`, `02-membership-state-machine-proof.md`, `03-cross-family-negative-proof.md`, `04-observer-read-only-proof.md`, `05-support-admin-boundary-proof.md`, `06-audit-event-proof.md`, and `16-validation-commands.log`.
- WP03 root now contains `00-credential-type-matrix.md`, `01-session-lifecycle-proof.md`, `02-token-expiry-replay-proof.md`, `03-refresh-revocation-proof.md`, `04-session-freshness-proof.md`, `05-csrf-origin-proof.md`, `06-token-redaction-proof.md`, and `16-validation-commands.log`; `05-csrf-origin-proof.md` is an explicit blocker note because this slice does not own a real browser request surface.
- WP04 root now contains `00-invite-state-machine-proof.md`, `01-invite-negative-proof.md`, `02-recovery-state-machine-proof.md`, `03-recovery-abuse-proof.md`, `04-delete-export-handoff-proof.md`, `05-support-recovery-audit-proof.md`, and `16-validation-commands.log`.
- WP05 root now contains `00-device-authority-matrix.md`, `01-revoked-device-negative-proof.md`, `02-wrong-household-negative-proof.md`, `03-controller-lease-proof.md`, `04-remote-capability-proof.md`, `05-export-delete-owner-proof.md`, `06-billing-owner-proof.md`, and `16-validation-commands.log`.
- WP07 root now contains `00-first-run-ui-state-machine.md`, `01-household-setup-ui-proof.md`, `02-device-role-ui-proof.md`, `03-observer-read-only-ui-proof.md`, `04-recovery-ui-proof.md`, `05-mobile-parent-child-claim-split-proof.md`, `06-source-custody-label-proof.md`, and `16-validation-commands.log`; the portal route/test/e2e surface is now real and keeps sibling runtime ownership explicit instead of pretending setup owns Cloudflare, trust, custody, or transport execution.
- `packages/family-domain/tests/unit/setup-lifecycle.test.ts` was repaired so the direct invite/recovery suite now matches the live schema, and `packages/family-domain/src/setup-lifecycle.ts` received a local exhaustiveness repair so the WP04 build gate is green again; no further production TS/Rust changes were required for WP02-WP03 closure, and WP05 only needed owner-only test additions in shared TypeScript/Rust authority suites.
- WP08's Rust schema/account-authority implementation and focused test surface are retained by the tracked durable manifest; Cloudflare WP06/WP08 and Account WP06 final aggregation remain open.
- WP08's bounded source repair is independently accepted as implementation evidence only. `AccountIdentityAuthorityHandoff` is now the exact `v0.7` canonical binding, and `family-identity-core` owns a sealed, fail-closed current-binding port. No production repository implementation or caller exists, so runtime authority remains unreachable by construction. Focused Rust/TypeScript tests and retained proof stay deferred, and the prior durable manifest does not validate this follow-up. Cloudflare WP06 retains only the isolated provider-subject D1 store and must implement the real adapter/caller without redefining authority.
- Cloudflare WP06 does not supply a provider-subject persistence handoff. No Cloudflare-owned token-verification route, account-family authority binding, deployed/migrated schema proof, Durable Object coordination, or runner proof exists; provider verification and any runtime-owned store caller remain manual-required.
- WP06's prior root contains `00-security-proof-pack.md`, `01-authn-negative-proof.md`, `02-authz-matrix-proof.md`, `03-token-replay-proof.md`, `04-recovery-abuse-proof.md`, `05-origin-csrf-open-redirect-proof.md`, `06-route-sync-proof.md`, `07-logging-redaction-proof.md`, `08-manual-required-gap-register.md`, and `16-validation-commands.log`; it is reopened and cannot be final-gate proof until `09-account-authority-cloudflare-storage-gate.md` aggregates green Account WP08, Cloudflare WP06, and Cloudflare WP08 proof. A blocker remains a scheduling block for payment, policy, remote, and device trust.
- Browser request-safety proof remains blocked at `output/account-identity-family-plan-proof/03-session-token-lifecycle/05-csrf-origin-proof.md` and `output/account-identity-family-plan-proof/06-security-proof-and-route-gate/05-origin-csrf-open-redirect-proof.md` because this plan slice still does not own a real browser request consumer.
- Adjacent runtime and schema work remain manual-required: provider verification and account/session runtime routes, D1/DO/KV account-family schema and migration proof, Cloudflare worker/runtime proof, payment execution, policy execution, data-custody execution, device-trust bootstrap, LAN transport, and remote transport.
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
WP08 Rust-owned schema and account-authority parity
Cloudflare WP06 D1/DO/KV binding/migration -> Cloudflare WP08 runner/proof
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
- Do not move canonical shared account/family shapes out of `crates/schema` or the owning Rust crate into sibling feature owners.
- Do not claim E2E readiness from a local workpack proof root; use the E2E tiers in `TEST_PROOF_EXPECTATIONS.md`.
