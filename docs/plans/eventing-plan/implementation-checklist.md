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
- [x] Manual clock controls TTL, retry, deadline, queue expiry, and request
      timeout tests. E-D added injectable `EventClock`, `ManualEventClock`,
      deadline-aware envelopes/dispatch, deterministic manual-clock tests, and
      `output/eventing-plan-proof/71-manual-clock/proof-summary.json` via
      `scripts/test/eventing-manual-clock-proof.mjs`.
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

| Step | Workpack                                                                       | Status | Owner/Lane | Branch/PR/Commit                                | Evidence Or Proof                                                                                           | Doc/Checklist Decision                                                                                     |
| ---- | ------------------------------------------------------------------------------ | ------ | ---------- | ----------------------------------------------- | ----------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------- |
| 01   | Source index and Ocentra Games semantics audit                                 | [x]    | primary    | main docs pass                                  | `docs/plans/eventing-plan/source-index.md`                                                                  | Planning only; no runtime claim.                                                                           |
| 02   | Reusable crate boundary decision                                               | [x]    | primary    | main docs pass                                  | `docs/plans/eventing-plan/README.md`, `01-rust-eventing-full-scope-plan.md`                                 | Planning only; no crate exists yet.                                                                        |
| 03   | Parent/controller versus child-agent runtime boundary decision                 | [x]    | primary    | main docs pass                                  | `current-eventing-snapshot.md`, `03-event-taxonomy-and-parent-integration.md`                               | Planning only; shared crate across Rust runtimes, no shared in-memory cross-process bus.                   |
| 04   | UI/Vite no-business-logic boundary decision                                    | [x]    | primary    | main docs pass                                  | `README.md`, `01-rust-eventing-full-scope-plan.md`, `04-tests-proof-and-validation.md`                      | Planning only; UI remains view/input.                                                                      |
| 05   | Cargo workspace and dependency decision record                                 | [x]    | E-D        | `codex/eventing-network-runtime-implementation` | `Cargo.toml`, `crates/ocentra-eventing/Cargo.toml`, proof harness                                           | Reusable crate added; no Parent/network dependency in eventing crate.                                      |
| 06   | EventType grammar, constants, duplicate registry, and tests                    | [x]    | E-D        | `codex/eventing-network-runtime-implementation` | `crates/ocentra-eventing/src/ids.rs`, `crates/ocentra-eventing/src/tests.rs`                                | Event type/newtype parsing and duplicate subscriber rejection covered.                                     |
| 07   | Strong id and runtime newtypes                                                 | [x]    | E-D        | `codex/eventing-network-runtime-implementation` | `crates/ocentra-eventing/src/ids.rs`, `cargo test -p ocentra-eventing`                                      | Event/correlation/aggregate/idempotency/source/subscriber/target newtypes added.                           |
| 08   | DomainEvent/EventContract trait and validated serde roundtrip tests            | [x]    | E-D        | `codex/eventing-network-runtime-implementation` | `crates/ocentra-eventing/src/envelope.rs`, `crates/ocentra-eventing/src/tests.rs`                           | Typed trait/contract and stored decode mismatch proof added.                                               |
| 09   | Typed live EventEnvelope and stored-envelope serialization                     | [x]    | E-D        | `codex/eventing-network-runtime-implementation` | `crates/ocentra-eventing/src/envelope.rs`, proof harness                                                    | Live typed envelope plus stored `serde_json::Value` boundary covered.                                      |
| 10   | EventSource, RuntimeRole, EventCustody, target handler                         | [x]    | E-D        | `codex/eventing-network-runtime-implementation` | `crates/ocentra-eventing/src/envelope.rs`, `crates/agent-core/src/network_event_runtime.rs`                 | Source/custody/role/target metadata carried through network chain.                                         |
| 11   | Subscriber registry with no lock-held awaits                                   | [x]    | E-D        | `codex/eventing-network-runtime-implementation` | `crates/ocentra-eventing/src/bus.rs`, `node scripts/check-source-shape.mjs`                                 | Registry snapshot is cloned before handler awaits; source-shape guard passes.                              |
| 12   | Sequential dispatch                                                            | [x]    | E-D        | `codex/eventing-network-runtime-implementation` | `crates/ocentra-eventing/src/tests.rs`                                                                      | Sequential publish dispatch covered.                                                                       |
| 13   | Concurrent dispatch                                                            | [x]    | E-D        | `codex/eventing-network-runtime-implementation` | `crates/ocentra-eventing/src/tests.rs`                                                                      | Concurrent dispatch plus dead-letter capture covered.                                                      |
| 14   | Aggregate-ordered dispatch                                                     | [x]    | E-D        | `codex/eventing-network-runtime-implementation` | `output/eventing-plan-proof/14-24-runtime-lifecycle/proof-summary.json`, `cargo test -p ocentra-eventing`   | Same aggregate transitions serialize; different aggregates can run concurrently.                           |
| 15   | Nested publish through safe event context                                      | [x]    | E-D        | `codex/eventing-network-runtime-implementation` | `crates/ocentra-eventing/src/tests.rs`, `output/eventing-plan-proof/14-24-runtime-lifecycle`                | Handlers receive typed `EventContext<E>` with nested publisher; nested publish does not deadlock.          |
| 16   | Fire-and-forget publish mode                                                   | [x]    | E-D        | `codex/eventing-network-runtime-implementation` | `publish_detached` test, `output/eventing-plan-proof/14-24-runtime-lifecycle`                               | Detached publish returns an observable join report instead of hidden fire-and-forget loss.                 |
| 17   | Publish-and-wait mode                                                          | [x]    | E-D        | `codex/eventing-network-runtime-implementation` | `publish_and_wait`, existing publish report tests, `output/eventing-plan-proof/14-24-runtime-lifecycle`     | Awaited publish returns exact handler reports.                                                             |
| 18   | Handler timeout and retry policy                                               | [x]    | E-D        | `codex/eventing-network-runtime-implementation` | `output/eventing-plan-proof/18-24-handler-policy/proof-summary.json`, `cargo test -p ocentra-eventing`      | `HandlerExecutionPolicy` retries handler failures/timeouts and reports final success/failure attempts.     |
| 19   | Panic isolation and runtime survival                                           | [x]    | E-D        | `codex/eventing-network-runtime-implementation` | `panicking_handler_isolated_as_dead_letter_report`, lifecycle proof harness                                 | Handler panic becomes a handler report/dead-letter; publish survives.                                      |
| 20   | Metrics and tracing fields                                                     | [x]    | E-D        | `codex/eventing-network-runtime-implementation` | `output/eventing-plan-proof/18-24-handler-policy/proof-summary.json`, handler trace report tests            | `EventTraceFields` captures event id, event type, correlation id, subscriber, target handler, outcome.     |
| 21   | EventRegistrar lifecycle                                                       | [x]    | E-D        | `codex/eventing-network-runtime-implementation` | `registrar_dispose_removes_all_owned_subscriptions`, lifecycle proof harness                                | Registrar owns handles, dispose is idempotent, and disposed registrars reject new subscriptions.           |
| 22   | Subscription handle drop and idempotent unsubscribe                            | [x]    | E-D        | `codex/eventing-network-runtime-implementation` | `subscription_handle_drop_unsubscribes_handler`, lifecycle proof harness                                    | Scoped subscription handle drop/unsubscribe removes the subscriber without clearing unrelated bus state.   |
| 23   | Target-handler registration and wrong-target reports                           | [x]    | E-D        | `codex/eventing-network-runtime-implementation` | `crates/ocentra-eventing/src/tests.rs`                                                                      | Targeted event executes only the matching handler.                                                         |
| 24   | Testkit bus construction and event recorder                                    | [x]    | E-D        | `codex/eventing-network-runtime-implementation` | `event_recorder_uses_real_subscription_and_can_unsubscribe`, handler-policy proof harness                   | `EventRecorder<E>` attaches through a real subscription handle and records typed envelopes.                |
| 25   | No-subscriber queue policy                                                     | [x]    | E-D        | `codex/eventing-network-runtime-implementation` | `output/eventing-plan-proof/25-30-queue-policy/proof-summary.json`, `cargo test -p ocentra-eventing`        | Local no-subscriber publishes can queue explicitly and drain only through observable `drain_queued`.       |
| 26   | Bounded queue capacity and overflow policy                                     | [x]    | E-D        | `codex/eventing-network-runtime-implementation` | `bounded_queue_overflow_dead_letters_rejected_event`, queue policy proof harness                            | Queue capacity is bounded; overflow rejects or dead-letters according to explicit policy.                  |
| 27   | TTL/deadline before dispatch and retry                                         | [x]    | E-D        | `codex/eventing-network-runtime-implementation` | `queued_event_expires_before_dispatch_when_ttl_elapsed`, handler policy proof                               | Queue TTL expiry dead-letters before handler dispatch/retry; handler retry/timeout policy remains covered. |
| 28   | In-flight duplicate guard                                                      | [x]    | E-D        | `codex/eventing-network-runtime-implementation` | `in_flight_duplicate_guard_rejects_concurrent_publish`, queue policy proof harness                          | Optional idempotency registry rejects concurrent duplicate publishes while first dispatch is in-flight.    |
| 29   | Idempotency key registry for commands                                          | [x]    | E-D        | `codex/eventing-network-runtime-implementation` | `idempotency_registry_rejects_queued_and_completed_duplicates`, queue policy proof harness                  | Optional registry rejects duplicate queued and completed idempotency keys.                                 |
| 30   | Dead-letter record and event                                                   | [x]    | E-D        | `codex/eventing-network-runtime-implementation` | `DeadLetterEvent`, `DEAD_LETTER_RECORDED_EVENT_TYPE`, queue policy proof harness                            | Dead letters carry explicit reason and can convert to a typed dead-letter domain event.                    |
| 31   | Local request completion registry                                              | [x]    | E-D        | `codex/eventing-network-runtime-implementation` | `output/eventing-plan-proof/31-35-request-response/proof-summary.json`, `cargo test -p ocentra-eventing`    | Local request registry owns completion senders outside event payloads and resolves through context only.   |
| 32   | RequestEvent::Response typed response resolution                               | [x]    | E-D        | `codex/eventing-network-runtime-implementation` | `publish_request_resolves_associated_response_type`, request-response proof harness                         | `publish_request<E>` returns `RequestReport<E::Response>` and handler completion is typed by request.      |
| 33   | Timeout and late-response handling                                             | [x]    | E-D        | `codex/eventing-network-runtime-implementation` | `request_timeout_reports_late_response_without_mutating_result`, request-response proof harness             | Timeout marks registry state; late completion is reported and does not mutate completed caller result.     |
| 34   | Double-completion guard                                                        | [x]    | E-D        | `codex/eventing-network-runtime-implementation` | `double_completion_is_ignored_and_reported`, request-response proof harness                                 | First completion resolves; second completion returns duplicate report.                                     |
| 35   | Durable result-event pattern docs/tests                                        | [x]    | E-D        | `codex/eventing-network-runtime-implementation` | `durable_result_event_pattern_remains_separate_from_local_completion`, request-response proof harness       | Durable result events remain explicit published events separate from local request completion.             |
| 36   | EventJournal trait                                                             | [x]    | E-D        | `codex/eventing-network-runtime-implementation` | `output/eventing-plan-proof/36-41-journal-replay/proof-summary.json`, `eventing-journal-replay-proof.mjs`   | Object-safe async journal trait exposed by reusable crate; no Parent/network dependency.                   |
| 37   | NDJSON append implementation                                                   | [x]    | E-D        | `codex/eventing-network-runtime-implementation` | `ndjson_journal_appends_one_object_per_line_with_hash_chain`, journal/replay proof harness                  | Real Tokio filesystem append writes one JSON object per line and flushes by policy.                        |
| 38   | Hash-chain journal option                                                      | [x]    | E-D        | `codex/eventing-network-runtime-implementation` | `JournalHashChain::Enabled`, `ndjson_journal_appends_one_object_per_line_with_hash_chain`                   | Optional previous/current deterministic chain records are emitted per append.                              |
| 39   | Replay cursor and filters                                                      | [x]    | E-D        | `codex/eventing-network-runtime-implementation` | `replay_cursor_and_filters_read_ordered_projection_records`, journal/replay proof harness                   | Ordered replay supports cursor, event type filter, and correlation filter.                                 |
| 40   | Projection-only replay safety gate                                             | [x]    | E-D        | `codex/eventing-network-runtime-implementation` | `projection_replay_cannot_run_handlers_without_action_mode`, journal/replay proof harness                   | Projection replay cannot dispatch handlers unless explicit action replay mode is used.                     |
| 41   | Journal-before/after dispatch modes                                            | [x]    | E-D        | `codex/eventing-network-runtime-implementation` | `bus_journal_policy_honors_before_after_and_selected_journaling`, journal/replay proof harness              | Bus supports disabled, before, after, and before-and-after durable journal policy hooks.                   |
| 42   | Parent event namespace constants                                               | [ ]    | -          | -                                               | -                                                                                                           | Open.                                                                                                      |
| 43   | Parent/controller event contracts                                              | [ ]    | -          | -                                               | -                                                                                                           | Open.                                                                                                      |
| 44   | Child-agent event contracts                                                    | [ ]    | -          | -                                               | -                                                                                                           | Open.                                                                                                      |
| 45   | Network event contracts                                                        | [~]    | E-D        | `codex/eventing-network-runtime-implementation` | `crates/agent-protocol/src/constants/network_flow.rs`, `crates/agent-core/src/network_event_runtime.rs`     | Rust metadata-only chain constants/payload exist; TS parity and broader contracts remain.                  |
| 46   | AI event contracts                                                             | [ ]    | -          | -                                               | -                                                                                                           | Open.                                                                                                      |
| 47   | Policy event contracts                                                         | [ ]    | -          | -                                               | -                                                                                                           | Open.                                                                                                      |
| 48   | Enforcement event contracts                                                    | [ ]    | -          | -                                               | -                                                                                                           | Open.                                                                                                      |
| 49   | Audit event contracts                                                          | [ ]    | -          | -                                               | -                                                                                                           | Open.                                                                                                      |
| 50   | Portal/read-model event contracts                                              | [ ]    | -          | -                                               | -                                                                                                           | Open.                                                                                                      |
| 51   | Rust parent/controller validated intent publisher                              | [ ]    | -          | -                                               | -                                                                                                           | Open.                                                                                                      |
| 52   | Vite/TypeScript typed-intent-only boundary                                     | [ ]    | -          | -                                               | -                                                                                                           | Open.                                                                                                      |
| 53   | Parent/controller child-command transport handoff                              | [ ]    | -          | -                                               | -                                                                                                           | Open.                                                                                                      |
| 54   | Child-agent command receive and local event publish                            | [ ]    | -          | -                                               | -                                                                                                           | Open.                                                                                                      |
| 55   | Journal-before-action enforcement proof                                        | [ ]    | -          | -                                               | -                                                                                                           | Open.                                                                                                      |
| 56   | Adapter result to audit/read-model proof                                       | [ ]    | -          | -                                               | -                                                                                                           | Open.                                                                                                      |
| 57   | Network Workpack 10 consumes reusable crate                                    | [~]    | E-D        | `codex/eventing-network-runtime-implementation` | `crates/agent-core/Cargo.toml`, `network_event_runtime*.rs`, proof harness                                  | In-process network chain consumes `ocentra-eventing`; broker/queue/request depth remains.                  |
| 58   | Network to AI to policy to enforcement event-chain proof                       | [~]    | E-D        | `codex/eventing-network-runtime-implementation` | `crates/agent-core/src/network_event_runtime_tests.rs`                                                      | Metadata-only chain phases covered; no real analyzer/model/policy adapter action.                          |
| 59   | Weak-network-evidence cannot publish enforcement command                       | [~]    | E-D        | `codex/eventing-network-runtime-implementation` | `ip_only_or_unknown_process_flow_requires_manual_review` test                                               | Weak evidence maps manual-required and `adapter_action_executed=false`; command-routing hardening remains. |
| 60   | AI cannot publish enforcement command                                          | [ ]    | -          | -                                               | -                                                                                                           | Open.                                                                                                      |
| 61   | Portal/UI cannot publish enforcement command                                   | [ ]    | -          | -                                               | -                                                                                                           | Open.                                                                                                      |
| 62   | Network event proof artifacts linked back to eventing plan                     | [~]    | E-D        | `codex/eventing-network-runtime-implementation` | `scripts/test/eventing-network-runtime-proof.mjs`, `test-results/eventing-network-runtime-proof/proof.json` | Harness cross-references eventing and network runtime proof.                                               |
| 63   | Type-safety and validation source gate                                         | [~]    | E-D        | `codex/eventing-network-runtime-implementation` | `node scripts/check-source-shape.mjs`, `crates/ocentra-eventing/src/ids.rs`                                 | Eventing public identifiers are newtypes; broader raw-domain-string audit remains.                         |
| 64   | Typed live envelope versus stored envelope proof                               | [x]    | E-D        | `codex/eventing-network-runtime-implementation` | `crates/ocentra-eventing/src/tests.rs`, `output/eventing-plan-proof/14-24-runtime-lifecycle`                | Handlers receive typed `EventContext<E>`/envelopes; JSON payload stays stored-boundary only.               |
| 65   | RequestEvent associated response proof                                         | [x]    | E-D        | `codex/eventing-network-runtime-implementation` | `RequestEvent::Response`, `EventResponseContract`, request-response proof harness                           | Associated response type is bound at compile time and validates before request completion.                 |
| 66   | Ownership, mutation, and interior-mutability guard                             | [x]    | E-D        | `codex/eventing-network-runtime-implementation` | `output/eventing-plan-proof/66-76-source-safety/proof-summary.json`, `eventing-source-safety-proof.mjs`     | Handler-facing `EventContext<E>` exposes immutable accessors only; no `&mut E` or payload mutation API.    |
| 67   | Borrow/await and no lock-held-await source audit                               | [x]    | E-D        | `codex/eventing-network-runtime-implementation` | `output/eventing-plan-proof/67-lock-await/proof-summary.json`, `eventing-lock-await-proof.mjs`              | No production `.lock().await`; async ordering gates are explicit.                                          |
| 68   | TypeScript/Rust branded fixture parity                                         | [ ]    | -          | -                                               | -                                                                                                           | Open.                                                                                                      |
| 69   | Unity/TypeScript semantics conformance matrix and compatibility suite          | [ ]    | -          | -                                               | -                                                                                                           | Open.                                                                                                      |
| 70   | Event topology manifest and orphan publisher/subscriber audit                  | [ ]    | -          | -                                               | -                                                                                                           | Open.                                                                                                      |
| 71   | Manual clock deterministic TTL, retry, deadline, and request-timeout proof     | [ ]    | -          | -                                               | -                                                                                                           | Open.                                                                                                      |
| 72   | Event contract registry and generated documentation                            | [ ]    | -          | -                                               | -                                                                                                           | Open.                                                                                                      |
| 73   | Duplicate subscription policy and constrained force/republish override         | [ ]    | -          | -                                               | -                                                                                                           | Open.                                                                                                      |
| 74   | Bus shutdown, drain, dead-letter, and test clear lifecycle                     | [ ]    | -          | -                                               | -                                                                                                           | Open.                                                                                                      |
| 75   | Event-family enum/wrapper variant proof for inherited/generic lineage patterns | [ ]    | -          | -                                               | -                                                                                                           | Open.                                                                                                      |
| 76   | No payload-carried deferred, cancellation, handle, or resource source gate     | [x]    | E-D        | `codex/eventing-network-runtime-implementation` | `output/eventing-plan-proof/66-76-source-safety/proof-summary.json`, `eventing-source-safety-proof.mjs`     | Local request senders stay in `RequestRegistry`; event/request payload boundaries carry serialized values. |
| 77   | Selected journaling by event type, namespace/family, and allowlist             | [x]    | E-D        | `codex/eventing-network-runtime-implementation` | `bus_journal_policy_honors_before_after_and_selected_journaling`, journal/replay proof harness              | Journal selector deterministically supports exact event type, namespace/family prefix, and allowlist.      |
| 78   | Runtime-owned bus handle and no hidden global singleton proof                  | [x]    | E-D        | `codex/eventing-network-runtime-implementation` | `crates/ocentra-eventing/src/bus.rs`, `crates/agent-core/src/network_event_runtime.rs`                      | Bus is constructed and owned explicitly; no hidden global singleton.                                       |

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
Journal/replay proof: `output/eventing-plan-proof/36-41-journal-replay/proof-summary.json`; validates rows 36-41 and 77 with real temp filesystem NDJSON append, optional hash-chain fields, ordered replay cursor/filter behavior, corrupt-line error handling, projection-only safety gate, and before/after dispatch journal policy hooks.
Parent runtime boundary proof:
UI boundary proof:
Security negative proof:
Feature docs updated:
Expectation docs updated:
Product capability checklist:
Known gaps/manual-required:
No-claim boundaries preserved:
```
