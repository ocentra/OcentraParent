# Tracking Agent Driver — Runtime Boundary Map

## Purpose

Tracking must not become a TypeScript-only proof package or a WebSocket transport blob. Runtime behavior is Rust-owned. TypeScript owns contracts, portal rendering, and schema mirrors.

This document maps ownership before implementation.

## Ownership summary

| Layer | Owns | Must not own |
| --- | --- | --- |
| `packages/tracking-domain` | Effect schemas, TS contracts, read-model shapes, fixture helpers | platform runtime execution, service transport, portal decisions |
| `crates/tracking-core` | platform-neutral tracking runtime logic, state decisions, read-model helpers | WebSocket routing, portal UI, TypeScript proof-only behavior |
| `crates/agent-protocol` | Rust protocol structs, event/command names, field constants | runtime decisions |
| `packages/agent-protocol-domain` | TS protocol contracts and command/event schemas | runtime decisions |
| `crates/agent-service` | transport/orchestration, command handling, local service wiring | tracking business logic that belongs in `tracking-core` |
| `apps/portal` | render service read models, send typed parent intents | publish business events directly, decide tracking outcomes |
| `packages/portal-domain` | portal route/content contracts | runtime decisions |
| proof scripts | validate real source behavior and emit artifacts | create the behavior being proved |

## TypeScript tracking-domain boundary

Current TypeScript package:

```text
packages/tracking-domain
```

It should own:

```text
tracking-primitives
tracking-evidence
tracking-geofence
tracking-local-place-store-schemas
tracking-local-place-store
tracking-evidence-quality-gate
tracking-read-model
tracking-retention-runtime
tracking-runtime schema mirrors and pure fixture helpers
```

It should not own:

```text
Android/iOS runtime adapters
service command dispatch
WebSocket routing
portal-local business decisions
real platform sampling
product readiness claims
```

## Rust tracking-core boundary

Current Rust crate:

```text
crates/tracking-core
```

Current modules include:

```text
ai_boundary
alerting
child_check_in
expected_place
geofence
local_place
location_validation
missing_device
nearby_place
parent_acknowledgement
read_model
read_model_guard
retention_settings
runtime_flow
status
temporary_live
```

This is the correct home for platform-neutral tracking behavior.

Add new behavior here when it is:

```text
tracking decision logic
location validation
geofence transition classification
expected-place evaluation
parent acknowledgement effects
nearby-place ambiguity classification
retention settings application
read-model derivation
missing-device / temporary-live state logic
```

Do not bury this logic inside `crates/agent-service/src/websocket.rs`.

## Agent-service boundary

`crates/agent-service` should:

```text
- receive typed commands
- call protocol parsers
- call tracking-core APIs
- compose service responses
- expose local service transport
- preserve audit/evidence refs
```

It should not:

```text
- implement geofence math directly
- implement expected-place policy directly
- implement retention state machine directly
- invent tracking field names outside protocol/domain constants
- make product-readiness claims
```

If service code currently has source-adjacent tests for private seams, they may remain only until a public crate boundary exists. New stable runtime behavior should move into `crates/tracking-core` with crate-level tests.

## Portal boundary

Portal may:

```text
- render tracking read-model rows
- render unsupported/manual-required/degraded states
- send typed parent intents
- show proof/citation references
- show local-only or manual-required explanations
```

Portal may not:

```text
- publish tracking business events directly
- decide geofence/expected-place outcomes
- infer location from LAN/IP/pairing
- turn weak or stale signals into alerts
- claim physical-device behavior from UI state
```

## Protocol boundary

Rust protocol and TS protocol-domain must agree on:

```text
command names
event names
field names
payload shapes
version constants
status labels
manual-required/degraded labels
```

Add parity tests when a field or event crosses TS/Rust.

Minimum parity guard:

```text
TS schema parses a Rust-produced fixture.
Rust deserializes a TS-produced fixture.
```

## Event chain boundary

Tracking work must preserve this chain:

```text
parent intent/config
  -> validated Rust command
  -> tracking config event
  -> child-agent command event
  -> tracking evidence event
  -> detection/cascade event
  -> AI/nearby-place analysis event when needed
  -> policy decision event
  -> live tracking / notification / escalation event
  -> audit event
  -> portal read-model event
```

Direct shortcuts are rejected:

```text
portal button -> child-agent command directly
AI result -> alert/escalation directly
nearby-place category -> accusation directly
weak/stale sample -> critical alert directly
replay -> resend notification or restart live tracking
```

## Source-shape rule

Do not add new aggregate re-export files.

Known concern:

```text
packages/tracking-domain/src/tracking.ts
```

This is currently an aggregate `export *` file. If the repository no-barrel rule is active, this must be handled deliberately:

```text
1. find consumers,
2. migrate imports to direct exports or explicit package export paths,
3. update package exports if needed,
4. add validation to prevent new aggregate re-exports.
```

Do not casually delete it in an unrelated tracking slice.

## Implementation ownership decision tree

Before editing, decide:

```text
Is this a pure schema/read-model contract?
  -> packages/tracking-domain or packages/agent-protocol-domain

Is this platform-neutral runtime logic?
  -> crates/tracking-core

Is this command/event protocol shape?
  -> crates/agent-protocol and packages/agent-protocol-domain

Is this local service transport/orchestration?
  -> crates/agent-service, but call tracking-core

Is this visible UI only?
  -> apps/portal / packages/portal-domain

Is this only proof accounting?
  -> scripts/test and output/test-results, but only after source behavior exists
```

## Validation by boundary

Use focused validation by owner:

```text
packages/tracking-domain:
  npm run test --workspace @ocentra-parent/tracking-domain
  npm run build --workspace @ocentra-parent/tracking-domain

crates/tracking-core:
  cargo test -p ocentra-tracking-core
  cargo clippy -p ocentra-tracking-core --all-targets -- -D warnings

crates/agent-protocol:
  cargo test -p ocentra-parent-agent-protocol

crates/agent-service:
  cargo test -p ocentra-parent-agent-service

portal:
  npm run test --workspace @ocentra-parent/portal
  npm run test:e2e --workspace @ocentra-parent/portal when UI behavior is touched
```

## Acceptance criteria

A tracking implementation slice is valid only when:

```text
- code lives in the owning layer
- tests live near the owning boundary
- protocol parity is preserved when crossing TS/Rust
- service code delegates to tracking-core for runtime decisions
- portal renders read models instead of deciding outcomes
- proof artifacts describe the exact proof tier
- manual-required gaps remain explicit
```
