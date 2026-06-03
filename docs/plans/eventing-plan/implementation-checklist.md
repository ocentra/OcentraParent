# Eventing Plan Implementation Checklist

This is the fill-in checklist for reusable Rust eventing work. Future AI
workers must update this file and the matching workpack checklist before
reporting `DONE` or PR-ready.

This checklist tracks eventing-plan execution only. It does not replace
`docs/product-capability-checklist.md`, and workers must not edit the product
checklist unless a feature row status, proof, or gap actually changes and the
worker holds the correct hub lock.

## Fill Rules

- Keep unchecked items unchecked until code, docs, tests, and proof artifacts
  exist.
- Use `[~]` for partial contract/runtime proof where the whole workpack is not
  complete.
- Record lane, branch, PR, commit, or proof path when an item moves.
- Leave intentionally deferred items unchecked and write the manual-required
  reason.
- Do not use this file to claim Parent network readiness without eventing
  implementation proof and Parent protocol integration proof.
- Fill the matching workpack checklist before reporting `DONE`.
- Report product-doc updates, or explicitly state why no product-doc update was
  needed.

## Required Proof Pack

Every implementation workpack needs a proof pack before the main workpack row
can be marked complete. Use this root unless the assignment names a stricter
location:

```text
output/eventing-plan-proof/<workpack-id>/
```

The proof pack must contain or explicitly mark N/A for each applicable item:

- [ ] `00-source-snapshot.md`: git branch, commit, `git status --short`,
      existing source paths inspected, existing behavior, and before-state gap.
- [ ] `01-contract-proof.log`: Rust event type/id/envelope/domain-event tests,
      validated newtypes, typed live envelope proof, no raw public API proof,
      serde roundtrips, duplicate event checks, generated registry docs,
      lineage compatibility proof, and invalid-state tests.
- [ ] `02-dispatch-proof.log`: sequential, concurrent, ordered, nested publish,
      target-handler, and panic-isolation tests.
- [ ] `03-queue-retry-timeout-proof.log`: queue, retry, TTL, timeout,
      idempotency, in-flight guard, and dead-letter tests.
- [ ] `04-request-response-proof.log`: local request completion, timeout,
      double-completion, late-response, and durable result-event tests.
- [ ] `05-journal-replay-proof.log`: NDJSON append, hash-chain, replay cursor,
      projection-only gate, and temp filesystem proof.
- [ ] `06-parent-runtime-boundary-proof.log`: parent/controller and child-agent
      Rust runtime integration tests when Parent runtime paths are touched.
- [ ] `07-ui-boundary-proof.log`: proof that Vite/TypeScript UI sends typed
      intents and cannot publish business events when portal paths are touched.
- [ ] `08-security-negative-proof.log`: no AI/UI direct enforcement, no weak
      evidence command, no mutable event payloads, no lock-held await, no
      silent queue loss, no payload-carried deferred/cancellation/resource
      handles, and no hidden global singleton.
- [ ] `09-manual-platform-proof.md`: explicit N/A unless the workpack touches a
      real platform/manual runtime path.
- [ ] `10-validation-commands.log`: focused validation plus any requested
      `npm run validate`/`ci:local`/manual command output.

## Evidence Quality Gates

- [ ] Every public event type is a constant.
- [ ] Every event type has a schema version.
- [ ] Every domain-bearing scalar is a validated Rust newtype, not a raw
      `String`, `&str`, `Uuid`, or loose enum text.
- [ ] Serde deserialization validates newtypes and event structs before any
      event can be published or replayed.
- [ ] Event contract registry docs are generated from registered contracts and
      duplicate event types are rejected.
- [ ] Event topology manifest records publishers, subscribers, event-family
      variants, no-publisher, no-subscriber, and accepted one-sided events.
- [ ] Every event envelope carries event id, correlation id, source, custody,
      runtime role, and published timestamp.
- [ ] Live dispatch uses typed `EventEnvelope<E>`/`EventContext<E>` and never
      routes `serde_json::Value` to handlers.
- [ ] Serialized `StoredEventEnvelope` appears only at journal, replay,
      dead-letter, export, or transport boundaries.
- [ ] Every dispatch report names each subscriber and outcome.
- [ ] Every failure path has an exact error code or dead-letter reason.
- [ ] Queue overflow cannot silently drop events.
- [ ] Handler panic cannot crash the service runtime.
- [ ] Ordered aggregate transitions are serialized by aggregate key.
- [ ] Request completion response type is bound through
      `RequestEvent::Response`, validates through `EventResponseContract`, and
      resolves once.
- [ ] Handler API never exposes `&mut E` or mutable event payload references.
- [ ] Event payloads do not use interior-mutability fields without an explicit
      exception proof.
- [ ] Event payloads do not carry deferred/completion handles, cancellation
      handles, disposable resources, file/socket/task handles, service
      pointers, or cleanup callbacks.
- [ ] Manual clock controls TTL, retry, deadline, queue expiry, and request
      timeout tests.
- [ ] Duplicate subscriber registration policy is explicit and tested.
- [ ] Shutdown/clear lifecycle drains, dead-letters, cancels, or test-clears
      state according to documented policy.
- [ ] Runtime owns the bus explicitly; reusable crate exposes no hidden global
      singleton.
- [ ] Lock-held-await source audit passes.
- [ ] Journal-before-action mode exists before Parent enforcement consumes the
      bus.
- [ ] Replay defaults to projection-only for Parent safety.
- [ ] Tests use real Tokio handlers, real serde, and real temp files; no mocks,
      fakes, stubs, spies, or replacement transports.
- [ ] Vite/TypeScript UI is not a business-event publisher.

## Main Execution Gates

- [ ] Source docs read: folder README, source index, current snapshot,
      full-scope plan, API/code shape, taxonomy, tests/proof plan,
      type-safety/ownership guide, implementation checklist, and assigned
      workpack.
- [ ] Hub lock covers the workpack file and exact implementation/docs paths.
- [ ] Existing Rust workspace inspected before editing.
- [ ] `crates/ocentra-eventing` stays reusable and Parent-product-type-free.
- [ ] Parent event constants land in protocol/domain boundaries before Parent
      runtime consumes them.
- [ ] Parent/controller and child-agent Rust runtimes use the same crate through
      typed contracts.
- [ ] Cross-process parent-to-child handoff uses typed service, IPC, WebSocket,
      LAN, relay, or journal/replay boundaries.
- [ ] Network Workpack 10 consumes this crate after eventing proof exists.
- [ ] Required proof pack exists with logs, JSON, or explicit N/A reasons for
      every applicable gate.
- [ ] Feature docs, expectation docs, module READMEs, and product capability
      checklist decisions are recorded.
- [ ] `DONE` report includes workpack, touched paths, validation, proof, known
      gaps, and documentation changes.

## Base Workpack Checklist

Use `[ ]` for not started, `[~]` for in progress, and `[x]` only after the
required proof pack exists. The `Evidence Or Proof` cell must name concrete
artifact paths, command logs, PR checks, or an explicit manual-required/N/A
file.

| Step | Workpack                                                                       | Status | Owner/Lane | Branch/PR/Commit | Evidence Or Proof                                                                      | Doc/Checklist Decision                                                                   |
| ---- | ------------------------------------------------------------------------------ | ------ | ---------- | ---------------- | -------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------- |
| 01   | Source index and Ocentra Games semantics audit                                 | [x]    | primary    | main docs pass   | `docs/plans/eventing-plan/source-index.md`                                             | Planning only; no runtime claim.                                                         |
| 02   | Reusable crate boundary decision                                               | [x]    | primary    | main docs pass   | `docs/plans/eventing-plan/README.md`, `01-rust-eventing-full-scope-plan.md`            | Planning only; no crate exists yet.                                                      |
| 03   | Parent/controller versus child-agent runtime boundary decision                 | [x]    | primary    | main docs pass   | `current-eventing-snapshot.md`, `03-event-taxonomy-and-parent-integration.md`          | Planning only; shared crate across Rust runtimes, no shared in-memory cross-process bus. |
| 04   | UI/Vite no-business-logic boundary decision                                    | [x]    | primary    | main docs pass   | `README.md`, `01-rust-eventing-full-scope-plan.md`, `04-tests-proof-and-validation.md` | Planning only; UI remains view/input.                                                    |
| 05   | Cargo workspace and dependency decision record                                 | [ ]    | -          | -                | -                                                                                      | Needs implementation decision.                                                           |
| 06   | EventType grammar, constants, duplicate registry, and tests                    | [ ]    | -          | -                | -                                                                                      | Open.                                                                                    |
| 07   | Strong id and runtime newtypes                                                 | [ ]    | -          | -                | -                                                                                      | Open.                                                                                    |
| 08   | DomainEvent/EventContract trait and validated serde roundtrip tests            | [ ]    | -          | -                | -                                                                                      | Open.                                                                                    |
| 09   | Typed live EventEnvelope and stored-envelope serialization                     | [ ]    | -          | -                | -                                                                                      | Open.                                                                                    |
| 10   | EventSource, RuntimeRole, EventCustody, target handler                         | [ ]    | -          | -                | -                                                                                      | Open.                                                                                    |
| 11   | Subscriber registry with no lock-held awaits                                   | [ ]    | -          | -                | -                                                                                      | Open.                                                                                    |
| 12   | Sequential dispatch                                                            | [ ]    | -          | -                | -                                                                                      | Open.                                                                                    |
| 13   | Concurrent dispatch                                                            | [ ]    | -          | -                | -                                                                                      | Open.                                                                                    |
| 14   | Aggregate-ordered dispatch                                                     | [ ]    | -          | -                | -                                                                                      | Open.                                                                                    |
| 15   | Nested publish through safe event context                                      | [ ]    | -          | -                | -                                                                                      | Open.                                                                                    |
| 16   | Fire-and-forget publish mode                                                   | [ ]    | -          | -                | -                                                                                      | Open.                                                                                    |
| 17   | Publish-and-wait mode                                                          | [ ]    | -          | -                | -                                                                                      | Open.                                                                                    |
| 18   | Handler timeout and retry policy                                               | [ ]    | -          | -                | -                                                                                      | Open.                                                                                    |
| 19   | Panic isolation and runtime survival                                           | [ ]    | -          | -                | -                                                                                      | Open.                                                                                    |
| 20   | Metrics and tracing fields                                                     | [ ]    | -          | -                | -                                                                                      | Open.                                                                                    |
| 21   | EventRegistrar lifecycle                                                       | [ ]    | -          | -                | -                                                                                      | Open.                                                                                    |
| 22   | Subscription handle drop and idempotent unsubscribe                            | [ ]    | -          | -                | -                                                                                      | Open.                                                                                    |
| 23   | Target-handler registration and wrong-target reports                           | [ ]    | -          | -                | -                                                                                      | Open.                                                                                    |
| 24   | Testkit bus construction and event recorder                                    | [ ]    | -          | -                | -                                                                                      | Open.                                                                                    |
| 25   | No-subscriber queue policy                                                     | [ ]    | -          | -                | -                                                                                      | Open.                                                                                    |
| 26   | Bounded queue capacity and overflow policy                                     | [ ]    | -          | -                | -                                                                                      | Open.                                                                                    |
| 27   | TTL/deadline before dispatch and retry                                         | [ ]    | -          | -                | -                                                                                      | Open.                                                                                    |
| 28   | In-flight duplicate guard                                                      | [ ]    | -          | -                | -                                                                                      | Open.                                                                                    |
| 29   | Idempotency key registry for commands                                          | [ ]    | -          | -                | -                                                                                      | Open.                                                                                    |
| 30   | Dead-letter record and event                                                   | [ ]    | -          | -                | -                                                                                      | Open.                                                                                    |
| 31   | Local request completion registry                                              | [ ]    | -          | -                | -                                                                                      | Open.                                                                                    |
| 32   | RequestEvent::Response typed response resolution                               | [ ]    | -          | -                | -                                                                                      | Open.                                                                                    |
| 33   | Timeout and late-response handling                                             | [ ]    | -          | -                | -                                                                                      | Open.                                                                                    |
| 34   | Double-completion guard                                                        | [ ]    | -          | -                | -                                                                                      | Open.                                                                                    |
| 35   | Durable result-event pattern docs/tests                                        | [ ]    | -          | -                | -                                                                                      | Open.                                                                                    |
| 36   | EventJournal trait                                                             | [ ]    | -          | -                | -                                                                                      | Open.                                                                                    |
| 37   | NDJSON append implementation                                                   | [ ]    | -          | -                | -                                                                                      | Open.                                                                                    |
| 38   | Hash-chain journal option                                                      | [ ]    | -          | -                | -                                                                                      | Open.                                                                                    |
| 39   | Replay cursor and filters                                                      | [ ]    | -          | -                | -                                                                                      | Open.                                                                                    |
| 40   | Projection-only replay safety gate                                             | [ ]    | -          | -                | -                                                                                      | Open.                                                                                    |
| 41   | Journal-before/after dispatch modes                                            | [ ]    | -          | -                | -                                                                                      | Open.                                                                                    |
| 42   | Parent event namespace constants                                               | [ ]    | -          | -                | -                                                                                      | Open.                                                                                    |
| 43   | Parent/controller event contracts                                              | [ ]    | -          | -                | -                                                                                      | Open.                                                                                    |
| 44   | Child-agent event contracts                                                    | [ ]    | -          | -                | -                                                                                      | Open.                                                                                    |
| 45   | Network event contracts                                                        | [ ]    | -          | -                | -                                                                                      | Open.                                                                                    |
| 46   | AI event contracts                                                             | [ ]    | -          | -                | -                                                                                      | Open.                                                                                    |
| 47   | Policy event contracts                                                         | [ ]    | -          | -                | -                                                                                      | Open.                                                                                    |
| 48   | Enforcement event contracts                                                    | [ ]    | -          | -                | -                                                                                      | Open.                                                                                    |
| 49   | Audit event contracts                                                          | [ ]    | -          | -                | -                                                                                      | Open.                                                                                    |
| 50   | Portal/read-model event contracts                                              | [ ]    | -          | -                | -                                                                                      | Open.                                                                                    |
| 51   | Rust parent/controller validated intent publisher                              | [ ]    | -          | -                | -                                                                                      | Open.                                                                                    |
| 52   | Vite/TypeScript typed-intent-only boundary                                     | [ ]    | -          | -                | -                                                                                      | Open.                                                                                    |
| 53   | Parent/controller child-command transport handoff                              | [ ]    | -          | -                | -                                                                                      | Open.                                                                                    |
| 54   | Child-agent command receive and local event publish                            | [ ]    | -          | -                | -                                                                                      | Open.                                                                                    |
| 55   | Journal-before-action enforcement proof                                        | [ ]    | -          | -                | -                                                                                      | Open.                                                                                    |
| 56   | Adapter result to audit/read-model proof                                       | [ ]    | -          | -                | -                                                                                      | Open.                                                                                    |
| 57   | Network Workpack 10 consumes reusable crate                                    | [ ]    | -          | -                | -                                                                                      | Open.                                                                                    |
| 58   | Network to AI to policy to enforcement event-chain proof                       | [ ]    | -          | -                | -                                                                                      | Open.                                                                                    |
| 59   | Weak-network-evidence cannot publish enforcement command                       | [ ]    | -          | -                | -                                                                                      | Open.                                                                                    |
| 60   | AI cannot publish enforcement command                                          | [ ]    | -          | -                | -                                                                                      | Open.                                                                                    |
| 61   | Portal/UI cannot publish enforcement command                                   | [ ]    | -          | -                | -                                                                                      | Open.                                                                                    |
| 62   | Network event proof artifacts linked back to eventing plan                     | [ ]    | -          | -                | -                                                                                      | Open.                                                                                    |
| 63   | Type-safety and validation source gate                                         | [ ]    | -          | -                | -                                                                                      | Open.                                                                                    |
| 64   | Typed live envelope versus stored envelope proof                               | [ ]    | -          | -                | -                                                                                      | Open.                                                                                    |
| 65   | RequestEvent associated response proof                                         | [ ]    | -          | -                | -                                                                                      | Open.                                                                                    |
| 66   | Ownership, mutation, and interior-mutability guard                             | [ ]    | -          | -                | -                                                                                      | Open.                                                                                    |
| 67   | Borrow/await and no lock-held-await source audit                               | [ ]    | -          | -                | -                                                                                      | Open.                                                                                    |
| 68   | TypeScript/Rust branded fixture parity                                         | [ ]    | -          | -                | -                                                                                      | Open.                                                                                    |
| 69   | Unity/TypeScript semantics conformance matrix and compatibility suite          | [ ]    | -          | -                | -                                                                                      | Open.                                                                                    |
| 70   | Event topology manifest and orphan publisher/subscriber audit                  | [ ]    | -          | -                | -                                                                                      | Open.                                                                                    |
| 71   | Manual clock deterministic TTL, retry, deadline, and request-timeout proof     | [ ]    | -          | -                | -                                                                                      | Open.                                                                                    |
| 72   | Event contract registry and generated documentation                            | [ ]    | -          | -                | -                                                                                      | Open.                                                                                    |
| 73   | Duplicate subscription policy and constrained force/republish override         | [ ]    | -          | -                | -                                                                                      | Open.                                                                                    |
| 74   | Bus shutdown, drain, dead-letter, and test clear lifecycle                     | [ ]    | -          | -                | -                                                                                      | Open.                                                                                    |
| 75   | Event-family enum/wrapper variant proof for inherited/generic lineage patterns | [ ]    | -          | -                | -                                                                                      | Open.                                                                                    |
| 76   | No payload-carried deferred, cancellation, handle, or resource source gate     | [ ]    | -          | -                | -                                                                                      | Open.                                                                                    |
| 77   | Selected journaling by event type, namespace/family, and allowlist             | [ ]    | -          | -                | -                                                                                      | Open.                                                                                    |
| 78   | Runtime-owned bus handle and no hidden global singleton proof                  | [ ]    | -          | -                | -                                                                                      | Open.                                                                                    |

## Worker Report Template

Use this shape in the hub report or PR-ready note:

```text
DONE eventing workpack <number/name>
Owner/lane:
Branch/commit/PR:
Touched paths:
Checklist updates:
Source snapshot:
Validation commands and logs:
Proof pack root:
Contract proof:
Dispatch proof:
Queue/retry/timeout proof:
Journal/replay proof:
Parent runtime boundary proof:
UI boundary proof:
Security negative proof:
Feature docs updated:
Expectation docs updated:
Product capability checklist:
Known gaps/manual-required:
No-claim boundaries preserved:
```
