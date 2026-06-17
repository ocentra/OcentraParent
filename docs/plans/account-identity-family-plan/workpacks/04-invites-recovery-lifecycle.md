<!-- agent-capsule -->

> Agent Capsule
> Plan: `account-identity-family-plan`
> Doc: `WP04 Invites Recovery Lifecycle`
> Kind: assigned implementation workpack.
> Read when: selected by WORKPACK_INDEX.md or explicit assignment.
> Stop rule: do not open sibling workpacks; do not implement data-custody side effects, UI polish, or support tooling here.
> Proves: invite and recovery lifecycle only after tests/proof pass.
> Does not prove: setup UI readiness, data export/delete execution, or support-admin production readiness.
> Proof rule: before DONE, write all WP04 proof artifacts and command log.

<!-- /agent-capsule -->

# WP04 Invites Recovery Lifecycle

## Goal

Define parent invite, co-parent invite, observer invite, child-device invite/pairing, recovery, transfer, and deletion/export handoff lifecycle.

## Required inputs

```text
workpacks/01-auth-provider-decision.md
workpacks/02-identity-household-role-model.md
workpacks/03-session-token-lifecycle.md
RESEARCH_AND_DECISIONS.md
docs/expectations/family-setup.md
docs/expectations/data-custody.md
packages/family-domain/src/setup-lifecycle.ts
packages/family-domain/src/session-lifecycle.ts
packages/family-domain/tests/unit/setup-lifecycle.test.ts
```

## Required lifecycle

```text
parent owner invite
co-parent invite
observer invite
child device invite/pairing
invite expiry
invite revoke
invite accept
forgot-login recovery
lost parent device recovery
compromised account recovery
child reinstall recovery
household transfer
account delete/export handoff to data custody
support recovery audit
```

## Required rules

```text
invites are single-purpose
invites are single-use
invites carry household, role, device intent, expiry, and inviter authority
recovery cannot grant child evidence access without household authority
support recovery is minimized and audited
delete/export effects are handed to data-custody-storage-plan
responses are enumeration-resistant
rate-limit behavior exists or is explicitly blocked
```

## Expected source changes

Likely paths:

```text
packages/family-domain/src/setup-lifecycle.ts
packages/family-domain/src/session-lifecycle.ts
packages/family-domain/src/household-authority.ts
packages/family-domain/tests/unit/setup-lifecycle.test.ts
packages/family-domain/tests/unit/session-lifecycle.test.ts
```

## Required proof root

```text
output/account-identity-family-plan-proof/04-invites-recovery-lifecycle/
```

Required artifacts:

```text
00-invite-state-machine-proof.md
01-invite-negative-proof.md
02-recovery-state-machine-proof.md
03-recovery-abuse-proof.md
04-delete-export-handoff-proof.md
05-support-recovery-audit-proof.md
16-validation-commands.log
```

## Acceptance criteria

- [ ] Invite state machine exists.
- [ ] Co-parent, observer, and child-device scopes are separated.
- [ ] Single-use invite proof exists.
- [ ] Expired, revoked, and reused invite negative proof exists.
- [ ] Wrong-household and wrong-role negative proof exists.
- [ ] Recovery state machine exists.
- [ ] Forgotten-login, lost-parent-device, compromised-account, child-reinstall, and transfer flows are represented.
- [ ] Recovery rate-limit or exact blocker exists.
- [ ] Enumeration-resistant response behavior is documented/tested or blocked.
- [ ] Delete/export handoff is explicit and does not implement data custody side effects here.
- [ ] Support recovery audit proof exists.
- [ ] Focused commands pass or blockers are recorded.

## Focused commands

```bash
npm run build --workspace @ocentra-parent/family-domain
npm run test --workspace @ocentra-parent/family-domain -- invite
npm run test --workspace @ocentra-parent/family-domain -- recovery
npm run lint:architecture -- --files packages/family-domain
```

## Negative cases

- Expired invite denied.
- Revoked invite denied.
- Reused invite denied.
- Wrong-household invite denied.
- Wrong-role invite denied.
- Recovery request cannot bypass owner authority.
- Support recovery cannot act as owner without explicit audited support state.
- Delete/export request is not executed by account plan.

## Manual-required gaps

Actual storage export/delete mechanics stay in `data-custody-storage-plan`. Support/admin UI and operational tooling remain blocked until later support/admin proof exists.

## Fill before DONE

- Workpack id and branch: `WP04 Invites Recovery Lifecycle`; `codex/tracking-plan-full-continuation-a`.
- Current status: partial. `01-invite-negative-proof.md`, `02-recovery-state-machine-proof.md`, and `16-validation-commands.log` exist under `output/account-identity-family-plan-proof/04-invites-recovery-lifecycle/`.
- Contract/source changes in this slice: the only code repair affecting this workpack was the direct helper fix in `packages/family-domain/tests/unit/setup-lifecycle.test.ts`; no new WP04-owned lifecycle source change was required.
- Touched files:
  - `packages/family-domain/tests/unit/setup-lifecycle.test.ts`
  - `docs/plans/account-identity-family-plan/CHECKLIST_INDEX.md`
  - `docs/plans/account-identity-family-plan/PLAN_STATE.md`
  - `docs/plans/account-identity-family-plan/WORKPACK_INDEX.md`
  - `docs/plans/account-identity-family-plan/workpacks/04-invites-recovery-lifecycle.md`
  - `output/account-identity-family-plan-proof/04-invites-recovery-lifecycle/01-invite-negative-proof.md`
  - `output/account-identity-family-plan-proof/04-invites-recovery-lifecycle/02-recovery-state-machine-proof.md`
  - `output/account-identity-family-plan-proof/04-invites-recovery-lifecycle/16-validation-commands.log`
- Validation commands and results:
  - `command: npm run build --workspace @ocentra-parent/family-domain`
  - `exit: 0`
  - `result: pass`
  - `artifact: n/a`
  - `notes: family-domain builds after direct lifecycle helper repair`
  - `command: npm run test --workspace @ocentra-parent/family-domain -- tests/unit/setup-lifecycle.test.ts tests/unit/invite-recovery-lifecycle.test.ts`
  - `exit: 0`
  - `result: pass`
  - `artifact: n/a`
  - `notes: direct and command-target invite/recovery suites passed with 10 tests`
  - `command: npm run test --workspace @ocentra-parent/setup-domain -- tests/unit/registration-entry.test.ts tests/unit/family-setup-bridge.test.ts`
  - `exit: 0`
  - `result: pass`
  - `artifact: n/a`
  - `notes: downstream setup-domain consumers passed with 20 tests`
  - `command: npm run lint:architecture -- --files packages/family-domain packages/setup-domain`
  - `exit: 0`
  - `result: pass`
  - `artifact: n/a`
  - `notes: focused architecture checks passed for family-domain and setup-domain`
- Proof artifacts:
  - `output/account-identity-family-plan-proof/04-invites-recovery-lifecycle/01-invite-negative-proof.md`
  - `output/account-identity-family-plan-proof/04-invites-recovery-lifecycle/02-recovery-state-machine-proof.md`
  - `output/account-identity-family-plan-proof/04-invites-recovery-lifecycle/16-validation-commands.log`
- Known gaps/manual-required states: `00-invite-state-machine-proof.md`, `03-recovery-abuse-proof.md`, `04-delete-export-handoff-proof.md`, and `05-support-recovery-audit-proof.md` are still missing; storage export/delete mechanics stay in `data-custody-storage-plan`; support/admin tooling remains external.
- No-claim boundaries: do not claim invite-state-machine closure, rate-limit/enumeration proof, export/delete execution, support tooling readiness, or WP04 completion.
