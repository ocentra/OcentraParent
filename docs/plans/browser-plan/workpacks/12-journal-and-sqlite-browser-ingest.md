# 12 Journal And SQLite Browser Ingest

<!-- agent-capsule -->

> Agent Capsule
> Plan: `browser-plan`
> Doc: `12 Journal And SQLite Browser Ingest`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

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

- [ ] Confirm source docs read: [folder README](../README.md), [source index](../source-index.md), [current snapshot](../current-browser-snapshot.md), [full scope plan](../v0-5-managed-browser-full-scope-plan.md), [test blueprint](../v0-5-managed-browser-test-blueprint.md), [UI/UX guide](../ui-ux-requirements-guide.md), [main checklist](../implementation-checklist.md), and this workpack.
- [ ] Check enhancement overlap: URL/video AI intelligence, social platform/account/feed gating, and browser games/cloud gaming.
- [ ] Hub lock covers this workpack and exact implementation/docs paths.
- [ ] Existing source layout inspected; no parallel browser truth created.
- [ ] Before-state source snapshot recorded in `output/browser-plan-proof/12-journal-and-sqlite-browser-ingest/00-source-snapshot.md`.
- [ ] Contracts were already in place; this workpack changes service capture ordering and replay proof.
- [ ] Rust/service parity updated through the existing service capture and core activity store path.
- [ ] Raw evidence artifacts captured where applicable: encrypted journal, SQLite/read-model rows, duplicate id state, stale timestamp, and restarted store read model.
- [ ] Tests/proof listed in this workpack are implemented or explicitly marked manual-required with reason.
- [ ] Validation command outputs saved in the proof pack and summarized in [main checklist](../implementation-checklist.md).
- [ ] UI snapshots are not applicable; `06-ui-snapshots/ui-not-applicable.md` records why.
- [ ] Security/no-claim negative proof captured where applicable: journal bytes do not expose browser URL plaintext and duplicate ids do not double count.
- [ ] Manual platform proof is not applicable; `09-manual-platform-proof.md` records why.
- [ ] Evidence/proof artifact paths recorded in [main checklist](../implementation-checklist.md).
- [ ] Feature/expectation/product-checklist/README update decision recorded in [main checklist](../implementation-checklist.md).
- [ ] Known gaps, deferred items, and no-claim boundaries recorded before `DONE`.

## Manual-Required Gaps

Stored evidence can be exact only if it originated from a valid managed browser
source.
