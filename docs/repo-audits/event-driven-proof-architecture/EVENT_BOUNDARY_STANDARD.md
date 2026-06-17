# Event Boundary Standard

## Core rule

Cross-responsibility behavior is command/event/request/read-model driven.

Direct imports are allowed for:

- schemas;
- constants;
- typed contracts;
- brands;
- parser and decoder helpers;
- local helpers inside the same owner package or crate.

Direct imports are not allowed for another owner's runtime/product behavior.

## Required chain fields

Every cross-owner chain must define:

| Field | Required content |
| --- | --- |
| name | Stable command, event, request, response, or read-model name. |
| producer | Package/crate/service that emits it. |
| consumer | Package/crate/service that consumes it. |
| schema | Exact typed contract path. |
| no-claim boundary | What this chain does not prove. |
| log points | Start, accept/reject, emit, consume, result. |
| proof layer | Unit, contract, integration, service, Playwright, or proof runner. |

## Allowed cross-owner patterns

| Pattern | Use |
| --- | --- |
| command | Request owner action. |
| event | Record that owner behavior happened. |
| request/response | Ask owner for typed result. |
| read model | Consume derived state. |
| journal/projection | Replay or prove state transition. |

## Rust eventing foundation

Rust chains should reuse `crates/ocentra-eventing` for event bus, envelope, journal, queue, request, replay, topology, and testkit where applicable.

Do not create a plan-local bus, journal, or replay abstraction when shared eventing already fits the boundary.
