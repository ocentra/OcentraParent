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
Workpack id and branch:
Parent install journey changes:
Touched files:
Validation commands and results:
Proof artifacts:
Known gaps/manual-required states:
No-claim boundaries:
```
