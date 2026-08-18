<!-- agent-capsule -->

> Agent Capsule
> Plan: `account-identity-family-plan`
> Doc: `WP02 Identity Household Role Model`
> Kind: assigned implementation workpack.
> Read when: selected by WORKPACK_INDEX.md or explicit assignment.
> Stop rule: do not open sibling workpacks; do not implement sessions, invites, UI, payment, policy, remote, or device-trust here.
> Proves: account/household/role/device authority model only after tests/proof pass.
> Does not prove: secure sessions, invite/recovery, trusted-device bootstrap, or setup UI readiness.
> Proof rule: before DONE, write all WP02 proof artifacts and command log.

<!-- /agent-capsule -->

# WP02 Identity Household Role Model

## Goal

Define the Ocentra authority model for users, households, memberships, child profiles, roles, support/admin actors, and devices.

## Required inputs

```text
workpacks/00-owner-boundary-proof-gate.md
workpacks/01-auth-provider-decision.md
RESEARCH_AND_DECISIONS.md
docs/features/family-setup-device-roles.md
docs/expectations/family-setup.md
docs/expectations/policy.md
packages/schema-domain account/family/session/reference exports when shared shape changes are required
packages/family-domain/src/household-authority.ts
packages/family-domain/tests/unit/household-authority.test.ts
crates/family-identity-core/** only when Rust parity is selected
```

## Target model

Required entities/states:

```text
AccountUser
Household
HouseholdMembership
HouseholdRole
ParentOwner
CoParentGuardian
Observer
ChildProfile
ParentControllerDevice
ParentObserverDevice
ChildDeviceAgent
SupportAdminActor
PendingMembership
InvitedMembership
RevokedMembership
DisabledMembership
```

Required rules:

```text
user identity is not household membership
child profile is not child device
parent account cannot read/write another household
observer is read-only
support/admin is separate, audited, and minimized
child profile cannot authorize device access
parent owner/co-parent authority is explicit by action
revoked/disabled/pending actors are denied or degraded
```

## Expected source changes

Likely paths:

```text
packages/schema-domain/** only when canonical shared account/family/role/reference shapes change
packages/family-domain/src/household-authority.ts
packages/family-domain/src/references.ts
packages/family-domain/src/reference-primitives.ts
packages/family-domain/tests/unit/household-authority.test.ts
packages/family-domain/package.json if exports change
crates/family-identity-core/** only when Rust parity is selected
```

Do not edit sibling plans.

## Current owner/import/proof constraints

This workpack owns the account/family authority model, not sessions, UI, payment, policy, remote, LAN, or physical device trust.

```text
schema-domain: canonical cross-boundary account/family/role/reference shapes.
family-domain: helper/projection and TS authority tests over canonical contracts.
family-identity-core: Rust parity only when selected.
setup/payment/policy/remote/LAN/device-trust/data-custody: consumers or adjacent owners only.
```

Allowed direct imports are limited to `schema-domain`, neutral protocol/evidence/logging/capability primitives, approved `family-domain` helpers, selected Rust parity crates, and pure common helpers. Do not import sibling feature runtime internals to satisfy role/action/resource authority.

Proof must state which tier it proves: TypeScript helper/projection, Rust parity, route handoff, or local proof artifact. It must not claim secure session, trusted-device, UI, payment, policy, remote, or data-custody readiness.

## Required proof root

```text
output/account-identity-family-plan-proof/02-identity-household-role-model/
```

Required artifacts:

```text
00-identity-entity-model-proof.md
01-role-action-resource-matrix.md
02-membership-state-machine-proof.md
03-cross-family-negative-proof.md
04-observer-read-only-proof.md
05-support-admin-boundary-proof.md
06-audit-event-proof.md
16-validation-commands.log
```

## Acceptance criteria

- [ ] Account user, household, membership, role, child profile, and device refs are typed.
- [ ] Role/action/resource matrix exists.
- [ ] Membership state machine exists.
- [ ] Cross-family access is denied by tests/proof.
- [ ] Observer read-only behavior is tested/proven.
- [ ] Support/admin boundary is audited/minimized.
- [ ] Child profile does not imply child-device authority.
- [ ] Audit event requirements are explicit.
- [ ] Focused commands pass or blockers are recorded.
- [ ] Checklist rows updated only after proof.

## Focused commands

```bash
npm run build --workspace @ocentra-parent/family-domain
npm run test --workspace @ocentra-parent/family-domain -- household
npm run test --workspace @ocentra-parent/family-domain -- authority
npm run lint:architecture -- --files packages/family-domain
```

If canonical schema or Rust parity changes:

```bash
npm run build --workspace @ocentra-parent/schema-domain
npm run test --workspace @ocentra-parent/schema-domain -- family
cargo test -p ocentra-family-identity-core household_authority
```

## Negative cases

- Wrong-household read/write is denied.
- Child profile id alone is rejected as device authority.
- Revoked, disabled, and pending members cannot perform restricted actions.
- Observer cannot perform write/control actions.
- Support/admin action requires explicit support-admin rule plus audit ref.
- Role alone cannot grant action without household, resource, and device context.

## Manual-required gaps

Session freshness, invite/recovery lifecycle, and parent trusted-device proof stay open for WP03/WP04/device-trust handoff.

## 2026-08-17 current code/test correction

The Rust household evaluator, proof/handoff code, provisioning consumer, policy
consumer, child-device scope consumer, and focused unit/contract tests are real.
They prove a bounded decision model, not a durable account authority runtime.
The live production consumers still pass caller-assembled family, membership,
device-trust, session, capability, and lease facts into the evaluator. The
sealed WP08 current-binding port and local SQLite repository/CAS exist, and
Cloudflare has a read adapter. Reviewed source at `86caae334` and `7934fb41b`
adds the Account-owned target-aware action owner and migrates the real storage-
custody consumer, but no authoritative Cloudflare writer/currentness or
shipped provider caller exists. The TypeScript owner paths named by this
historical workpack no longer exist.

Reviewed production source now present:

- an Account-owned target-aware resolver over the sealed WP08 binding that
  keeps the actor parent-controller device separate from the target
  child/profile/device for Pair, Register, Revoke, View, ChangePolicy, and
  Remote actions;
- server-derived current account, household, member, device, role, and same-
  family identity; capability, controller-lease, and step-up actions reject
  because those authority sources are not present and cannot be accepted from
  the request/caller;
- correct `ViewChildStatus` composition for ParentOwner, CoParent, and Observer
  actors while independently resolving the child/profile/device target.

Production source still required outside the bounded resolver:
- owned capability, controller-lease, and step-up authority composition for
  actions that currently reject as unavailable;
- a Cloudflare WP06 authoritative D1 writer/update/revocation/CAS owner and
  shipped Firebase/provider-to-sealed-authority caller;
- a minimized, receipt-bound, audited support authority rather than a public
  support actor or caller boolean;
- monotonic membership/role transitions and a typed audit sink.

Expected test source still required:

- actor-device/target-device mismatch, cross-child target, and cross-household
  target tests for every affected action;
- tests proving caller-supplied `same_family`, capability, controller lease,
  and step-up state cannot authorize an action;
- positive parent-owner/co-parent/observer `ViewChildStatus` cases with an
  independently resolved target;
- repository reload and concurrent transition tests;
- pending, invited, revoked, and disabled membership matrix negatives;
- minimized support/admin scope and audit-reference tests;
- a real production-caller test proving sealed current binding is consumed and
  caller-supplied authority cannot bypass it.

### Accepted replacement source delta and reopened review

The six patch-unique commits on remote `codex/account-wp02-source-wave` at
`35edb2830` are superseded/rejected and are not integrated. The canonical
reconciled roots retain the sealed capability and local SQLite repository/CAS/
invariant boundary. Live review reopened action composition and
the bounded correction is now implemented at `86caae334` and `7934fb41b`:
the target-aware owner consumes opaque current Account authority, keeps actor
and target identities separate, and does not accept same-family/capability/
lease/step-up authority from the request. Cloudflare WP06 separately owns the
authoritative D1 writer and provider caller. Expected tests, validation, proof,
routes, and DONE remain open.

The remote packet `ac03afee3a` is rejected/quarantined: its public
deserializable account/membership/support records had no caller or persistence
and would have introduced parallel mintable authority. It is not WP02 progress.

## Fill before DONE

- Workpack id and branch: `WP02 Identity Household Role Model`; `codex/tracking-plan-full-continuation-a`.
- Current branch note: this historical completion record predates the plan-harness branch. On `codex/plan-harness-update`, treat it as prior proof evidence only; new edits must follow `workpacks/00-owner-boundary-proof-gate.md`, `TEST_PROOF_EXPECTATIONS.md`, and `PROOF_INDEX.md`.
- Current status: complete for the local contract/proof slice. `00-identity-entity-model-proof.md`, `01-role-action-resource-matrix.md`, `02-membership-state-machine-proof.md`, `03-cross-family-negative-proof.md`, `04-observer-read-only-proof.md`, `05-support-admin-boundary-proof.md`, `06-audit-event-proof.md`, and `16-validation-commands.log` now exist under `output/account-identity-family-plan-proof/02-identity-household-role-model/`.
- Contract/source changes in this slice: no new WP02-owned production TypeScript or Rust logic was required. The authority contract was already present in `packages/family-domain/src/household-authority.ts`, and the proof closure is derived from the existing TypeScript and Rust authority suites that already exercised role, membership, observer, support-admin, and audit behavior.
- Touched files:
  - `docs/plans/account-identity-family-plan/CHECKLIST_INDEX.md`
  - `docs/plans/account-identity-family-plan/PLAN_STATE.md`
  - `docs/plans/account-identity-family-plan/WORKPACK_INDEX.md`
  - `docs/plans/account-identity-family-plan/workpacks/02-identity-household-role-model.md`
  - `output/account-identity-family-plan-proof/02-identity-household-role-model/00-identity-entity-model-proof.md`
  - `output/account-identity-family-plan-proof/02-identity-household-role-model/01-role-action-resource-matrix.md`
  - `output/account-identity-family-plan-proof/02-identity-household-role-model/02-membership-state-machine-proof.md`
  - `output/account-identity-family-plan-proof/02-identity-household-role-model/03-cross-family-negative-proof.md`
  - `output/account-identity-family-plan-proof/02-identity-household-role-model/04-observer-read-only-proof.md`
  - `output/account-identity-family-plan-proof/02-identity-household-role-model/05-support-admin-boundary-proof.md`
  - `output/account-identity-family-plan-proof/02-identity-household-role-model/06-audit-event-proof.md`
  - `output/account-identity-family-plan-proof/02-identity-household-role-model/16-validation-commands.log`
- Validation commands and results:
  - `command: npm run build --workspace @ocentra-parent/family-domain`
  - `exit: 0`
  - `result: pass`
  - `artifact: n/a`
  - `notes: family-domain build passed after the local WP04 repair and before WP02 proof closure`
  - `command: npm run test --workspace @ocentra-parent/family-domain -- tests/unit/household-authority.test.ts tests/unit/session-lifecycle.test.ts tests/unit/token-lifecycle.test.ts`
  - `exit: 0`
  - `result: pass`
  - `artifact: n/a`
  - `notes: direct household/session/token contract suite now passes with 24 tests after the export/delete owner-only additions in the shared authority suite`
  - `command: cargo test -p ocentra-family-identity-core household_authority`
  - `exit: 0`
  - `result: pass`
  - `artifact: n/a`
  - `notes: Rust parity household-authority subset passed with 12 tests covering role, observer, support, device, and wrong-household negatives`
  - `command: npm run lint:architecture -- --files packages/family-domain`
  - `exit: 0`
  - `result: pass`
  - `artifact: n/a`
  - `notes: focused TypeScript architecture gate passed for the touched family-domain scope`
  - `command: cargo lint-architecture crates/family-identity-core/tests/unit/household_authority.rs`
  - `exit: 0`
  - `result: pass`
  - `artifact: n/a`
  - `notes: focused Rust architecture gate passed for the touched household_authority test file; crate-wide lint remains affected by pre-existing lib.rs re-export debt outside this slice`
- Proof artifacts:
  - `output/account-identity-family-plan-proof/02-identity-household-role-model/00-identity-entity-model-proof.md`
  - `output/account-identity-family-plan-proof/02-identity-household-role-model/01-role-action-resource-matrix.md`
  - `output/account-identity-family-plan-proof/02-identity-household-role-model/02-membership-state-machine-proof.md`
  - `output/account-identity-family-plan-proof/02-identity-household-role-model/03-cross-family-negative-proof.md`
  - `output/account-identity-family-plan-proof/02-identity-household-role-model/04-observer-read-only-proof.md`
  - `output/account-identity-family-plan-proof/02-identity-household-role-model/05-support-admin-boundary-proof.md`
  - `output/account-identity-family-plan-proof/02-identity-household-role-model/06-audit-event-proof.md`
  - `output/account-identity-family-plan-proof/02-identity-household-role-model/16-validation-commands.log`
- Known gaps/manual-required states: downstream audit-log pipeline/storage remains unproven here; session freshness and browser request-safety stay owned by WP03; invite/recovery stays owned by WP04; physical trusted-device proof remains external; WP07 and WP06 still need their own proof roots before any broader readiness claim.
- No-claim boundaries: do not claim browser session completion, invite/recovery completion, trusted-device bootstrap readiness, setup UI readiness, or whole-plan completion from this WP02 closure.

## 2026-08-17 live-code review correction

The accepted Rust source is now a bounded target-aware identity model, not a
complete provider runtime. The parent-controller actor and target child/profile/
device are separated for Pair, Register, Revoke, View, ChangePolicy, and Remote
actions. Current account, household, member, device, role, and same-family
identity come from opaque Account authority. Capability, lease, and step-up
actions reject because their owned authority sources are not present. A
production provider-to-authority caller is still absent, and the raw evaluator
remains diagnostic/legacy risk when fed caller-assembled facts.

The bounded source correction preserves the sealed WP08 boundary, derives
target identity from owned current state, and fails closed when a target action
requires capability/lease/step-up authority that is unavailable. Normal
expected-test, focused-validation, proof, PR, and DONE gates remain open; no
test or workpack completion is claimed.
