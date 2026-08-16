# Crate API And Code Shape

<!-- agent-capsule -->

> Agent Capsule
> Plan: `eventing-plan`
> Doc: `Crate API And Code Shape`
> Kind: plan reference document; read only when routed by AGENTS, DOC_INDEX, or workpack.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

This file is the target implementation shape for `crates/ocentra-eventing`.
It is intentionally concrete enough that a worker can implement it without
reopening the architecture decision.

## Crate Layout

```text
crates/ocentra-eventing/
  Cargo.toml
  src/
    lib.rs
    aggregate_key.rs
    bus.rs
    clock.rs
    dead_letter.rs
    dispatch.rs
    domain_event.rs
    envelope.rs
    event_type.rs
    handler.rs
    ids.rs
    journal.rs
    metrics.rs
    queue.rs
    registrar.rs
    replay.rs
    request.rs
    result.rs
    source.rs
    target.rs
    testkit.rs
    trace.rs
    validation.rs
  tests/
    aggregate_ordering.rs
    async_concurrent_dispatch.rs
    async_sequential_dispatch.rs
    bus_shutdown_clear.rs
    clock_manual.rs
    dead_letter.rs
    event_type_registry.rs
    event_contract_registry_docs.rs
    handler_timeout.rs
    idempotency.rs
    journal_ndjson.rs
    nested_publish.rs
    no_subscriber_queue.rs
    panic_isolation.rs
    publish_subscribe.rs
    queue_capacity.rs
    registrar_dispose.rs
    subscription_duplicate_policy.rs
    request_response.rs
    retry_policy.rs
    serde_roundtrip.rs
    target_handler.rs
    typed_live_envelope.rs
    ttl_deadline.rs
    validation_newtypes.rs
```

`lib.rs` should re-export the stable public API and keep internal helper modules
private where possible.

## Cargo Direction

Planned dependencies should stay minimal:

```toml
[package]
name = "ocentra-eventing"
version.workspace = true
edition.workspace = true
license.workspace = true

[dependencies]
async-trait = "0.1"
chrono = { version = "0.4", features = ["serde"] }
futures = "0.3"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
thiserror = "2"
tokio = { version = "1", features = ["fs", "io-util", "macros", "rt-multi-thread", "sync", "time"] }
tracing = "0.1"
uuid = { version = "1", features = ["serde", "v4"] }

[dev-dependencies]
tempfile = "3"
```

Avoid adding broad concurrency crates until the standard Tokio and std types are
insufficient. If a dependency is added for maps, queues, or metrics, the
workpack must justify it.

## Clock API

The crate must own an injectable clock for deterministic timeout behavior:

```rust
pub trait EventClock: Clone + Send + Sync + 'static {
    fn now(&self) -> DateTime<Utc>;
    fn sleep(&self, duration: Duration) -> impl Future<Output = ()> + Send;
}

pub struct SystemEventClock;
pub struct ManualEventClock;
```

Runtime code uses `SystemEventClock`. Tests use `ManualEventClock` so TTL,
deadline, retry delay, queued expiry, and request timeout proof never depends
on long wall-clock sleeps.

## Core Types

## Validation And Brand Rule

Rust must follow the same discipline as Effect Schema brands in TypeScript:

```text
raw boundary value
  -> parse/validate
  -> domain newtype
  -> typed event struct
  -> typed live envelope
  -> serialized envelope only at journal/replay/transport boundary
```

Do not use raw `String`, `&str`, `Uuid`, or `serde_json::Value` for
domain-bearing values in public eventing APIs. Use validated newtypes with
`TryFrom`, parse helpers, serde validation, and exact invalid-state tests.

### Event Type

```rust
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct EventType(String);

impl EventType {
    pub fn from_static(value: &'static str) -> Self;
    pub fn from_owned(value: String) -> Result<Self, EventingError>;
    pub fn as_str(&self) -> &str;
}
```

Rules:

- validate non-empty;
- reject whitespace-only values;
- reject values outside the documented event-name grammar;
- do not accept ad hoc local event names in Parent runtime code;
- Parent event type values live in `agent-protocol` constants when Parent
  runtime crosses the boundary.

Recommended event type grammar:

```text
namespace.segment.action
```

Examples:

```text
eventing.dead_letter.created
network.flow.observed
policy.decision.completed
```

### Event Contract Registry And Docs

Every event contract registered by the generic crate or Parent protocol must
produce an `EventContractDescriptor`:

```rust
pub struct EventContractDescriptor {
    pub event_type: EventType,
    pub schema_version: SchemaVersion,
    pub namespace: EventNamespace,
    pub payload_type_name: PayloadTypeName,
    pub owner: ContractOwner,
    pub journal_policy: JournalPolicy,
}
```

Tests must reject duplicate event types and generated markdown must list all
registered event contracts. The registry is also the source for event graph
proof: publisher paths, subscriber paths, no-publisher, no-subscriber,
intentionally-unhandled, and fail states.

### Id Types

```rust
pub struct EventId(Uuid);
pub struct CorrelationId(Uuid);
pub struct SubscriberId(Uuid);
pub struct RequestId(Uuid);
pub struct IdempotencyKey(String);
pub struct AggregateKey(String);
```

Every id type must implement serde, display, debug, equality, hashing, and
parse/constructor helpers. Use explicit newtypes instead of raw strings or raw
UUIDs in public API.

### Additional Branded Runtime Values

```rust
pub struct SchemaVersion(NonZeroU16);
pub struct SubscriberName(String);
pub struct HandlerName(String);
pub struct ServiceName(String);
pub struct ComponentName(String);
pub struct InstanceId(String);
pub struct DeviceRef(String);
pub struct JournalPath(PathBuf);
```

Each branded value must validate on construction and serde deserialize.
Cosmetic aliases such as `type SubscriberName = String` are forbidden.

### Domain Event Trait

```rust
pub trait EventContract:
    Clone + Send + Sync + serde::Serialize + serde::de::DeserializeOwned + 'static
{
    const EVENT_TYPE: &'static str;
    const SCHEMA_VERSION: u16;

    fn validate(&self) -> Result<(), EventValidationError>;
}

pub trait DomainEvent: EventContract {
    fn event_id(&self) -> EventId;
    fn occurred_at(&self) -> DateTime<Utc>;
    fn correlation_id(&self) -> CorrelationId;
    fn causation_id(&self) -> Option<EventId>;

    fn aggregate_key(&self) -> Option<AggregateKey> {
        None
    }

    fn idempotency_key(&self) -> Option<IdempotencyKey> {
        None
    }
}
```

Consumer event structs are normal Rust structs. They do not inherit from a base
class, and they do not carry business logic.

Implementation requirements:

- constructors validate and return `Result<Self, EventValidationError>`;
- serde deserialization validates before returning an event;
- `publish` validates before creating a live envelope;
- replay validates before projection or re-publish;
- invalid events never reach handlers.

### Event Families And Variants

Unity/C# base-event inheritance patterns, such as a base decision event plus
concrete decision events, translate to explicit Rust event-family enums or
wrapper structs.

Rules:

- family subscribers receive a typed enum/wrapper and match variants directly;
- no handler may downcast, inspect `serde_json::Value`, or infer variant shape
  from loose strings;
- each variant that crosses journal, replay, transport, or protocol boundaries
  has a registered event type or registered family variant id and serde
  fixture;
- generic event shapes must resolve to explicit registered contracts before
  crossing a runtime, journal, replay, or transport boundary.

### Event Envelope

```rust
pub struct EventMetadata {
    pub event_id: EventId,
    pub event_type: EventType,
    pub schema_version: SchemaVersion,
    pub occurred_at: DateTime<Utc>,
    pub published_at: DateTime<Utc>,
    pub correlation_id: CorrelationId,
    pub causation_id: Option<EventId>,
    pub aggregate_key: Option<AggregateKey>,
    pub idempotency_key: Option<IdempotencyKey>,
    pub source: EventSource,
    pub custody: EventCustody,
    pub target: Option<TargetHandler>,
    pub priority: EventPriority,
    pub deadline_at: Option<DateTime<Utc>>,
    pub republish: RepublishPolicy,
}

pub struct EventEnvelope<E: DomainEvent> {
    pub metadata: EventMetadata,
    pub payload: Arc<E>,
}

pub struct StoredEventEnvelope {
    pub metadata: EventMetadata,
    pub payload_json: serde_json::Value,
}
```

`EventEnvelope<E>` is the live dispatch truth. `StoredEventEnvelope` is only for
journal, replay, dead-letter, export, or external transport boundaries. Handlers
must receive typed events, not `serde_json::Value`.

### Republish And Publish Override

The Unity and TypeScript systems allowed loose `force` or mutable republish
flags. Rust keeps the capability only as typed policy:

```rust
pub enum RepublishPolicy {
    RejectDuplicateInFlight,
    AllowDuplicateInFlight,
    ForcePublish(OverrideReason),
}

pub enum PublishOverride {
    None,
    RepublishWithReason(OverrideReason),
    TestHarnessBypass(OverrideReason),
}
```

Defaults:

- `RepublishPolicy::RejectDuplicateInFlight`;
- `PublishOverride::None`;
- no boolean `force` in public production APIs.

Republishable event families need contract-level justification and tests.
Production overrides must be reported or journaled. Test-only bypasses belong
in testkit or explicit harness options.

### Event Source And Custody

```rust
pub struct EventSource {
    pub runtime_role: RuntimeRole,
    pub service: ServiceName,
    pub component: Option<ComponentName>,
    pub instance: Option<InstanceId>,
    pub device_ref: Option<DeviceRef>,
}

pub struct RuntimeRole(String);

pub struct EventCustody(String);
```

The reusable crate owns only generic validated source/custody labels.
Parent-specific values are constants or protocol structs in Parent crates.

## Bus API

Target public API:

```rust
pub struct EventBus;

impl EventBus {
    pub fn new(options: EventBusOptions) -> Self;

    pub fn subscribe<E, F, Fut>(
        &self,
        descriptor: SubscriberDescriptor,
        handler: F,
    ) -> SubscriptionHandle
    where
        E: DomainEvent,
        F: Fn(EventContext<E>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = HandlerResult> + Send + 'static;

    pub async fn publish<E>(&self, event: E) -> PublishResult
    where
        E: DomainEvent;

    pub async fn publish_with<E>(
        &self,
        event: E,
        options: PublishOptions,
    ) -> PublishResult
    where
        E: DomainEvent;

    pub fn publish_detached<E>(&self, event: E, options: PublishOptions)
    where
        E: DomainEvent;

    pub async fn publish_request<E>(
        &self,
        event: E,
        options: RequestOptions,
    ) -> RequestResult<E::Response>
    where
        E: RequestEvent;
}
```

`EventContext<E>` gives handlers the parsed event, envelope metadata, and a
safe publish handle for nested events. It must not expose mutable bus internals,
`serde_json::Value` payloads, or mutable event payloads.

## Handler API

```rust
pub struct SubscriberDescriptor {
    pub subscriber_id: SubscriberId,
    pub name: SubscriberName,
    pub target: Option<TargetHandler>,
    pub dispatch_mode: DispatchMode,
    pub duplicate_policy: SubscriptionDuplicatePolicy,
    pub retry_policy: RetryPolicy,
    pub timeout: Option<Duration>,
    pub queue_policy: QueuePolicy,
}

pub enum SubscriptionDuplicatePolicy {
    RejectDuplicate,
    ReplaceExisting,
    AllowDuplicateWithReason(OverrideReason),
}

pub enum HandlerOutcome {
    Handled,
    Ignored,
    Deferred,
}

pub struct HandlerReport {
    pub subscriber_id: SubscriberId,
    pub subscriber_name: SubscriberName,
    pub outcome: HandlerOutcome,
    pub attempts: u32,
    pub latency: Duration,
    pub error: Option<EventingErrorCode>,
}
```

Handler failures should be represented in reports and dead-letter records, not
hidden behind a boolean.

## Dispatch Rules

```rust
pub enum DispatchMode {
    Sequential,
    Concurrent,
    OrderedByAggregateKey,
}
```

Implementation requirements:

- clone the matching subscriber list before awaiting handlers;
- never hold registry or queue locks across `.await`;
- sequential mode awaits each handler in order;
- concurrent mode uses Tokio tasks or `join_all` and aggregates reports;
- ordered mode serializes events sharing the same aggregate key while allowing
  different aggregate keys to proceed independently;
- nested publish is allowed through `EventContext` and must not deadlock.

## Queue Rules

```rust
pub enum QueueWhenNoSubscriber {
    Disabled,
    Enabled,
}

pub enum QueueOverflowPolicy {
    RejectNew,
    DropOldestToDeadLetter,
}

pub struct QueuePolicy {
    pub when_no_subscriber: QueueWhenNoSubscriber,
    pub max_events: usize,
    pub ttl: Duration,
    pub overflow: QueueOverflowPolicy,
}
```

The bus should queue only when policy allows it. Safety-sensitive Parent
commands should usually reject or dead-letter when no handler exists instead of
silently waiting.

## Retry And Dead Letter

```rust
pub struct RetryPolicy {
    pub max_attempts: u32,
    pub delay: Duration,
}

pub struct DeadLetterRecord {
    pub dead_letter_id: EventId,
    pub original_event_id: EventId,
    pub event_type: EventType,
    pub subscriber_id: Option<SubscriberId>,
    pub reason: DeadLetterReason,
    pub attempts: u32,
    pub created_at: DateTime<Utc>,
    pub envelope: StoredEventEnvelope,
}
```

Dead-letter events should be publishable as normal `eventing.dead_letter.created`
events so monitoring and audit paths can observe them.

## Request/Response

There are two response patterns.

### Durable Result Event Pattern

Use this for auditable product flows:

```text
policy.evaluation.requested
  -> policy.decision.completed
```

This is the default for Parent evidence, policy, enforcement, and audit.

### Local Request Completion Pattern

Use this for local command/query boundaries where the caller needs an immediate
typed answer:

```rust
pub trait EventResponseContract:
    Clone + Send + Sync + serde::Serialize + serde::de::DeserializeOwned + 'static
{
    fn validate(&self) -> Result<(), EventValidationError>;
}

pub trait RequestEvent: DomainEvent {
    type Response: EventResponseContract;
}

let response: PolicyDecision = bus
    .publish_request::<PolicyEvaluationRequested>(
        event,
        RequestOptions::with_timeout(Duration::from_millis(500)),
    )
    .await?;
```

Rules:

- response type is associated with the request event; callers cannot choose an
  unrelated `R`;
- response completion validates through `EventResponseContract` before
  resolving;
- deferred/completion handles must not appear in event payload structs, stored
  envelopes, TypeScript domain contracts, or journal JSON;
- caller cancellation or shutdown cancellation is carried by request/publish
  options or handler context, not serialized event payload fields;
- cancellation removes local completion registry entries with an exact
  cancelled report;
- request completion is local-only and not stored directly in the event payload;
- the event envelope remains serializable;
- completion resolves exactly once;
- late completion is ignored and reported;
- timeout produces a request failure and optional dead-letter record;
- durable product flows should still publish explicit result events.

## Journal API

```rust
#[async_trait]
pub trait EventJournal: Send + Sync {
    async fn append(
        &self,
        envelope: &StoredEventEnvelope,
    ) -> Result<JournalAppend, EventingError>;
}

pub struct NdjsonEventJournal;

pub enum JournalMode {
    Disabled,
    BeforeDispatch,
    AfterDispatch,
    BeforeAndAfterDispatch,
}

pub enum JournalSelector {
    All,
    EventTypes(Vec<EventType>),
    Namespaces(Vec<EventNamespace>),
    ContractAllowlist(Vec<EventType>),
}

pub struct JournalPolicy {
    pub mode: JournalMode,
    pub selector: JournalSelector,
}
```

NDJSON implementation rules:

- use Tokio async file IO;
- write exactly one JSON object per line;
- flush according to configured policy;
- include schema version and custody metadata;
- optionally include previous hash and current hash;
- support selected-only journaling by event type, namespace/family, or explicit
  contract allowlist;
- use temp filesystem paths in tests;
- do not block inside async handlers.

SQLite projection is Parent-specific and should be a consumer of replayed or
journaled envelopes, not a hard dependency of the reusable crate.

## Shutdown And Clear Lifecycle

The Games bus had `clear()`. Rust should expose only explicit lifecycle
operations:

```rust
pub enum ShutdownMode {
    Drain,
    DeadLetterQueued,
    DropQueuedForTestOnly,
}

pub async fn shutdown(&self, mode: ShutdownMode) -> ShutdownReport;
pub fn clear_for_test(&self) -> ClearReport;
```

Rules:

- production shutdown drains or dead-letters queued work according to policy;
- in-flight and request registries are resolved, timed out, or reported;
- subscribers are disposed through handles/registrars;
- `clear_for_test` is testkit-only or visibly marked as a test helper;
- production code must not casually clear custody, journal, queue, request, or
  subscriber state.

## Testkit API

```rust
pub fn create_test_event_bus() -> EventBus;
pub fn create_concurrent_test_event_bus() -> EventBus;
pub fn create_ordered_test_event_bus() -> EventBus;

pub struct PublishedEvents;
pub struct TestEventRecorder;
```

The testkit may provide real subscriber functions and record published events.
It must not replace behavior with mocks or spies. Assertions should check exact
event ids, event types, handler outcomes, queue states, and serialized lines.

## Implementation Rules

- Keep the crate platform-neutral.
- Keep files under source-shape budgets by splitting queue, dispatch, journal,
  request, and registrar ownership.
- Avoid blocking IO in async code.
- Use Tokio's multithreaded runtime in tests that exercise concurrency.
- Keep shared mutable state lock-scoped.
- Do not hold sync locks across await.
- Use `#![deny(clippy::await_holding_lock)]` or an equivalent explicit source
  audit gate once the crate exists.
- Do not expose `serde_json::Value` to live handlers.
- Do not pass `&mut E` or mutable payload references to handlers.
- Event payload structs must not contain `Cell`, `RefCell`, `Mutex`, `RwLock`,
  atomics, raw pointers, or interior-mutability fields unless a workpack records
  a narrow exception, source audit, and tests.
- Mutable runtime state belongs in services, actors, queues, or registries, not
  inside event payloads.
- Response types for request events are bound through `RequestEvent::Response`,
  not caller-selected generic `R`.
- No `unsafe`.
- No Parent business constants inside the reusable crate except generic
  `eventing.*` internal event types.
- Parent event names live in protocol constants before Parent runtime consumes
  them.
- Parent portal Vite/TypeScript code must not publish business events directly;
  it sends typed intents to the Rust parent/controller runtime and renders
  service-backed read models.
- The same crate can run in parent/controller and child-agent Rust runtimes, but
  each process owns its own bus instance. Cross-process traffic goes through
  typed service, IPC, WebSocket, LAN, relay, or journal/replay boundaries.
