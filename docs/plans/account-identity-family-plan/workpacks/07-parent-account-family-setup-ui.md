<!-- agent-capsule -->

> Agent Capsule
> Plan: `account-identity-family-plan`
> Doc: `WP07 Parent Account Family Setup UI`
> Kind: assigned implementation workpack.
> Read when: selected by WORKPACK_INDEX.md or explicit assignment.
> Stop rule: do not create UI-only fake setup; contracts/source states must exist or blockers must be recorded.
> Proves: parent-visible setup UI state only after tests/proof pass.
> Does not prove: provider auth readiness, device trust, LAN/remote transport, or product-ready setup.
> Proof rule: before DONE, write all WP07 proof artifacts and command log.

<!-- /agent-capsule -->

# WP07 Parent Account Family Setup UI

## Goal

Define and, when assigned, implement the first-run parent account and family setup UI states over typed account/family contracts.

## Required inputs

```text
workpacks/02-identity-household-role-model.md
workpacks/03-session-token-lifecycle.md
workpacks/04-invites-recovery-lifecycle.md
workpacks/05-device-ownership-authz.md
docs/features/family-setup-device-roles.md
docs/expectations/family-setup.md
docs/expectations/portal.md
docs/expectations/platforms.md
packages/family-domain/src/**
packages/portal-domain/src/**
apps/portal/src/** selected route only
```

## Required UI states

```text
welcome/sign-in
signed-in-no-household
create-household
join-household
create-child-profile
add-child-device
pair-child-device
pending-device
trusted-device
revoked-device
stale-device
invite-co-parent
invite-observer
observer-read-only
recovery
support-access-status
account-security-settings
manual-required
```

## Required source/custody labels

```text
live local
LAN
parent cache
parent-owned storage
stale
degraded
unavailable
manual-required
```

## UI rules

```text
UI reads typed contracts/read models.
UI does not imply login equals household/device trust.
UI does not show fake child activity data.
UI distinguishes parent account, child profile, and child device.
UI labels unavailable/manual-required states visibly.
UI keeps support/admin separate from parent owner.
UI uses domain-owned route text/DOM ids where available.
```

## Expected source changes

Likely paths:

```text
packages/family-domain/src/**
packages/portal-domain/src/**
apps/portal/src/** selected setup route/components only
apps/portal/tests/** account/family setup tests
apps/portal/e2e/** account/family setup proof only if e2e is assigned
```

Do not edit setup-install-provisioning-plan unless the user explicitly assigns route-sync.

## Required proof root

```text
output/account-identity-family-plan-proof/07-parent-account-family-setup-ui/
```

Required artifacts:

```text
00-first-run-ui-state-machine.md
01-household-setup-ui-proof.md
02-device-role-ui-proof.md
03-observer-read-only-ui-proof.md
04-recovery-ui-proof.md
05-mobile-parent-child-claim-split-proof.md
06-source-custody-label-proof.md
16-validation-commands.log
```

## Acceptance criteria

- [ ] First-run UI state machine exists.
- [ ] Sign-in/no-household/create/join states are visible/tested.
- [ ] Add child profile flow is visible/tested or blocked with exact missing contract.
- [ ] Pair child device flow is visible/tested or blocked with exact missing device-trust/LAN handoff.
- [ ] Co-parent and observer invite states are visible/tested.
- [ ] Role visibility matrix is honored.
- [ ] Revoked/stale/expired/manual-required states are visible.
- [ ] Recovery and support-access status states are visible.
- [ ] Source/custody labels are visible and honest.
- [ ] UI does not imply login equals device trust.
- [ ] UI does not present hosted child activity storage as available by default.
- [ ] Focused portal/domain commands pass or blockers are recorded.

## Focused commands

```bash
npm run build --workspace @ocentra-parent/family-domain
npm run build --workspace @ocentra-parent/portal-domain
npm run test --workspace @ocentra-parent/portal -- account
npm run test --workspace @ocentra-parent/portal -- family
npm run test:e2e --workspace @ocentra-parent/portal -- account
npm run lint:architecture -- --files packages/family-domain packages/portal-domain apps/portal
```

If UI/e2e tests do not exist yet, record the exact missing test path and keep relevant rows open.

## Negative cases

- Logged-in user with no household sees no policy/payment/remote authority.
- Observer cannot see owner-only controls.
- Revoked device shows unavailable/revoked state, not trusted.
- Expired session shows reauth/manual-required state.
- Child profile exists but device is not trusted.
- Support access is visible as separate audited support state, not parent owner.

## Manual-required gaps

Real child-device pairing proof remains gated by device-trust/LAN plans. Hosted account site remains setup-install scope. Child activity evidence remains data-custody scope.

## Fill before DONE

```text
Workpack id and branch: WP07 Parent Account Family Setup UI / codex/tracking-plan-full-continuation-a
Current status: complete for the local contract/proof slice. `00-first-run-ui-state-machine.md`, `01-household-setup-ui-proof.md`, `02-device-role-ui-proof.md`, `03-observer-read-only-ui-proof.md`, `04-recovery-ui-proof.md`, `05-mobile-parent-child-claim-split-proof.md`, `06-source-custody-label-proof.md`, and `16-validation-commands.log` now exist under `output/account-identity-family-plan-proof/07-parent-account-family-setup-ui/`.
UI/contract changes: the existing portal-domain Start-route projection now renders explicit invite-role-support visibility and trust/session distinction rows over the typed setup-domain first-run and readiness surfaces. `apps/portal/src/SetupFirstRunRoutePanel.tsx` remained a thin renderer over that projection.
Touched files:
- `packages/portal-domain/src/setup-first-run-panel.ts`
- `packages/portal-domain/tests/unit/setup-first-run-panel.test.ts`
- `apps/portal/tests/setup-first-run-route-panel.test.ts`
- `apps/portal/e2e/setup-first-run-ui-proof.spec.ts`
- `docs/plans/account-identity-family-plan/CHECKLIST_INDEX.md`
- `docs/plans/account-identity-family-plan/PLAN_STATE.md`
- `docs/plans/account-identity-family-plan/WORKPACK_INDEX.md`
- `docs/plans/account-identity-family-plan/workpacks/07-parent-account-family-setup-ui.md`
- `output/account-identity-family-plan-proof/07-parent-account-family-setup-ui/00-first-run-ui-state-machine.md`
- `output/account-identity-family-plan-proof/07-parent-account-family-setup-ui/01-household-setup-ui-proof.md`
- `output/account-identity-family-plan-proof/07-parent-account-family-setup-ui/02-device-role-ui-proof.md`
- `output/account-identity-family-plan-proof/07-parent-account-family-setup-ui/03-observer-read-only-ui-proof.md`
- `output/account-identity-family-plan-proof/07-parent-account-family-setup-ui/04-recovery-ui-proof.md`
- `output/account-identity-family-plan-proof/07-parent-account-family-setup-ui/05-mobile-parent-child-claim-split-proof.md`
- `output/account-identity-family-plan-proof/07-parent-account-family-setup-ui/06-source-custody-label-proof.md`
- `output/account-identity-family-plan-proof/07-parent-account-family-setup-ui/16-validation-commands.log`
Validation commands and results:
- `command: npm run build --workspace @ocentra-parent/portal-domain`
- `exit: 0`
- `result: pass`
- `artifact: n/a`
- `notes: portal-domain build passed after the WP07 route-binding projection update`
- `command: npm run test --workspace @ocentra-parent/portal-domain -- tests/unit/setup-first-run-panel.test.ts`
- `exit: 0`
- `result: pass`
- `artifact: n/a`
- `notes: focused portal-domain unit suite passed with 5 tests`
- `command: npm run test --workspace @ocentra-parent/portal -- tests/setup-first-run-route-panel.test.ts`
- `exit: 1`
- `result: blocked`
- `artifact: n/a`
- `notes: the workspace test script expands to `vitest run tests ...` and pulled unrelated `tests/live-activity-surface-adapter.test.ts` failures outside the WP07 scope; focused route validation used `npx vitest run tests/setup-first-run-route-panel.test.ts` instead`
- `command: npx vitest run tests/setup-first-run-route-panel.test.ts`
- `exit: 0`
- `result: pass`
- `artifact: n/a`
- `notes: focused Start-route validation passed with 2 tests`
- `command: npx playwright test e2e/setup-first-run-ui-proof.spec.ts`
- `exit: 1`
- `result: blocked`
- `artifact: test-results/portal-playwright/setup-first-run-ui-proof-s-36a41-ers-and-no-fake-ready-state-chromium/`
- `notes: direct Playwright invocation did not start the portal shell on this host; the workspace proof runner was used instead`
- `command: npm run test:e2e --workspace @ocentra-parent/portal -- e2e/setup-first-run-ui-proof.spec.ts`
- `exit: 0`
- `result: pass`
- `artifact: n/a`
- `notes: focused Playwright proof runner passed with 1 Chromium test`
- `command: npm run lint:architecture -- --files packages/portal-domain/src/setup-first-run-panel.ts packages/portal-domain/tests/unit/setup-first-run-panel.test.ts apps/portal/tests/setup-first-run-route-panel.test.ts apps/portal/e2e/setup-first-run-ui-proof.spec.ts`
- `exit: 0`
- `result: pass`
- `artifact: n/a`
- `notes: touched-file architecture and test-integrity gates passed for the WP07 slice`
Proof artifacts:
- `output/account-identity-family-plan-proof/07-parent-account-family-setup-ui/00-first-run-ui-state-machine.md`
- `output/account-identity-family-plan-proof/07-parent-account-family-setup-ui/01-household-setup-ui-proof.md`
- `output/account-identity-family-plan-proof/07-parent-account-family-setup-ui/02-device-role-ui-proof.md`
- `output/account-identity-family-plan-proof/07-parent-account-family-setup-ui/03-observer-read-only-ui-proof.md`
- `output/account-identity-family-plan-proof/07-parent-account-family-setup-ui/04-recovery-ui-proof.md`
- `output/account-identity-family-plan-proof/07-parent-account-family-setup-ui/05-mobile-parent-child-claim-split-proof.md`
- `output/account-identity-family-plan-proof/07-parent-account-family-setup-ui/06-source-custody-label-proof.md`
- `output/account-identity-family-plan-proof/07-parent-account-family-setup-ui/16-validation-commands.log`
Known gaps/manual-required states: physical trusted-device proof remains owned by `device-trust-bootstrap-plan` and `lan-plan`; Cloudflare account/runtime implementation remains external; data-custody export/delete execution, payment runtime, and WP06 route-gate aggregation remain open; the broad `npm run test --workspace @ocentra-parent/portal -- ...` path is still mis-scoped for single-route proof and should not be used as the focused WP07 signal.
No-claim boundaries: do not claim Cloudflare account runtime readiness, physical trusted-device bootstrap, LAN transport execution, custody execution, payment/customer runtime, or whole-plan readiness from this WP07 closure.
```
