# Portal UX Household Surfaces Plan Next Actions

<!-- agent-capsule -->

> Agent Capsule
> Plan: `portal-ux-household-surfaces-plan`
> Doc: `Portal UX Household Surfaces Plan Next Actions`
> Kind: resume queue and highest-open work.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.

<!-- /agent-capsule -->

This file is the short resume list for the next worker. It is derived from open workpack/checklist status and does not replace the assigned workpack.

## How to use

1. Confirm the hub assignment and lane.
2. Pick only the assigned workpack from the list below.
3. Open that workpack and exact checklist rows only.
4. Do not claim `DONE` unless the assigned workpack's acceptance/proof rows are updated and validation is listed.

## Highest-open workpacks by unchecked boxes

- [01 Service-Backed Shell And Navigation](workpacks/01-service-backed-shell-and-navigation.md): 5 open of 5 boxes.
- [02 Household First-Run And Profiles](workpacks/02-household-first-run-and-profiles.md): 5 open of 5 boxes.
- [03 Device Inventory And Source States](workpacks/03-device-inventory-and-source-states.md): 5 open of 5 boxes.
- [04 Selected Device Context](workpacks/04-selected-device-context.md): 5 open of 5 boxes.
- [05 Policy Authoring Control Center](workpacks/05-policy-authoring-control-center.md): 5 open of 5 boxes.
- [06 Schedules, Time Budgets, And Templates](workpacks/06-schedules-time-budgets-and-templates.md): 5 open of 5 boxes.
- [07 Parent Requests And Approvals](workpacks/07-parent-requests-and-approvals.md): 5 open of 5 boxes.
- [08 Activity Evidence And Diagnostics](workpacks/08-activity-evidence-and-diagnostics.md): 5 open of 5 boxes.
- [09 Browser, App, And Network Surfaces](workpacks/09-browser-app-and-network-surfaces.md): 5 open of 5 boxes.
- [11 Assistant Action Preview Flow](workpacks/11-assistant-action-preview-flow.md): 5 open of 5 boxes.
- [12 Reports, Notifications, And Custody](workpacks/12-reports-notifications-and-custody.md): 5 open of 5 boxes.
- [13 Degraded, Empty, Stale, And Error States](workpacks/13-degraded-empty-stale-and-error-states.md): 5 open of 5 boxes.
- [14 Audit History And Copy/Debug](workpacks/14-audit-history-and-copy-debug.md): 5 open of 5 boxes.
- [15 Accessibility, Responsive, And Keyboard UX](workpacks/15-accessibility-responsive-keyboard-ux.md): 5 open of 5 boxes.
- [16 No-Fake-Data Contract Adapter](workpacks/16-no-fake-data-contract-adapter.md): 5 open of 5 boxes.
- [17 Playwright Screenshot Proof](workpacks/17-playwright-screenshot-proof.md): 5 open of 5 boxes.
- [18 Parent Mobile Shell Readiness](workpacks/18-parent-mobile-shell-readiness.md): 5 open of 6 boxes.
- [19 Product Docs And Checklist Sync](workpacks/19-product-docs-and-checklist-sync.md): 5 open of 5 boxes.
- [20 Manual User Review Gate](workpacks/20-manual-user-review-gate.md): 5 open of 5 boxes.

## PR readiness guard

A PR-ready slice should close a named workpack or clearly explain the exact remaining rows. Do not create a tiny PR that only updates one proof note while leaving the assigned workpack/checklist state stale.

Before reporting `DONE` or `PR_READY`, update the workpack, checklist row(s), proof reference(s), and feature/product docs if product status changed.

## Actioned completion tracker

- [ ] Re-check this plan route from AGENTS/PLAN_STATE and confirm the assigned workpack path.
- [ ] Update one assigned workpack and matching checklist/proof rows before reporting progress.
- [ ] Record failure conditions, skipped checks, and evidence path in PLAN_STATE/TEST_PROOF_EXPECTATIONS for every claimed progress.
