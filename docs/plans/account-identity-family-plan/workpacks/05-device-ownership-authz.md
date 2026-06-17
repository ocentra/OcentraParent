<!-- agent-capsule -->

> Agent Capsule
> Plan: `account-identity-family-plan`
> Doc: `WP05 Device Ownership AuthZ`
> Kind: assigned implementation workpack.
> Read when: selected by WORKPACK_INDEX.md or explicit assignment.
> Stop rule: do not implement LAN transport, remote transport, device-trust bootstrap, billing, policy, or export/delete mechanics here.
> Proves: account-family device ownership authorization only after tests/proof pass.
> Does not prove: physical device trust, LAN pairing, remote access execution, payment readiness, or data custody readiness.
> Proof rule: before DONE, write all WP05 proof artifacts and command log.

<!-- /agent-capsule -->

# WP05 Device Ownership AuthZ

## Goal

Define authorization for parent devices, child devices, service agents, remote capability sessions, export/delete, and billing ownership gates.

## Required inputs

```text
workpacks/02-identity-household-role-model.md
workpacks/03-session-token-lifecycle.md
RESEARCH_AND_DECISIONS.md
docs/features/child-agent-local-service.md
docs/features/remote-lan-mobile-platforms.md
docs/features/family-setup-device-roles.md
docs/expectations/family-setup.md
docs/expectations/platforms.md
packages/family-domain/src/household-authority.ts
packages/family-domain/tests/unit/household-authority.test.ts
```

## Required authority dimensions

```text
actor identity
household membership
role
device membership
device trust state
session freshness
capability grant
resource ownership
revocation status
controller lease when needed
```

## Expected action matrix

```text
create household
invite member
create child profile
pair child device
revoke child device
view child status
change policy
start remote view
start remote control
export data
delete data
manage billing
support/admin review
```

## Expected source changes

Likely paths:

```text
packages/family-domain/src/household-authority.ts
packages/family-domain/src/setup-lifecycle.ts
packages/family-domain/tests/unit/household-authority.test.ts
crates/agent-protocol/** only if typed protocol parity is needed
crates/agent-service/** only if selected service boundary proof is implemented
```

## Required proof root

```text
output/account-identity-family-plan-proof/05-device-ownership-authz/
```

Required artifacts:

```text
00-device-authority-matrix.md
01-revoked-device-negative-proof.md
02-wrong-household-negative-proof.md
03-controller-lease-proof.md
04-remote-capability-proof.md
05-export-delete-owner-proof.md
06-billing-owner-proof.md
16-validation-commands.log
```

## Acceptance criteria

- [ ] Actor/household/role/device/session/capability matrix exists.
- [ ] Parent controller authority is explicit.
- [ ] Observer read-only behavior is explicit.
- [ ] Child agent authority is scoped to its device/household only.
- [ ] Pending/trusted/revoked/disabled/stale device states are represented.
- [ ] Wrong-household denial proof exists.
- [ ] Controller lease required/expired/revoked proof exists.
- [ ] Remote view and remote control are separated by capability.
- [ ] Export/delete require parent owner authority and data-custody handoff.
- [ ] Billing requires parent owner authority and payment handoff.
- [ ] Audit event requirements are explicit.
- [ ] Focused commands pass or blockers are recorded.

## Focused commands

```bash
npm run build --workspace @ocentra-parent/family-domain
npm run test --workspace @ocentra-parent/family-domain -- device
npm run test --workspace @ocentra-parent/family-domain -- authorization
cargo test -p ocentra-parent-agent-protocol device
cargo test -p ocentra-parent-agent-service device
npm run lint:architecture -- --files packages/family-domain crates/agent-protocol crates/agent-service
```

If Rust/service paths are not touched, record `not-applicable` in the command log instead of forcing unrelated failures.

## Negative cases

- Wrong-household device action denied.
- Revoked device denied.
- Stale device denied.
- Observer denied write/control action.
- Remote view denied without remote-view capability.
- Remote control denied without remote-control capability.
- Export/delete denied without owner authority.
- Billing denied without parent-owner authority.

## Manual-required gaps

Physical trusted-device proof remains owned by `device-trust-bootstrap-plan`. LAN/remote transport execution remains owned by `lan-plan` and `remote-access-plan`.

## Fill before DONE

- Workpack id and branch: `WP05 Device Ownership AuthZ`; `codex/tracking-plan-full-continuation-a`.
- Current status: partial. Only `00-device-authority-matrix.md` and `16-validation-commands.log` exist under `output/account-identity-family-plan-proof/05-device-ownership-authz/`.
- Contract/source changes in this slice: none in WP05-owned device authority sources; current proof is derived from existing TypeScript, Rust parity, and provisioning-readiness coverage.
- Touched files:
  - `docs/plans/account-identity-family-plan/CHECKLIST_INDEX.md`
  - `docs/plans/account-identity-family-plan/PLAN_STATE.md`
  - `docs/plans/account-identity-family-plan/WORKPACK_INDEX.md`
  - `docs/plans/account-identity-family-plan/workpacks/05-device-ownership-authz.md`
  - `output/account-identity-family-plan-proof/05-device-ownership-authz/00-device-authority-matrix.md`
  - `output/account-identity-family-plan-proof/05-device-ownership-authz/16-validation-commands.log`
- Validation commands and results:
  - `command: npm run build --workspace @ocentra-parent/family-domain`
  - `exit: 0`
  - `result: pass`
  - `artifact: n/a`
  - `notes: family-domain builds after direct lifecycle helper repair`
  - `command: npm run test --workspace @ocentra-parent/family-domain -- tests/unit/household-authority.test.ts tests/unit/session-lifecycle.test.ts tests/unit/token-lifecycle.test.ts`
  - `exit: 0`
  - `result: pass`
  - `artifact: n/a`
  - `notes: direct household/session/token suite passed with 22 tests`
  - `command: cargo test -p ocentra-family-identity-core`
  - `exit: 0`
  - `result: pass`
  - `artifact: n/a`
  - `notes: Rust parity suite passed with device authority, session, and setup negatives`
  - `command: cargo test -p ocentra-provisioning-core readiness`
  - `exit: 0`
  - `result: pass`
  - `artifact: n/a`
  - `notes: provisioning consumer suite passed with 29 readiness and family-context tests`
- Proof artifacts:
  - `output/account-identity-family-plan-proof/05-device-ownership-authz/00-device-authority-matrix.md`
  - `output/account-identity-family-plan-proof/05-device-ownership-authz/16-validation-commands.log`
- Known gaps/manual-required states: `01-revoked-device-negative-proof.md`, `02-wrong-household-negative-proof.md`, `03-controller-lease-proof.md`, `04-remote-capability-proof.md`, `05-export-delete-owner-proof.md`, and `06-billing-owner-proof.md` are still missing; physical trusted-device proof remains owned by `device-trust-bootstrap-plan`; LAN/remote transport execution remains external.
- No-claim boundaries: do not claim full device-state closure, wrong-household proof completion, remote transport execution, physical trusted-device bootstrap, or WP05 completion.
