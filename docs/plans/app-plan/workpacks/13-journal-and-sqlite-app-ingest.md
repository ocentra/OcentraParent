# 13 Journal And SQLite App Ingest

Sources: [test blueprint](../v0-5-native-apps-test-blueprint.md),
`docs/expectations/evidence-storage.md`, and
`docs/architecture/app-game-evidence-sessions.md`.

## Where We Are

SQLite-backed app/game observation helpers exist, but the native app plan needs
complete ingest for inventory, runtime, foreground, session, authority,
policy/action, approval, and degraded-state records.

## Where We Want To Be

Raw evidence writes before consumers use it. Journal replay rebuilds inventory,
running-now, foreground-now, session, daily rollup, approval, and capability
read models deterministically.

## Scope

- Journal entry contracts and storage.
- SQLite ingest and replay.
- Evidence refs for sessions, policies, approvals, and enforcement results.
- Duplicate, stale, invalid, and out-of-order evidence handling.
- Redacted proof output and support bundle safety.

## Touched Paths

- `crates/agent-core/src/activity_store_app_game*`
- `crates/agent-service/src/app.rs`
- `crates/agent-protocol/src/app_game.rs`
- `output/app-plan-proof/<workpack-id>/04-journal-sqlite-proof.json`

## Tests And Proof

- Inventory, runtime, and foreground evidence write to journal.
- SQLite replay creates inventory, running-now, foreground-now, and daily rollup
  read models.
- Invalid evidence is rejected before SQLite.
- Duplicate observations do not double count duration.
- Restart preserves approval/manual-required state.

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

In-memory portal state is not evidence. Claims must survive journal/SQLite
replay unless explicitly marked prototype or N/A.
