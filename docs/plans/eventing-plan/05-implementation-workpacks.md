# Implementation Workpacks

<!-- agent-capsule -->

> Agent Capsule
> Plan: `eventing-plan`
> Doc: `Implementation Workpacks`
> Kind: plan reference document; read only when routed by AGENTS, DOC_INDEX, or workpack.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

No workpack is complete until implementation, tests, validation output, and
proof artifacts exist. A code sketch or plan does not count as proof.

## Proof Tiers

```text
P0_DOCS_PLAN
P1_GENERIC_CRATE_CONTRACT
P2_GENERIC_CRATE_RUNTIME
P3_GENERIC_JOURNAL_REPLAY
P4_PARENT_PROTOCOL_INTEGRATION
P5_PARENT_RUNTIME_INTEGRATION
P6_NETWORK_CONSUMER_READY
```

Every workpack must report:

```text
requiredProofTier
currentProofTier
status
artifactPath
validation
knownGaps
nextAction
```

## Workpacks

### Planning And Boundary

1. Source index and Ocentra Games semantics audit.
2. Reusable crate boundary decision.
3. Parent/controller versus child-agent runtime boundary decision.
4. UI/Vite no-business-logic boundary decision.
5. Cargo workspace and dependency decision record.

### Core Contract

6. `EventType` grammar, constants, duplicate registry, and tests.
7. Strong id and runtime newtypes: event, correlation, subscriber, request,
   aggregate, idempotency, source, path, name, and handler values.
8. `EventContract`/`DomainEvent` traits and validated serde roundtrip tests.
9. Typed live `EventEnvelope<E>` metadata and stored-envelope serialization.
10. `EventSource`, `RuntimeRole`, `EventCustody`, and target-handler model.

### Dispatch Runtime

11. Subscriber registry with lock-scoped access and no lock-held awaits.
12. Sequential dispatch with exact handler reports.
13. Concurrent dispatch with parallel Tokio execution and aggregated reports.
14. Aggregate-ordered dispatch by aggregate key.
15. Nested publish through safe event context.
16. Fire-and-forget publish mode.
17. Publish-and-wait mode.
18. Handler timeout and retry policy.
19. Panic isolation and service-runtime survival.
20. Metrics and tracing fields.

### Registrar And Lifecycle

21. `EventRegistrar` subscribe/dispose lifecycle.
22. Subscription handle drop and idempotent unsubscribe.
23. Target-handler registration and wrong-target ignore reports.
24. Testkit bus construction and event recorder.

### Queue, Idempotency, And Dead Letter

25. No-subscriber queue policy.
26. Bounded queue capacity and overflow policy.
27. TTL/deadline before dispatch and retry.
28. In-flight duplicate guard.
29. Idempotency key registry for commands.
30. Dead-letter record and `eventing.dead_letter.created` event.

### Request/Response

31. Local request completion registry.
32. `RequestEvent::Response` typed response resolution.
33. Timeout and late-response handling.
34. Double-completion guard.
35. Durable result-event pattern documentation and tests.

### Journal And Replay

36. `EventJournal` trait.
37. NDJSON append implementation.
38. Hash-chain journal option.
39. Replay cursor and replay filters.
40. Projection-only replay safety gate.
41. Journal-before-dispatch and journal-after-dispatch modes.

### Parent Protocol Integration

42. Parent event namespace constants in protocol boundary.
43. Parent/controller event contracts.
44. Child-agent event contracts.
45. Network event contracts.
46. AI event contracts.
47. Policy event contracts.
48. Enforcement event contracts.
49. Audit event contracts.
50. Portal/read-model event contracts.

### Parent Runtime Integration

51. Rust parent/controller runtime publishes validated parent intents.
52. Vite/TypeScript UI sends typed intents only and cannot publish business
    events.
53. Parent/controller forwards child commands through typed service or transport
    boundary.
54. Child-agent runtime receives command and republishes local child-agent
    event.
55. Journal-before-action enforcement command proof.
56. Adapter result to audit and read-model proof.

### Network Consumer Readiness

57. Network plan Workpack 10 consumes reusable crate instead of private bus.
58. Network to AI to policy to enforcement event-chain proof.
59. Weak-network-evidence cannot publish enforcement command.
60. AI cannot publish enforcement command.
61. Portal/UI cannot publish enforcement command.
62. Network event proof artifacts linked back to eventing plan.

### LAN / Household Mesh Consumer Integration

79. Household mesh bridge boundary decision.
80. Typed LAN message envelope for selected event export/import.
81. Provider advertisement and heartbeat event contracts.
82. AI work claim/lease/result event contracts.
83. Child-agent-owned AI work ledger integration.
84. Mesh bridge incoming-message validation and local republish proof.
85. Mesh event topology proof: no remote direct publish into local bus.
86. Cross-device idempotency and stale/duplicate message proof.
87. Policy authority proof: provider cannot publish policy/enforcement events.

These are consumer-layer workpacks. They do not change the generic crate proof
tier and must not be marked complete until focused product proofs exist.

### Type Safety And Ownership Hardening

63. Type-safety and validation source gate.
64. Typed live envelope versus stored envelope proof.
65. `RequestEvent` associated response proof.
66. Ownership, mutation, and interior-mutability guard.
67. Borrow/await and no lock-held-await source audit.
68. TypeScript/Rust branded fixture parity.

### Lineage Preservation And Core Safety

69. Unity/TypeScript semantics conformance matrix and compatibility suite.
70. Event topology manifest and orphan publisher/subscriber audit.
71. Manual clock deterministic TTL, retry, deadline, and request-timeout proof.
72. Event contract registry and generated documentation.
73. Duplicate subscription policy and constrained force/republish override.
74. Bus shutdown, drain, dead-letter, and test clear lifecycle.
75. Event-family enum/wrapper variant proof for inherited/generic lineage
    patterns.
76. No payload-carried deferred, cancellation, handle, or resource source gate.
77. Selected journaling by event type, namespace/family, and allowlist.
78. Runtime-owned bus handle and no hidden global singleton proof.

## Main Gates

- [ ] Eventing plan folder exists.
- [ ] Ocentra Games eventing semantics are indexed.
- [ ] Pasted planning inputs are indexed.
- [ ] Reusable `crates/ocentra-eventing` exists.
- [ ] Workspace includes `crates/ocentra-eventing`.
- [ ] Core event types, runtime newtypes, and validation errors are
      implemented.
- [ ] `EventContract`, `DomainEvent`, typed live `EventEnvelope<E>`, and
      `StoredEventEnvelope` are implemented.
- [ ] Live handlers receive typed `EventContext<E>`, not `serde_json::Value`.
- [ ] Request completion response types are bound through
      `RequestEvent::Response` and validate through `EventResponseContract`.
- [ ] Event payload mutation and unproved interior-mutability fields are
      rejected by API shape, audit, and tests.
- [ ] Unity/TypeScript lineage compatibility suite passes.
- [ ] Event topology manifest identifies publishers, subscribers, event-family
      variants, no-publisher, no-subscriber, and accepted one-sided events.
- [ ] Manual clock controls TTL, retry, deadline, queue expiry, and local
      request timeout tests.
- [ ] No event payload carries deferred/completion handles, cancellation
      handles, file/socket/task handles, service pointers, or cleanup callbacks.
- [ ] Bus ownership is explicit; no hidden global singleton is exposed by the
      reusable crate.
- [ ] Sequential, concurrent, and aggregate-ordered dispatch are implemented.
- [ ] Queue, retry, timeout, TTL, dead-letter, and idempotency are implemented.
- [ ] Registrar lifecycle is implemented.
- [ ] Request/response local completion is implemented.
- [ ] NDJSON journal and replay are implemented.
- [ ] Testkit is implemented without mocks/fakes/stubs/spies.
- [ ] `cargo test -p ocentra-eventing` passes.
- [ ] `cargo clippy -p ocentra-eventing --all-targets -- -D warnings` passes.
- [ ] Parent protocol event constants exist before Parent runtime consumes them.
- [ ] Parent/controller and child-agent Rust runtimes both use the shared crate
      through typed contracts.
- [ ] Vite/TypeScript UI has no business-event publishing path.
- [ ] Network plan Workpack 10 is updated from plan dependency to implementation
      proof.
- [ ] Household mesh bridge is implemented as consumer-layer transport and
      validation, not as a shared LAN event bus.
- [ ] Mesh bridge selected-event export/import proof exists.
- [ ] Cross-device AI claim/lease/idempotency proof exists.
- [ ] Provider-result validation proof exists before policy consumption.
- [ ] Provider-cannot-publish-policy/enforcement proof exists.

## Merge-Blocking Failures

- reusable crate depends on Parent product types;
- Parent runtime invents local event strings;
- public eventing APIs expose raw domain-bearing `String`, `&str`, `Uuid`, or
  `serde_json::Value`;
- live dispatch routes `serde_json::Value` payloads to handlers;
- request response type can be chosen independently from the request event;
- handlers receive `&mut E` or mutable event payload references;
- event payload structs contain unproved interior-mutability fields;
- event payload structs carry deferred/completion handles, cancellation handles,
  disposable resources, file/socket/task handles, service pointers, or cleanup
  callbacks;
- feature work changes core eventing dispatch, queue, retry, request, or journal
  semantics without the full compatibility suite;
- reusable crate exposes a hidden global singleton;
- Vite/TypeScript UI owns evidence, AI, policy, cascade, enforcement, or audit
  logic;
- parent/controller and child-agent share in-memory state across process
  boundaries instead of typed transport or journal/replay boundaries;
- handler panic can crash the service runtime;
- ordered aggregate transitions run concurrently;
- locks are held across handler awaits;
- no-subscriber event disappears silently;
- queue overflow has no configured policy;
- retry count is not reported;
- request completion can resolve twice;
- enforcement command can run before journaled policy decision;
- AI or UI can publish enforcement command directly;
- household mesh work exposes `ocentra-eventing` as a LAN-wide shared bus;
- remote peers can publish directly into another runtime's local bus;
- AI provider result can reach policy before child-agent validation;
- tests use mocks, fakes, stubs, spies, or weak existence assertions.

## Worker Instruction

```text
Implement reusable Rust eventing before network bus work.

Target plan:
docs/plans/eventing-plan

Build:
crates/ocentra-eventing

Rules:
- reusable crate must not depend on Parent product types;
- Parent event names live in protocol constants before runtime consumption;
- parent/controller Rust runtime and child-agent Rust runtime can both use the
  same crate;
- Vite/TypeScript UI is rendering and typed command entry only;
- raw external input must parse into validated Rust newtypes before publish;
- live dispatch uses typed `EventEnvelope<E>`/`EventContext<E>` only;
- serialized envelopes are journal/replay/dead-letter/export/transport
  boundaries only;
- request response type is associated with the request event;
- local completion and cancellation handles stay in request/publish context or
  local registries, not event payloads or journals;
- handlers cannot mutate event payloads;
- bus ownership is explicit and runtime-scoped; no hidden global singleton;
- Unity/TypeScript lineage semantics are checked by compatibility tests before
  network consumes the crate;
- no UI business logic for evidence, policy, AI, cascade, enforcement, audit, or
  durable event state;
- no mocks/fakes/stubs/spies in tests;
- Tokio async, no blocking IO in async handlers, no lock-held awaits;
- no unsafe;
- no private network-only event bus.

First implementation proof:
cargo test -p ocentra-eventing
cargo clippy -p ocentra-eventing --all-targets -- -D warnings
```
