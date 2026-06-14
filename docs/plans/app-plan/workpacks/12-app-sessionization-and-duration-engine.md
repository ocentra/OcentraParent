# 12 App Sessionization And Duration Engine

Sources: [test blueprint](../v0-5-native-apps-test-blueprint.md),
[current snapshot](../current-app-snapshot.md), and
`docs/architecture/app-game-evidence-sessions.md`.

## Where We Are

The architecture already requires sessions to be derived read models backed by
raw evidence IDs. Current app/game helpers provide a foundation, but native app
sessionization needs stronger running, foreground, gap, replay, and stale-state
proof.

Update on 2026-06-03: this workpack is now covered by the shared app/game WP13
implementation on `codex/app-game-sessionization-duration`. The before-state
gap was the old SQLite summary path grouping stored rows without a deterministic
duration reducer.

## Where We Want To Be

Running duration derives only from process/package evidence. Foreground duration
derives only from foreground evidence. Replay from journal/SQLite reconstructs
the same summaries after restart.

Current proof now covers deterministic replay from stored SQLite observation
rows. Encrypted journal-file ingest/replay and service restart integration
remain later storage work.

## Scope

- Running session state machine.
- Foreground interval state machine.
- Background duration derived from valid running minus foreground intervals.
- Stale gap, device sleep, restart, clock skew, process exit, and pid reuse.
- Source evidence refs and confidence.

## Touched Paths

- `crates/agent-core/src/activity_store_app_game*`
- `crates/agent-protocol/src/app_game.rs`
- `packages/activity-domain/src/app-game*.ts`
- `output/app-plan-proof/12-app-sessionization-and-duration-engine/*`
- `output/app-game-plan-proof/13-sessionization-and-duration-engine/*`

## Tests And Proof

- Session starts on first process observation.
- Session continues within allowed gap.
- Session closes on process exit or stale timeout.
- Foreground duration never exceeds running duration.
- Portal refresh interval cannot create duration.
- Restart/replay reconstructs the same summary.
- Daily app rollup totals equal summed session totals.

## AI Worker Checklist

Fill this before reporting `DONE` or PR-ready:

- [ ] Confirm source docs read: [folder README](../README.md), [source index](../source-index.md), [current snapshot](../current-app-snapshot.md), [full scope plan](../v0-5-native-apps-full-scope-plan.md), [platform deep dive](../v0-5-native-apps-platform-deep-dive.md), [test blueprint](../v0-5-native-apps-test-blueprint.md), [UI/UX guide](../ui-ux-requirements-guide.md), [main checklist](../implementation-checklist.md), and this workpack.
- [ ] Confirm this is native/installed-app scope, not browser pages, browser games, or game-specific product semantics unless the source docs explicitly route that handoff.
- [ ] Hub lock covers this workpack and exact implementation/docs paths.
- [ ] Existing app/game source layout inspected; no parallel app-control truth created.
- [ ] Before-state source snapshot recorded in `output/app-plan-proof/12-app-sessionization-and-duration-engine/00-source-snapshot.md`.
- [ ] Contracts updated first where this workpack changes behavior.
- [ ] Rust parity updated only after TypeScript contracts exist; service/portal parity remains out of scope and is recorded as a gap.
- [ ] Raw evidence artifacts captured where applicable: process observations, foreground observations, session summaries, SQLite/read-model rows, and daily rollups. Policy, approval, authority-tier, enforcement, and UI artifacts are explicit N/A/no-claim for this slice.
- [ ] Tests/proof listed in this workpack and [test blueprint](../v0-5-native-apps-test-blueprint.md) are implemented or explicitly marked manual-required with reason.
- [ ] Required fixtures are present or N/A with reason for inventory, runtime, foreground, session, policy, enforcement, UI, malicious metadata, stale state, and manual-required state.
- [ ] Validation command outputs saved in the proof pack and summarized in [main checklist](../implementation-checklist.md).
- [ ] UI snapshots recorded as not applicable because no portal, child UX, policy authoring, approval, evidence drawer, dashboard, stale, degraded, or manual-required state changed.
- [ ] Security/no-claim negative proof captured where applicable: inventory is not usage, running is not foreground, foreground is not content, AI cannot enforce, manual-required cannot call adapters, and private paths/command lines do not leak.
- [ ] Manual platform proof recorded as not applicable because this workpack makes no claim stronger than local stored-row observe-only proof.
- [ ] Platform limitations use capability status language and remain not-claimed/manual-required until later workpacks prove real adapters.
- [ ] Evidence/proof artifact paths recorded in [main checklist](../implementation-checklist.md).
- [ ] Feature/product-checklist update decision recorded in [main checklist](../implementation-checklist.md).
- [ ] Known gaps, deferred items, and no-claim boundaries recorded before `DONE`.

## Manual-Required Gaps

Session summaries prove duration only from stored evidence. They do not prove
content, intent, live capture, encrypted journal-file replay, service events,
portal rows, policy decisions, or enforcement capability.
