# Workpack 03: Parent Install Journey

Goal: define how a parent gets from the family site/account to a trusted parent app or portal controller.

Owns: bootstrap code entry, download selection, platform eligibility, installer copy, install progress labels, update-channel handoff, and support/degraded states.

Handoff: `parent-client-runtime-distribution-plan` owns package build, signing, notarization, update, rollback, and artifact proof.

Expected shape:

- Platform detection is advisory; user can choose platform manually.
- Parent install is a bootstrap code flow first, then a full parent package install.
- Every platform has one of: supported, preview, manual-required, unavailable, or planned.
- Download integrity, bootstrap code state, and version visibility are user-visible when available.
- Failed install and unsupported OS states link to support/recovery.

Expected proof:

- Platform matrix.
- Bootstrap parent code state machine and negative proof.
- Download/version/integrity artifact references.
- UI proof for supported, unavailable, and manual-required states.
- Handoff proof to package plan.

Failure: claiming production installer readiness from a website download button alone.

## Execution Detail

Minimum context:

- `docs/plans/parent-desktop-runtime-package-plan/AGENTS.md`
- `docs/expectations/release-installer.md`
- `docs/expectations/platform-deliverables.md`
- `docs/features/production-distribution-support.md`
- `docs/plans/setup-install-provisioning-plan/SETUP_STATE_MACHINE.md`

Agent decision tree:

- If the task is installer artifact generation/signing/update, route to `parent-client-runtime-distribution-plan`.
- If the task is what the parent sees and how setup progresses, stay in this workpack.
- If install requires login or entitlement, route to `account-identity-family-plan` or `payment-subscription-plan` for that boundary.

Required output:

- Parent bootstrap installer flow: tutorial, agreement, consent, code entry, code validation, full package selection, full package download, full parent portal install, parent portal launch.
- Platform install matrix: Windows, macOS, Linux, Android parent, iOS parent, web-only fallback.
- States: available, preview, manual-required, unsupported, blocked by permissions, failed, installed, update-required.
- Download integrity and version display expectations.
- Support/recovery routes for failed install and wrong platform.

Expected tests/proof names:

- `setup.bootstrap.parent-code-state-machine`
- `setup.bootstrap.parent-code-expired-rejected`
- `setup.bootstrap.parent-code-revoked-rejected`
- `setup.bootstrap.parent-code-wrong-household-rejected`
- `setup.bootstrap.parent-code-wrong-role-rejected`
- `setup.bootstrap.parent-download-authorized-only`
- `setup.bootstrap.parent-base-installer-no-child-data`
- `setup.bootstrap.parent-full-package-selection-proof`
- `setup.parent-install.platform-matrix`
- `setup.parent-install.download-integrity-visible`
- `setup.parent-install.unsupported-platform-state`
- `setup.parent-install.update-channel-handoff`
- `setup.parent-install.no-fake-installed-state`

Proof artifact expectations:

- `03-parent-platform-matrix-proof.md`
- `03-download-integrity-proof.md`
- `03-unsupported-platform-proof.md`
- `03-update-rollback-handoff-proof.md`
- `03-parent-install-ui-proof.md`
- `bootstrap-parent-code-flow-proof.md`
- `bootstrap-parent-installer-proof.md`
