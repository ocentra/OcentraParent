# Current Eventing Snapshot

<!-- agent-capsule -->

> Agent Capsule
> Plan: `eventing-plan`
> Doc: `Current Eventing Snapshot`
> Kind: current snapshot; read for status/gap claims.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

## Current Product State

Reusable Rust eventing now exists in `crates/ocentra-eventing` as generic
runtime infrastructure. It is not network-specific code, AI code, policy code,
enforcement code, or portal UI code.

Current truth:

- Ocentra Parent has a reusable Rust eventing crate with typed contracts,
  envelopes, handlers, subscriptions, queue/drain behavior, request/response,
  retry/timeout/idempotency/dead-letter behavior, NDJSON journaling/replay,
  metrics snapshots, lifecycle controls, and real testkit coverage.
- Ocentra Games remains the TypeScript lineage reference for bus, registrar,
  queue, retry, TTL, async subscribers, target handler, deferred completion,
  and test bus semantics.
- Parent network runtime can consume `ocentra-eventing` for typed runtime
  events without moving network, AI, policy, or enforcement business logic into
  the reusable crate.
- Parent/controller, child-agent, external transport/relay, LAN/relay, service,
  and portal integrations remain consumer-layer proof work unless a focused
  proof path names them explicitly.
- Vite/TypeScript portal surfaces remain view/input only; they must not own
  evidence, policy, AI, cascade, enforcement, or audit business logic.

## Existing Foundation

| Area                    | Existing Evidence                                                                  | Status                                                                    |
| ----------------------- | ---------------------------------------------------------------------------------- | ------------------------------------------------------------------------- |
| Ocentra Games reference | `E:\ocentra-games\packages\eventing-domain`                                        | Reference semantics covered by compatibility tests.                       |
| Parent Rust workspace   | `Cargo.toml`, `crates/agent-core`, `crates/agent-service`, `crates/agent-protocol` | Existing service/protocol foundation; consumer integrations stay layered. |
| Reusable eventing crate | `crates/ocentra-eventing`                                                          | Implemented generic event bus/runtime with focused Rust proof harnesses.  |
| Parent feature doc      | `docs/features/child-agent-local-service.md`                                       | Names reusable eventing as local-service infrastructure.                  |
| Network dependency      | `docs/features/network-domain-control.md`, `docs/plans/network-plan/README.md`     | Network consumes the reusable eventing crate instead of a private bus.    |
| Rules                   | `.ocentra-ai/rules/ocentra-parent-*.mdc`                                           | Contract-first, Rust async, no test doubles, no UI business logic.        |

## Implemented Eventing Coverage

- Strong event/correlation/causation/subscriber/request/aggregate/idempotency
  id types with validation and serde boundaries.
- Event type taxonomy validation, schema versions, duplicate event-contract
  rejection, generated contract registry docs, compatibility matrix, and
  topology manifest proof.
- Typed `DomainEvent`, live `EventEnvelope<E>`, stored
  `StoredEventEnvelope`, runtime source metadata, target handler routing,
  custody/runtime role/source metadata, event priority, deadlines, aggregate
  keys, and stored payload decode gates.
- Sequential, concurrent, aggregate-ordered, nested, detached, and
  publish-and-wait dispatch with panic isolation and exact handler reports.
- Registrar-owned subscriptions, deterministic disposal, duplicate-subscriber
  rejection, explicit shutdown, drain, clear-for-test, queued dead-letter, and
  pending-request cancellation behavior.
- No-subscriber queueing, auto-drain on subscription, bounded queue overflow
  with oldest-event dead-letter by default, explicit reject-new policy,
  TTL/deadline expiry, retry, handler timeout, request timeout, idempotency,
  event-id duplicate protection, and in-flight duplicate guards.
- Request/response local completion bound through `RequestEvent::Response` and
  `EventResponseContract`, including double-completion, late-response, timeout,
  publish-failure cancellation, and durable result-event separation.
- NDJSON append/replay with hash-chain continuity across reopen, replay cursor,
  projection-only replay gate, and temp-filesystem proof.
- Metrics snapshots for queue, dead-letter, journal, subscriber, and pending
  request state, alongside tracing/testkit behavior using real Tokio handlers
  and real serialization.
- Source gates for no raw public domain-bearing strings, no public
  `serde_json::Value`, no `Uuid`, no payload-carried deferred/cancellation or
  cleanup handles, no mutable event payload references, no hidden singleton,
  and no lock-held await in the eventing crate.

## Remaining Consumer-Layer Gaps

- Parent protocol event payloads and service read-model bridges still need their
  own feature proofs before they can claim full parent/controller or
  child-agent product eventing completion.
- Cross-process parent-to-child, external transport/relay, LAN/relay, and
  service transport delivery are not implemented by the reusable crate. They
  must publish into a local bus on each side after typed transport/API
  boundaries.
- Household AI Provider Mesh is consumer-layer work. The reusable eventing
  crate supplies local bus semantics, typed envelopes, idempotency, TTL, retry,
  dead-letter, aggregate ordering, request/response, journal/replay, and
  topology proof. It does not provide cross-device transport, peer discovery,
  provider trust, job authority, payload custody, policy behavior, or
  enforcement behavior.
- Network AI classification, policy decisions, enforcement commands, adapter
  side effects, audit storage, and portal rendering remain network/service/UI
  consumer work, not event bus responsibilities.
- External transport delivery currently proves local
  queue/idempotency/dead-letter semantics and route-decision requirements only.
  A live transport/relay delivery implementation remains a separate workpack.
- The NDJSON journal is the reusable append/replay proof layer. Production
  durability requirements such as fsync policy, SQLite projections, remote
  replication, or retention/deletion enforcement remain consumer/platform
  decisions.

## Boundary

Correct flow:

```text
Vite/TypeScript UI
  -> typed intent only
  -> Rust parent/controller runtime
  -> parent/controller event bus instance
  -> typed local/LAN/relay/service boundary when child-agent is involved
  -> Rust child-agent runtime
  -> child-agent event bus instance
  -> evidence, AI, policy, enforcement, audit, read-model events
```

Household mesh rule:

```text
ocentra-eventing is local runtime infrastructure only. Cross-device
coordination is handled by a Household Mesh Bridge that converts selected local
events into typed authenticated LAN messages and republishes validated incoming
messages into the receiving runtime's local bus.
```

Incorrect flow:

```text
Vite/TypeScript UI
  -> policy decision
  -> adapter command
```

Incorrect flow:

```text
network evidence
  -> private network bus
  -> adapter command
```

## Product Boundary

The eventing plan upgrades reusable runtime infrastructure only. Product status
can move only when implementation, tests, proof artifacts, and owning
feature/checklist docs are updated for the specific consumer feature.
