# 07 Windows Installer And Preview

<!-- agent-capsule -->

> Agent Capsule
> Plan: `parent-desktop-runtime-package-plan`
> Doc: `07 Windows Installer And Preview`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

Sources: [20-step plan](../parent-desktop-runtime-package-20-step-plan.md),
[test blueprint](../parent-desktop-runtime-package-test-blueprint.md),
[requirements guide](../runtime-package-requirements-guide.md), and
[folder README](../README.md).

## Where We Are

Windows MSI/updater scaffolding and package preview mechanics exist. Production
signing is not claimed.

## Where We Want To Be

Windows package preview can be built and smoke-checked while signing and release
states remain explicit.

## Decision Tree

| If the assignment touches...        | Read next                                                                                            | Required handoff                                     |
| ----------------------------------- | ---------------------------------------------------------------------------------------------------- | ---------------------------------------------------- |
| Public download/install journey     | `../../setup-install-provisioning-plan/AGENTS.md`                                                    | family site/download wording and account setup route |
| Windows package preview             | this workpack, package scripts, and `../TEST_PROOF_EXPECTATIONS.md`                                  | package artifact proof                               |
| Signing/notarization/store claim    | WP10 signing/notarization/store claims                                                               | no production claim without certificate/store proof  |
| Update/rollback                     | WP09 update channel and rollback scaffold                                                            | rollback proof path                                  |
| Parent login/pairing during install | `../../account-identity-family-plan/AGENTS.md` and `../../setup-install-provisioning-plan/AGENTS.md` | account/session/device authority route               |

## Expected Install Journey

- Parent starts from the public family site or approved internal preview link.
- Download page states platform, version, checksum/signature status, preview/production status, privacy posture, and support limits.
- Installer creates only the parent shell/runtime package it owns; child-agent install, permissions, pairing, and account authority are separate routes.
- First launch must route parent to login/session state, household selection, device pairing, service availability, and support diagnostics without pretending setup is complete.
- Unsigned/dev builds must remain visibly labeled in docs, UI proof, and release notes.

## Requirement Checklist

- [ ] Build or verify Windows package preview where available.
- [ ] Smoke launch the parent shell where feasible.
- [ ] Label unsigned/dev preview.
- [ ] Keep production release boundary explicit.
- [ ] Record artifacts/commands in reports.
- [ ] Verify installer/download copy does not claim signed production readiness unless proved.
- [ ] Record checksum/signature/update-channel status or explicit missing proof.
- [ ] Link setup/account/pairing handoffs instead of duplicating those plans.

## Acceptance And Proof

Package proof shows preview mechanics without claiming signed production
installer readiness.

Expected proof names:

- `parent-desktop.windows-package-preview.build-log`
- `parent-desktop.windows-package-preview.artifact-manifest`
- `parent-desktop.installer.unsigned-preview-label-proof`
- `parent-desktop.installer.checksum-signature-status`
- `parent-desktop.first-launch.setup-handoff-proof`
- `parent-desktop.update-channel.boundary-note`

Proof must include artifact path, version/build identity, command log, environment, platform, and a clear `preview` or `production` label.

## Failure Conditions

- Do not claim signed installer, auto-update, store distribution, enterprise deployment, or production support without the corresponding proof.
- Do not merge parent installer, child agent installer, account login, and pairing into one undocumented flow.
- Do not use a successful local build as evidence of public download readiness.

## Parallel Ownership Notes

Primary watches CI before merge. D owns branch fixes.
