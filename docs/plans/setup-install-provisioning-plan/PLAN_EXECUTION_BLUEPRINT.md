# Setup + Install + Provisioning Plan � HID Execution Blueprint

## Execution objective

Create deterministic first-run flow from website to paired household-ready state with failure paths and manual recovery.

## Slice 01 � Website-to-Account Handoff

### Acceptance

- Registration/login handoff into account/household identity is auditable.

### Tests

- `setup.account-handoff.authn-authz`

### Proof

- `docs/proof/setup-install-provisioning-plan/slice-01-handoff.md`

## Slice 02 � First-Run State Machine

### Acceptance

- Parent/device/permission lifecycle includes success, partial failure, and rollback branches.

### Tests

- `setup.first-run.state-machine`

### Proof

- `docs/proof/setup-install-provisioning-plan/slice-02-state-machine.md`

## Slice 03 � Platform Install Matrix

### Acceptance

- Platform installers/install states are verified with smoke coverage.

### Tests

- `setup.platform.install.smoke`

### Proof

- `docs/proof/setup-install-provisioning-plan/slice-03-platform-install.md`

## Slice 04 � Recovery and Manual States

### Acceptance

- Recovery/manual handoff path is explicit and tested.

### Tests

- `setup.recovery.manual-fallback`

### Proof

- `docs/proof/setup-install-provisioning-plan/slice-04-recovery.md`

## Workpacks (execution lane)

### Slice-to-workpack binding

- Slice 01: docs/plans/setup-install-provisioning-plan/workpacks/01-family-web-info-site.md
- Slice 02: docs/plans/setup-install-provisioning-plan/workpacks/02-registration-login-entry.md
- Slice 03: docs/plans/setup-install-provisioning-plan/workpacks/03-parent-install-journey.md
- Slice 04: docs/plans/setup-install-provisioning-plan/workpacks/04-child-install-permission-journey.md

## PR-ready gate

- No setup claim until end-to-end state machine and artifacts matrix are attached.

## HID test floor (this plan)

### Required test families for closed slice

- Unit: identity/site/setup contract checks
- Integration: website-to-runtime handoff
- E2E: first-run install + recovery paths
- Security: onboarding abuse and unauthorized state entry
- Non-functional: platform matrix and manual fallback behavior

### Mandatory slice evidence checks

- negative cases documented (at least one per slice)
- rollback/teardown proof recorded
- proof manifest references command output, artifacts, and manual review notes
