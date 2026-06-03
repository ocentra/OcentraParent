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
- Proof pack:
  `output/browser-plan-proof/12-journal-and-sqlite-browser-ingest/`.

## Implementation Notes

- Service activity capture now appends events to the encrypted journal first and
  replays the newly appended journal lines into SQLite, instead of passing the
  original in-memory events directly to the store.
- Browser evidence tests prove duplicate browser event ids do not double count
  and that restarted store reads preserve stable evidence ids, stale timestamps,
  and target-list-only active proof source.
- Existing core browser store tests cover encrypted journal replay for browser
  evidence and browser interventions.

## AI Worker Checklist

Fill this before reporting `DONE` or PR-ready:

- [x] Confirm source docs read: [folder README](../README.md), [source index](../source-index.md), [current snapshot](../current-browser-snapshot.md), [full scope plan](../v0-5-managed-browser-full-scope-plan.md), [test blueprint](../v0-5-managed-browser-test-blueprint.md), [UI/UX guide](../ui-ux-requirements-guide.md), [main checklist](../implementation-checklist.md), and this workpack.
- [x] Check enhancement overlap: URL/video AI intelligence, social platform/account/feed gating, and browser games/cloud gaming.
- [x] Hub lock covers this workpack and exact implementation/docs paths.
- [x] Existing source layout inspected; no parallel browser truth created.
- [x] Before-state source snapshot recorded in `output/browser-plan-proof/12-journal-and-sqlite-browser-ingest/00-source-snapshot.md`.
- [x] Contracts were already in place; this workpack changes service capture ordering and replay proof.
- [x] Rust/service parity updated through the existing service capture and core activity store path.
- [x] Raw evidence artifacts captured where applicable: encrypted journal, SQLite/read-model rows, duplicate id state, stale timestamp, and restarted store read model.
- [x] Tests/proof listed in this workpack are implemented or explicitly marked manual-required with reason.
- [x] Validation command outputs saved in the proof pack and summarized in [main checklist](../implementation-checklist.md).
- [x] UI snapshots are not applicable; `06-ui-snapshots/ui-not-applicable.md` records why.
- [x] Security/no-claim negative proof captured where applicable: journal bytes do not expose browser URL plaintext and duplicate ids do not double count.
- [x] Manual platform proof is not applicable; `09-manual-platform-proof.md` records why.
- [x] Evidence/proof artifact paths recorded in [main checklist](../implementation-checklist.md).
- [x] Feature/expectation/product-checklist/README update decision recorded in [main checklist](../implementation-checklist.md).
- [x] Known gaps, deferred items, and no-claim boundaries recorded before `DONE`.

## Manual-Required Gaps

Stored evidence can be exact only if it originated from a valid managed browser
source.
