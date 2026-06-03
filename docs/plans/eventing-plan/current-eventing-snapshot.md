# Current Eventing Snapshot

## Current Product State

Reusable Rust eventing is planned, not implemented.

Current truth:

- Ocentra Parent has Rust service, protocol, journal, read-model, and proof
  foundations in existing crates.
- Ocentra Games has a mature TypeScript eventing reference with bus,
  registrar, queue, retry, TTL, async subscribers, target handler, deferred
  completion, and test bus semantics.
- Ocentra Parent does not yet have `crates/ocentra-eventing`.
- Parent/controller and child-agent Rust runtimes do not yet share a reusable
  bus implementation.
- Network must not begin a private network-only bus while this shared bus is
  still missing.
- Vite/TypeScript portal surfaces remain view/input only; they must not own
  evidence, policy, AI, cascade, enforcement, or audit business logic.

## Existing Foundation

| Area | Existing Evidence | Status |
| --- | --- | --- |
| Ocentra Games reference | `E:\ocentra-games\packages\eventing-domain` | Reference only. |
| Parent Rust workspace | `Cargo.toml`, `crates/agent-core`, `crates/agent-service`, `crates/agent-protocol` | Existing service/protocol foundation. |
| Parent feature doc | `docs/features/child-agent-local-service.md` | Updated to name reusable eventing as a gap. |
| Network dependency | `docs/plans/network-plan/README.md` and workpacks | Updated to depend on reusable eventing. |
| Rules | `.ocentra-ai/rules/ocentra-parent-*.mdc` | Contract-first, Rust async, no test doubles, no UI business logic. |

## Known Gaps

- `crates/ocentra-eventing` does not exist.
- Workspace membership and crate dependency policy are not implemented.
- Event type grammar and duplicate registry are not implemented.
- Strong event/correlation/subscriber/request/aggregate/idempotency id types are
  not implemented.
- Runtime source/name/path newtypes and serde validation are not implemented.
- `EventContract`, `DomainEvent`, typed live `EventEnvelope<E>`,
  `StoredEventEnvelope`, `EventSource`, `RuntimeRole`, custody, target handler,
  and event priority are not implemented.
- There is no proof that live dispatch avoids `serde_json::Value` payload
  routing.
- There is no proof that request/response types are bound by the request event.
- There is no ownership/mutation proof for immutable event payloads, no
  interior-mutability guard, and no lock-held-await source audit.
- Sequential, concurrent, and aggregate-ordered dispatch are not implemented.
- Registrar dispose lifecycle is not implemented.
- No-subscriber queue, bounded queue, retry, timeout, TTL, dead-letter, and
  idempotency behavior are not implemented.
- Request/response local completion is not implemented.
- NDJSON journal, hash-chain option, replay, and projection-only replay gate are
  not implemented.
- Metrics and tracing hooks are not implemented.
- Parent protocol event constants for parent-controller, child-agent, network,
  AI, policy, enforcement, audit, portal, LAN, and sync are not implemented.
- Parent/controller and child-agent runtime integration is not implemented.
- UI cannot be audited yet for "no business event publish" because the Rust bus
  does not exist.
- Network Workpack 10 remains blocked on reusable eventing proof.

## Boundary

Correct future flow:

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

The eventing plan does not upgrade any product capability by itself. It is
runtime infrastructure. Product status can move only when implementation,
tests, proof artifacts, and owning feature/checklist docs are updated.
