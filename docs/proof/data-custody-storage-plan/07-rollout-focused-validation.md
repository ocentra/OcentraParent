# WP07 focused lifecycle validation

This proof is intentionally limited to the child-runtime tombstone lifecycle;
it does not accept the ignored aggregate `output/` roots or claim plan-wide
custody readiness.

## Commands

```text
cargo test -p ocentra-storage-custody-core --test unit retention_delete_tombstone_store
cargo test -p ocentra-child-runtime --test unit_runtime_gate tombstone
cargo test -p ocentra-child-runtime --test unit_runtime_gate child_runtime_custody_event_flow
npm run lint:architecture -- --files crates/storage-custody-core,crates/child-runtime,docs/plans/data-custody-storage-plan
npm run hub:guard
npm run precommit
git diff --check
```

## Observed results

The focused commands completed successfully on this revision: the custody
store filter ran **9 passed**, the child-runtime tombstone filter ran **3
passed**, and the event-flow filter ran **2 passed**. Architecture policy,
Enforcer guard, and the staged pre-commit validation also completed without
findings.

## Assertions covered

- typed delete intents survive restart until terminal publication;
- legacy v1 pending rows are replaced by replayable typed rows;
- terminal acknowledgement compacts the row to a replay-protection marker;
- unknown terminal acknowledgement references fail closed with `NotFound` and
  leave the pending record unchanged;
- the child-runtime event flow records durable-outbox and journal milestones;
- transient journal I/O remains retryable while identity/corruption errors
  propagate to the owning runtime path.

The file is a tracked audit pointer rather than an ignored generated log.
