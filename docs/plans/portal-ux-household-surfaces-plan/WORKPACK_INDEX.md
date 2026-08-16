# Portal UX Household Surfaces Plan Workpack Index

<!-- agent-capsule -->

> Agent Capsule
> Plan: `portal-ux-household-surfaces-plan`
> Doc: `Portal UX Household Surfaces Plan Workpack Index`
> Kind: workpack selector; use before opening any workpack.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this index changes status or claims, update the assigned workpack, checklist row, and proof path.

<!-- /agent-capsule -->

Use this index to open exactly one assigned workpack. Do not read every file in `workpacks/`.

Use `WORKPACK_FAMILIES.md` only when the selected workpack owner/proof family is unclear. Do not use it as permission to scan multiple workpacks.

| Status  | Workpack                                                                                              |  Size | Boxes                 |
| ------- | ----------------------------------------------------------------------------------------------------- | ----: | --------------------- |
| checked | [10 LAN Pairing State Consumption](workpacks/10-lan-pairing-state-consumption.md)                     | 3,257 | 11/11 checked; 0 open |
| checked | [01 Service-Backed Shell And Navigation](workpacks/01-service-backed-shell-and-navigation.md)         | 5,408 | 8/8 checked; 0 open   |
| checked | [02 Household First-Run And Profiles](workpacks/02-household-first-run-and-profiles.md)               | 5,470 | 8/8 checked; 0 open   |
| checked | [03 Device Inventory And Source States](workpacks/03-device-inventory-and-source-states.md)           | 3,254 | 5/5 checked; 0 open   |
| checked | [04 Selected Device Context](workpacks/04-selected-device-context.md)                                 | 1,027 | 5/5 checked; 0 open   |
| open    | [05 Policy Authoring Control Center](workpacks/05-policy-authoring-control-center.md)                 | 3,935 | 0/5 checked; 5 open   |
| open    | [06 Schedules, Time Budgets, And Templates](workpacks/06-schedules-time-budgets-and-templates.md)     | 1,071 | 0/5 checked; 5 open   |
| open    | [07 Parent Requests And Approvals](workpacks/07-parent-requests-and-approvals.md)                     | 1,024 | 0/5 checked; 5 open   |
| open    | [08 Activity Evidence And Diagnostics](workpacks/08-activity-evidence-and-diagnostics.md)             | 1,049 | 0/5 checked; 5 open   |
| open    | [09 Browser, App, And Network Surfaces](workpacks/09-browser-app-and-network-surfaces.md)             | 1,094 | 0/5 checked; 5 open   |
| open    | [11 Assistant Action Preview Flow](workpacks/11-assistant-action-preview-flow.md)                     | 4,071 | 0/5 checked; 5 open   |
| open    | [12 Reports, Notifications, And Custody](workpacks/12-reports-notifications-and-custody.md)           | 1,114 | 0/5 checked; 5 open   |
| open    | [13 Degraded, Empty, Stale, And Error States](workpacks/13-degraded-empty-stale-and-error-states.md)  | 1,023 | 0/5 checked; 5 open   |
| open    | [14 Audit History And Copy/Debug](workpacks/14-audit-history-and-copy-debug.md)                       | 1,063 | 0/5 checked; 5 open   |
| open    | [15 Accessibility, Responsive, And Keyboard UX](workpacks/15-accessibility-responsive-keyboard-ux.md) | 1,348 | 0/5 checked; 5 open   |
| open    | [16 No-Fake-Data Contract Adapter](workpacks/16-no-fake-data-contract-adapter.md)                     | 1,051 | 0/5 checked; 5 open   |
| open    | [17 Playwright Screenshot Proof](workpacks/17-playwright-screenshot-proof.md)                         | 1,013 | 0/5 checked; 5 open   |
| open    | [18 Parent Mobile Shell Readiness](workpacks/18-parent-mobile-shell-readiness.md)                     | 1,323 | 1/6 checked; 5 open   |
| open    | [19 Product Docs And Checklist Sync](workpacks/19-product-docs-and-checklist-sync.md)                 | 1,036 | 0/5 checked; 5 open   |
| open    | [20 Manual User Review Gate](workpacks/20-manual-user-review-gate.md)                                 | 1,116 | 0/5 checked; 5 open   |

## Production reachability audit (2026-08-16)

The portal entrypoint and typed host bridge are real and reach Rust parent
route snapshots/actions. WP01-WP04 and WP10 have source-backed consumption
boundaries; WP05 has the existing bounded Rust authoring/confirmation seam.
The remaining open rows either lack a sibling-owned service read model/action,
remain domain-authority dependent, or are explicitly tests, proof, mobile
scaffold, docs, or manual review. The index must not promote route existence,
static/proof panels, fixtures, generic JSON, or DTO-only components into
product truth. No portal-only production edit was legal in this audit.

## Selection rules

- Choose exactly one workpack.
- If owner/proof family is unclear, classify through `WORKPACK_FAMILIES.md`; do not scan every family.
- Do not use checked shell/device/LAN rows to close open policy, assistant, reports, degraded-state, accessibility, no-fake-data, screenshot, mobile, docs, or manual-review workpacks.
- Do not use portal-domain exports or route existence as proof of rendered portal UX.
- Do not use screenshot proof as service-backed validation.
- Do not use service-backed parser proof as rendered user-facing proof.
