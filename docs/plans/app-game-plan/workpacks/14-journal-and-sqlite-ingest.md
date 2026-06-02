# 14 Journal And SQLite Ingest

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

Use the standard checklist in [workpacks README](README.md).
