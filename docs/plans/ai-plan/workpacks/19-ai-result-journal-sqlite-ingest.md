# 19 - AI Result Journal SQLite Ingest

## Target State

AI results and policy decisions are written to the encrypted journal and replayed
into SQLite read models.

## Where We Are

Activity store and memory graph pieces exist. AI result journaling must be a
first-class event family with refs and replay proof.

## Checklist

- [ ] Define AI result journal event.
- [ ] Define policy decision journal event.
- [ ] Include evidence, rule, runtime, prompt, memory, and graph refs.
- [ ] Add SQLite ingest/read model.
- [ ] Add replay tests.

## Proof

- Journal serialization tests.
- SQLite ingest tests.
- Replay proof from stored evidence to portal read model.
