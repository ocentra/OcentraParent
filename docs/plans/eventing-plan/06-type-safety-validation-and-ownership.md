# Type Safety, Validation, And Ownership

<!-- agent-capsule -->

> Agent Capsule
> Plan: `eventing-plan`
> Doc: `Type Safety, Validation, And Ownership`
> Kind: plan reference document; read only when routed by AGENTS, DOC_INDEX, or workpack.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

The reusable Rust eventing system must be stronger than the Games TypeScript
bus. It should preserve the same workflow value, but it must use Rust's type
system, ownership model, and validation boundaries to prevent raw-string,
unvalidated-payload, shared-mutable-state, and borrow/await bugs.

Current parent architecture is Rust-first. For Parent product surfaces,
canonical contracts, schemas, route snapshots, action handling, read models,
and business logic belong in `crates/schema` or the owning Rust domain/runtime
crate. TypeScript may keep generated bridge DTO consumers, thin adapters,
presentation helpers, or temporary edge decoders; it must not be described as
the product contract owner.

## Effect-Schema-Like Rule For Rust

TypeScript uses Effect Schema brands and decode helpers. Rust must use the
equivalent discipline:

```text
raw external input
  -> parse/validate at boundary
  -> branded/newtype value
  -> typed event struct
  -> typed live envelope
  -> optional serialized journal envelope
```

No domain-bearing value should be represented as raw `String`, `&str`, `Uuid`,
`serde_json::Value`, or loose enum text inside the bus API or Parent runtime
contracts.

Allowed raw boundaries:

- deserializing JSON from journal/replay;
- receiving WebSocket/local API payloads;
- reading files;
- command-line/test fixture input;
- third-party library calls;
- low-level serde implementation internals.

At those boundaries, raw values must be parsed immediately into validated
newtypes or rejected with an exact validation error.

## Branded/Newtype Requirements

Every meaningful scalar gets a Rust newtype with validation:

```rust
pub struct EventType(String);
pub struct EventNameSegment(String);
pub struct SchemaVersion(NonZeroU16);
pub struct EventId(Uuid);
pub struct CorrelationId(Uuid);
pub struct SubscriberId(Uuid);
pub struct SubscriberName(String);
pub struct HandlerName(String);
pub struct TargetHandler(String);
pub struct AggregateKey(String);
pub struct IdempotencyKey(String);
pub struct ServiceName(String);
pub struct ComponentName(String);
pub struct InstanceId(String);
pub struct DeviceRef(String);
pub struct JournalPath(PathBuf);
```

Each newtype must provide:

- `TryFrom` or explicit `parse` constructor;
- validation error with exact reason;
- serde deserialize that validates, not a blind field assignment;
- serde serialize that emits the canonical representation;
- `Display`, `Debug`, `Clone`, `Eq`, `Hash` where needed;
- tests for valid, invalid, boundary, and roundtrip cases.

Avoid cosmetic aliases:

```rust
type EventType = String; // forbidden
type DeviceRef = String; // forbidden
```

## Event Contract Shape

Consumer events are structs with validated fields. They should not expose
mutable business state to handlers.

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
    fn aggregate_key(&self) -> Option<AggregateKey>;
    fn idempotency_key(&self) -> Option<IdempotencyKey>;
}
```

Implementation rule:

- event constructors validate and return `Result<Self, EventValidationError>`;
- serde deserialization validates before returning the event;
- publishing validates before creating a live envelope;
- replay validates before re-publishing or projecting;
- invalid events never reach handlers.

## Typed Live Envelope Versus Serialized Envelope

The live bus must not route `serde_json::Value` payloads.

Correct live dispatch shape:

```rust
pub struct EventEnvelope<E: DomainEvent> {
    pub metadata: EventMetadata,
    pub payload: Arc<E>,
}

pub struct EventContext<E: DomainEvent> {
    pub envelope: EventEnvelope<E>,
    pub publisher: EventPublisher,
}
```

Serialized form is a separate boundary:

```rust
pub struct StoredEventEnvelope {
    pub metadata: EventMetadata,
    pub payload_json: serde_json::Value,
}
```

Rules:

- live handlers receive `EventContext<E>`, not untyped JSON;
- `StoredEventEnvelope` is used only for journal, replay, dead-letter, export,
  or external transport boundaries;
- converting stored to live requires event type lookup plus typed deserialize
  plus validation;
- converting live to stored requires typed serialize and metadata preservation.

## Request/Response Type Safety

Request/response must be typed on both request and response:

```rust
pub trait EventResponseContract:
    Clone + Send + Sync + serde::Serialize + serde::de::DeserializeOwned + 'static
{
    fn validate(&self) -> Result<(), EventValidationError>;
}

pub trait RequestEvent: DomainEvent {
    type Response: EventResponseContract;
}

pub async fn publish_request<E>(
    &self,
    event: E,
    options: RequestOptions,
) -> Result<E::Response, RequestError>
where
    E: RequestEvent;
```

Rules:

- caller cannot request an arbitrary `R` unrelated to the event type;
- response validates before completion through `EventResponseContract`;
- completion resolves once;
- timeout and late-response paths are typed errors;
- durable product flows still publish explicit result events.

## Ownership And Mutation Rules

Published events are immutable facts. Handlers must not mutate event payloads.

Rules:

- handlers receive `Arc<E>` or immutable references through `EventContext<E>`;
- no handler receives `&mut E`;
- event payloads must not contain `Cell`, `RefCell`, `Mutex`, `RwLock`,
  atomics, raw pointers, or other interior-mutability fields unless a workpack
  proves a narrow exception;
- mutable runtime state belongs in the owning service/actor, not the event;
- state changes should happen by publishing a new event or by a handler mutating
  its own owned state through a narrow lock-scoped boundary;
- do not hold locks across `.await`;
- clone subscriber lists before awaiting handlers;
- drop queue/registry locks before dispatch;
- use Tokio `Mutex`/`RwLock` only for small async state that must survive
  across awaits;
- use standard or parking-lot locks only in synchronous sections that never
  cross await.

## Borrow And Await Safety

The implementation must avoid borrow patterns that fight async Rust:

- do not store references into events across `.await`;
- do not return borrowed data from handlers;
- use owned newtypes or `Arc` for data shared across tasks;
- avoid self-referential structs;
- avoid `Rc` and `RefCell` in async runtime code;
- spawned tasks require `Send + 'static`;
- errors should own their message/code data rather than borrowing from transient
  inputs.

## State Transition Safety

Ordered state transitions must be explicit:

- use `AggregateKey` for per-device, per-child, per-policy, per-command, or
  per-audit-chain ordering;
- ordered dispatch serializes same aggregate key;
- different aggregate keys can run concurrently;
- command idempotency prevents duplicate adapter execution;
- event id in-flight guard prevents recursive duplicate publish loops;
- replay defaults to projection-only and cannot execute action handlers unless
  explicitly configured.

## Public API Ban List

The reusable crate public API must not expose these for domain-bearing values:

- raw `String`;
- raw `&str`;
- raw `Uuid`;
- raw `serde_json::Value`;
- `HashMap<String, _>` for event metadata;
- unvalidated enum-from-string helpers;
- `Box<dyn Any>` payload routing;
- mutable event payload references.

Exceptions are limited to explicit raw-boundary modules and must convert to
validated types before live dispatch.

## Parent Cross-Language Contract

When TypeScript and Rust both represent the same Parent event family:

- Rust owns the canonical DTO/event shape, newtypes, serde validation, and
  route/action/read-model semantics in `crates/schema` or the owning Rust
  domain/runtime crate;
- TypeScript owns only generated bridge DTO consumers, thin adapters, or
  temporary edge decoders for untrusted TS edges;
- fixtures or generated-artifact checks prove Rust encoded JSON is accepted by
  TypeScript consumers when needed, and temporary TypeScript edge input is
  accepted by Rust only after Rust-owned validation;
- raw strings remain only at the external boundary and are decoded immediately;
- code generation is preferred for cross-boundary DTOs and must preserve Rust
  field names, discriminants, nullability, enum values, and version semantics.

## Required Proof

Before any workpack can claim "typed eventing":

- invalid event type values are rejected;
- invalid branded values are rejected during construction and deserialization;
- live dispatch never exposes `serde_json::Value` to handlers;
- request response cannot use the wrong response type;
- handlers cannot mutate event payloads;
- duplicate idempotency keys do not execute commands twice;
- lock-held-await audit passes;
- TypeScript/Rust fixture parity passes for Parent event contracts that cross
  both languages.
