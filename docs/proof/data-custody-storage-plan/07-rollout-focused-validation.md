# WP07 focused lifecycle validation

This proof is intentionally limited to the child-runtime tombstone lifecycle;
it does not accept the ignored aggregate `output/` roots or claim plan-wide
custody readiness.

## Commands

```text
cargo test -p ocentra-storage-custody-core --test unit retention_delete_tombstone_store
cargo test -p ocentra-child-runtime --test unit_runtime_gate tombstone
npm run lint:architecture -- --files crates/storage-custody-core,crates/child-runtime,docs/plans/data-custody-storage-plan
npm run hub:guard
npm run precommit
git diff --check
```

## Assertions covered

- typed delete intents survive restart until terminal publication;
- legacy v1 pending rows are replaced by replayable typed rows;
- terminal acknowledgement compacts the row to a replay-protection marker;
- the child-runtime event flow records durable-outbox and journal milestones;
- transient journal I/O remains retryable while identity/corruption errors
  propagate to the owning runtime path.

The command results are refreshed by the PR validation run for this revision;
the file is a tracked audit pointer rather than an ignored generated log.
