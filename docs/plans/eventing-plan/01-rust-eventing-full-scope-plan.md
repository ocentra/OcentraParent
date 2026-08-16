# Rust Eventing Full-Scope Plan

<!-- agent-capsule -->

> Agent Capsule
> Plan: `eventing-plan`
> Doc: `Rust Eventing Full-Scope Plan`
> Kind: plan reference document; read only when routed by AGENTS, DOC_INDEX, or workpack.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

## Product And Engineering Objective

Build a reusable Rust event bus that gives Ocentra projects the same practical
value as the Ocentra Games TypeScript eventing package, but with stronger Rust
runtime guarantees for async service work, local evidence, policy, audit, and
future multi-device coordination.

The reusable crate must be generic enough for:

- Ocentra Parent child-agent service orchestration;
- Ocentra Parent parent/controller runtime orchestration;
- Parent network, AI, policy, enforcement, audit, portal, LAN, sync, and support
  flows;
- future Rust game servers or local tools;
- proof harnesses and replay tools.

The crate must not contain Parent business behavior. It should provide typed
event delivery, handler lifecycle, queueing, retry, timeout, ordering,
request/response, dead-letter, journal hooks, replay hooks, metrics, tracing,
and test utilities.

## Design Principles

### Preserve The Games Semantics

Keep these ideas from Ocentra Games:

- central `EventBus`;
- `EventRegistrar` ownership and dispose lifecycle;
- canonical event type keys;
- typed event contracts;
- sync/async handler split, translated to Rust async handlers;
- queued events when no handler exists;
- retry, TTL, queue capacity, and timeout behavior;
- in-flight duplicate guard unless republish is explicitly allowed;
- target-handler markers;
- `OperationResult` style publish and handler reports;
- `OperationDeferred` style request completion, translated to safe Rust
  one-shot completion;
- isolated test bus.

### Rebuild It Rust-First

Do not copy TypeScript class inheritance. Rust should use:

- traits for domain events;
- structs for event payloads and envelopes;
- enums for dispatch, priority, delivery, dead-letter, and queue policies;
- serde for event serialization;
- Tokio for async execution;
- bounded queues and explicit backpressure;
- constants for event type values;
- explicit handler reports instead of boolean-only handled state.

### Keep Runtime Authority Outside The Bus

The bus routes events. It does not decide policy, classify content, run AI,
block traffic, write product claims, or update UI by itself.

Correct authority split:

```text
eventing crate
  owns delivery semantics and handler lifecycle

project event contracts
  own event names, schemas, and versioning

runtime services
  own publisher/subscriber behavior

policy engine
  owns decisions

adapters
  own platform actions

journal/audit
  owns proof

Vite/TypeScript UI
  owns rendering and user input only
```

The Vite/TypeScript portal surface must never own business logic for evidence,
policy, AI, cascade, enforcement, or audit. It sends typed intents to Rust and
renders service-backed read models.

## Event Layers

The reusable system has two related layers.

### Live Runtime Bus

The live bus handles in-process dispatch:

```text
publisher
  -> typed EventEnvelope<E>
  -> routing by EventType
  -> typed subscriber handlers
  -> HandlerReport
  -> PublishReport
  -> optional result event or request completion
```

The live bus supports fast local orchestration such as:

- network flow observed;
- parent intent received;
- child-agent command received;
- AI analysis requested;
- policy decision completed;
- enforcement command issued;
- audit append requested;
- portal read model updated.

### Durable Journal And Replay

The journal is the durable proof layer:

```text
StoredEventEnvelope
  -> NDJSON append
  -> optional hash chain
  -> optional SQLite projection
  -> replay cursor
  -> test or audit reconstruction
```

The bus should support journaling before dispatch, after dispatch, or both, but
consumer flows choose the policy. Parent enforcement paths should journal
decisions before issuing adapter commands and journal adapter results after
execution.

## Shared Parent/Child Runtime Model

The same crate should run in multiple Rust runtimes:

```text
Rust parent/controller runtime
  -> parent intents
  -> parent-owned read models
  -> local service command validation
  -> LAN/relay command handoff
  -> policy/admin/audit orchestration where applicable

Rust child-agent runtime
  -> evidence capture
  -> local AI/policy evaluation
  -> adapter capability checks
  -> enforcement command execution
  -> child-device audit and read models
```

Each runtime has its own in-process bus instance. Shared crate does not mean a
global shared memory bus. Cross-process or cross-device events must cross a
typed boundary first, such as local API, WebSocket, IPC, LAN, relay,
parent-owned export, or journal replay, then publish into the destination
runtime's local bus.

Household mesh coordination must not be implemented as a shared in-memory or
shared LAN event bus. Each runtime publishes local events to its own bus. A
Household Mesh Bridge may subscribe to selected local events, convert them to
typed authenticated transport messages, and publish validated incoming messages
into the destination runtime's local bus.

## Required Runtime Behaviors

### Dispatch Modes

The bus must support:

- sequential dispatch: handlers run in registration order;
- concurrent dispatch: handlers run in parallel and aggregate reports;
- aggregate-ordered dispatch: events sharing an aggregate key dispatch in order;
- fire-and-forget publish: enqueue or schedule without blocking the caller;
- publish-and-wait: caller awaits handler completion report;
- request/response: local-only command/query completion with timeout and
  double-completion guard.

### Queue And Backpressure

Events published without subscribers may be queued only when the event type or
bus policy allows it. Queues must be bounded.

Backpressure policies:

- reject new event;
- drop oldest queued event with a dead-letter record;
- block only through explicit async wait and never through blocking IO;
- route to dead letter after timeout, retry exhaustion, or queue overflow.

Queue records must carry:

- event id;
- event type;
- queued at;
- attempts;
- deadline or TTL;
- next retry time;
- last error code where applicable.

### Retry, Timeout, TTL, And Dead Letter

Handlers can fail, time out, or panic. The bus must:

- isolate handler panics from the service runtime;
- retry only when the handler policy permits it;
- apply handler timeout per attempt;
- apply event deadline/TTL before dispatch and before each retry;
- create dead-letter records with event id, event type, subscriber id, attempt
  count, error class, and final state;
- emit metrics and tracing fields for each failure.

### Idempotency And In-Flight Guards

Every event must carry an event id. Optional idempotency keys should prevent
duplicate command execution.

The in-flight guard should reject duplicate non-republishable event ids while a
publish is still active. Republishable events require explicit metadata.

### Target Handler Routing

Events can optionally target a handler family, such as a specific adapter,
projector, command handler, or runtime component. Non-target handlers must not
execute targeted events unless the event explicitly allows broadcast fallback.

### Nested Publish Chains

Handlers may publish follow-up events:

```text
network.activity.classified
  -> ai.analysis.requested
  -> ai.analysis.completed
  -> policy.evaluation.requested
```

Nested publish must not deadlock. The implementation must not hold registry or
queue locks across handler awaits.

### Observability

The bus must expose:

- publish latency;
- per-handler latency;
- subscriber count;
- queue depth;
- retry count;
- dead-letter count;
- timeout count;
- panic count;
- dropped event count;
- journal append latency;
- replay cursor metrics.

Tracing spans should include event id, event type, correlation id, causation id,
aggregate key, subscriber id, subscriber name, source, and outcome.

## Parent Safety Rules

When Ocentra Parent consumes the bus:

- parent/controller Rust runtime and child-agent Rust runtime can both use the
  same reusable crate;
- TypeScript/Vite UI can only send intents and render read models;
- AI cannot publish enforcement commands directly.
- Portal/UI cannot publish enforcement commands directly.
- Weak or network-only evidence cannot publish adapter commands directly.
- Enforcement command events require evidence refs, policy decision refs,
  capability proof refs, expiry or rollback state, and audit refs.
- Journal-before-action is required for policy decisions and enforcement
  commands.
- Adapter results must publish result events and audit events.

## Reuse Boundary

Reusable crate owns:

- event type wrapper and ids;
- envelope;
- domain event trait;
- event source/custody metadata;
- handler trait and reports;
- event bus and registry;
- registrar;
- queue policies;
- retry/timeout/dead-letter policies;
- request/response local completion;
- journal trait and NDJSON implementation;
- replay cursor trait;
- testkit.

Parent-specific crates own:

- network event constants and payloads;
- AI event constants and payloads;
- policy event constants and payloads;
- enforcement event constants and payloads;
- audit event constants and payloads;
- protocol/WebSocket exposure;
- SQLite projections and read models;
- feature-specific handlers.

## Implementation Gate

Do not let network implementation start consuming an event bus until:

- `crates/ocentra-eventing` exists and is in the workspace;
- core public API is documented in crate docs;
- unit and integration tests cover all required core behaviors;
- no source-shape hard limits are violated;
- `cargo test -p ocentra-eventing` passes;
- relevant docs are updated with proof paths and known gaps.
