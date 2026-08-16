# 04 Selected Device Context

<!-- agent-capsule -->

> Agent Capsule
> Plan: `portal-ux-household-surfaces-plan`
> Doc: `04 Selected Device Context`
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

Some portal surfaces can show device state, but cross-route selected-device
context still needs product-level consistency.

## Where We Want To Be

Policy, Activity, AI, Reports, and Account surfaces clearly show which
child/device/route they apply to before any action.

## Requirement Checklist

- [x] Show selected child/device/route source on every relevant route.
- [x] Preserve context across navigation.
- [x] Fail visibly when no valid device is selected.
- [x] Prevent wrong-device action confusion.
- [x] Test context persistence.

## Acceptance And Proof

Playwright navigates across routes and verifies selected-device state is visible.

Current checkpoint truth on this branch/worktree (2026-06-18):

- Focused proof is now recorded under `output/portal-ux-household-surfaces-plan-proof/04-selected-device-context/`.
- The current packet proves the WP04 selected-device seam on current source: `/#/browser-settings` fails visibly with `No device selected` before any device is chosen, then a real LAN device selection on `/#/devices` persists across policy, AI, account, and activity/report surfaces instead of silently reverting to family scope.
- The shared manage-target state is now session-backed and survives route changes into `/#/browser-settings`, `/#/ai-runtime`, `/#/entitlements`, and `/#/activity`, with exact route copy showing `Browser target`, `AI device`, `Account device`, and `Report device` for the same selected child device.
- Account manage surfaces now participate in the shared target selector and no longer reset back to `Whole family` on context changes when a valid selected device already exists.
- This workpack closes only the selected-device context consumption boundary. `/#/rule-management` route authority remains explicitly out of scope for WP04 and is still owned by the separate route-authority/manage-surface frontier.
- This checklist row is now locally closed on this branch/worktree because the current portal source, focused portal TS compile, and the assigned Playwright proof all reran green after the account-surface target-selector and context-preservation repair.

## Parallel Ownership Notes

This workpack is UX/state consumption, not route authority implementation.
