<!-- agent-capsule -->

> Agent Capsule
> Plan: `setup-install-provisioning-plan`
> Doc: `WP07 First-Run Setup UI And State Machine`
> Kind: assigned implementation/research workpack.
> Read when: selected by WORKPACK_INDEX.md or explicit assignment.
> Stop rule: do not create UI-only fake setup; source/custody/readiness states must exist or blockers must be recorded.
> Proves: first-run setup UI/state-machine shape only after proof artifacts exist.
> Does not prove: account readiness, installer readiness, pairing readiness, or production setup readiness.
> Proof rule: before DONE, write all WP07 proof artifacts and command log.

<!-- /agent-capsule -->

# WP07 First-Run Setup UI And State Machine

## Goal

Define the exact first-run parent-visible setup sequence and state machine from public site to setup-complete/manual-required status.

## Required inputs

```text
workpacks/01-family-web-info-site.md
workpacks/02-registration-login-entry.md
workpacks/03-parent-install-journey.md
workpacks/04-child-install-permission-journey.md
workpacks/05-pairing-readiness-recovery.md
docs/expectations/family-setup.md
docs/expectations/portal.md
docs/expectations/release-installer.md
docs/expectations/platform-deliverables.md
docs/expectations/data-custody.md
docs/plans/setup-install-provisioning-plan/SETUP_STATE_MACHINE.md
docs/plans/setup-install-provisioning-plan/PAIRING_READINESS_MODEL.md
```

## Required screens/states

```text
Welcome
Sign in / create account
Create or join household
Parent install link / QR / code
Parent bootstrap tutorial / agreement
Parent bootstrap code entry
Parent package download / install progress
Parent portal guided setup start
Create child profile
Generate pairing link / QR / code
Child install instructions
Waiting for device
Device detected / confirm trust
Permission readiness checklist
Policy baseline setup
Data custody status
Setup complete
Setup blocked
Manual required
```

## Required state labels

```text
notImplemented
previewOnly
manualRequired
readyForTest
productionReady
blocked
stale
degraded
unavailable
```

## UI rules

```text
Keep account, parent bootstrap, child bootstrap, pairing, readiness, and recovery separate.
Show manual-required states explicitly.
Never claim setup complete until the readiness matrix is visible.
Render adjacent handoff blockers instead of hiding them.
Use source/custody labels for live local, LAN, parent cache, parent-owned storage, stale, degraded, unavailable, and manual-required.
```

## Expected source changes

Likely paths:

```text
packages/family-domain/src/** selected setup state contracts
packages/portal-domain/src/** selected setup route text/DOM ids
apps/portal/src/** selected setup route/components
apps/portal/tests/** selected setup tests
apps/portal/e2e/** selected setup proof
```

## Required proof root

```text
output/setup-install-provisioning-plan-proof/07-first-run-setup-ui-and-state-machine/
```

Required artifacts:

```text
00-first-run-state-machine-proof.md
01-first-run-ui-screen-map.md
02-empty-error-degraded-ui-proof.md
03-manual-required-visible-proof.md
04-adjacent-handoff-visible-proof.md
05-no-fake-ready-state-proof.md
06-source-custody-label-proof.md
16-validation-commands.log
```

## Acceptance criteria

- [ ] First-run setup state machine exists.
- [ ] Screen map covers the required screens.
- [ ] Empty/error/degraded UI states exist or blockers are recorded.
- [ ] Manual-required state is visible.
- [ ] Adjacent handoff blockers are visible.
- [ ] Source/custody labels are visible.
- [ ] Setup complete cannot render unless readiness matrix is satisfied or explicitly mocked as blocked.
- [ ] Portal tests or exact missing test blocker recorded.
- [ ] Focused commands pass or blockers are recorded.

## Focused commands

```bash
npm run build --workspace @ocentra-parent/family-domain
npm run build --workspace @ocentra-parent/portal-domain
npm run test --workspace @ocentra-parent/portal -- setup
npm run test:e2e --workspace @ocentra-parent/portal -- setup
npm run lint:architecture -- --files packages/family-domain packages/portal-domain apps/portal docs/plans/setup-install-provisioning-plan
```

If setup UI/e2e paths do not exist yet, write exact blockers and keep rows open.

## Negative states

- Setup complete appears without account state.
- Setup complete appears without parent app state.
- Setup complete appears without device/permission/pairing state.
- Manual-required state is hidden.
- Unsupported platform looks successful.
- UI implies a package or pairing owner claim that belongs to a sibling plan.

## Manual-required gaps

Production setup readiness remains blocked until account, distribution, child runtime, LAN/device trust, data custody, and policy baseline owner proofs exist.

## Fill before DONE

```text
Workpack id and branch:
Setup UI/state changes:
Touched files:
Validation commands and results:
Proof artifacts:
Known gaps/manual-required states:
No-claim boundaries:
```
