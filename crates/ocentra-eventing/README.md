# ocentra-eventing

Reusable Rust eventing primitives for Ocentra Parent runtime code.

## Owns

- Validated event identifiers, correlation ids, aggregate keys, idempotency
  keys, source ids, subscriber ids, target handlers, and recorded timestamps.
- `DomainEvent` contracts, typed `EventEnvelope<E>` live dispatch, and
  `StoredEventEnvelope` serialization boundaries.
- Explicit `EventBus` instances owned by the runtime that constructs them.
- Sequential and concurrent typed dispatch with target-handler filtering,
  duplicate subscriber rejection, stored-envelope journal snapshots, and
  dead-letter capture.

## Must Not Own

- Parent-specific event payloads or product policy.
- Network-only bus, queue, retry, request, or broker machinery.
- Portal UI business behavior.
- Hidden global singleton state.

## Current Gap

This first crate slice does not yet implement aggregate-ordered dispatch,
bounded queues, TTL/retry, request-response completion, durable NDJSON journal
replay, panic isolation, shutdown/drain lifecycle, or broker-backed delivery.
Consumers must keep those claims manual-required until the matching eventing
workpacks are implemented and validated.
