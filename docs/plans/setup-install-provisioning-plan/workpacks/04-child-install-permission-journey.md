# Workpack 04: Child Install Permission Journey

Goal: define the child-agent install and permission journey from the parent perspective.

Owns: child-device setup steps, permission readiness labels, child disclosure handoff, platform-specific warnings, and recovery.

Handoff: `app-plan` owns child local service runtime; `parent-desktop-runtime-package-plan` owns package artifacts; `portal-ux-household-surfaces-plan` owns rendered setup UI.

Expected shape:

- Platform-specific child install steps and manual-required states.
- Permission checklist for capture, location, network, app/game, notifications, remote access, and background service where applicable.
- Parent-visible readiness state separates installed, running, permissioned, paired, and trusted.
- Child-visible disclosure is required for sensitive capture/remote features.

Expected proof:

- Permission matrix.
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

Agent decision tree:

- If child service runtime or platform adapter is in scope, route to `app-plan`.
- If package artifact or installer mechanics are in scope, route to `parent-desktop-runtime-package-plan`.
- If parent-facing setup state is in scope, stay here.
- If sensitive capture/remote permissions are involved, require disclosure and custody handoffs.

Required output:

- Platform permission matrix for child devices.
- Separation of installed, running, permissioned, paired, trusted, and policy-ready.
- Child disclosure requirements for screen, location, network, app/game, browser, and remote access.
- Manual-required and unsupported states.

Expected tests/proof names:

- `child-install.permission-matrix`
- `child-install.missing-permission-degraded`
- `child-install.installed-not-trusted`
- `child-install.child-disclosure-visible`
- `child-install.reinstall-recovery`

Proof artifact expectations:

- Permission screenshots or platform proof artifacts when available.
- Redacted setup logs.
- Recovery path note for denied permission and reinstall.
