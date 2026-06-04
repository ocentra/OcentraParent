# ocentra-eventing

Reusable Rust eventing primitives for Ocentra Parent runtime code.

## Owns

- Validated event identifiers, correlation ids, aggregate keys, idempotency
  keys, source ids, subscriber ids, target handlers, and recorded timestamps.
- `DomainEvent` contracts, typed `EventEnvelope<E>` live dispatch, and
  `StoredEventEnvelope` serialization boundaries.
- Explicit `EventBus` instances owned by the runtime that constructs them.
- Sequential, concurrent, and aggregate-ordered typed dispatch with
  target-handler filtering, duplicate subscriber rejection, stored-envelope
  journal snapshots, exact handler reports, panic-isolation dead letters, and
  nested publish through typed `EventContext<E>`.
- Observable detached publish, awaitable publish reports, scoped
  `SubscriptionHandle` unsubscribe/drop behavior, and `EventRegistrar`
  ownership/dispose lifecycle.
- Handler execution policy for timeout and retry attempts, handler trace fields
  for event id/type/correlation/handler/outcome, and a real-subscription
  `EventRecorder<E>` testkit helper.
- Local bounded no-subscriber queue policy with observable drain reports,
  overflow rejection/dead-letter behavior, queue TTL expiry before dispatch,
  in-flight duplicate rejection, optional completed idempotency registry, and
  typed dead-letter event conversion.

## Must Not Own

- Parent-specific event payloads or product policy.
- Network-only bus, external queue, request broker, or platform transport
  machinery.
- Portal UI business behavior.
- Hidden global singleton state.

## Current Gap

This crate does not yet implement request-response completion, durable NDJSON
journal replay, shutdown/drain lifecycle, or broker-backed delivery. Consumers
must keep those claims manual-required until the matching eventing workpacks are
implemented and validated.
