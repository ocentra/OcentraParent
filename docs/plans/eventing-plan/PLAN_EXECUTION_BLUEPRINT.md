# Eventing Plan � HID Execution Blueprint

## Execution objective

Complete the reusable eventing contract with clear separation from consumer behavior.

## Slice 01 � Envelope and Version Contract

### Acceptance

- Version-skew and envelope parse failures fail closed.

### Tests

- `eventing.versioning.schema-drift`
- `eventing.idempotency.replay-duplicate`

### Proof

- `docs/proof/eventing-plan/slice-01-envelope-version.md`

## Slice 02 � Ordering, Replay, Dead-letter

### Acceptance

- Replay, duplication, timeout, ordering and dead-letter behavior are explicit.

### Tests

- `eventing.idempotency.replay-duplicate`
- `eventing.journal.recover-corruption`

### Proof

- `docs/proof/eventing-plan/slice-02-ordering-replay.md`

## Slice 03 � Consumer Boundary and Product Claims

### Acceptance

- Reusable eventing crate does not claim downstream product semantics.

### Tests

- `eventing.consumer.no-product-claim`

### Proof

- `docs/proof/eventing-plan/slice-03-consumer-boundary.md`

## Workpacks (execution lane)

### Slice-to-workpack binding

- Slice 01: docs/plans/eventing-plan/workpacks/01-source-boundary-and-semantics-audit.md
- Slice 02: docs/plans/eventing-plan/workpacks/02-crate-contract-and-type-boundary.md
- Slice 03: docs/plans/eventing-plan/workpacks/03-dispatch-runtime-and-lifecycle.md

## PR-ready gate

- Any downstream plan claiming behavior must cite its own proof files; eventing proof remains transport/domain-only.

## HID test floor (this plan)

### Required test families for closed slice

- Unit: envelope/version parsing
- Integration: ordering, dead-letter, and consumer parity
- E2E: restart/replay recovery
- Security: version skew and schema abuse probes
- Non-functional: throughput/queue stability under load

### Mandatory slice evidence checks

- negative cases documented (at least one per slice)
- rollback/teardown proof recorded
- proof manifest references command output, artifacts, and manual review notes
