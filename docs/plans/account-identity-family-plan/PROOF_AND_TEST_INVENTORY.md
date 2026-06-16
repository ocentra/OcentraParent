<!-- agent-capsule -->

> Agent Capsule
> Plan: `account-identity-family-plan`
> Doc: `Account Identity Family Plan Proof And Test Inventory`
> Kind: compatibility proof inventory; canonical proof routes live in PROOF_INDEX.md.
> Read when: an older route names this file instead of PROOF_INDEX.md.
> Stop rule: use this as an alias, then continue to PROOF_INDEX.md and TEST_PROOF_EXPECTATIONS.md.
> Proves: proof inventory routing only.
> Does not prove: implementation completion or PR readiness.

<!-- /agent-capsule -->

# Account Identity Family Plan Proof And Test Inventory

This file is kept for older plan routes. The canonical proof and command matrix is now split:

```text
PROOF_INDEX.md                 exact proof roots and required artifact names
TEST_PROOF_EXPECTATIONS.md     commands, test families, negative cases
CHECKLIST_INDEX.md             execution rows per workpack
```

## Proof roots

```text
output/account-identity-family-plan-proof/01-auth-provider-decision/
output/account-identity-family-plan-proof/02-identity-household-role-model/
output/account-identity-family-plan-proof/03-session-token-lifecycle/
output/account-identity-family-plan-proof/04-invites-recovery-lifecycle/
output/account-identity-family-plan-proof/05-device-ownership-authz/
output/account-identity-family-plan-proof/07-parent-account-family-setup-ui/
output/account-identity-family-plan-proof/06-security-proof-and-route-gate/
```

## Required flow

- [ ] Select exactly one workpack.
- [ ] Identify source/doc/test files before editing.
- [ ] Run focused command set from `TEST_PROOF_EXPECTATIONS.md`.
- [ ] Write proof artifacts under the selected proof root.
- [ ] Record command log as `16-validation-commands.log`.
- [ ] Update `CHECKLIST_INDEX.md` only for proven rows.
- [ ] Update selected workpack Fill-before-DONE section.
- [ ] Update `PLAN_STATE.md` only if state changed.

## Failure conditions

- Do not store proof artifacts inside this plan folder.
- Do not mark DONE or PR_READY from generic “tests passed” statements.
- Do not accept provider/session/security claims without negative tests and proof roots.
- Do not claim account/family product readiness until WP06 aggregates route-gate proof from the earlier workpacks.
