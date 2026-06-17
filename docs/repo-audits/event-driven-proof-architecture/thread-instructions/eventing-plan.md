# eventing-plan Event Architecture Instruction

## Owns

- shared Rust event bus, envelope, journal, queue, request, replay, topology, and testkit foundations;
- household-mesh consumer bridge contract only where assigned.

## Must not own

- physical LAN/provider execution;
- remote-access product behavior;
- plan-specific policy or tracking logic.

## Required chain

```text
domain owner emits typed event
-> eventing envelope/journal/queue records it
-> subscriber consumes through registered contract
-> replay/projection proves idempotency and no-claim boundaries
```

## Logging/proof

Eventing proof must log publisher, envelope type, correlation/causation ids, queue decision, subscriber result, replay cursor, and rejected duplicate/stale cases.

## Tests

Public event behavior belongs in crate-level contract/unit/integration tests. Inline Rust tests may stay supplemental only.

## First architecture slice

Run WP10-A: type household-mesh runtime bridge end-to-end and add crate-level tests. Then align the proof script to current tests and source paths.
