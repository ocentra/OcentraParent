# 13 Sessionization And Duration Engine

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `13 Sessionization And Duration Engine`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

## Target State

Running and foreground durations are derived from stored evidence and replayable
for apps, games, launchers, and candidates.

## Completion Notes - 2026-06-03

Branch: `codex/app-game-sessionization-duration`

Implemented scope:

- `packages/activity-domain` session schemas now require closed sessions to pair
  `endedAt` with `endReason`, require foreground/background duration evidence
  timestamps, and add daily rollup contracts with exact duration totals.
- `crates/agent-protocol` mirrors session end reasons, observation gaps,
  foreground/background timestamps, and daily rollup payloads.
- `crates/agent-core` now derives session summaries from stored SQLite
  observation rows through a deterministic reducer instead of grouping rows with
  zero durations.
- Reducer proof covers process-derived running duration, foreground-window
  duration bounded by running duration, background duration, stale-gap closure,
  process-exit closure, replay-order stability, session end reasons,
  observation-gap tracking, and daily rollup totals.

Out of scope and still not claimed:

- encrypted journal-file ingest/replay;
- live process or foreground subscriptions;
- live launcher crawling or child-game linkage;
- service events and portal app/game dashboard rows;
- policy execution, time-budget enforcement, or broad blocking;
- UI snapshots, because this workpack did not change UI.

## Scope

- Session start, continuation, gap, close, stale, restart/replay, foreground
  interval, background duration, launcher-only duration, and game-candidate
  duration.
- App and game daily rollups.

## Tests And Proof

- Session starts on first runtime observation.
- Session continues within gap window.
- Session closes on exit/stale timeout.
- Foreground duration never exceeds running duration.
- Replay reconstructs the same summary.
- Daily rollup totals equal summed session totals.
- Launcher-only session does not become game session. Existing launcher
  no-upgrade proof remains from WP10; this workpack does not add live launcher
  crawler proof.

Proof pack:

```text
output/app-game-plan-proof/13-sessionization-and-duration-engine/
```

## Done Signal

Session summaries are deterministic read models, not portal refresh counters.

Use the standard checklist in [workpacks README](README.md).

## AI Worker Checklist

- [ ] Source docs read: feature doc, expectation routing, architecture session
      doc, app-plan bridge, source index, current snapshot, implementation
      checklist, and this workpack.
- [ ] Hub lock covered exact implementation, docs, and proof paths before edits.
- [ ] Existing source layout inspected; reducer extends current
      `agent-core` app/game store paths instead of creating parallel truth.
- [ ] TypeScript Effect Schema contracts updated before Rust parity and core
      reducer behavior.
- [ ] Rust protocol parity updated for new session summary fields and daily
      rollup payloads.
- [ ] Stored-row runtime evidence proof recorded in
      `03-runtime-evidence.json` and `04-journal-sqlite-proof.json`.
- [ ] Policy/action and UI items recorded as N/A with explicit no-claim files
      because this slice is read-model only.
- [ ] Security negative proof records inventory-is-not-use,
      running-is-not-foreground, foreground-is-not-content, launcher-is-not-game,
      AI-cannot-enforce, manual-required-cannot-execute, and private path
      non-surfacing boundaries.
- [ ] Validation commands are recorded in the proof pack.
- [ ] Feature doc and product capability checklist proof text were updated
      without moving product status to complete.
- [ ] Known gaps are recorded above and in the proof pack.
