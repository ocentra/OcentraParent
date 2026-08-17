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

## Ownership boundary

```text
setup-install-provisioning-plan owns selected first-run route state-machine projection, readiness cards, source/custody labels, and adjacent handoff visibility.
portal-domain/apps/portal own only the selected setup route/panel proof when named here.
portal-ux-household-surfaces-plan owns broader portal shell and visual completion.
sibling plans own account, package, runtime, LAN, device trust, custody, policy, and entitlement readiness.
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

## Required proof fields

The selected proof must name, at minimum:

```text
first_run_state_machine_state
screen_map_state
welcome_state
sign_in_state
household_state
parent_install_state
child_profile_state
child_install_state
pairing_state
permission_readiness_state
policy_baseline_state
data_custody_state
setup_complete_state
setup_blocked_state
manual_required_state
empty_error_degraded_state
adjacent_handoff_state
source_custody_label_state
no_fake_ready_state
account_owner_state
package_owner_state
child_runtime_owner_state
lan_device_trust_owner_state
custody_policy_owner_state
payment_owner_state
no_product_onboarding_claim
no_claim
```

These are proof-routing fields, not implementation code prescriptions.

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

## 2026-08-17 source-wave status

The bounded Rust source packet is accepted at
`a368bced832753e468a81f087930ca48d15f2116`. The Start route is now Rust-owned
through `crates/parent-runtime-core/src/setup_first_run.rs` and exposes all 15
required authorities as explicit manual-required/unavailable states. LAN
selected-device, pairing, and reachability values are observation-only and do
not establish household ownership, device trust, pairing authority, or setup
readiness. Start reads status only: the provisioning evaluator and action
planner are not invoked.

`crates/agent-protocol/src/transport.rs` is the canonical source for
`AgentCommandName::is_lan_command` across all 13 LAN command variants.
`crates/parent-runtime-core/src/parent_ui_bridge/action_dispatch/generic_command.rs`
rejects a LAN command on a non-LAN route, while the parent action boundary
rejects LAN discovery outside LAN-owned routes.

This source wave intentionally does not run or claim tests, builds, proof,
precommit, CI, or PR readiness. Deferred tests are exact: update
`crates/parent-runtime-core/tests/integration/parent_ui_bridge/snapshot_and_dispatch_tests.rs`
for the new Start snapshot and non-LAN rejection behavior; add coverage for
all 13 classifier variants; then update the existing portal setup route unit
and E2E fixtures and run the focused commands listed above. The current graph
completion contract therefore remains open for implementation review, tests,
proof, and checklist evidence.

Authenticated composition remains missing for account/session/household,
signed parent package, child package/service/permission, device trust, trusted
LAN pairing, custody sync, policy baseline, network reachability, and recovery.
No setup-complete, production-onboarding, trusted-device, signed-installer,
child-readiness, LAN-pairing, custody, policy, payment, test, proof, CI, or
PR-ready claim is made.

## Historical proof record (not current status)

The following retained block belongs to an earlier portal-projection packet. It
is preserved for provenance only and is not revalidated, regenerated, or
promoted by the 2026-08-17 source wave. The current status is source accepted;
tests, builds, proof, precommit, CI, and PR remain deferred.

```text
Historical proof packet (not refreshed by this source wave): WP07 First-Run Setup UI And State Machine / codex/tracking-plan-full-continuation-a
Setup UI/state changes: projected the typed setup-domain first-run state machine into `packages/portal-domain/src/setup-first-run-panel.ts`, rendered it on `PortalRoute.Start` through `apps/portal/src/SetupFirstRunRoutePanel.tsx`, added focused portal-domain, portal render, and portal Playwright proof coverage, and repaired the portal dev-log fallback so the first-run proof harness now completes end to end on this host.
Touched files: packages/portal-domain/package.json, packages/portal-domain/src/setup-first-run-panel.ts, packages/portal-domain/tests/unit/setup-first-run-panel.test.ts, apps/portal/src/ParentPortalRoute.tsx, apps/portal/src/SetupFirstRunRoutePanel.tsx, apps/portal/src/dev-logger.ts, apps/portal/tests/setup-first-run-route-panel.test.ts, apps/portal/tests/live-activity-network-flow.test.ts, apps/portal/tests/logging/portal-dev-log-route.test.ts, apps/portal/e2e/setup-first-run-ui-proof.spec.ts, output/setup-install-provisioning-plan-proof/07-first-run-setup-ui-and-state-machine/00-first-run-state-machine-proof.md, output/setup-install-provisioning-plan-proof/07-first-run-setup-ui-and-state-machine/01-first-run-ui-screen-map.md, output/setup-install-provisioning-plan-proof/07-first-run-setup-ui-and-state-machine/02-empty-error-degraded-ui-proof.md, output/setup-install-provisioning-plan-proof/07-first-run-setup-ui-and-state-machine/03-manual-required-visible-proof.md, output/setup-install-provisioning-plan-proof/07-first-run-setup-ui-and-state-machine/04-adjacent-handoff-visible-proof.md, output/setup-install-provisioning-plan-proof/07-first-run-setup-ui-and-state-machine/05-no-fake-ready-state-proof.md, output/setup-install-provisioning-plan-proof/07-first-run-setup-ui-and-state-machine/06-source-custody-label-proof.md, output/setup-install-provisioning-plan-proof/07-first-run-setup-ui-and-state-machine/16-validation-commands.log
Validation commands and results: `npm run build --workspace @ocentra-parent/portal-domain` PASS; `npm run test --workspace @ocentra-parent/portal-domain -- setup-first-run-panel.test.ts` PASS; `npm run lint:architecture -- --files packages/portal-domain/package.json packages/portal-domain/src/setup-first-run-panel.ts packages/portal-domain/tests/unit/setup-first-run-panel.test.ts apps/portal/src/SetupFirstRunRoutePanel.tsx apps/portal/src/ParentPortalRoute.tsx apps/portal/src/dev-logger.ts apps/portal/tests/setup-first-run-route-panel.test.ts apps/portal/tests/live-activity-network-flow.test.ts apps/portal/tests/logging/portal-dev-log-route.test.ts apps/portal/e2e/setup-first-run-ui-proof.spec.ts packages/setup-domain/src/setup-state-machine.ts packages/setup-domain/tests/unit/setup-state-machine.test.ts` PASS; `Push-Location apps/portal; npx vitest run tests/logging/portal-dev-log-route.test.ts tests/setup-first-run-route-panel.test.ts tests/live-activity-network-flow.test.ts; Pop-Location` PASS (3 files, 19 tests); `Push-Location apps/portal; npx tsc -p tsconfig.json --noEmit; Pop-Location` PASS; `Push-Location apps/portal; npx vite build; Pop-Location` PASS; `$env:OCENTRA_PARENT_PORTAL_PLAYWRIGHT_SPEC='setup-first-run-ui-proof.spec.ts'; node scripts/test/portal-playwright-runner.mjs` PASS (1 Playwright spec); `npm run lint:architecture -- --files docs/plans/setup-install-provisioning-plan` PASS.
Proof artifacts: output/setup-install-provisioning-plan-proof/07-first-run-setup-ui-and-state-machine/00-first-run-state-machine-proof.md, output/setup-install-provisioning-plan-proof/07-first-run-setup-ui-and-state-machine/01-first-run-ui-screen-map.md, output/setup-install-provisioning-plan-proof/07-first-run-setup-ui-and-state-machine/02-empty-error-degraded-ui-proof.md, output/setup-install-provisioning-plan-proof/07-first-run-setup-ui-and-state-machine/03-manual-required-visible-proof.md, output/setup-install-provisioning-plan-proof/07-first-run-setup-ui-and-state-machine/04-adjacent-handoff-visible-proof.md, output/setup-install-provisioning-plan-proof/07-first-run-setup-ui-and-state-machine/05-no-fake-ready-state-proof.md, output/setup-install-provisioning-plan-proof/07-first-run-setup-ui-and-state-machine/06-source-custody-label-proof.md, output/setup-install-provisioning-plan-proof/07-first-run-setup-ui-and-state-machine/16-validation-commands.log
Known gaps/manual-required states: the owned WP07 route projection is implementation-complete on this branch, but it intentionally leaves account/provider/session, signed installer/runtime distribution, child runtime/package execution, LAN/device-trust proof, data-custody execution, policy baseline production proof, and subscription/entitlement proof with sibling plans; `npm run build --workspace @ocentra-parent/portal` still resolves a missing local Vite bin on this host, so the proof pack uses direct `npx tsc` plus `npx vite build` inside `apps/portal`, and the formerly failing dev-log harness gap is now fixed.
No-claim boundaries: no live account readiness claim, no signed installer readiness claim, no child runtime/package readiness claim, no LAN or trusted-device readiness claim, no production policy or custody execution claim, and no product onboarding completion claim outside the typed first-run route projection.
```
