# 21 Journal And SQLite Ingest

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
