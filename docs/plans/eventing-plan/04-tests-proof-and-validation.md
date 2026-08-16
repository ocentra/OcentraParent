# Tests, Proof, And Validation

<!-- agent-capsule -->

> Agent Capsule
> Plan: `eventing-plan`
> Doc: `Tests, Proof, And Validation`
> Kind: proof reference; read only when validating matching claim.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

The reusable event bus must be test-backed before Ocentra Parent network work
depends on it. Tests are specifications. They must use real Rust structs, real
serde roundtrips, real Tokio execution, real temp filesystem paths for journal
tests, and exact assertions.

## Test Folder Shape

```text
crates/ocentra-eventing/tests/
  aggregate_ordering.rs
  async_concurrent_dispatch.rs
  async_sequential_dispatch.rs
  bus_shutdown_clear.rs
  clock_manual.rs
  compile_fail_contracts.rs
  dead_letter.rs
  event_contract_registry_docs.rs
  event_topology_manifest.rs
  event_type_registry.rs
  handler_timeout.rs
  idempotency.rs
  journal_ndjson.rs
  nested_publish.rs
  no_subscriber_queue.rs
  panic_isolation.rs
  publish_subscribe.rs
  queue_capacity.rs
  registrar_dispose.rs
  request_response.rs
  request_response_type_binding.rs
  retry_policy.rs
  serde_roundtrip.rs
  subscription_duplicate_policy.rs
  target_handler.rs
  ttl_deadline.rs
  typed_live_envelope.rs
  validation_newtypes.rs
  ownership_mutation_guard.rs
  lock_held_await_audit.rs
```

Unit tests may also live beside modules when that makes private behavior easier
to specify, but public behavior needs integration tests under `tests/`.

## Shared Test Event Contracts

Tests should define small real event structs such as:

```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
struct TestPingEvent {
    event_id: EventId,
    occurred_at: DateTime<Utc>,
    correlation_id: CorrelationId,
    aggregate_key: Option<AggregateKey>,
    message: TestMessage,
}

impl EventContract for TestPingEvent {
    const EVENT_TYPE: &'static str = TEST_PING_EVENT_TYPE;
    const SCHEMA_VERSION: u16 = 1;

    fn validate(&self) -> Result<(), EventValidationError> {
        self.message.validate()
    }
}

impl DomainEvent for TestPingEvent {
    fn event_id(&self) -> EventId {
        self.event_id.clone()
    }

    fn occurred_at(&self) -> DateTime<Utc> {
        self.occurred_at
    }

    fn correlation_id(&self) -> CorrelationId {
        self.correlation_id.clone()
    }

    fn causation_id(&self) -> Option<EventId> {
        None
    }
}
```

The test event type value should be a constant, not a scattered string.
Every test event constructor and serde deserialize path should validate fields
before an event can be published.

## Required Behavior Tests

### Event Type Registry

- rejects empty event type;
- rejects malformed event type;
- accepts documented `namespace.segment.action` shape;
- detects duplicate registered event type constants;
- proves schema version starts at `1`;
- proves every registered event serde roundtrips exactly.

### Validation Newtypes

- rejects invalid newtype construction for event type, schema version,
  subscriber name, handler name, aggregate key, idempotency key, target handler,
  runtime source fields, and journal path;
- rejects invalid values during serde deserialization, not only during manual
  construction;
- proves canonical serialize output for every newtype;
- proves public eventing API does not accept raw `String`, `&str`, raw `Uuid`,
  or `serde_json::Value` for domain-bearing values;
- proves TypeScript/Rust fixture parity for Parent contracts once those
  contracts cross both languages.

### Typed Live Envelope

- live publish creates `EventEnvelope<E>` with `Arc<E>` payload;
- handler receives `EventContext<E>` with typed payload;
- handler does not receive `serde_json::Value`;
- live-to-stored conversion preserves metadata and serializes typed payload;
- stored-to-live replay performs event type lookup, typed deserialize, and
  validation before dispatch;
- invalid stored payload never reaches a handler.

### Publish And Subscribe

- one subscriber receives the exact typed event;
- multiple subscribers receive the same event id and correlation id;
- publish report records subscriber count, handled count, ignored count, failed
  count, and per-handler reports;
- publishing with no subscribers succeeds only according to queue policy and
  reports unhandled state exactly.

### Event Topology And Source Usage Audit

- proof emits a generated event topology manifest for implemented event
  contracts;
- manifest lists each event type, publisher sites, subscriber descriptors,
  request/response pairing, and event-family variants;
- each event is classified as covered, no publisher, no subscriber,
  intentionally one-sided, or fail with a workpack reason;
- product/runtime event chains cannot report ready while an event is orphaned
  without an explicit accepted exception;
- duplicate subscriber registration for the same event type, subscriber id, and
  target is idempotent or rejected unless the descriptor explicitly allows
  duplicate delivery.

### Manual Clock

- `ManualEventClock` controls TTL, deadline, retry delay, queued expiry, and
  request timeout tests;
- tests do not rely on long wall-clock sleeps;
- advancing the manual clock triggers deterministic timeout and retry behavior;
- system clock is used only in runtime/default construction tests.

### Async Sequential Dispatch

- subscribers run in registration order;
- second handler starts only after first handler finishes;
- handler reports preserve order;
- nested publish from a handler succeeds without deadlock.

### Async Concurrent Dispatch

- handlers run in parallel;
- total elapsed time proves concurrency with exact timing margin;
- one failed handler does not hide successful handler reports;
- publish report aggregates all outcomes.

### Aggregate Ordering

- events with the same aggregate key dispatch in order;
- events with different aggregate keys can dispatch concurrently;
- ordered state transitions are not parallelized for speed;
- missing aggregate key in ordered mode is rejected or downgraded according to
  explicit policy.

### Registrar Dispose

- registrar subscribes multiple handlers;
- dispose unsubscribes all handlers;
- dropping registrar disposes subscriptions;
- dispose is idempotent;
- disposed registrar cannot add new subscriptions.

### Queue When No Subscriber

- event queues when policy allows no-subscriber queueing;
- event rejects when policy disables queueing;
- queued event drains when a matching handler registers;
- queued event retains original event id, correlation id, causation id, and
  payload;
- queued event TTL expiry dead-letters or drops according to policy.

### Queue Capacity

- max queue size is enforced;
- reject-new policy returns an exact error;
- drop-oldest policy writes a dead-letter record for the dropped event;
- queue depth metrics update.

### Retry Policy

- handler retries exactly `max_attempts`;
- zero-delay retry works without sleeps;
- failed final attempt creates handler report and optional dead-letter record;
- successful retry records exact attempts count.

### Handler Timeout

- handler timeout returns a timeout report;
- timeout can retry if policy allows;
- timeout does not leave the event in-flight forever;
- timeout metric increments.

### TTL And Deadline

- event with deadline in the past is not dispatched;
- event expiring while queued does not dispatch;
- retry does not run after event deadline;
- failure reason is exact.

### Idempotency And In-Flight Guard

- duplicate non-republishable event id is rejected while in flight;
- duplicate idempotency key does not execute command twice;
- republishable event requires explicit metadata;
- idempotent result report states prior execution rather than pretending a new
  execution happened.

### Target Handler

- targeted event goes only to matching target handler;
- non-targeted event broadcasts according to subscription type;
- wrong target handler is ignored with exact report;
- targeted enforcement commands cannot be picked up by generic subscribers.

### Request/Response

- request resolves with typed response;
- response type is associated with the request event through
  `RequestEvent::Response`;
- caller cannot choose an unrelated response type;
- response implements `EventResponseContract` and validates before completion;
- deferred/completion handles are rejected from event payload contracts, stored
  envelopes, TypeScript domain contracts, and journal JSON;
- caller cancellation and shutdown cancellation use request/publish options or
  handler context and produce exact cancelled reports;
- response resolves exactly once;
- double completion is ignored and reported;
- request timeout returns exact timeout result;
- late response after timeout does not mutate the completed request;
- durable product result event pattern is still supported separately.

### Ownership And Mutation Guard

- handlers cannot mutate event payloads through `EventContext<E>`;
- handler API does not expose `&mut E`;
- event payload structs do not contain `Cell`, `RefCell`, `Mutex`, `RwLock`,
  atomics, raw pointers, or other interior-mutability fields unless a workpack
  records a narrow exception and proof;
- mutable handler state is owned by the handler/service and remains
  lock-scoped;
- nested publish cannot borrow transient handler data across `.await`;
- spawned handler tasks require `Send + 'static`.

### Lock-Held Await Audit

- subscriber registry locks are dropped before awaiting handlers;
- queue locks are dropped before awaiting dispatch or journal writes;
- request completion registry locks are dropped before awaiting callbacks or
  publish paths;
- aggregate-order locks serialize only the intended aggregate and do not block
  unrelated aggregate keys;
- proof includes source audit notes in addition to behavior tests.

### Panic Isolation

- handler panic is captured as handler failure;
- service runtime continues;
- other subscribers still report according to dispatch mode;
- panic dead-letter record includes event id, event type, subscriber id, and
  panic classification.

### NDJSON Journal

- append writes one JSON object per line;
- line contains event id, event type, schema version, correlation id, source,
  custody, and payload;
- append flush policy is honored;
- hash-chain fields are correct when enabled;
- selected-only journaling by event type, namespace/family, and explicit
  contract allowlist is deterministic;
- corrupt line handling is explicit in replay;
- temp directory proof uses real filesystem IO.

### Replay

- replay reads event envelopes in order;
- replay filters by event type;
- replay filters by correlation id;
- replay feeds real handlers or projectors;
- replay records cursor and known gaps;
- replay never executes enforcement handlers unless the replay mode explicitly
  allows action handlers, and Parent should default replay to projection-only.

### Shutdown And Clear Lifecycle

- production shutdown drains or dead-letters queued work according to policy;
- in-flight events and local requests resolve, cancel, timeout, or report exact
  shutdown state;
- registrar and subscription drops remove handlers;
- test clear/reset removes subscribers, queued events, in-flight state, and
  request registry entries between tests;
- production code cannot call test-only clear helpers.

### Compile-Fail And Source Gates

- mutable event payload access fails to compile or is rejected by source gate;
- payload fields using `Mutex`, `RwLock`, `RefCell`, `Cell`, atomics, raw
  pointers, task handles, sockets, or file handles fail without explicit
  exception proof;
- wrong request response type fails to compile;
- payload-carried deferred/completion or cancellation handles fail the source
  gate;
- raw `serde_json::Value` live handler routing fails the source gate;
- `#![deny(clippy::await_holding_lock)]` or equivalent audit is part of the
  proof command set when the crate exists.

## Parent Integration Proof Tests

These tests belong after Parent event contracts exist. They must not land before
the reusable crate passes its own tests.

```text
parent_portal_intent_becomes_parent_controller_event_only_after_rust_validation
vite_ui_cannot_publish_business_event_directly
parent_controller_forwards_child_command_through_typed_transport_boundary
child_agent_receives_command_and_publishes_local_child_agent_event
network_classification_requests_ai_analysis
ai_completion_requests_policy_evaluation
policy_decision_issues_enforcement_command_only_when_allowed
ai_cannot_publish_enforcement_command_directly
portal_cannot_publish_enforcement_command_directly
portal_static_gate_cannot_import_or_instantiate_event_bus
portal_static_gate_cannot_compute_evidence_grade_or_policy_decision
weak_network_evidence_cannot_block
enforcement_command_requires_policy_decision_ref
enforcement_command_journaled_before_adapter_handler_runs
enforcement_result_updates_audit_and_portal_read_model
```

The parent/controller runtime and child-agent runtime both use the same
`ocentra-eventing` crate, but integration tests must prove cross-process
handoff uses typed service or transport boundaries rather than shared in-memory
state.

## Banned Test Patterns

- mocks;
- fakes;
- stubs;
- spies;
- replacement transports;
- raw JSON payload routing in live dispatch;
- raw `String`, `&str`, `Uuid`, or `serde_json::Value` in public eventing API
  positions for domain-bearing values;
- `Box<dyn Any>` payload routing;
- mutable event payload references;
- interior-mutability event payloads without explicit exception proof;
- weak assertions such as `is_some` as the main proof;
- sleeping for long periods instead of using zero-delay retry or explicit past
  deadlines;
- testing only that a type exists.

## Focused Validation

Docs-only plan validation:

```powershell
git diff --check
```

First implementation proof:

```powershell
cargo test -p ocentra-eventing
cargo clippy -p ocentra-eventing --all-targets -- -D warnings
```

Parent integration proof after event contracts are wired:

```powershell
cargo test
npm run test:local
```

PR-ready gate unless the user scopes it lower:

```powershell
npm run validate
```

## Proof Artifact Shape

Implementation workpacks should store proof under:

```text
output/eventing-plan-proof/<workpack-id>/
  00-source-snapshot.md
  01-contract-proof.md
  02-dispatch-proof.md
  03-queue-retry-timeout-proof.md
  04-journal-replay-proof.md
  05-parent-integration-proof.md
  10-validation-log.md
  proof-summary.json
```

Not every workpack needs every file, but every completed workpack must name the
exact proof artifacts and validation commands that support its claim.
