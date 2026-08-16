# Logged Proof Chain Standard

## Goal

A proof run must tell the chain story:

```text
command -> owner accepts or rejects -> event/request emitted -> consumer handles -> result stored -> read model/UI updates
```

## Existing logging surfaces

Use the existing `@ocentra-parent/logging-domain` package:

| Surface | Use |
| --- | --- |
| `core/logger` | controlled TS logger |
| `test-log/*` | NDJSON test/proof logs |
| `app-log/*` | app log storage/writers |
| `transport/*` | bridge transport |

Rust runtime chains should use `crates/ocentra-eventing` journal/event/request surfaces and expose trace fields that tests can read.

## Required trace fields

| Field | Meaning |
| --- | --- |
| run id | one proof/test run |
| correlation id | one command chain |
| owner | package/crate/service for this step |
| boundary name | command, event, request, response, or read model |
| result | accepted, rejected, emitted, consumed, stored, rendered, blocked |
| no-claim reason | manual-required, degraded, external, host-limited, or not-owned |
| redaction state | confirms sensitive data was not logged raw |

## Logging modes

| Context | Rule |
| --- | --- |
| production | minimal, gated, redacted |
| dev command route | enabled for selected flow |
| tests | enabled for selected run |
| proof | required as artifact |
| failed proof debug | expanded only if scoped and redacted |

## Core rule

Core source participating in a chain must be able to log its boundary milestones. Do not add a second logging system.

## Stop condition

A slice is not ready if failure only says pass/fail and cannot identify the boundary where the chain stopped.
