# 01 Service-Backed Shell And Navigation

<!-- agent-capsule -->

> Agent Capsule
> Plan: `portal-ux-household-surfaces-plan`
> Doc: `01 Service-Backed Shell And Navigation`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

Sources: [20-step plan](../portal-ux-household-surfaces-20-step-plan.md),
[test blueprint](../portal-ux-household-surfaces-test-blueprint.md),
[UI/UX guide](../ui-ux-requirements-guide.md), and [folder README](../README.md).

## Where We Are

The portal has route surfaces, but the product still needs one coherent shell
that makes connection, source, and selected-device state visible.

## Where We Want To Be

Navigation feels like one parent product and every route starts from validated
service/read-model state.

## Decision Tree

| If the assignment touches...       | Read next                                             | Required proof                             |
| ---------------------------------- | ----------------------------------------------------- | ------------------------------------------ |
| Shell layout/navigation            | this workpack and nearest route shell source          | desktop/mobile navigation screenshot proof |
| Service-backed state               | owning domain plan for the selected read-model        | service/read-model fixture or live proof   |
| Account/session/household selector | `../../account-identity-family-plan/AGENTS.md`        | auth/session/role boundary proof           |
| Parent client shell handoff        | `../../parent-desktop-runtime-package-plan/AGENTS.md` | local shell/service state proof            |
| Public family site entry           | `../../setup-install-provisioning-plan/AGENTS.md`     | install/login handoff proof                |

## Required Shell State

- Global connection state: online, offline, degraded, stale, manual-required, or unauthenticated.
- Selected household and selected child/device state with redacted IDs and source labels.
- Route capability state: available, unavailable, not configured, permission missing, platform unsupported, or proof missing.
- Data source label: live local, LAN, relay, parent cache, parent-owned cloud, Ocentra-hosted metadata, or unavailable.
- No route may render a green success state without a named service/read-model source.

## Requirement Checklist

- [x] Show connection and source state consistently.
- [x] Keep route ids, DOM ids, and display tokens domain-owned.
- [x] Avoid fake route-local success.
- [x] Support desktop and mobile navigation without overlap.
- [x] Add Playwright checks for shell state.
- [x] Prove selected household/device changes do not leak stale state across routes.
- [x] Prove unauthenticated and no-household states.
- [x] Route public website, desktop shell, and portal app responsibilities separately.

## Acceptance And Proof

Playwright verifies shell connection/source labels against the real service.

Expected proof names:

- `portal.shell.service-backed-navigation`
- `portal.shell.unauthenticated-state`
- `portal.shell.no-household-state`
- `portal.shell.selected-device-switch`
- `portal.shell.mobile-desktop-layout`
- `portal.shell.no-fake-success-negative`

Proof must include route URLs, screenshots or DOM snapshots, selected state fixtures/live source, and missing-service behavior.

Current checkpoint truth on this branch/worktree (2026-06-18):

- Focused proof is recorded under `output/portal-ux-household-surfaces-plan-proof/01-service-backed-shell-and-navigation/`.
- The current packet proves real shell-state/navigation behavior on `/#/commands`, `/#/overview`, and `/#/devices`, including the service-backed shell labels, the repaired overview route crash, the real devices-route contract where `Pair` is conditional on the selected device readiness, and the mobile shell-status layout check that runs inside Playwright.
- The current packet now covers the named shell proof lines in this workpack on this branch/worktree: `portal.shell.service-backed-navigation`, `portal.shell.unauthenticated-state`, `portal.shell.no-household-state`, `portal.shell.selected-device-switch`, `portal.shell.mobile-desktop-layout`, and `portal.shell.no-fake-success-negative`.
- The public family site responsibility is explicitly routed to `setup-install-provisioning-plan`: `family.ocentra.ca` is a public family information/download/account-entry surface, not account/family authority, and it routes registration/login to account identity plus package/signing/update and pairing/device-trust claims to their owning plans.
- The desktop shell/package responsibility is explicitly routed to `parent-desktop-runtime-package-plan` (canonical scope: parent client runtime distribution): that plan owns the parent client distribution boundary and the desktop shell/package, local-service bridge, launch smoke, update boundary, and signing claims for the parent desktop artifact.
- The portal app responsibility stays in this plan: `SetupFirstRunRoutePanel` renders only on `PortalRoute.Start`, and the current route-panel unit/e2e proof already displays the exact handoff labels `account-identity-family-plan` and `parent-desktop-runtime-package-plan` instead of collapsing those ownership boundaries into one portal success claim.
- This checklist row is now closed on this branch/worktree because the public website, desktop shell, and portal app responsibilities are named separately in the owning plan docs and echoed by the portal start-route handoff proof.

## Failure Conditions

- Do not use static demo cards as product state.
- Do not hide degraded/unavailable/manual-required state behind empty green UI.
- Do not merge public marketing site, parent portal, and desktop shell into one unexplained route.

## Parallel Ownership Notes

C owns visual structure. Runtime truth remains with the service and domain
contracts.
