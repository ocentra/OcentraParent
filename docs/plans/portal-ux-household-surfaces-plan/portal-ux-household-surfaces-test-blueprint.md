# Portal UX Household Surfaces Test Blueprint

<!-- agent-capsule -->

> Agent Capsule
> Plan: `portal-ux-household-surfaces-plan`
> Doc: `Portal UX Household Surfaces Test Blueprint`
> Kind: test blueprint reference; read only when local expectations route here.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.

<!-- /agent-capsule -->

This is the companion requirement blueprint for the
[Portal UX Household Surfaces 20-Step Plan](portal-ux-household-surfaces-20-step-plan.md).

## Scope

Included:

- Portal shell, navigation, selected device context, and source labels.
- Household setup, child profiles, device inventory, and route state.
- Policy authoring, schedules, budgets, ask-parent, approvals, and audit.
- Activity/evidence diagnostics, browser/app/network/screen distinction.
- Assistant action preview, reports, notifications, custody, and copy/debug.
- Degraded, empty, stale, responsive, accessibility, and visual proof.

Excluded:

- Rust service implementation.
- LAN scanner/pairing implementation.
- OS enforcement adapters.
- Policy evaluator execution.
- AI model execution.
- Tauri/mobile package runtime.

## Core Test Principle

### UX-TEST-001: Service-backed truth

Requirement: Product routes render validated service/read-model state or
explicit fixture/demo labels.

Proof: Playwright runs against the real Rust service path and asserts visible
connection/source state.

Acceptance: A route cannot silently show fake household data as product data.

### UX-TEST-002: Source and authority visible

Requirement: Parent decisions must show whether state is live local, LAN, relay,
cache, parent-owned storage, unavailable, stale, degraded, or manual-required.

Proof: Route tests check source-state labels on Devices, Policy, Activity, and
Assistant/Reports where present.

Acceptance: A parent can tell what the UI is allowed to do before acting.

### UX-TEST-003: Selected device is clear

Requirement: Device/child/route context must be visible before policy,
activity, assistant, or report actions.

Proof: Tests navigate across routes and assert selected-device state persists or
fails visibly.

Acceptance: A command cannot look like it applies to the wrong child/device.

### UX-TEST-004: Evidence types stay distinct

Requirement: Browser URL, unmanaged browser, app/game session, network flow, and
screen-analysis evidence must have distinct labels and limitations.

Proof: Tests check route copy/state for exact URL versus process-only versus
network summary and unknown states.

Acceptance: UI cannot imply page content, decrypted network traffic, or AI
certainty from weaker evidence.

### UX-TEST-005: Parent actions are typed intents

Requirement: UI controls produce typed intent/request state and render
child-agent result when available.

Proof: Playwright clicks supported controls and confirms visible pending,
accepted, rejected, unavailable, or manual-required service output.

Acceptance: UI cannot mark an action successful from browser-local state alone.

### UX-TEST-006: Empty and degraded states are useful

Requirement: Empty, loading, stale, offline, service unavailable,
permission-limited, unsupported, and manual-required states must be visually
clear and action-oriented.

Proof: Fixture-backed and service-backed tests cover each state.

Acceptance: Empty states do not masquerade as successful product data.

### UX-TEST-007: No overlap or clipping

Requirement: Text, controls, cards, tables, drawers, and dialogs must not
overlap or clip at common desktop/mobile widths.

Proof: Browser screenshots and visual inspection cover key routes and long text.

Acceptance: The product remains usable during manual testing.

### UX-TEST-008: Browser-visible errors are product issues

Requirement: Touched routes must not log page errors or unexplained console
errors/warnings.

Proof: Playwright fails on page errors and relevant console errors.

Acceptance: A green visual screenshot alone is not enough.

## Required Fixture Families

- Household: empty household, one child, multiple children, co-parent observer,
  setup incomplete, recovery needed.
- Devices: local agent, LAN confirmed agent, passive neighbor, router,
  ignored device, stale device, offline device, revoked device, manual-required.
- Policy: no rules, active schedule, conflict, dry-run preview, ask-parent
  pending, approved, denied, expired, manual-required enforcement.
- Activity: browser exact URL, unmanaged browser process, app/game session,
  network IP-only, network domain-known, screen summary, unknown state.
- Assistant/report: cited answer, action preview, provider degraded, report
  unavailable, notification queued, notification manual-required.
- Layout: long child name, long hostname, no network, many devices, many rules,
  small width, keyboard focus.

## Acceptance And Proof

- Real-service Playwright coverage for route connection and current live states.
- Fixture-driven visual coverage only where the service cannot yet produce a
  state, with explicit fixture/demo labels.
- Browser screenshots for user review when layout or interaction changes.
- Product docs/checklist updates when a surface moves status.
- No UI readiness claim before user/C manual review for look and ergonomics.
