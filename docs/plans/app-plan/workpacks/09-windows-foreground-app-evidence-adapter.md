# 09 Windows Foreground App Evidence Adapter

Sources: [full scope plan](../v0-5-native-apps-full-scope-plan.md),
[test blueprint](../v0-5-native-apps-test-blueprint.md), and
[UI/UX guide](../ui-ux-requirements-guide.md).

## Where We Are

The app/game architecture already separates running time from foreground time.
The Windows-specific foreground evidence adapter is not yet fully specified or
proved for native app sessions.

## Where We Want To Be

Foreground window evidence proves active app focus only. It can contribute to
foreground duration and UI "active app" claims without implying app content,
messages, documents, video details, or browser URL semantics.

## Scope

- Foreground window/app observation.
- Optional and permission-gated window title handling.
- Active app identity mapping back to runtime/inventory evidence.
- Foreground switch intervals.
- Permission_limited, title_omitted, adapter_error, and stale states.
- Redaction of private paths and sensitive titles where required.

## Touched Paths

- future Windows foreground adapter in `crates/agent-core/src/`
- `crates/agent-core/src/activity_store_app_game*`
- `packages/activity-domain/src/app-game*.ts`
- `fixtures/app/runtime/windows_foreground_window.json`
- portal live-activity tests when UI-facing.

## Tests And Proof

- Foreground switch updates foreground interval.
- Background apps do not gain foreground time.
- Foreground duration never exceeds running duration.
- Title disabled stores title_omitted.
- Permission denied stores permission_limited.
- UI labels foreground as focus, not content knowledge.

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

Completion note: this shared app/game WP09 slice adds contract/protocol/parser
proof only. Live Windows foreground polling, journal/SQLite ingest, service
events, portal foreground rows, content knowledge, policy execution, and broad
blocking remain explicitly unclaimed.

## Manual-Required Gaps

Foreground app evidence is not app content, feed, chat, browser URL, or screen
capture proof.
