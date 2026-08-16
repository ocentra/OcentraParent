<!-- agent-capsule -->

> Agent Capsule
> Plan: `portal-ux-household-surfaces-plan`
> Doc: `Portal UX Household Surfaces Workpack Families`
> Kind: owner-path classifier for selected workpacks.
> Read when: only after `WORKPACK_INDEX.md` selects or names a workpack and owner/proof family is unclear.
> Stop rule: classify the selected workpack only; do not use this file as permission to scan every workpack.
> Proves: routing and owner-path classification only.
> Does not prove: domain runtime readiness, policy readiness, enforcement readiness, AI readiness, transport readiness, custody readiness, or PR readiness.
> Proof rule: if this file changes route/status claims, update `AGENTS.md`, `PLAN_STATE.md`, and any affected selected workpack route.

<!-- /agent-capsule -->

# Portal UX Household Surfaces Workpack Families

Use this file to classify a selected workpack before opening source. This plan owns parent-facing UX presentation and proof. It consumes sibling domain read models and must not become the source of product truth.

## Shell, first-run, device, and selected-context family

```text
Workpacks:
WP01 Service-Backed Shell And Navigation
WP02 Household First-Run And Profiles
WP03 Device Inventory And Source States
WP04 Selected Device Context

Owners:
apps/portal for rendered route composition and local view behavior
portal-domain for public route/panel/projection contracts
account/setup/device-trust/domain owners for source truth through typed read models only

Rule:
Shell and device UX proof shows presentation of service-backed state. It does not prove account setup, device trust, install readiness, or child runtime behavior.
```

## Policy and approval UX family

```text
Workpacks:
WP05 Policy Authoring Control Center
WP06 Schedules, Time Budgets, And Templates
WP07 Parent Requests And Approvals

Owners:
portal UX plan for rendered parent-facing authoring, schedule, approval, state, and no-claim UX
policy-control-plane-plan for policy source truth, schedule/conflict semantics, approval/override contracts, and delivery state
account/device-trust owners for actor/step-up authority

Rule:
Portal previews and approvals render typed policy states only. They do not compile, evaluate, enforce, or approve without parent authority and policy handoff proof.
```

## Activity, diagnostics, and domain projection family

```text
Workpacks:
WP08 Activity Evidence And Diagnostics
WP09 Browser, App, And Network Surfaces
WP10 LAN Pairing State Consumption

Owners:
portal UX plan for read-model projection, labels, empty/degraded/error states, and presentation proof
browser/app-game/network/tracking/screen/lan plans for source evidence and runtime state

Rule:
Portal projection proof is not capture, browser URL truth, network truth, app/game truth, screen truth, or LAN transport proof.
```

## Assistant, reports, notifications, and custody family

```text
Workpacks:
WP11 Assistant Action Preview Flow
WP12 Reports, Notifications, And Custody

Owners:
portal UX plan for rendered explanations, citations, action previews, report surfaces, notification surfaces, and custody labels
AI, policy, notification, tracking/report, and data-custody plans for source truth and runtime behavior

Rule:
Assistant output is cited explanation or typed preview only. It cannot mutate policy, enforcement, billing, account, or child-device state without owning-plan proof and parent confirmation.
```

## Degraded state, audit, accessibility, and no-fake-data family

```text
Workpacks:
WP13 Degraded, Empty, Stale, And Error States
WP14 Audit History And Copy/Debug
WP15 Accessibility, Responsive, And Keyboard UX
WP16 No-Fake-Data Contract Adapter

Owners:
portal UX plan for visible state, labels, keyboard/responsive behavior, audit copy presentation, and fixture/runtime separation
owning domain/custody/logging plans for source truth and runtime/audit data

Rule:
Every visible product state must identify service/fixture/source/custody status. Portal-local placeholders are never product truth.
```

## Screenshot, mobile shell, docs sync, and manual review family

```text
Workpacks:
WP17 Playwright Screenshot Proof
WP18 Parent Mobile Shell Readiness
WP19 Product Docs And Checklist Sync
WP20 Manual User Review Gate

Owners:
portal UX plan for screenshot, responsive/mobile-width, manual-review, and docs/status proof
parent-client-runtime-distribution-plan for parent mobile/desktop package/runtime distribution
product owner/user for final visual approval where manual review is required

Rule:
Screenshots and mobile-width proof support review; they do not replace service-backed validation, runtime package proof, or user visual approval.
```
