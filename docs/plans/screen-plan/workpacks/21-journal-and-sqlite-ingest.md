# 21 Journal And SQLite Ingest

<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-plan`
> Doc: `21 Journal And SQLite Ingest`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

Summary/evidence refs/deletion state/model refs are stored, read models rebuild from journal, and no raw images are stored.

## Current State

Partial store foundation exists in `crates/agent-core/src/activity_store_screen_evidence.rs`.

## Checklist

- [ ] Store summary.
- [ ] Store evidence refs.
- [ ] Store deletion state.
- [ ] Store model refs.
- [ ] Rebuild read model from journal.
- [ ] Prove no raw image blobs/paths by default.

## Proof

- Store/query tests.
- SQLite output showing summary-only data.
