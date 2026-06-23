<!-- agent-capsule -->

> Agent Capsule
> Plan: `eventing-plan`
> Doc: `Eventing Workpack Families`
> Kind: owner-path classifier for selected workpacks.
> Read when: only after `WORKPACK_INDEX.md` selects or names a workpack and the owner/proof family is unclear.
> Stop rule: classify the selected workpack only; do not use this file as permission to scan every workpack in the family.
> Proves: routing and owner-path classification only.
> Does not prove: reusable runtime readiness, consumer integration readiness, cross-device delivery, product behavior, or PR readiness.
> Proof rule: if this file changes route/status claims, update `AGENTS.md`, `PLAN_STATE.md`, and any affected selected workpack route.

<!-- /agent-capsule -->

# Eventing Workpack Families

Use this file to classify a selected workpack before opening source. This plan owns reusable local eventing semantics. It does not own consumer feature behavior, cross-device transport, LAN/remote relay delivery, UI rendering, policy/enforcement decisions, data custody policy, or product-specific runtime effects.

## Source boundary and semantics family

```text
Workpacks:
WP01 Source Boundary And Semantics Audit

Owners:
docs/plans/eventing-plan
crates/ocentra-eventing for reusable eventing crate boundaries
schema-domain for shared cross-boundary event contract shapes

Rule:
Source-boundary proof documents what the eventing crate owns and rejects. It is not runtime behavior proof and does not prove any consumer feature.
```

## Crate contract and type boundary family

```text
Workpacks:
WP02 Crate Contract And Type Boundary
WP11 Type Safety And Ownership Hardening when type/source safety is selected

Owners:
crates/ocentra-eventing envelope, ids, contract registry, topology, error, and typed constructor surfaces
schema-domain when shared TS/Rust event shapes cross package/plan boundaries

Rule:
Contract proof must show typed identifiers, event contracts, schema versions, stored payload wrappers, source ownership, and no raw string/JSON public escape hatches where the selected slice forbids them. Contract proof is not dispatch, transport, or consumer behavior proof.
```

## Dispatch runtime and lifecycle family

```text
Workpacks:
WP03 Dispatch Runtime And Lifecycle

Owners:
crates/ocentra-eventing bus, publisher, subscriber, registrar, execution policy, lifecycle, shutdown, and testkit surfaces

Rule:
Dispatch proof is local runtime proof only. It must not claim cross-process transport, LAN/remote delivery, feature side effects, or production broker behavior.
```

## Queue, idempotency, and dead-letter family

```text
Workpacks:
WP04 Queue Idempotency Dead Letter

Owners:
crates/ocentra-eventing queue, idempotency, retry, TTL, overflow, no-subscriber, dead-letter, and metrics/report surfaces

Rule:
Queue proof must cover duplicate handling, backpressure, TTL expiry, retry/dead-letter, no-subscriber behavior, and drain/clear semantics where selected. Queue proof is not consumer policy/enforcement or remote delivery proof.
```

## Request-response family

```text
Workpacks:
WP05 Request Response Contracts

Owners:
crates/ocentra-eventing request registry and response contract surfaces
agent-protocol/service only when the selected workpack names a protocol/service consumer

Rule:
Request-response proof must name request id, response contract, timeout/cancel/duplicate completion behavior, and local response routing. It is not parent/child transport or service delivery proof.
```

## Journal, replay, and lineage family

```text
Workpacks:
WP06 Journal Replay And Lineage

Owners:
crates/ocentra-eventing journal, NDJSON append/replay, hash chain, replay cursor/filter, lineage, version skew, and compatibility surfaces

Rule:
NDJSON journal/replay proof proves append/replay and lineage semantics only. Production fsync policy, SQLite projection, retention/deletion, remote replication, and export/import are consumer/platform decisions.
```

## Protocol event contracts family

```text
Workpacks:
WP07 Parent Protocol Event Contracts

Owners:
crates/agent-protocol and packages/agent-protocol-domain when selected
schema-domain when protocol/event shapes are neutral shared contracts

Rule:
Protocol contract proof is wire/type-shape proof only. It does not prove service delivery, parent/child transport, adapter execution, UI rendering, policy decisioning, or consumer storage.
```

## Parent runtime integration family

```text
Workpacks:
WP08 Parent Runtime Integration

Owners:
consumer runtime plan named by the selected route
crates/agent-core/agent-service only when selected by workpack
crates/ocentra-eventing for reusable local bus semantics only

Rule:
Runtime integration proof must separate local bus publish/subscribe from service transport, parent/controller behavior, child-agent behavior, and adapter effects. Eventing proof does not own the product runtime.
```

## Network consumer chain family

```text
Workpacks:
WP09 Network Consumer Event Chain

Owners:
network-plan and network/service surfaces for network behavior
crates/ocentra-eventing for reusable local bus semantics
agent-protocol/service only when selected

Rule:
Network consumer proof may prove the network event-chain handoff. It does not prove eventing crate completion, host filtering, adapter execution, policy/enforcement behavior, AI classification, or portal rendering.
```

## LAN household mesh consumer family

```text
Workpacks:
WP10 LAN Household Mesh Consumer

Owners:
lan-plan for LAN mesh/pairing/transport behavior
remote-access-plan when relay/remote access is selected
crates/ocentra-eventing for selected export/import validation and local republish semantics only

Rule:
LAN household mesh proof must validate custody, source, family, authority, idempotency, replay, stale-message, and local republish boundaries. It must not turn the local event bus into a remote shared bus or allow provider/peer devices to publish policy/enforcement events directly.
```

## Type safety and ownership hardening family

```text
Workpacks:
WP11 Type Safety And Ownership Hardening

Owners:
crates/ocentra-eventing and selected TS/Rust mirror surfaces named by the workpack

Rule:
Type-safety proof proves source-boundary hardening only. It does not close consumer transport, product behavior, or WP10.
```

## Rollout proof and PR gate family

```text
Workpacks:
WP12 Rollout Proof And PR Gate

Owners:
selected eventing proof roots and route docs

Rule:
WP12 route proof may reconcile proof inventory and no-claim boundaries. It does not close open workpacks by itself and cannot mark PR_READY while WP10 lacks accepted proof or exact blockers.
```

## Test folder regression family

```text
Workpacks:
WP13 Test Folder Layout Regression Audit

Owners:
crates/ocentra-eventing test layout and proof root

Rule:
Test-folder layout proof proves test organization and focused crate revalidation only. It does not close runtime, consumer, transport, or route-gate gaps.
```
