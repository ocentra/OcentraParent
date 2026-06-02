# 12 Journal And SQLite Browser Ingest

## Where We Are

Browser bridge events can be recorded through activity journal and SQLite paths
when service browser runtime polls a configured bridge port.

## Where We Want To Be

All browser evidence and intervention state is journaled first, replayed into
SQLite second, and served through read models third.

## Scope

- Browser evidence journal envelope.
- Browser intervention journal envelope.
- SQLite ingest/replay.
- Stable evidence ids through replay.
- Stale/degraded reconstruction after restart.
- Duplicate id handling.
- Portal/policy/AI read APIs using stored read models only.

## Touched Paths

- `crates/agent-core/src/activity_store_browser*.rs`
- `crates/agent-service/src/activity_capture.rs`
- `crates/agent-service/src/browser_runtime.rs`
- `packages/activity-domain/src/browser*.ts`

## Tests And Proof

- Journal write tests.
- SQLite replay tests.
- Restart/stale tests.
- Service read-model tests.

## AI Worker Checklist

Fill this before reporting `DONE` or PR-ready:

- [ ] Confirm source docs read: [folder README](../README.md), [source index](../source-index.md), [current snapshot](../current-browser-snapshot.md), [full scope plan](../v0-5-managed-browser-full-scope-plan.md), [test blueprint](../v0-5-managed-browser-test-blueprint.md), [UI/UX guide](../ui-ux-requirements-guide.md), [main checklist](../implementation-checklist.md), and this workpack.
- [ ] Check enhancement overlap: URL/video AI intelligence, social platform/account/feed gating, and browser games/cloud gaming.
- [ ] Hub lock covers this workpack and exact implementation/docs paths.
- [ ] Existing source layout inspected; no parallel browser truth created.
- [ ] Before-state source snapshot recorded in `output/browser-plan-proof/<workpack-id>/00-source-snapshot.md` or explicit docs-only N/A reason.
- [ ] Contracts updated first where this workpack changes behavior.
- [ ] Rust/service/portal parity updated only after contracts exist.
- [ ] Raw evidence artifacts captured where applicable: bridge/CDP payloads, managed session state, unmanaged process rows, journal entries, SQLite/read-model rows, policy decisions, and action results.
- [ ] Tests/proof listed in this workpack are implemented or explicitly marked manual-required with reason.
- [ ] Validation command outputs saved in the proof pack and summarized in [main checklist](../implementation-checklist.md).
- [ ] UI snapshots captured for every touched parent portal, child UX, block/warn, policy authoring, or dashboard state; if no UI changed, `ui-not-applicable.md` records why.
- [ ] Security/no-claim negative proof captured where applicable: no default profile attach, no unowned bridge, no unmanaged exact URL claim, no raw debugger URL exposure, and no AI direct enforcement.
- [ ] Manual platform proof captured for real browser/OS claims, including OS/browser version, command steps, screenshots/logs, and manual-required labels.
- [ ] Evidence/proof artifact paths recorded in [main checklist](../implementation-checklist.md).
- [ ] Feature/expectation/product-checklist/README update decision recorded in [main checklist](../implementation-checklist.md).
- [ ] Known gaps, deferred items, and no-claim boundaries recorded before `DONE`.

## Manual-Required Gaps

Stored evidence can be exact only if it originated from a valid managed browser
source.
