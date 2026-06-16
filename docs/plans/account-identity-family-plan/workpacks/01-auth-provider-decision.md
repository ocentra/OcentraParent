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
RESEARCH_AND_DECISIONS.md
docs/features/family-setup-device-roles.md
docs/expectations/family-setup.md
docs/expectations/cloud.md
docs/expectations/platforms.md
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
packages/family-domain/src/session-lifecycle.ts
packages/family-domain/src/household-authority.ts
packages/family-domain/tests/unit/*provider*.test.ts
```

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

## Fill before DONE

```text
Workpack id and branch:
Decision outcome:
Rejected options:
Touched files:
Validation commands and results:
Proof artifacts:
Known gaps/manual-required states:
No-claim boundaries:
```
