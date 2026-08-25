# Eventing Plan Workpacks

Each workpack is a narrow proof-backed slice. Workers should implement one
workpack or a clearly named sub-slice, update the main
[implementation checklist](../implementation-checklist.md), and report exact
validation and proof.

## Workpack Matrix

| Id  | Workpack                                                                       | Required Proof Tier | Primary Proof                                                                                                                                         |
| --- | ------------------------------------------------------------------------------ | ------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------- |
| 01  | Source index and Ocentra Games semantics audit                                 | P0                  | Source index names Games reference files, pasted inputs, Parent rules, and current workspace state.                                                   |
| 02  | Reusable crate boundary decision                                               | P0                  | Plan states generic crate ownership and Parent-specific contract boundary.                                                                            |
| 03  | Parent/controller versus child-agent runtime boundary decision                 | P0                  | Plan states same crate, separate in-process bus instances, typed cross-process boundary.                                                              |
| 04  | UI/Vite no-business-logic boundary decision                                    | P0                  | Plan states Vite/TS view/input only and Rust owns business event chains.                                                                              |
| 05  | Cargo workspace and dependency decision record                                 | P1                  | `Cargo.toml` workspace update, dependency rationale, license/security notes.                                                                          |
| 06  | EventType grammar, constants, duplicate registry, and tests                    | P1                  | Invalid/valid event type tests and duplicate registry proof.                                                                                          |
| 07  | Strong id and runtime newtypes                                                 | P1                  | Event/correlation/subscriber/request/aggregate/idempotency/source/path serde/display/parse tests.                                                     |
| 08  | DomainEvent/EventContract trait and validated serde roundtrip tests            | P1                  | Real event structs validate on construction/deserialization and expose schema version.                                                                |
| 09  | Typed live EventEnvelope and stored-envelope serialization                     | P1                  | `EventEnvelope<E>` live dispatch, `StoredEventEnvelope` boundary conversion, metadata preservation, invalid payload proof.                            |
| 10  | EventSource, RuntimeRole, EventCustody, target handler                         | P1                  | Runtime role/custody/target metadata tests.                                                                                                           |
| 11  | Subscriber registry with no lock-held awaits                                   | P2                  | Dispatch test plus source audit proving registry locks are not held across await.                                                                     |
| 12  | Sequential dispatch                                                            | P2                  | Handler order and report order tests.                                                                                                                 |
| 13  | Concurrent dispatch                                                            | P2                  | Parallel execution and aggregate report tests.                                                                                                        |
| 14  | Aggregate-ordered dispatch                                                     | P2                  | Same aggregate serial, different aggregate concurrent tests.                                                                                          |
| 15  | Nested publish through safe event context                                      | P2                  | Network-like chain publishes follow-up events without deadlock.                                                                                       |
| 16  | Fire-and-forget publish mode                                                   | P2                  | Detached publish executes and reports/metrics are observable.                                                                                         |
| 17  | Publish-and-wait mode                                                          | P2                  | Caller receives exact publish report.                                                                                                                 |
| 18  | Handler timeout and retry policy                                               | P2                  | Timeout, retry attempts, final failure/success report tests.                                                                                          |
| 19  | Panic isolation and runtime survival                                           | P2                  | Panicking handler becomes failure/dead-letter without crashing runtime.                                                                               |
| 20  | Metrics and tracing fields                                                     | P2                  | Metrics/tracing hooks capture event id, event type, correlation id, handler, outcome.                                                                 |
| 21  | EventRegistrar lifecycle                                                       | P2                  | Subscribe/dispose/drop lifecycle tests.                                                                                                               |
| 22  | Subscription handle drop and idempotent unsubscribe                            | P2                  | Drop unsubscribes and double-unsubscribe is harmless.                                                                                                 |
| 23  | Target-handler registration and wrong-target reports                           | P2                  | Targeted events execute only matching handlers.                                                                                                       |
| 24  | Testkit bus construction and event recorder                                    | P2                  | Testkit records real published events without mocks/fakes/spies.                                                                                      |
| 25  | No-subscriber queue policy                                                     | P2                  | Queue allowed/rejected behavior and later drain proof.                                                                                                |
| 26  | Bounded queue capacity and overflow policy                                     | P2                  | Reject-new and drop-oldest/dead-letter behavior.                                                                                                      |
| 27  | TTL/deadline before dispatch and retry                                         | P2                  | Past deadline, queued expiry, retry deadline proof.                                                                                                   |
| 28  | In-flight duplicate guard                                                      | P2                  | Duplicate event id rejected while in-flight unless republishable.                                                                                     |
| 29  | Idempotency key registry for commands                                          | P2                  | Duplicate command idempotency key does not execute twice.                                                                                             |
| 30  | Dead-letter record and event                                                   | P2                  | Dead-letter envelope and `eventing.dead_letter.created` event proof.                                                                                  |
| 31  | Local request completion registry                                              | P2                  | Request id and completion registry behavior.                                                                                                          |
| 32  | RequestEvent::Response typed response resolution                               | P2                  | Request resolves the response type associated with the request event.                                                                                 |
| 33  | Timeout and late-response handling                                             | P2                  | Timeout result and late response ignored/reported.                                                                                                    |
| 34  | Double-completion guard                                                        | P2                  | Second completion ignored and reported.                                                                                                               |
| 35  | Durable result-event pattern docs/tests                                        | P2                  | Result-event pattern proves auditable command flow.                                                                                                   |
| 36  | EventJournal trait                                                             | P3                  | Trait and test implementation proof.                                                                                                                  |
| 37  | NDJSON append implementation                                                   | P3                  | One JSON object per line with real temp filesystem IO.                                                                                                |
| 38  | Hash-chain journal option                                                      | P3                  | Previous/current hash chain proof.                                                                                                                    |
| 39  | Replay cursor and filters                                                      | P3                  | Ordered replay, event type filter, correlation filter proof.                                                                                          |
| 40  | Projection-only replay safety gate                                             | P3                  | Replay cannot run action handlers unless explicit action mode is enabled.                                                                             |
| 41  | Journal-before/after dispatch modes                                            | P3                  | Before, after, and both modes tested.                                                                                                                 |
| 42  | Parent event namespace constants                                               | P4                  | Protocol/domain constants and duplicate tests.                                                                                                        |
| 43  | Parent/controller event contracts                                              | P4                  | Serde/protocol tests for parent-controller events.                                                                                                    |
| 44  | Child-agent event contracts                                                    | P4                  | Serde/protocol tests for child-agent events.                                                                                                          |
| 45  | Network event contracts                                                        | P4                  | Serde/protocol tests for network events.                                                                                                              |
| 46  | AI event contracts                                                             | P4                  | Serde/protocol tests for AI events.                                                                                                                   |
| 47  | Policy event contracts                                                         | P4                  | Serde/protocol tests for policy events.                                                                                                               |
| 48  | Enforcement event contracts                                                    | P4                  | Serde/protocol tests for enforcement events and required refs.                                                                                        |
| 49  | Audit event contracts                                                          | P4                  | Serde/protocol tests for audit events.                                                                                                                |
| 50  | Portal/read-model event contracts                                              | P4                  | Serde/protocol tests for read-model events and UI-safe payloads.                                                                                      |
| 51  | Rust parent/controller validated intent publisher                              | P5                  | Rust validation publishes parent-controller event after typed intent only.                                                                            |
| 52  | Vite/TypeScript typed-intent-only boundary                                     | P5                  | UI cannot publish business events; Rust service owns event publish.                                                                                   |
| 53  | Parent/controller child-command transport handoff                              | P5                  | Typed service/transport boundary proof, no shared in-memory cross-process state.                                                                      |
| 54  | Child-agent command receive and local event publish                            | P5                  | Child-agent receives typed command and publishes local event.                                                                                         |
| 55  | Journal-before-action enforcement proof                                        | P5                  | Policy decision journaled before enforcement command handler runs.                                                                                    |
| 56  | Adapter result to audit/read-model proof                                       | P5                  | Adapter result event leads to audit and read-model update.                                                                                            |
| 57  | Network Workpack 10 consumes reusable crate                                    | P6                  | Network plan uses `ocentra-eventing`, not private bus.                                                                                                |
| 58  | Network to AI to policy to enforcement event-chain proof                       | P6                  | Full typed chain with exact refs and no direct enforcement shortcuts.                                                                                 |
| 59  | Weak-network-evidence cannot publish enforcement command                       | P6                  | Negative test/proof.                                                                                                                                  |
| 60  | AI cannot publish enforcement command                                          | P6                  | Negative test/proof.                                                                                                                                  |
| 61  | Portal/UI cannot publish enforcement command                                   | P6                  | Negative test/proof.                                                                                                                                  |
| 62  | Network event proof artifacts linked back to eventing plan                     | P6                  | Eventing and network proof packs cross-reference.                                                                                                     |
| 63  | Type-safety and validation source gate                                         | P1                  | Source audit plus tests prove no raw domain-bearing `String`, `&str`, `Uuid`, or `serde_json::Value` in public eventing APIs.                         |
| 64  | Typed live envelope versus stored envelope proof                               | P1                  | Handlers receive typed `EventContext<E>` only; JSON payloads are limited to journal/replay/dead-letter/export/transport.                              |
| 65  | RequestEvent associated response proof                                         | P2                  | Request response type is bound to the request event through `RequestEvent::Response` and validates through `EventResponseContract` before completion. |
| 66  | Ownership, mutation, and interior-mutability guard                             | P2                  | Handler API exposes no `&mut E`; event payloads have no unproved interior mutability.                                                                 |
| 67  | Borrow/await and no lock-held-await source audit                               | P2                  | Registry, queue, request, and aggregate-order locks are dropped before await and covered by source audit.                                             |
| 68  | TypeScript/Rust branded fixture parity                                         | P4                  | Cross-language fixtures prove Effect Schema brands and Rust newtypes accept the same canonical JSON and reject invalid values.                        |
| 69  | Unity/TypeScript semantics conformance matrix and compatibility suite          | P1/P2               | Matrix maps lineage semantics to Rust behavior, tests, and intentional deviations.                                                                    |
| 70  | Event topology manifest and orphan publisher/subscriber audit                  | P2/P4               | Generated manifest lists event types, publishers, subscribers, family variants, no-publisher, no-subscriber, and accepted one-sided states.           |
| 71  | Manual clock deterministic TTL, retry, deadline, and request-timeout proof     | P2                  | `ManualEventClock` drives timeout/retry/expiry tests without long wall-clock sleeps.                                                                  |
| 72  | Event contract registry and generated documentation                            | P1/P4               | Registered event descriptors generate markdown/docs and reject duplicate event types.                                                                 |
| 73  | Duplicate subscription policy and constrained force/republish override         | P2                  | Duplicate subscriptions reject/replace/allow only by typed policy; republish/force requires reason and report.                                        |
| 74  | Bus shutdown, drain, dead-letter, and test clear lifecycle                     | P2/P3               | Shutdown drains, dead-letters, cancels, or reports state; test clear cannot leak into production paths.                                               |
| 75  | Event-family enum/wrapper variant proof for inherited/generic lineage patterns | P1/P4               | Family subscribers use typed enums/wrappers, not downcasts, loose strings, or JSON shape inspection.                                                  |
| 76  | No payload-carried deferred, cancellation, handle, or resource source gate     | P1/P2               | Event payloads cannot carry local completion/cancellation handles, resources, service pointers, or cleanup callbacks.                                 |
| 77  | Selected journaling by event type, namespace/family, and allowlist             | P3                  | Journal policy selectors are deterministic and test-backed.                                                                                           |
| 78  | Runtime-owned bus handle and no hidden global singleton proof                  | P2/P5               | Services own and pass bus handles explicitly; reusable crate exposes no hidden global singleton.                                                      |

Rows 05-41 and 63-78 are the phase-1 reusable event bus merge gate. Use
`scripts/test/eventing-runtime-proof.mjs` for that gate; it intentionally does
not run parent/controller, child-agent, portal, service, network, external
transport, external relay, policy, AI, enforcement, or platform-adapter consumer
proofs.

Rows 42-62 are consumer integration rows layered on top of the reusable bus.
Keep those in follow-up branches so other lanes can consume the stable bus API
without waiting on network-specific integration.

Household Mesh consumer proof is the separate consumer-layer bridge check for
Household AI provider mesh handoff. Its proof harness is
`scripts/test/eventing-household-mesh-consumer-proof.mjs`, with proof artifacts
under `output/eventing-plan-proof/10-lan-household-mesh-consumer/` and
`test-results/eventing-household-mesh-consumer-proof/`. It must stay outside
`crates/ocentra-eventing`, must reject unselected or mismatched event/message
refs, and must not claim a shared LAN bus, direct remote publish into another
runtime bus, provider policy authority, raw screenshot or capture payload
transfer, production model quality, portal UI, enforcement, or adapter
execution.

## AI Worker Checklist

Each assigned workpack should fill this shape before reporting `DONE`:

```text
Workpack:
Required proof tier:
Current proof tier:
Status:
Branch/commit:
Docs read:
Touched files:
Implementation summary:
Validation:
Proof artifacts:
Known gaps:
Manual-required or N/A reason:
Feature/checklist docs updated:
No-claim boundaries preserved:
Next action:
```

## Execution Rules

- Do not implement Parent-specific event payloads inside the generic crate.
- Do not let network implement a private bus.
- Do not let Vite/TypeScript UI publish business events.
- Do not route live events as `serde_json::Value`.
- Do not expose raw domain-bearing strings, UUIDs, or JSON values in public
  eventing APIs.
- Do not allow handlers to mutate event payloads.
- Do not place deferred/completion handles, cancellation handles, file handles,
  sockets, task handles, service pointers, or cleanup callbacks in event
  payloads.
- Do not expose a hidden global singleton from the reusable crate.
- Do not change core dispatch, queue, retry, request, or journal semantics for a
  consumer feature without a core workpack and compatibility proof.
- Do not use mocks, fakes, stubs, spies, or replacement transports.
- Do not hold locks across handler awaits.
- Do not mark a workpack `[ ]` without proof artifacts.
- Report product-doc updates, or explicitly state why no product-doc update was
  needed.
