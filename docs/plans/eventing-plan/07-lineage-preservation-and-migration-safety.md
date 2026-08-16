# Lineage Preservation And Migration Safety

<!-- agent-capsule -->

> Agent Capsule
> Plan: `eventing-plan`
> Doc: `Lineage Preservation And Migration Safety`
> Kind: plan reference document; read only when routed by AGENTS, DOC_INDEX, or workpack.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

This plan is Rust-first, but it must not lose the eventing ideas that worked in
the Unity/C# and Ocentra Games TypeScript systems. The implementation should
preserve the behavior contract, not the old language shape.

## Source Lineage

The Rust crate is informed by three generations:

```text
Unity/C# EventBus
  -> Ocentra Games TypeScript eventing-domain
  -> Ocentra Parent reusable Rust eventing
```

Primary Unity/C# lineage files:

```text
E:\ocentra-games\References\Scripts\OcentraAI\LLMGames\LLMGamesCommon\EventBus\EventBus.cs
E:\ocentra-games\References\Scripts\OcentraAI\LLMGames\LLMGamesCommon\EventBus\EventRegistrar.cs
E:\ocentra-games\References\Scripts\OcentraAI\LLMGames\LLMGamesCommon\EventBus\EventInfo.cs
E:\ocentra-games\References\Scripts\OcentraAI\LLMGames\LLMGamesCommon\EventBus\ScriptInfo.cs
E:\ocentra-games\References\Scripts\OcentraAI\LLMGames\LLMGamesCommon\EventBus\UsageInfo.cs
E:\ocentra-games\References\Scripts\OcentraAI\LLMGames\LLMGamesCommon\EventBus\Editor\EventBusManager.cs
E:\ocentra-games\References\Scripts\OcentraAI\LLMGames\LLMGamesCommon\EventBus\Editor\UsageInfoDrawer.cs
```

Primary TypeScript lineage files are listed in
[source-index.md](source-index.md).

## Preserve The Ideas

| Lineage idea                               | Rust preservation rule                                                                                                                                                          |
| ------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Central event bus per runtime              | Use one runtime-owned bus handle per process or service boundary. Do not add hidden global singletons in the reusable crate.                                                    |
| Registrar-owned subscriptions              | Keep registrar/handle disposal deterministic and idempotent.                                                                                                                    |
| Sync and async subscribers                 | Provide async handlers as the base model and a `subscribe_sync` or ready-future adapter for sync handlers.                                                                      |
| Publish without awaiting async subscribers | Keep `publish_detached`/fire-and-forget behavior with observable reports, metrics, or dead-letter paths.                                                                        |
| Publish and await async subscribers        | Keep awaitable publish with exact handler reports and timeout behavior.                                                                                                         |
| `force` subscribe/publish                  | Replace loose boolean force with explicit `SubscriptionPolicy` and `PublishOverride` values that carry reason and proof.                                                        |
| Republish flag                             | Replace mutable `isRePublishable` with validated `RepublishPolicy` metadata on the envelope.                                                                                    |
| In-flight duplicate guard                  | Reject duplicate non-republishable event ids while active; record idempotency behavior.                                                                                         |
| Queue when no handler exists               | Queue only by explicit policy, drain when a matching handler registers, and record drops/dead letters.                                                                          |
| Queue batch/retry/timeout/TTL              | Make batch size, retry attempts, queue timeout, handler timeout, max queue, and TTL explicit options with tests.                                                                |
| Target handler marker                      | Keep typed target-handler routing and wrong-target reports.                                                                                                                     |
| OperationResult                            | Keep structured publish/handler/request results with success, value or typed outcome, attempts, and exact error.                                                                |
| OperationDeferred                          | Keep local request completion with timeout, double-completion guard, late-response reporting, and settled-state inspection.                                                     |
| Event dispose hook                         | Do not copy event payload disposal into Rust. Published events are immutable facts. Resource cleanup belongs to owned services, guards, or explicit commands, not event `Drop`. |
| EventBus clear/reset                       | Provide test-only or runtime-shutdown clear with audit/logging; do not expose casual production clearing that can drop custody state.                                           |
| Event usage editor                         | Replace Unity editor scanning with a Rust/source audit that builds an event graph: publishers, subscribers, orphan events, no-publisher, and no-subscriber states.              |
| Event health states                        | Preserve pass/fail/no-publisher/no-subscriber status as generated proof artifacts and optional docs, not as runtime business logic.                                             |

## Conformance Matrix

Implementation workpack 69 must turn this into an executable proof matrix.

| Lineage Semantic                              | Rust Owner                                           | Required Test/Proof                                                   | Intentional Deviation                                                 |
| --------------------------------------------- | ---------------------------------------------------- | --------------------------------------------------------------------- | --------------------------------------------------------------------- |
| `EventBus` publish/subscribe                  | `ocentra-eventing::EventBus`                         | publish/subscribe, publish report, unhandled state, panic isolation   | Runtime-owned handle instead of hidden global singleton.              |
| `EventRegistrar` lifecycle                    | registrar and subscription handles                   | dispose/drop/idempotent unsubscribe tests                             | Rust drop/handle lifecycle replaces Unity object lifecycle callbacks. |
| `EventArgsBase` timestamp/id/republish/target | event metadata and typed payload contracts           | metadata/newtype/target-handler/republish tests                       | No inheritance; no mutable republish flag.                            |
| `EventArgsBase` dispose hook                  | owning services and explicit cleanup commands        | no-payload-resource source gate                                       | Events are immutable facts and do not run cleanup in `Drop`.          |
| `OperationResult`                             | publish/handler/request reports                      | result shape, attempts, exact error tests                             | Rich typed report instead of boolean-only handled result.             |
| `OperationDeferred`                           | local request registry plus `RequestEvent::Response` | request timeout, cancellation, double completion, late response tests | No completion handle in event payload or journal.                     |
| Queue/TTL/retry                               | queue, retry, clock modules                          | queued drain, manual clock, retry/dead-letter tests                   | Dead-letter/audit replaces silent drop.                               |
| In-flight duplicate guard                     | idempotency/in-flight registry                       | duplicate event id and idempotency key tests                          | Republish requires typed reasoned override.                           |
| Target handler                                | target module and subscriber descriptor              | targeted/wrong-target tests                                           | Typed target id instead of marker class.                              |
| Test bus                                      | testkit                                              | isolated bus, clear/reset, manual clock tests                         | Test-only clear cannot leak into production API.                      |
| Publisher/subscriber usage graph              | event topology manifest                              | generated manifest and orphan-state proof                             | Static/generated proof replaces Unity editor window.                  |

## Do Not Copy These Shapes

- Unity `ScriptableObject` global config.
- Hidden process-wide singleton in the generic crate.
- Mutable event payload state.
- Event payload `Dispose`/cleanup side effects.
- Loose `force = true` behavior.
- Raw string event keys.
- JSON payload routing inside live dispatch.
- Regex-only usage proof as the sole safety gate.

Each copied idea needs a Rust equivalent with typed contracts, validation,
tests, and proof artifacts.

## Core Stability Rule

The generic eventing core is high-risk infrastructure. Once the first working
implementation lands, feature work must not casually rewrite core dispatch,
queue, retry, request, or journal semantics.

Required safety rules:

- core eventing changes need a focused workpack and proof pack;
- consumer work may add event contracts and handlers without changing core
  runtime semantics;
- any core semantic change must rerun the full eventing compatibility suite;
- no network, AI, policy, enforcement, or portal worker may patch core dispatch
  to make a local feature pass;
- behavior compatibility fixtures must cover Unity/TypeScript lineage
  semantics before Parent network consumes the crate;
- public API changes need a migration note and affected workpack checklist
  updates.

## Runtime Ownership Rule

Correct:

```text
agent-service starts
  -> creates EventBus with explicit EventBusOptions
  -> passes EventPublisher/EventBus handles to owned services
  -> services register through EventRegistrar
  -> shutdown drops registrars, drains or dead-letters queues, closes journal
```

Incorrect:

```text
feature module
  -> EventBus::global()
  -> hidden subscription
  -> untracked queue state
```

The bus can be cloned as a handle, but its root ownership must be explicit.

## Constrained Force And Republish

Unity and TypeScript had `force` escape hatches. Rust should keep the capability
only as typed policy:

```rust
pub enum SubscriptionPolicy {
    RejectDuplicate,
    ReplaceExisting,
    AllowDuplicateWithReason(OverrideReason),
}

pub enum PublishOverride {
    None,
    RepublishWithReason(OverrideReason),
    TestHarnessBypass(OverrideReason),
}
```

Rules:

- no unlabelled boolean `force`;
- override reason is required;
- production overrides are journaled or reported;
- test-only bypasses are available only through testkit or explicit harness
  options;
- republish does not bypass idempotency for commands unless a command-specific
  policy allows it.

## Event Lifecycle Rule

Events are immutable facts. They should not own handles that require disposal.

Allowed event payload fields:

- validated ids and domain newtypes;
- timestamps;
- small owned values;
- `Arc` references to immutable, stable data when needed;
- serializable evidence refs.

Forbidden event payload fields unless a workpack proves a narrow exception:

- file handles;
- sockets;
- async tasks;
- locks;
- mutable buffers;
- cleanup callbacks;
- service pointers;
- UI handles;
- platform adapter handles.

If cleanup is needed, publish an explicit command/result event or let the owning
service clean up its own state after handling.

## Event Graph And Orphan Audit

The Unity editor exposed publisher/subscriber health. Rust should preserve the
same operational value with source/proof artifacts:

```text
event type
publisher paths
subscriber paths
runtime role
schema version
status: pass | no_publisher | no_subscriber | intentionally_unhandled | fail
proof path
```

The audit can use static registry data, generated manifests, source scans, or
test registration snapshots. It must not become the only validation layer, and
it must not replace typed compile-time contracts.

## Compatibility Suite

Before network work consumes the crate, tests should prove these lineage
semantics:

- duplicate subscription rejected unless explicit replacement/override;
- queued event drains when matching handler registers;
- expired queued event does not dispatch;
- publish reports handled versus unhandled exactly;
- sync subscriber order is deterministic;
- async awaited subscribers can run concurrently where configured;
- fire-and-forget async publish remains observable;
- registrar dispose removes all handlers;
- drop of subscription handle unsubscribes;
- republish override is explicit and reported;
- event graph identifies no-subscriber and no-publisher states;
- test clear/reset does not leak subscribers or queued events between tests.
