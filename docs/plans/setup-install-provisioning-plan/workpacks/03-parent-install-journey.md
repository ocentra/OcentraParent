<!-- agent-capsule -->

> Agent Capsule
> Plan: `setup-install-provisioning-plan`
> Doc: `WP03 Parent Install Journey`
> Kind: assigned implementation/research workpack.
> Read when: selected by WORKPACK_INDEX.md or explicit assignment.
> Stop rule: do not implement package build, signing, notarization, update, rollback, or entitlement here.
> Proves: parent-visible install journey/state only after proof artifacts exist.
> Does not prove: signed package readiness, updater readiness, store readiness, or product install readiness.
> Proof rule: before DONE, write all WP03 proof artifacts and command log.

<!-- /agent-capsule -->

# WP03 Parent Install Journey

## Goal

Define how a parent gets from family site/account state to a parent app/controller install journey with honest platform, version, bootstrap, integrity, and recovery states.

## Ownership boundary

```text
setup-install-provisioning-plan owns parent-visible bootstrap/install journey state, platform matrix, version/integrity labels, and runtime-distribution handoff.
parent-desktop-runtime-package-plan owns package build, signing, notarization, store delivery, update, rollback, installer checksum/signature execution, and production publishing.
payment-subscription-plan owns entitlement if a package route depends on billing.
```

## Required inputs

```text
RESEARCH_AND_DECISIONS.md
docs/plans/parent-desktop-runtime-package-plan/AGENTS.md
docs/expectations/release-installer.md
docs/expectations/platform-deliverables.md
docs/features/production-distribution-support.md
docs/plans/setup-install-provisioning-plan/SETUP_STATE_MACHINE.md
```

## Owned scope

```text
parent bootstrap code/link visible state
parent platform selection UX
parent download/version/integrity display expectation
install progress labels
unsupported/manual-required/update-required states
handoff to runtime distribution owner
support/recovery links
```

## Out of scope

```text
package build
code signing
notarization
store packaging
update server
rollback implementation
payment entitlement
```

## Required proof fields

The selected proof must name, at minimum:

```text
parent_bootstrap_state
platform_matrix_state
download_state
version_display_state
integrity_display_state
unsupported_state
manual_required_state
update_required_state
runtime_distribution_handoff_state
package_wrapper_state
support_recovery_state
signed_installer_state
notarization_state
store_delivery_state
update_rollback_state
payment_entitlement_state
no_package_ready_claim
no_product_install_claim
no_claim
```

These are proof-routing fields, not implementation code prescriptions.

## Required output

```text
parent bootstrap state machine
platform matrix: Windows, macOS, Linux, Android parent, iOS parent, web-only fallback
states: unsupported, planned, previewOnly, manualRequired, readyForTest, productionReady, blocked
version/integrity display expectations
runtime distribution handoff contract
support/recovery route states
```

## Required proof root

```text
output/setup-install-provisioning-plan-proof/03-parent-install-journey/
```

Required artifacts:

```text
00-parent-bootstrap-code-state-proof.md
01-parent-platform-matrix-proof.md
02-download-integrity-proof.md
03-unsupported-platform-proof.md
04-update-rollback-handoff-proof.md
05-parent-install-ui-proof.md
16-validation-commands.log
```

## Acceptance criteria

- [ ] Parent bootstrap code/link state machine exists.
- [ ] Platform matrix exists.
- [ ] Download/version/integrity display expectation exists.
- [ ] Unsupported/manual-required/update-required states are visible.
- [ ] Runtime distribution handoff is explicit.
- [ ] Download button cannot imply signed package readiness.
- [ ] Focused commands pass or blocker recorded.

## Focused commands

```bash
node -e "console.log('parent-install-journey-handoff')"
npm run lint:architecture -- --files docs/plans/setup-install-provisioning-plan docs/plans/parent-desktop-runtime-package-plan
```

If UI routes exist later:

```bash
npm run test --workspace @ocentra-parent/portal -- setup
npm run test:e2e --workspace @ocentra-parent/portal -- setup
```

## Negative states

- Website download button claims production installer readiness without package proof.
- Platform detection forces wrong platform without manual choice.
- Unsupported OS looks successful.
- Update-required state is hidden.
- Parent installed state implies child device readiness.

## Manual-required gaps

Signed installers, notarization, store delivery, update/rollback, and package artifact verification remain owned by runtime distribution plans.

## Fill before DONE

```text
Workpack id and branch: WP03 Parent Install Journey / codex/tracking-plan-full-continuation-a
Parent install journey changes: proved the parent bootstrap state machine, platform matrix, version/integrity display expectations, unsupported/manual-required/update-required visibility, runtime distribution handoff boundary, and support/recovery link expectations without claiming signed package readiness.
Touched files: output/setup-install-provisioning-plan-proof/03-parent-install-journey/00-parent-bootstrap-code-state-proof.md, output/setup-install-provisioning-plan-proof/03-parent-install-journey/01-parent-platform-matrix-proof.md, output/setup-install-provisioning-plan-proof/03-parent-install-journey/02-download-integrity-proof.md, output/setup-install-provisioning-plan-proof/03-parent-install-journey/03-unsupported-platform-proof.md, output/setup-install-provisioning-plan-proof/03-parent-install-journey/04-update-rollback-handoff-proof.md, output/setup-install-provisioning-plan-proof/03-parent-install-journey/05-parent-install-ui-proof.md, output/setup-install-provisioning-plan-proof/03-parent-install-journey/16-validation-commands.log, docs/plans/setup-install-provisioning-plan/CHECKLIST_INDEX.md, docs/plans/setup-install-provisioning-plan/WORKPACK_INDEX.md, docs/plans/setup-install-provisioning-plan/PLAN_STATE.md, docs/plans/setup-install-provisioning-plan/workpacks/03-parent-install-journey.md, output/setup-install-provisioning-plan-proof/06-rollout-proof-and-route-gate/00-rollout-proof-pack.md, output/setup-install-provisioning-plan-proof/06-rollout-proof-and-route-gate/04-manual-required-gap-register.md, docs/plans/setup-install-provisioning-plan/workpacks/06-rollout-proof-and-route-gate.md
Validation commands and results: `node -e "console.log('parent-install-journey-handoff')"` PASS; `npm run lint:architecture -- --files packages/setup-domain/src/setup-state-machine.ts packages/setup-domain/tests/unit/setup-state-machine.test.ts packages/production-domain/src/production-release-public-runtime-handoff-values.ts packages/production-domain/src/production-release-public-runtime-handoff-read-model.ts packages/production-domain/tests/unit/production-release-public-runtime-handoff.test.ts packages/parent-domain/src/parent-desktop-release-support.ts packages/parent-domain/tests/unit/parent-desktop-release-support.test.ts packages/parent-domain/tests/unit/parent-desktop-release-support-fixtures.ts docs/plans/setup-install-provisioning-plan docs/plans/parent-desktop-runtime-package-plan` PASS; `npm run build --workspace @ocentra-parent/setup-domain` PASS; `npm run test --workspace @ocentra-parent/setup-domain -- setup-state-machine` PASS; `npm run build --workspace @ocentra-parent/production-domain` PASS; `npm run test --workspace @ocentra-parent/production-domain -- production-release-public-runtime-handoff` PASS; `npm run build --workspace @ocentra-parent/parent-domain` PASS after the export-map repair in the adjacent billing/production/network package manifests; `npm run test --workspace @ocentra-parent/parent-domain -- parent-desktop-release-support` BLOCKED because the workspace wrapper detours into `scripts/test/app-game-source-gated-policy-preview-read-model-proof.mjs` and requests nonexistent `app-game-*` filters; `Push-Location packages/parent-domain; npx vitest run tests/unit/parent-desktop-release-support.test.ts; Pop-Location` PASS.
Proof artifacts: output/setup-install-provisioning-plan-proof/03-parent-install-journey/00-parent-bootstrap-code-state-proof.md, output/setup-install-provisioning-plan-proof/03-parent-install-journey/01-parent-platform-matrix-proof.md, output/setup-install-provisioning-plan-proof/03-parent-install-journey/02-download-integrity-proof.md, output/setup-install-provisioning-plan-proof/03-parent-install-journey/03-unsupported-platform-proof.md, output/setup-install-provisioning-plan-proof/03-parent-install-journey/04-update-rollback-handoff-proof.md, output/setup-install-provisioning-plan-proof/03-parent-install-journey/05-parent-install-ui-proof.md, output/setup-install-provisioning-plan-proof/03-parent-install-journey/16-validation-commands.log
Known gaps/manual-required states: signed installers, notarization, store delivery, checksum/signature execution proof, updater rollback execution, and production publishing remain owned by `docs/plans/parent-desktop-runtime-package-plan`; the remaining adjacent `@ocentra-parent/parent-domain` gap is the workspace test-wrapper path, which currently detours into `scripts/test/app-game-source-gated-policy-preview-read-model-proof.mjs` instead of proving `parent-desktop-release-support` directly; rendered first-run setup UI proof is now green in WP07.
No-claim boundaries: no package build, signing, notarization, store packaging, update server, rollback execution, payment entitlement, or product install readiness claim.
```
