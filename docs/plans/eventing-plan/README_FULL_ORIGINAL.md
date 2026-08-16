# Reusable Rust Eventing Plan

This folder is the implementation plan for a reusable Rust event bus inspired
by the Ocentra Games `@ocentra/eventing-domain` package, but rebuilt as a
Rust-first crate that can be used by Ocentra Parent and other Ocentra Rust
projects.

The event bus must be reusable infrastructure, not network-specific code. The
same Rust crate should be usable by the parent/controller runtime, the child
agent runtime, LAN/relay helpers, proof harnesses, and future Ocentra Rust
projects.

Network, AI, policy, enforcement, audit, parent-controller, child-agent, portal,
and sync events are consumer contracts layered on top of the bus.

`ocentra-eventing` is local runtime infrastructure only. Cross-device
coordination, including the Household AI Provider Mesh, is handled by consumer
bridges that convert selected local events into typed authenticated transport
messages and republish validated incoming messages into the receiving runtime's
local bus.

## Scope

Build a solid reusable Rust eventing system that supports:

- typed domain events;
- canonical event type constants;
- shared Rust runtime usage across parent/controller and child-agent services;
- async Tokio handlers;
- sequential, concurrent, and aggregate-ordered dispatch;
- registrar-owned subscriptions with deterministic disposal;
- publish-and-wait and fire-and-forget modes;
- request/response command handling where local-only results are needed;
- bounded queues for events published before handlers exist;
- retry, timeout, TTL, idempotency, in-flight duplicate guards, and dead-letter
  behavior;
- target-handler routing;
- event envelopes with event id, correlation id, causation id, source, custody,
  priority, deadline, aggregate key, and schema version;
- explicit runtime role metadata so parent-controller events and child-agent
  events can share the crate while staying auditable;
- optional NDJSON journaling, replay, hash-chain proof, and SQLite projection
  hooks;
- metrics, tracing, panic isolation, and exact handler reports;
- a testkit that uses real Tokio handlers, real serialization, real temp
  filesystem paths, and no mocks or fake services.

## Non-Goals

- Do not copy the TypeScript implementation directly.
- Do not make the reusable crate depend on Ocentra Parent product types.
- Do not put network, AI, policy, or enforcement business logic into the bus.
- Do not make the reusable crate a shared LAN-wide event bus or cross-device
  broker.
- Do not let remote peers publish directly into another runtime's local bus.
- Do not put evidence, policy, AI, enforcement, cascade, or audit business logic
  in Vite/TypeScript UI code.
- Do not let AI, UI, or network evidence directly enforce anything.
- Do not introduce local string event names in app or service code.
- Do not use mocks, fakes, stubs, spies, or replacement transports in tests.

## Planned Crate Boundary

```text
crates/ocentra-eventing/
  generic eventing runtime, event envelope, registrar, dispatch, queue,
  request/response, retry, timeout, dead-letter, journal traits, NDJSON journal,
  metrics hooks, tracing hooks, and reusable testkit.

crates/agent-protocol/
  Parent-specific event type constants and protocol-facing event payloads when
  Rust parent/controller or child-agent services send, receive, store, journal,
  or expose them.

crates/agent-core and crates/agent-service
  Parent runtime publishers, subscribers, projectors, evidence writers,
  WebSocket/read-model bridges, parent-controller orchestration, child-agent
  orchestration, and adapter orchestration.
```

If this crate is later extracted to a shared Ocentra workspace or Git
dependency, its public API should not need to change.

## Source Inputs

- Ocentra Games `E:\ocentra-games\packages\eventing-domain`
- `C:\Users\sujan\.codex\attachments\800513e4-7e64-4fa3-8835-4180f7ec8b82\pasted-text.txt`
- `C:\Users\sujan\.codex\attachments\ebee5dc4-0786-4445-a1f3-bb9e1f42c557\pasted-text.txt`
- Ocentra Parent Rust, domain-boundary, source-shape, validation, and test
  rules.

## Plan Files

- [Source Index](source-index.md)
- [Current Eventing Snapshot](current-eventing-snapshot.md)
- [Rust Eventing Full-Scope Plan](01-rust-eventing-full-scope-plan.md)
- [Crate API And Code Shape](02-crate-api-and-code-shape.md)
- [Event Taxonomy And Parent Integration](03-event-taxonomy-and-parent-integration.md)
- [Tests, Proof, And Validation](04-tests-proof-and-validation.md)
- [Implementation Workpacks](05-implementation-workpacks.md)
- [Type Safety, Validation, And Ownership](06-type-safety-validation-and-ownership.md)
- [Lineage Preservation And Migration Safety](07-lineage-preservation-and-migration-safety.md)
- [Implementation Checklist](implementation-checklist.md)
- [Workpacks](workpacks/README.md)
- [Pasted Content Coverage Audit](pasted-content-coverage-audit.md)

## Network Prerequisite

Network work should not build its own bus. Network Workpack 10 must consume this
eventing plan first, then define network-specific event contracts and handlers
on top of the reusable Rust bus.

## UI Boundary

The parent portal's Vite/TypeScript surface is a view and command-entry layer.
It can render state and send parent intents to the Rust runtime, but it cannot
own evidence interpretation, policy decisions, cascade routing, AI analysis,
enforcement, audit, or durable event state.

```text
Vite/TypeScript UI
  -> sends typed parent intent to Rust service
  -> renders service-backed read model

Rust parent/controller runtime
  -> validates intent
  -> publishes parent/controller events
  -> evaluates policy or forwards to child-agent as typed service command

Rust child-agent runtime
  -> captures evidence
  -> publishes child-agent events
  -> executes supported local actions only after policy and adapter proof
```

The same `ocentra-eventing` crate can run in both Rust runtimes, but an
in-process event bus is not a cross-process broker. Cross-process parent-to-child
traffic must use typed local API, WebSocket, LAN, relay, or journal/replay
boundaries, then publish into each runtime's local bus.

## Completion Proofs

Use `scripts/test/eventing-runtime-proof.mjs` for the reusable bus merge gate.
It proves the generic `crates/ocentra-eventing` runtime without running
network, service, portal, AI, policy, enforcement, broker, relay-hub, or
platform-adapter consumer proofs.

Use `scripts/test/eventing-full-plan-proof.mjs` for full event-plan completion.
It runs the reusable bus proof plus parent/controller, child-agent,
network-consumer, service runtime delivery, service event-chain streaming,
TypeScript parity, UI typed-intent boundary, command-boundary, and enforcement
journal/action eventing proofs. It also runs
`scripts/test/eventing-household-mesh-consumer-proof.mjs` for the Household Mesh
consumer bridge boundary: selected local events can become typed authenticated
LAN messages, incoming messages must validate before local republish, direct
remote publish into another runtime bus is rejected, unselected or mismatched
event/message refs are rejected, and child-agent-only AI policy authority is
preserved. Its network-consumer and Household Mesh consumer proofs must stay
outside `crates/ocentra-eventing`: they prove consumers use the generic bus
boundary, not that the generic bus owns network, LAN, AI, policy, or enforcement
behavior.

## Quality Bar

The event bus is ready for Parent network work only when:

```text
The generic crate compiles with unsafe code forbidden.
Every public runtime string is owned by constants.
Every domain-bearing scalar is a validated Rust newtype, not raw String/Uuid.
Live dispatch uses typed EventEnvelope<E> and EventContext<E>, not serde_json::Value.
Serialized envelopes are limited to journal/replay/transport boundaries.
Events are strongly typed and serializable.
Events validate on construction, deserialize, publish, and replay.
Published event payloads are immutable to handlers.
Request/response is tied to the request event's associated response type.
Unity/C# and TypeScript eventing semantics are preserved through a Rust
compatibility suite before Parent network consumes the crate.
The reusable crate has no hidden global singleton; bus ownership is explicit.
Force/republish behavior is explicit, typed, reasoned, and reported.
Event payloads do not own disposable runtime resources.
Event graph proof identifies publishers, subscribers, and orphan event states.
Sequential, concurrent, and aggregate-ordered dispatch are test-backed.
Retry, TTL, timeout, queue, dead-letter, and idempotency behavior is test-backed.
Registrar disposal is deterministic.
Request/response resolves exactly once and times out cleanly.
Nested publish chains do not deadlock.
Journaling can happen before action.
Replay can rebuild published event history.
No handler panic can crash the service runtime.
No registry, queue, or shared-state lock is held across handler await.
Tests use real Tokio execution, real serde, real temp files, and exact assertions.
Parent-specific network, AI, policy, enforcement, and audit event contracts are
added only after the reusable bus passes its own proof gate.
```
