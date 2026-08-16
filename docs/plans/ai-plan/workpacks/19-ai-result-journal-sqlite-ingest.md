# 19 - AI Result Journal SQLite Ingest

<!-- agent-capsule -->

> Agent Capsule
> Plan: `ai-plan`
> Doc: `19 - AI Result Journal SQLite Ingest`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

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
