# 04 Portal UX And First-Run Handoff

<!-- agent-capsule -->

> Agent Capsule
> Plan: `lan-plan`
> Doc: `04 Portal UX And First-Run Handoff`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

Sources: [folder README](../README.md), [feature doc](../../features/family-setup-device-roles.md),
[family setup expectations](../../expectations/family-setup.md).
Assumes WP01 contracts, WP02 SQLite tables, WP03 command handlers all exist.

## Where We Are

The parent portal's Devices/LAN surface has command-backed add, route-select,
rename, trust, ignore, restore, and revoke controls wired to the existing V0.9
LAN spine read model (`output/playwright/lan-source-matrix-plan-completion/`).
These controls expose raw LAN slot fields (IP, MAC, source kind) that are
service-backed but require technical knowledge to interpret.

What is **missing**:

- A first-run wizard that lets a nontechnical parent create a household, name
  children, and pair a child device **without seeing raw protocol fields**.
- Add-device UX that issues typed commands (`AssignDevice`, `RenameDevice`,
  `TrustDevice`) rather than raw LAN slot IDs.
- Recovery UX: no portal flow triggers `StartRecovery` or `ConfirmPairing`.
- Source-label UX: `local`, `lan`, `relay`, `cache`, `unavailable` labels exist in
  the read model but are not rendered as distinct, explained UI states in the
  portal. "Relay unavailable" and "cache" are shown but not explained.
- Portal tests for full setup, recovery, and degraded first-run states (feature
  doc checklist row still open).
- `ObserverPermission` role is not displayed or editable in the portal.

## Where We Want To Be

A nontechnical parent opens the portal, sees a "Set up your household" prompt,
names their household, adds a child profile by name, taps "Add device", and
pairs it — all through typed commands that the portal issues to the service.
The same parent can later see device source labels explained in plain language,
initiate recovery if a device goes stale, and understand what "protected" vs
"unprotected" means before enabling enforcement.

## Scope

### First-Run Wizard

New portal route: `/setup` (or modal if no household exists on first load).
Steps:

1. Create household — POST `agent.household.create` command with `displayName`.
   Maps to `HouseholdProfile` from WP01; service stores it (WP02 table).
2. Add child profile — POST `agent.child-profile.create` command with `displayName`.
3. Add device — opens existing Devices/LAN surface, but wraps it in a context that
   issues `AssignDevice` using the `childId` from step 2.
4. Summary: show household read model response with device `routeState` label and
   explain each label in plain language.

Touched portal files:

- `packages/portal/src/routes/setup/` (new directory)
- `packages/portal/src/routes/setup/SetupWizard.tsx` (or `.svelte`)
- `packages/portal/src/routes/setup/CreateHousehold.tsx`
- `packages/portal/src/routes/setup/AddChildProfile.tsx`
- `packages/portal/src/routes/setup/AddDevice.tsx`
- `packages/portal/src/routes/setup/SetupSummary.tsx`

### Recovery UX

New recovery panel in existing Devices/LAN route:

- Button: "Recover device" — visible when `DeviceRegistration.trustState = "stale" | "revoked"`.
- On confirm: POST `agent.household.start-recovery` command; poll for `RecoveryState` in read model.
- On fresh pairing: POST `agent.household.confirm-pairing`; clear `RecoveryState` from UI.
- Touched portal files:
  - `packages/portal/src/components/DeviceRecoveryPanel.tsx` (new)
  - `packages/portal/src/routes/devices/DevicesLan.tsx` (add recovery panel)

### Source-Label Explanation

In `packages/portal/src/components/DeviceRouteLabel.tsx` (new): render
`routeState` as a badge with a tooltip explaining each label:

- `local` → "This device is on this machine (loopback)."
- `lan` → "Connected over your local network."
- `relay` → "Connected through an Ocentra relay."
- `cache` → "Showing cached data; device may be offline."
- `unavailable` → "Device is not reachable. Recovery may be needed."

## Touched Paths

- `packages/portal/src/routes/setup/` (new — all setup wizard files)
- `packages/portal/src/components/DeviceRecoveryPanel.tsx` (new)
- `packages/portal/src/components/DeviceRouteLabel.tsx` (new)
- `packages/portal/src/routes/devices/DevicesLan.tsx` (add recovery panel import)
- `packages/portal/tests/family-setup.playwright.ts` (new — Playwright tests)

## Tests And Proof

- [ ] Playwright test: full setup wizard flow — create household, add child, add device; assert service-backed read model appears in `/setup` summary.
- [ ] Playwright test: recovery flow — stale device, click "Recover", confirm pairing; assert `routeState` changes to `local` or `lan` after re-pair.
- [ ] Playwright test: degraded first-run — service offline; wizard step 1 shows error state, does not claim device is protected.
- [ ] Playwright test: source-label tooltip renders for `relay`, `cache`, `unavailable` states using fixture read model responses.
- [ ] Playwright test: observer-role parent sees read-only controls; no assign/rename/revoke buttons visible.
- [ ] UI snapshot proof: screenshots saved to `output/lan-plan-proof/04-portal-ux-and-first-run-handoff/06-ui-snapshots/`.
- [ ] No raw LAN slot field (IP, MAC, source kind enum value) exposed directly as copy in the first-run wizard. Portal must translate service fields to user-legible labels.

## AI Worker Checklist

Fill this before reporting `DONE` or PR-ready:

- [ ] Confirm source docs read: [folder README](../README.md), [feature doc](../../features/family-setup-device-roles.md), [family setup expectations](../../expectations/family-setup.md), [current PLAN_STATE](../PLAN_STATE.md), and this workpack.
- [ ] Confirmed WP01–03 (contracts, tables, command handlers) exist before building portal routes.
- [ ] Check enhancement overlap: `portal-ux-household-surfaces-plan` — coordinate to avoid duplicate household surface implementations.
- [ ] Hub lock covers this workpack and exact implementation/docs paths.
- [ ] Setup wizard issues typed commands (`agent.household.create`, `agent.child-profile.create`, `AssignDevice`) not raw LAN slot mutations.
- [ ] Recovery panel triggers `agent.household.start-recovery` and `agent.household.confirm-pairing` commands only.
- [ ] No portal surface invents a "device is protected" claim before service-backed capability status confirms it.
- [ ] Playwright tests: setup wizard, recovery, degraded, source-label, observer-role all written and passing.
- [ ] UI snapshots captured to proof folder.
- [ ] [main checklist](../implementation-checklist.md) rows 04 updated.
- [ ] Known gaps (co-parent invite email delivery, push notification) recorded as open/deferred.

## Manual-Required Gaps

First-run QR-code-based pairing (scanning a QR code on the child device to pair) is an OS-level camera/permission flow and must be tested manually if implemented. Record as manual-required.
Co-parent `SetupInvite` delivery via email or SMS relies on external send services and is out of scope; record as deferred.
