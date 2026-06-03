# 15 Parent Portal App Inventory Running Session Surfaces

Sources: [UI/UX guide](../ui-ux-requirements-guide.md),
[source index](../source-index.md), and
[test blueprint](../v0-5-native-apps-test-blueprint.md).

## Where We Are

Portal currently has live activity and policy-preview surfaces. A polished app
inventory, running, foreground, unknown, risk, capability, and evidence-detail
dashboard is not product-complete.

## Where We Want To Be

Parent UI renders service-backed native app state clearly and honestly. It
separates installed apps, running apps, foreground apps, duration, unknown/new
apps, risk candidates, policy decisions, authority/capability state, stale
state, degraded state, manual-required state, and evidence details.

## Scope

- App overview and installed apps.
- Running now, foreground now, recent sessions, and daily rollups.
- Unknown/new app review cards.
- Risk app cards with confidence and evidence refs.
- Authority/capability matrix rows.
- Evidence drawer and policy decision refs.
- Malicious/long app metadata layout and escaping.

## Touched Paths

- `apps/portal/src/live-activity-state.ts`
- `apps/portal/src/live-activity-panel.ts`
- `apps/portal/src/activity-timeline.ts`
- `apps/portal/src/portal-capability-guidance.ts`
- `apps/portal/src/PortalAppLayoutSurfacePanel.tsx`
- `vendor/ocentra-parent-core-ui/AppPages/ParentPortal/activity-ui-intent.ts`
- `apps/portal/tests/*app*` and Playwright specs when assigned.

## Tests And Proof

- Portal render tests for empty, normal, stale, degraded, manual-required,
  unknown, risk, and capability states.
- Playwright snapshots for dashboard, inventory details, evidence drawer,
  platform matrix, policy authoring, and malicious metadata.
- No portal-side OS scanning, SQLite reads, timer inference, AI classification,
  or enforcement calls.
- Inventory is not labeled as usage; running is not labeled as foreground;
  foreground is not labeled as content.

## AI Worker Checklist

Fill this before reporting `DONE` or PR-ready:

- [ ] Confirm source docs read: [folder README](../README.md), [source index](../source-index.md), [current snapshot](../current-app-snapshot.md), [full scope plan](../v0-5-native-apps-full-scope-plan.md), [platform deep dive](../v0-5-native-apps-platform-deep-dive.md), [test blueprint](../v0-5-native-apps-test-blueprint.md), [UI/UX guide](../ui-ux-requirements-guide.md), [main checklist](../implementation-checklist.md), and this workpack.
- [ ] Confirm this is native/installed-app scope, not browser pages, browser games, or game-specific product semantics unless the source docs explicitly route that handoff.
- [ ] Hub lock covers this workpack and exact implementation/docs paths.
- [ ] Existing app/game source layout inspected; no parallel app-control truth created.
- [ ] Before-state source snapshot recorded in `output/app-plan-proof/<workpack-id>/00-source-snapshot.md` or explicit docs-only N/A reason.
- [ ] Contracts updated first where this workpack changes behavior.
- [ ] Rust/service/portal parity updated only after TypeScript contracts exist.
- [ ] Raw evidence artifacts captured where applicable: inventory rows, process/package observations, foreground observations, session summaries, journal entries, SQLite/read-model rows, policy decisions, approval requests, authority-tier rows, and enforcement results.
- [ ] Tests/proof listed in this workpack and [test blueprint](../v0-5-native-apps-test-blueprint.md) are implemented or explicitly marked manual-required with reason.
- [ ] Required fixtures are present or N/A with reason for inventory, runtime, foreground, session, policy, enforcement, UI, malicious metadata, stale state, and manual-required state.
- [ ] Validation command outputs saved in the proof pack and summarized in [main checklist](../implementation-checklist.md).
- [ ] UI snapshots captured for every touched parent portal, child UX, policy authoring, approval, evidence drawer, dashboard, stale, degraded, or manual-required state; if no UI changed, `ui-not-applicable.md` records why.
- [ ] Security/no-claim negative proof captured where applicable: inventory is not usage, running is not foreground, foreground is not content, AI cannot enforce, manual-required cannot call adapters, and private paths/command lines do not leak.
- [ ] Manual platform proof captured for any claim stronger than observe-only, including OS/device version, authority tier, permission/enrollment setup, commands/UI steps, screenshots/logs, rollback, and cleanup.
- [ ] Platform limitations use capability status language: observe-only, permission-required, managed-device-required, admin/root-required, system-extension-required, supervised-device-required, manual-required, or not-claimed, with proof needed to move up.
- [ ] Evidence/proof artifact paths recorded in [main checklist](../implementation-checklist.md).
- [ ] Feature/expectation/product-checklist/README update decision recorded in [main checklist](../implementation-checklist.md).
- [ ] Known gaps, deferred items, and no-claim boundaries recorded before `DONE`.

## Manual-Required Gaps

UI can display a capability only when service/read-model state provides that
capability and proof state. UI cannot invent evidence or enforcement support.

## Completion Notes - 2026-06-03

- Branch: `codex/app-game-read-model-service-events`.
- Proof root:
  `output/app-plan-proof/15-parent-portal-app-inventory-running-session-surfaces`.
- Shared proof root:
  `output/app-game-plan-proof/16-parent-portal-app-game-dashboard-surfaces`.
- Source docs read: folder README, source index, current snapshot, full scope
  plan, platform deep dive, test blueprint, UI/UX guide, main checklist, and
  this workpack.
- Implementation: native app-use read-model rows feed the shared App/Game
  Sessions dashboard intent and surface. The dashboard keeps installed
  inventory, running, foreground, duration, unknown/risk/manual-required
  capability, and evidence counts separate.
- Product checklist: not updated because live adapters, unknown approval,
  policy integration, platform proof, and broad blocking remain gaps.
