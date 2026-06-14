# Workpack 04: Child Install Permission Journey

Goal: define the child-agent install and permission journey from the parent perspective.

Owns: child-device setup steps, permission readiness labels, child disclosure handoff, platform-specific warnings, and recovery.

Handoff: `app-plan` owns child local service runtime; `child-agent-runtime-distribution-plan` owns child package artifacts; `portal-ux-household-surfaces-plan` owns rendered setup UI.

Expected shape:

- Parent portal generates child pairing code/link.
- Child opens the code/link on the child device.
- Base/bootstrap installer downloads.
- Tutorial/disclosure/consent are shown.
- Code is entered and validated.
- Full child agent package is selected and installed.
- Permissions are requested.
- Service starts.
- Signed hello/readiness is returned.
- Platform-specific child install steps and manual-required states stay explicit.

Expected proof:

- Permission matrix.
- Bootstrap child code state machine and negative proof.
- Missing/denied permission proof.
- Unsupported/degraded platform proof.
- Recovery and reinstall proof path.

Failure: treating installed process as fully provisioned child protection.

## Execution Detail

Minimum context:

- `docs/features/child-agent-local-service.md`
- `docs/expectations/platforms.md`
- `docs/expectations/tamper-uninstall-protection.md`
- `docs/plans/app-plan/AGENTS.md`
- `docs/plans/parent-desktop-runtime-package-plan/AGENTS.md`
- `docs/plans/setup-install-provisioning-plan/SETUP_STATE_MACHINE.md`
- `docs/plans/setup-install-provisioning-plan/CHILD_PERMISSION_MATRIX.md`

Agent decision tree:

- If child service runtime or platform adapter is in scope, route to `app-plan`.
- If package artifact or installer mechanics are in scope, route to `child-agent-runtime-distribution-plan` for child artifacts or `parent-client-runtime-distribution-plan` for parent artifacts.
- If parent-facing setup state is in scope, stay here.
- If sensitive capture/remote permissions are involved, require disclosure and custody handoffs.

Required output:

- Child bootstrap installer flow: parent portal pairing authority, child link/code entry, tutorial/disclosure/consent, code validation, full child agent package selection, install, permissions, service start, signed hello/readiness return.
- Platform permission matrix for child devices.
- Separation of installed, running, permissioned, paired, trusted, and policy-ready.
- Child disclosure requirements for screen, location, network, app/game, browser, and remote access.
- Manual-required and unsupported states.

Expected tests/proof names:

- `setup.bootstrap.child-code-state-machine`
- `setup.bootstrap.child-code-expired-rejected`
- `setup.bootstrap.child-code-revoked-rejected`
- `setup.bootstrap.child-code-replayed-rejected`
- `setup.bootstrap.child-code-wrong-household-rejected`
- `setup.bootstrap.child-code-wrong-profile-rejected`
- `setup.bootstrap.child-download-authorized-only`
- `setup.bootstrap.child-base-installer-disclosure-proof`
- `setup.bootstrap.child-full-package-selection-proof`
- `setup.child-install.platform-matrix`
- `setup.child-install.permission-matrix`
- `setup.child-install.missing-permission-degraded`
- `setup.child-install.installed-not-trusted`
- `setup.child-install.child-disclosure-visible`
- `setup.child-install.reinstall-recovery`

Proof artifact expectations:

- `04-child-platform-matrix-proof.md`
- `04-permission-matrix-proof.md`
- `04-missing-permission-negative-proof.md`
- `04-child-disclosure-proof.md`
- `04-reinstall-recovery-proof.md`
- `04-child-install-ui-proof.md`
- `bootstrap-child-code-flow-proof.md`
- `bootstrap-child-installer-disclosure-proof.md`
