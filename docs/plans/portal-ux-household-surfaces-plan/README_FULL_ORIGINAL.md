# Portal UX Household Surfaces Plan

This folder is the single working plan location for the C lane: coherent
parent-facing household UX across setup, devices, policy, activity, LAN state,
assistant/report surfaces, degraded states, and proof.

- [Portal UX Household Surfaces 20-Step Plan](portal-ux-household-surfaces-20-step-plan.md)
- [Portal UX Household Surfaces Test Blueprint](portal-ux-household-surfaces-test-blueprint.md)
- [Portal UX Requirements Guide](ui-ux-requirements-guide.md)

The rule remains:

```text
Service tells truth. Portal makes it usable. Parent decides.
```

## Where We Are

- The portal already has development surfaces for devices, activity, policy, AI,
  account, and diagnostics against the Rust service path.
- Current product docs say first-run household setup, child profiles,
  parent-controller/observer state, route source labels, policy authoring,
  reports, notifications, and assistant action flow are still incomplete.
- C is active on `codex/portal-ux-real-household-surfaces` and owns visual
  coherence, layout, interaction, responsiveness, and ergonomics.
- C does not own child-device authority, LAN discovery implementation,
  enforcement adapters, policy evaluation, AI execution, timers, package
  runtime, or release engineering.

## B-Lane LAN UX Addendum - 2026-06-02

This folder remains the broad household UX plan, but the LAN-specific workflow
is not split away from the LAN implementation. For the current
`codex/v0-9-lan-signed-discovery-relay-spine` branch, B owns the service-backed
LAN UX wiring needed for signed discovery, trusted registry/route custody,
relay/cache unavailable states, assign/rename/ignore/revoke decisions,
degraded/offline/manual-required states, and Activity/Network LAN diagnostics.
C still owns broad shell visual coherence and general portal look/feel.

- [ ] The LAN plan and feature docs now identify B as the owner of the
      service-to-UI LAN workflow for this slice.
- [ ] Existing portal screenshots prove there are visible Devices/LAN,
      Activity/Network, and Network policy surfaces to wire against.
- [ ] B now wires LAN-specific selected-device detail, Activity/Network
      diagnostics, signed discovery, route custody, relay/cache unavailable,
      manual-required proof, parent-decision/audit state, first-class LAN
      action controls, and the existing add-device/route command surfaces
      through typed service/domain state.
- [ ] Browser/Playwright snapshots prove the current service-backed LAN UX
      surfaces on B-lane ports. Proof artifacts:
      `output/playwright/lan-ux-proof/devices-lan-controls.png`,
      `output/playwright/lan-ux-proof/activity-network-diagnostics.png`, and
      `output/playwright/lan-ux-proof/policy-network-targets.png`.

## Where We Want To Be

Ocentra Parent needs a service-backed parent portal that:

- feels like one household product instead of disconnected proof panels;
- starts with household, child, device, and route state before policy controls;
- shows local, LAN, relay, cache, parent-owned storage, unavailable, stale,
  degraded, and manual-required source states consistently;
- makes discovered, assigned, confirmed, trusted, ignored, stale, offline, and
  agent-connected device states understandable without claiming false control;
- lets parents author rules, schedules, requests, approvals, and previews through
  typed intents;
- shows activity, network, browser, app/game, screen, AI, reports, and assistant
  states with evidence/custody labels;
- passes Playwright and browser checks on the real service path before any UI
  readiness claim.

## Coverage Check Against Product Docs

This plan was grounded in:

- `docs/features/family-setup-device-roles.md`
- `docs/features/remote-lan-mobile-platforms.md`
- `docs/features/policy-schedules-approvals.md`
- `docs/features/reports-notifications-sync.md`
- `docs/features/parent-assistant-actions.md`
- `docs/features/evidence-store-query.md`
- `docs/expectations/portal.md`
- `docs/expectations/family-setup.md`
- `docs/expectations/lan-pairing.md`
- `docs/expectations/policy.md`
- `docs/expectations/real-evidence-proof.md`

The plan does not ask C to invent backend truth. It asks C to make the real
service states usable, visibly source-labeled, and testable.

## Parallel Coordination Rules

- C owns this UX program while the user is steering look, feel, layout,
  interaction, and ergonomics.
- Other lanes own runtime, LAN, enforcement, package, mobile, and backend proof.
- C should lock exact portal/domain/test paths before editing and report which
  workpacks changed.
- New portal actions need typed intents or queries before UI buttons claim
  success.
- Visual review artifacts should be attached to reports, but product readiness
  still requires service-backed tests and browser-console checks.
- Every `DONE` report must name workpacks, touched paths, validation, screenshots
  or browser proof when relevant, product-doc updates, and known runtime gaps.

## Workpack Checklist

| Step | Workpack                                                                                           | Target State                                                                                   |
| ---- | -------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------- |
| 01   | [Service-backed shell and navigation](workpacks/01-service-backed-shell-and-navigation.md)         | Portal navigation presents one household product with no fake local state.                     |
| 02   | [Household first-run and profiles](workpacks/02-household-first-run-and-profiles.md)               | Parents can understand household, child profiles, and setup completion.                        |
| 03   | [Device inventory and source states](workpacks/03-device-inventory-and-source-states.md)           | Devices show role, evidence, source, trust, stale/offline, and controllability state.          |
| 04   | [Selected device context](workpacks/04-selected-device-context.md)                                 | Every route makes selected child/device/route status clear.                                    |
| 05   | [Policy authoring control center](workpacks/05-policy-authoring-control-center.md)                 | Rule authoring is parent-friendly while still typed and preview-first.                         |
| 06   | [Schedules, time budgets, and templates](workpacks/06-schedules-time-budgets-and-templates.md)     | Parents can scan schedules, limits, modes, exceptions, and conflicts.                          |
| 07   | [Parent requests and approvals](workpacks/07-parent-requests-and-approvals.md)                     | Pending requests, approvals, denials, bonus time, and expiry have clear UI states.             |
| 08   | [Activity evidence and diagnostics](workpacks/08-activity-evidence-and-diagnostics.md)             | Activity views cite source evidence and degraded/custody state.                                |
| 09   | [Browser, app, and network surfaces](workpacks/09-browser-app-and-network-surfaces.md)             | Evidence types stay visually distinct and honest about unknowns.                               |
| 10   | [LAN pairing state consumption](workpacks/10-lan-pairing-state-consumption.md)                     | LAN B-lane states render without becoming a second LAN source of truth.                        |
| 11   | [Assistant action preview flow](workpacks/11-assistant-action-preview-flow.md)                     | Assistant suggestions become cited previews, not direct writes.                                |
| 12   | [Reports, notifications, and custody](workpacks/12-reports-notifications-and-custody.md)           | Reports and alerts show source, custody, delivery, retention, and drill-in state.              |
| 13   | [Degraded, empty, stale, and error states](workpacks/13-degraded-empty-stale-and-error-states.md)  | Empty and failure states are useful and never look like fake success.                          |
| 14   | [Audit history and copy/debug](workpacks/14-audit-history-and-copy-debug.md)                       | Parents and developers can copy concise redacted diagnostic state.                             |
| 15   | [Accessibility, responsive, and keyboard UX](workpacks/15-accessibility-responsive-keyboard-ux.md) | Core workflows work on common desktop/mobile widths and keyboard paths.                        |
| 16   | [No-fake-data contract adapter](workpacks/16-no-fake-data-contract-adapter.md)                     | UI fixtures are separated from runtime claims and real service payloads are validated.         |
| 17   | [Playwright screenshot proof](workpacks/17-playwright-screenshot-proof.md)                         | Visual behavior is tested through the real service path with artifacts.                        |
| 18   | [Parent mobile shell readiness](workpacks/18-parent-mobile-shell-readiness.md)                     | Layout and source-state patterns can map to parent mobile without claiming mobile child-agent. |
| 19   | [Product docs and checklist sync](workpacks/19-product-docs-and-checklist-sync.md)                 | Feature docs/checklist reflect UX status and remaining runtime gaps.                           |
| 20   | [Manual user review gate](workpacks/20-manual-user-review-gate.md)                                 | User/C review is a named gate before declaring UX ready.                                       |

## UI/UX Requirement Links

- Main UI rule: source, authority, and proof level must be visible anywhere a
  parent could make a decision.
- Portal routes must not look like marketing pages or developer-only cards.
  They should be dense, calm, scan-friendly product surfaces.
- The detailed UX expectations live in
  [the UI/UX guide](ui-ux-requirements-guide.md).
