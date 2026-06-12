# Workpack 03: Parent Install Journey

Goal: define how a parent gets from the family site/account to a trusted parent app or portal controller.

Owns: download selection, platform eligibility, installer copy, install progress labels, update-channel handoff, and support/degraded states.

Handoff: `parent-desktop-runtime-package-plan` owns package build, signing, notarization, update, rollback, and artifact proof.

Expected shape:

- Platform detection is advisory; user can choose platform manually.
- Every platform has one of: supported, preview, manual-required, unavailable, or planned.
- Download integrity and version visibility are user-visible when available.
- Failed install and unsupported OS states link to support/recovery.

Expected proof:

- Platform matrix.
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

Agent decision tree:

- If the task is installer artifact generation/signing/update, route to `parent-desktop-runtime-package-plan`.
- If the task is what the parent sees and how setup progresses, stay in this workpack.
- If install requires login or entitlement, route to `account-identity-family-plan` or `payment-subscription-plan` for that boundary.

Required output:

- Platform install matrix: Windows, macOS, Linux, Android parent, iOS parent, web-only fallback.
- States: available, preview, manual-required, unsupported, blocked by permissions, failed, installed, update-required.
- Download integrity and version display expectations.
- Support/recovery routes for failed install and wrong platform.

Expected tests/proof names:

- `parent-install.platform-matrix`
- `parent-install.download-integrity-visible`
- `parent-install.unsupported-platform-state`
- `parent-install.update-channel-handoff`
- `parent-install.no-fake-installed-state`

Proof artifact expectations:

- Package artifact refs from package plan.
- Screenshot proof for download/unsupported/failure states.
- Version and checksum display proof when implemented.
