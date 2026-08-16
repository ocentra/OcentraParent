<!-- agent-capsule -->

> Agent Capsule
> Plan: `account-identity-family-plan`
> Doc: `WP01 Auth Provider Decision`
> Kind: assigned implementation/research workpack.
> Read when: selected by WORKPACK_INDEX.md or explicit assignment.
> Stop rule: do not implement runtime account/session flows until this provider/custody decision is accepted or blocked with proof.
> Proves: provider/custody decision only after proof artifacts exist.
> Does not prove: login implementation, session security, household authority, or product readiness.
> Proof rule: before DONE, write all WP01 proof artifacts and command log.

<!-- /agent-capsule -->

# WP01 Auth Provider Decision

## Goal

Decide the identity-provider architecture and custody boundary before login/user work spreads across setup, portal, Cloudflare, payment, or backend docs.

## Required inputs

```text
AGENTS.md
PLAN_STATE.md
workpacks/00-owner-boundary-proof-gate.md
RESEARCH_AND_DECISIONS.md
docs/features/family-setup-device-roles.md
docs/expectations/family-setup.md
docs/expectations/cloud.md
docs/expectations/platforms.md
packages/schema-domain package exports when shared account/family/session shapes are touched
packages/family-domain/package.json
```

Use official provider docs only for current API/security facts.

## Target decision

Write a source-backed decision that answers:

```text
Is Firebase Auth used? If yes, what exactly does it own?
Is Auth.js used? If yes, what session strategy and adapter boundary are allowed?
Does Cloudflare D1/DO state own users, households, roles, and sessions after token verification?
What data can the identity provider see?
What data is forbidden in custom claims or IdP profile fields?
How are provider outage/degraded states represented?
How can the provider be replaced later without migrating family product truth out of Ocentra storage?
What MVP and later auth methods are allowed: email link, password, OAuth, MFA, passkey, device step-up?
```

## Expected source/doc changes

Likely docs-only in this workpack:

```text
docs/plans/account-identity-family-plan/RESEARCH_AND_DECISIONS.md
docs/plans/account-identity-family-plan/PLAN_STATE.md
docs/plans/account-identity-family-plan/CHECKLIST_INDEX.md
```

If implementation starts here, keep it to provider adapter boundaries only:

```text
packages/schema-domain/** only when canonical shared account/session/provider shapes are added or changed
packages/family-domain/src/session-lifecycle.ts
packages/family-domain/src/household-authority.ts
packages/family-domain/tests/unit/*provider*.test.ts
```

## Current owner/import/proof constraints

This workpack is a provider/custody decision gate. It must not become login runtime, household authority, or setup implementation.

```text
schema-domain: canonical shared provider/session/account shapes only when cross-boundary shape changes are required.
family-domain: helper/projection and local contract tests only.
Cloudflare runtime/schema: target implementation remains future work unless explicitly selected.
IdP/Firebase/Auth.js: external adapter only; never family product truth.
```

Allowed direct imports are limited to `schema-domain`, neutral protocol/evidence/logging/capability primitives, approved `family-domain` helpers, and pure common helpers. Do not import setup/payment/policy/remote/device-trust/data-custody runtime internals to settle provider authority.

Proof must include a custody/no-claim note: provider decision proof is not runtime login/session readiness, and no IdP/custom-claim field may become household/member/child/device product truth.

## Required proof root

```text
output/account-identity-family-plan-proof/01-auth-provider-decision/
```

Required artifacts:

```text
00-provider-decision-record.md
01-provider-rejected-options.md
02-provider-custody-boundary-proof.md
03-custom-claims-data-minimization-proof.md
04-provider-outage-degraded-proof.md
05-migration-path-proof.md
16-validation-commands.log
```

## Acceptance criteria

- [ ] Provider decision is source-backed and explicit.
- [ ] Cloudflare D1/DO/KV/R2 custody split is accepted or blocked.
- [ ] Firebase/Auth.js/other provider role is accepted/rejected/staged.
- [ ] Identity provider cannot own household membership, child profiles, devices, invites, recovery, policy authority, child evidence, or product readiness.
- [ ] Custom claims are access hints only and have a size/data-minimization rule.
- [ ] Dev-mode bypass cannot satisfy production proof.
- [ ] Provider outage has degraded/manual-required behavior.
- [ ] Replacement/migration path exists.
- [ ] Proof artifacts and command log exist.
- [ ] Checklist rows updated only after proof.

## Focused commands

```bash
node -e "console.log('provider-decision-docs-only')"
npm run lint:architecture -- --files docs/plans/account-identity-family-plan
```

If provider boundary code changes:

```bash
npm run build --workspace @ocentra-parent/schema-domain
npm run build --workspace @ocentra-parent/family-domain
npm run test --workspace @ocentra-parent/family-domain -- provider
```

## Negative cases

- Provider owns family product data.
- Custom claims contain household/member/child/device product data.
- Provider outage makes privileged flows look available.
- Dev-mode provider bypass is accepted in production proof.
- Provider-specific user id is treated as household authority.

## Manual-required gaps

Implementation remains open until WP02/WP03 convert this decision into family-domain contracts and tests.

## 2026-08-04 runtime slice evidence

The account-identity lane added a narrow Cloudflare persistence boundary without changing the provider decision or historical acceptance rows. `infra/cloudflare/src/storage/account-identity-store.ts` stores only a verified provider subject to an Ocentra account id in the optional/manual-required `ACCOUNT_IDENTITY_D1` binding. Its real SQLite-backed unit proof covers persistence, same-account status update, lookup, missing-binding manual-required behavior, malformed input, and cross-account uniqueness conflict. The durable proof is `docs/proof/account-identity-family-plan/01-auth-provider-decision/06-account-identity-storage-adapter-proof.md`; the ignored `output/` copy remains a local raw-evidence pointer.

This evidence does not close WP01 or the plan. External provider token verification, login/session route wiring, household/membership/role/device authority, D1 deployment/migration proof, and production readiness remain manual-required. Historical checkboxes and the prior validation log are intentionally unchanged.

## 2026-08-16 production reachability audit

`infra/cloudflare/src/auth/verifier.ts` is reached by the Worker route
dispatcher, but both Wrangler configurations use
`AUTH_ADAPTER_MODE=account-auth-adapter-manual-required`. The only non-blocked
bearer path is `local-safe-fixture`; it normalizes a caller token and is not a
cryptographic provider verifier. The route manifest contains billing,
admin, and provider-webhook routes, not Account identity routes. The D1 store
has no production caller. Provider library, issuer, trust material, and the
runtime-owned Account caller remain unresolved; no Account auth or D1
persistence implementation slice is authorized from this workpack.

## Fill before DONE

- Workpack id and branch: `WP01 Auth Provider Decision`; `codex/tracking-plan-full-continuation-a`.
- Current branch note: this historical completion record predates the plan-harness branch. On `codex/plan-harness-update`, treat it as prior proof evidence only; new edits must follow `workpacks/00-owner-boundary-proof-gate.md`, `TEST_PROOF_EXPECTATIONS.md`, and `PROOF_INDEX.md`.
- Decision outcome: Cloudflare-first custody; Firebase Auth and Auth.js stay adapter-only; D1/DO own family truth; no family data in IdP/custom claims.
- Rejected options:
  - Firebase owns family product data.
  - Auth.js as a hidden product-data owner.
  - Firebase custom claims as household truth.
  - IdP user profile as child/device registry.
  - Cloudflare Access as the consumer family identity product.
  - Third-party IdP as the source of truth for household membership.
- Touched files:
  - `docs/plans/account-identity-family-plan/CHECKLIST_INDEX.md`
  - `docs/plans/account-identity-family-plan/PLAN_STATE.md`
  - `docs/plans/account-identity-family-plan/WORKPACK_INDEX.md`
  - `docs/plans/account-identity-family-plan/workpacks/01-auth-provider-decision.md`
  - `output/account-identity-family-plan-proof/01-auth-provider-decision/00-provider-decision-record.md`
  - `output/account-identity-family-plan-proof/01-auth-provider-decision/01-provider-rejected-options.md`
  - `output/account-identity-family-plan-proof/01-auth-provider-decision/02-provider-custody-boundary-proof.md`
  - `output/account-identity-family-plan-proof/01-auth-provider-decision/03-custom-claims-data-minimization-proof.md`
  - `output/account-identity-family-plan-proof/01-auth-provider-decision/04-provider-outage-degraded-proof.md`
  - `output/account-identity-family-plan-proof/01-auth-provider-decision/05-migration-path-proof.md`
  - `output/account-identity-family-plan-proof/01-auth-provider-decision/16-validation-commands.log`
- Validation commands and results:
  - `command: node -e "console.log('provider-decision-docs-only')"`
  - `exit: 0`
  - `result: pass`
  - `artifact: n/a`
  - `notes: docs-only sentinel`
  - `command: npm run lint:architecture -- --files docs/plans/account-identity-family-plan`
  - `exit: 0`
  - `result: pass`
  - `artifact: n/a`
  - `notes: re-export gate skipped; no JS/TS or Rust files in scope`
- Proof artifacts:
  - `output/account-identity-family-plan-proof/01-auth-provider-decision/00-provider-decision-record.md`
  - `output/account-identity-family-plan-proof/01-auth-provider-decision/01-provider-rejected-options.md`
  - `output/account-identity-family-plan-proof/01-auth-provider-decision/02-provider-custody-boundary-proof.md`
  - `output/account-identity-family-plan-proof/01-auth-provider-decision/03-custom-claims-data-minimization-proof.md`
  - `output/account-identity-family-plan-proof/01-auth-provider-decision/04-provider-outage-degraded-proof.md`
  - `output/account-identity-family-plan-proof/01-auth-provider-decision/05-migration-path-proof.md`
  - `output/account-identity-family-plan-proof/01-auth-provider-decision/16-validation-commands.log`
- Known gaps/manual-required states: runtime login/session implementation remains open in WP02/WP03 and beyond; provider role is fixed but route-sync details for passkey/step-up remain external dependencies.
- No-claim boundaries: provider selection is external-only; no runtime login/session implementation; no household/device authority claim; no PR-ready claim.
