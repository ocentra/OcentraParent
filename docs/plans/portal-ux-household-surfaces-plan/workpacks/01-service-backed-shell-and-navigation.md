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
| Installer/desktop shell handoff    | `../../parent-desktop-runtime-package-plan/AGENTS.md` | local shell/service state proof            |
| Public family site entry           | `../../setup-install-provisioning-plan/AGENTS.md`     | install/login handoff proof                |

## Required Shell State

- Global connection state: online, offline, degraded, stale, manual-required, or unauthenticated.
- Selected household and selected child/device state with redacted IDs and source labels.
- Route capability state: available, unavailable, not configured, permission missing, platform unsupported, or proof missing.
- Data source label: live local, LAN, relay, parent cache, parent-owned cloud, Ocentra-hosted metadata, or unavailable.
- No route may render a green success state without a named service/read-model source.

## Requirement Checklist

- [ ] Show connection and source state consistently.
- [ ] Keep route ids, DOM ids, and display tokens domain-owned.
- [ ] Avoid fake route-local success.
- [ ] Support desktop and mobile navigation without overlap.
- [ ] Add Playwright checks for shell state.
- [ ] Prove selected household/device changes do not leak stale state across routes.
- [ ] Prove unauthenticated and no-household states.
- [ ] Route public website, desktop shell, and portal app responsibilities separately.

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

## Failure Conditions

- Do not use static demo cards as product state.
- Do not hide degraded/unavailable/manual-required state behind empty green UI.
- Do not merge public marketing site, parent portal, and desktop shell into one unexplained route.

## Parallel Ownership Notes

C owns visual structure. Runtime truth remains with the service and domain
contracts.
