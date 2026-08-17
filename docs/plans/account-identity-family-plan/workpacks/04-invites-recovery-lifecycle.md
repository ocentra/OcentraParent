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
workpacks/00-owner-boundary-proof-gate.md
workpacks/01-auth-provider-decision.md
workpacks/02-identity-household-role-model.md
workpacks/03-session-token-lifecycle.md
RESEARCH_AND_DECISIONS.md
docs/expectations/family-setup.md
docs/expectations/data-custody.md
packages/schema-domain setup/invite/recovery exports when shared shape changes are required
packages/family-domain/src/setup-lifecycle.ts
packages/family-domain/src/session-lifecycle.ts
packages/family-domain/tests/unit/setup-lifecycle.test.ts
packages/setup-domain/** only when setup consumer proof is selected
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
packages/schema-domain/** only when canonical shared setup/invite/recovery shapes change
packages/family-domain/src/setup-lifecycle.ts
packages/family-domain/src/session-lifecycle.ts
packages/family-domain/src/household-authority.ts
packages/family-domain/tests/unit/setup-lifecycle.test.ts
packages/family-domain/tests/unit/session-lifecycle.test.ts
packages/setup-domain/** only when setup consumer proof is selected
```

## Current owner/import/proof constraints

This workpack owns invite/recovery lifecycle contracts and handoff proof. It does not own storage export/delete execution, support tooling, setup UI polish, or provider/session runtime implementation.

```text
schema-domain: canonical shared invite/recovery/setup handoff shapes when cross-boundary.
family-domain: helper/projection and TypeScript lifecycle tests.
setup-domain: consumer proof only when selected.
data-custody-storage-plan: export/delete execution owner.
support/admin tooling: external owner unless explicitly selected.
```

Allowed direct imports are limited to `schema-domain`, neutral protocol/evidence/logging/capability primitives, approved `family-domain`/`setup-domain` helpers when selected, and pure common helpers. Do not import data-custody or support tooling runtime internals to prove lifecycle handoff.

Proof must state that delete/export is authorization/handoff only, not custody execution. It must also include negative cases for expired/revoked/reused/wrong-household/wrong-role invites and recovery misuse.

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

If canonical schema or setup consumer proof changes:

```bash
npm run build --workspace @ocentra-parent/schema-domain
npm run build --workspace @ocentra-parent/setup-domain
npm run test --workspace @ocentra-parent/setup-domain -- family
```

## Negative cases

- Expired invite denied.
- Revoked invite denied.
- Reused invite denied.
- Wrong-household invite denied.
- Wrong-role invite denied.
- Recovery request cannot replace owner authority.
- Support recovery cannot act as owner without explicit audited support state.
- Delete/export request is not executed by account plan.

## Manual-required gaps

Actual storage export/delete mechanics stay in `data-custody-storage-plan`. Support/admin UI and operational tooling remain blocked until later support/admin proof exists.

## 2026-08-17 current code/test correction

The Rust invite/recovery evaluators and focused tests are real and are consumed
by provisioning readiness/pairing projections. They cover the bounded decision
matrix, but persistence, time, rate-limit, replay, identity proof, and owner
approval are still supplied as facts by callers. No runtime owns an atomic
single-use invite or a monotonic recovery transition, and the data-custody
handoff remains a local enum rather than a delivered typed request.

Production source still required:

- a durable invite/recovery repository with trusted clock and atomic
  compare-and-swap redemption;
- opaque identity, owner-approval, and audited support authorizations;
- monotonic terminal lifecycle, durable rate-limit/replay custody, and typed
  correlated export/delete handoff;
- a shipped account runtime caller over WP08 canonical identity.

Expected test source still required:

- concurrent redemption and restart/replay;
- pre-issuance, expiry, revocation, wrong household/role, and malformed state;
- rejected recovery decisions cannot advance;
- enumeration-resistant timing/rate-limit behavior and constrained support
  recovery with an audit receipt;
- typed custody delivery/correlation and retry behavior.

### Accepted replacement source delta

The accepted `35edb2830` source adds owner-derived invite and recovery
lifecycle records with private construction and monotonic terminal semantics;
it does not trust request-supplied proof, replay, freshness, same-family,
abuse, timing, support, or owner-approval facts. Atomic durable issue/consume,
rate-limit custody, shipped account runtime composition, typed export/delete
delivery, and the full expected-test family remain open.

The remote packet `ac03afee3a` is rejected/quarantined: it allowed callers to
supply `Verified`, same-family, abuse, timing, and owner-approval facts; public
serde records could reset lifecycle/use state; and there was no durable atomic
owner. It is not WP04 progress.

## Fill before DONE

- Workpack id and branch: `WP04 Invites Recovery Lifecycle`; `codex/tracking-plan-full-continuation-a`.
- Current branch note: this historical completion record predates the plan-harness branch. On `codex/plan-harness-update`, treat it as prior proof evidence only; new edits must follow `workpacks/00-owner-boundary-proof-gate.md`, `TEST_PROOF_EXPECTATIONS.md`, and `PROOF_INDEX.md`.
- Current status: complete for the local contract/proof slice. `00-invite-state-machine-proof.md`, `01-invite-negative-proof.md`, `02-recovery-state-machine-proof.md`, `03-recovery-abuse-proof.md`, `04-delete-export-handoff-proof.md`, `05-support-recovery-audit-proof.md`, and `16-validation-commands.log` now exist under `output/account-identity-family-plan-proof/04-invites-recovery-lifecycle/`.
- Contract/source changes in this slice: `packages/family-domain/tests/unit/setup-lifecycle.test.ts` had already been repaired to include the live anti-abuse schema inputs, and `packages/family-domain/src/setup-lifecycle.ts` needed one local exhaustiveness repair so the WP04 build gate could pass truthfully.
- Touched files:
  - `packages/family-domain/tests/unit/setup-lifecycle.test.ts`
  - `packages/family-domain/src/setup-lifecycle.ts`
  - `docs/plans/account-identity-family-plan/CHECKLIST_INDEX.md`
  - `docs/plans/account-identity-family-plan/PLAN_STATE.md`
  - `docs/plans/account-identity-family-plan/WORKPACK_INDEX.md`
  - `docs/plans/account-identity-family-plan/workpacks/04-invites-recovery-lifecycle.md`
  - `output/account-identity-family-plan-proof/04-invites-recovery-lifecycle/00-invite-state-machine-proof.md`
  - `output/account-identity-family-plan-proof/04-invites-recovery-lifecycle/01-invite-negative-proof.md`
  - `output/account-identity-family-plan-proof/04-invites-recovery-lifecycle/02-recovery-state-machine-proof.md`
  - `output/account-identity-family-plan-proof/04-invites-recovery-lifecycle/03-recovery-abuse-proof.md`
  - `output/account-identity-family-plan-proof/04-invites-recovery-lifecycle/04-delete-export-handoff-proof.md`
  - `output/account-identity-family-plan-proof/04-invites-recovery-lifecycle/05-support-recovery-audit-proof.md`
  - `output/account-identity-family-plan-proof/04-invites-recovery-lifecycle/16-validation-commands.log`
- Validation commands and results:
  - `command: npm run build --workspace @ocentra-parent/family-domain`
  - `exit: 0`
  - `result: pass`
  - `artifact: n/a`
  - `notes: family-domain build now passes after the local exhaustiveness repair in packages/family-domain/src/setup-lifecycle.ts`
  - `command: npm run test --workspace @ocentra-parent/family-domain -- tests/unit/setup-lifecycle.test.ts tests/unit/invite-recovery-lifecycle.test.ts`
  - `exit: 0`
  - `result: pass`
  - `artifact: n/a`
  - `notes: direct and command-target invite/recovery suites passed with 11 tests against the repaired anti-abuse helper inputs`
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
  - `output/account-identity-family-plan-proof/04-invites-recovery-lifecycle/00-invite-state-machine-proof.md`
  - `output/account-identity-family-plan-proof/04-invites-recovery-lifecycle/01-invite-negative-proof.md`
  - `output/account-identity-family-plan-proof/04-invites-recovery-lifecycle/02-recovery-state-machine-proof.md`
  - `output/account-identity-family-plan-proof/04-invites-recovery-lifecycle/03-recovery-abuse-proof.md`
  - `output/account-identity-family-plan-proof/04-invites-recovery-lifecycle/04-delete-export-handoff-proof.md`
  - `output/account-identity-family-plan-proof/04-invites-recovery-lifecycle/05-support-recovery-audit-proof.md`
  - `output/account-identity-family-plan-proof/04-invites-recovery-lifecycle/16-validation-commands.log`
- Known gaps/manual-required states: storage export/delete mechanics stay in `data-custody-storage-plan`; support/admin tooling remains external; WP07 and WP06 still need their own proof roots before any broader plan-ready claim.
- No-claim boundaries: do not claim export/delete execution, support tooling readiness, WP07 UI readiness, or whole-plan completion from this workpack closure.
