# 14 Journal And SQLite Ingest

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `14 Journal And SQLite Ingest`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

App/game inventory, runtime, foreground, launcher, session, policy, approval, and
enforcement evidence is journaled and replayed before consumers use it.

## Scope

- Journal event schema and custody labels.
- SQLite ingest/replay rows.
- Invalid evidence rejection.
- Replay proof for app/game sessions and read models.

## Tests And Proof

- Inventory evidence writes to journal.
- Runtime and foreground evidence write to journal.
- Launcher evidence writes to journal.
- SQLite replay produces inventory, running-now, foreground-now, launcher, and
  rollup rows.
- Invalid evidence is rejected before SQLite.

## Done Signal

Consumers use stored/replayed evidence, not live portal state or direct OS reads.

## AI Worker Checklist

- [ ] Confirm source docs read: [folder README](../README.md), [source index](../source-index.md), [current snapshot](../current-app-game-snapshot.md), implementation checklist, app-plan bridge workpack, feature doc, app/game evidence expectation, evidence-storage expectation, architecture doc, and this workpack.
- [ ] Confirm this is shared native app/game evidence-spine scope, not browser pages or browser games.
- [ ] Hub lock covers this workpack and exact implementation/docs/proof paths.
- [ ] Existing app/game source layout inspected; no parallel app-control truth created.
- [ ] Before-state source snapshot recorded in `output/app-game-plan-proof/14-journal-and-sqlite-ingest/00-source-snapshot.md`.
- [ ] Protocol constants updated before Rust replay consumers depend on them.
- [ ] Raw evidence artifacts captured for inventory, runtime, foreground, launcher, journal, SQLite replay, and daily rollup fixture rows.
- [ ] Tests/proof listed in this workpack are implemented or explicitly marked N/A/manual-required with reason.
- [ ] Validation command outputs saved in the proof pack and summarized in [main checklist](../implementation-checklist.md).
- [ ] UI snapshots marked N/A because no portal, child UX, policy authoring, or evidence drawer UI changed.
- [ ] Security/no-claim negative proof captured: inventory is not use, running is not foreground, foreground is not content, launcher is not game, duplicate rows do not double count, and invalid evidence is rejected before SQLite.
- [ ] Manual platform proof marked N/A because no live adapter or enforcement authority changed.
- [ ] Evidence/proof artifact paths recorded in [main checklist](../implementation-checklist.md).
- [ ] Feature/expectation/product-checklist/README update decision recorded in [main checklist](../implementation-checklist.md).
- [ ] Known gaps, deferred items, and no-claim boundaries recorded before `DONE`.

## Completion Notes

Completed on branch `codex/app-game-journal-sqlite-ingest` with proof under
`output/app-game-plan-proof/14-journal-and-sqlite-ingest/`.

This slice proves staged typed app/game inventory, runtime, foreground, and
launcher rows can be appended to the real encrypted journal path, replayed into
SQLite, and projected into inventory, running-now, foreground-now, launcher, and
daily rollup rows. It also proves invalid inventory-use claims are rejected
before persistence and duplicate runtime observations do not double count
duration.

Current source/test review (2026-08-29):

- the shipped capture path appends typed app/game events to the encrypted
  `ActivityJournal`, ingests them transactionally into SQLite, and projects the
  stored read model;
- registered behavioral tests cover invalid-row rejection,
  duplicate/idempotent ingestion, persisted restart replay, SQLite rollback,
  tamper/ciphertext rejection, and retry without duplicate journal append;
- corrupt journal input fails closed. Automatic quarantine/recovery remains a
  deferred custody policy and must not be invented in this workpack.

Still open beyond this source/test slice:

- live source adapters appending production rows;
- service events and portal app/game read models;
- policy/action/approval integration;
- current focused execution and retained proof refresh;
- live platform authority and broad blocking proof.
