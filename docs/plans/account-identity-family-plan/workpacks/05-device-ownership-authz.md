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
workpacks/00-owner-boundary-proof-gate.md
workpacks/02-identity-household-role-model.md
workpacks/03-session-token-lifecycle.md
RESEARCH_AND_DECISIONS.md
docs/features/child-agent-local-service.md
docs/features/remote-lan-mobile-platforms.md
docs/features/family-setup-device-roles.md
docs/expectations/family-setup.md
docs/expectations/platforms.md
packages/schema-domain account/device/capability exports when shared shape changes are required
packages/family-domain/src/household-authority.ts
packages/family-domain/tests/unit/household-authority.test.ts
crates/family-identity-core/** only when Rust parity is selected
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
packages/schema-domain/** only when canonical shared device/capability/action shapes change
packages/family-domain/src/household-authority.ts
packages/family-domain/src/setup-lifecycle.ts
packages/family-domain/tests/unit/household-authority.test.ts
crates/family-identity-core/** only when Rust parity is selected
crates/agent-protocol/** only if typed protocol parity is needed
crates/agent-service/** only if selected service boundary proof is implemented
```

## Current owner/import/proof constraints

This workpack owns account-family authorization for device/resource/capability decisions. It does not own physical trusted-device proof, LAN transport, remote transport, payment execution, policy runtime, or data-custody execution.

```text
schema-domain: canonical shared device/capability/action shapes when cross-boundary.
family-domain: helper/projection and TypeScript authority tests.
family-identity-core: Rust parity only when selected.
agent-protocol/agent-service: protocol/service only when selected.
device-trust/LAN/remote/payment/policy/data-custody: adjacent owners only.
```

Allowed direct imports are limited to `schema-domain`, neutral protocol/evidence/logging/capability primitives, approved `family-domain` helpers, selected Rust/protocol/service crates when the workpack names them, and pure common helpers. Do not import LAN, remote, payment, policy, data-custody, or device-trust runtime internals to prove account-family authorization.

Proof must keep capability authorization separate from execution. Remote-view/remote-control authorization is not remote transport readiness. Export/delete or billing authorization is not data-custody/payment execution.

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

If canonical schema or Rust parity changes:

```bash
npm run build --workspace @ocentra-parent/schema-domain
cargo test -p ocentra-family-identity-core household_authority
```

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

## 2026-08-17 current code/test correction

The household/device evaluators, signed-current-state proof, record-derived
handoff, provisioning/policy/child-runtime consumers, and focused tests are
real. The general reachable API nevertheless accepts caller-assembled
ownership, trust, freshness, capability, and controller-lease state. The safer
record-derived path is not the universal production entrypoint, and several
authorized actions have no typed downstream consumer.

Production source still required:

- one runtime composer that resolves WP08 household/device binding, Device
  Trust current state, session freshness, scoped capability, lease, and step-up
  receipt from their owning repositories;
- view/control-specific grants and identity/action-bound leases with trusted
  expiry/revocation;
- mandatory step-up consumption and durable audit emission;
- typed remote, export, delete, and billing handoffs instead of a decision flag.

Expected test source still required:

- view grant cannot authorize control;
- lease identity/action/expiry/revoke/rebind races;
- missing/replayed/mismatched step-up denial;
- audit persistence failure and retry;
- real downstream consumers for remote, export, delete, and billing authority.

### Accepted replacement source delta

The canonical Account/Payment reconciliation now binds
billing and support/admin operations to current repository-derived account,
household, role, device, session, target, and support-receipt authority.
Request bodies, provider objects, fixtures, and fallback identity strings
cannot mint ownership. Device Trust step-up, view/control lease composition,
remote/export/delete consumers, durable audit emission, and the full expected
test matrix remain open.

The remote packet `ac03afee3a` is rejected/quarantined because its new public
records did not connect to this composer or any production caller and weakened
authority provenance. It is not WP05 progress.

## Fill before DONE

- Workpack id and branch: `WP05 Device Ownership AuthZ`; `codex/tracking-plan-full-continuation-a`.
- Current branch note: this historical completion record predates the plan-harness branch. On `codex/plan-harness-update`, treat it as prior proof evidence only; new edits must follow `workpacks/00-owner-boundary-proof-gate.md`, `TEST_PROOF_EXPECTATIONS.md`, and `PROOF_INDEX.md`.
- Current status: complete for the local contract/proof slice. `00-device-authority-matrix.md`, `01-revoked-device-negative-proof.md`, `02-wrong-household-negative-proof.md`, `03-controller-lease-proof.md`, `04-remote-capability-proof.md`, `05-export-delete-owner-proof.md`, `06-billing-owner-proof.md`, and `16-validation-commands.log` now exist under `output/account-identity-family-plan-proof/05-device-ownership-authz/`.
- Contract/source changes in this slice: no new WP05-owned production TypeScript or Rust authority logic was required beyond the earlier WP04 repair in `packages/family-domain/src/setup-lifecycle.ts`. This slice only added owner-only export/delete assertions in the shared authority tests at `packages/family-domain/tests/unit/household-authority.test.ts` and `crates/family-identity-core/tests/unit/household_authority.rs` so the proof root could close honestly.
- Touched files:
  - `packages/family-domain/tests/unit/household-authority.test.ts`
  - `crates/family-identity-core/tests/unit/household_authority.rs`
  - `docs/plans/account-identity-family-plan/CHECKLIST_INDEX.md`
  - `docs/plans/account-identity-family-plan/PLAN_STATE.md`
  - `docs/plans/account-identity-family-plan/WORKPACK_INDEX.md`
  - `docs/plans/account-identity-family-plan/workpacks/05-device-ownership-authz.md`
  - `output/account-identity-family-plan-proof/05-device-ownership-authz/00-device-authority-matrix.md`
  - `output/account-identity-family-plan-proof/05-device-ownership-authz/01-revoked-device-negative-proof.md`
  - `output/account-identity-family-plan-proof/05-device-ownership-authz/02-wrong-household-negative-proof.md`
  - `output/account-identity-family-plan-proof/05-device-ownership-authz/03-controller-lease-proof.md`
  - `output/account-identity-family-plan-proof/05-device-ownership-authz/04-remote-capability-proof.md`
  - `output/account-identity-family-plan-proof/05-device-ownership-authz/05-export-delete-owner-proof.md`
  - `output/account-identity-family-plan-proof/05-device-ownership-authz/06-billing-owner-proof.md`
  - `output/account-identity-family-plan-proof/05-device-ownership-authz/16-validation-commands.log`
- Validation commands and results:
  - `command: npm run test --workspace @ocentra-parent/family-domain -- tests/unit/household-authority.test.ts`
  - `exit: 0`
  - `result: pass`
  - `artifact: n/a`
  - `notes: added direct export/delete owner-only assertions; suite now passes with 14 tests`
  - `command: cargo test -p ocentra-family-identity-core household_authority`
  - `exit: 0`
  - `result: pass`
  - `artifact: n/a`
  - `notes: Rust parity household-authority subset passed with 12 tests including export/delete owner-only coverage`
  - `command: npm run build --workspace @ocentra-parent/family-domain`
  - `exit: 0`
  - `result: pass`
  - `artifact: n/a`
  - `notes: family-domain build passed after the local WP04 repair and before WP05 proof closure`
  - `command: npm run test --workspace @ocentra-parent/family-domain -- tests/unit/household-authority.test.ts tests/unit/session-lifecycle.test.ts tests/unit/token-lifecycle.test.ts`
  - `exit: 0`
  - `result: pass`
  - `artifact: n/a`
  - `notes: direct household/session/token suite now passes with 24 tests after the export/delete owner-only additions`
  - `command: cargo test -p ocentra-provisioning-core readiness`
  - `exit: 0`
  - `result: pass`
  - `artifact: n/a`
  - `notes: provisioning readiness consumer suite passed with 29 tests after the WP05 proof fill-in`
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
  - `output/account-identity-family-plan-proof/05-device-ownership-authz/00-device-authority-matrix.md`
  - `output/account-identity-family-plan-proof/05-device-ownership-authz/01-revoked-device-negative-proof.md`
  - `output/account-identity-family-plan-proof/05-device-ownership-authz/02-wrong-household-negative-proof.md`
  - `output/account-identity-family-plan-proof/05-device-ownership-authz/03-controller-lease-proof.md`
  - `output/account-identity-family-plan-proof/05-device-ownership-authz/04-remote-capability-proof.md`
  - `output/account-identity-family-plan-proof/05-device-ownership-authz/05-export-delete-owner-proof.md`
  - `output/account-identity-family-plan-proof/05-device-ownership-authz/06-billing-owner-proof.md`
  - `output/account-identity-family-plan-proof/05-device-ownership-authz/16-validation-commands.log`
- Known gaps/manual-required states: physical trusted-device proof remains owned by `device-trust-bootstrap-plan`; LAN/remote transport execution remains owned by `lan-plan` and `remote-access-plan`; payment runtime and data-custody execution remain external; crate-wide `cargo lint-architecture crates/family-identity-core` is still red because of pre-existing `src/lib.rs` re-export debt outside this slice, so validation stayed file-scoped as required.
- No-claim boundaries: do not claim physical trusted-device bootstrap, LAN/remote runtime execution, payment runtime readiness, data-custody execution, WP07 UI readiness, or whole-plan completion from this WP05 closure.
