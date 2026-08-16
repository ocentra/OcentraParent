<!-- agent-capsule -->

> Agent Capsule
> Plan: `logging-domain-parity`
> Doc: `Logging Domain Parity Workpack Families`
> Kind: owner-path classifier for selected workpacks.
> Read when: only after `WORKPACK_INDEX.md` selects or names a workpack and the owner/proof family is unclear.
> Stop rule: classify the selected workpack only; do not use this file as permission to scan every workpack in the family.
> Proves: routing and owner-path classification only.
> Does not prove: implementation completion, validation closure, product telemetry readiness, production support readiness, or PR readiness.
> Proof rule: if this file changes route/status claims, update `AGENTS.md`, `PLAN_STATE.md`, and any affected selected workpack route.

<!-- /agent-capsule -->

# Logging Domain Parity Workpack Families

Use this file to classify a selected workpack before opening source. This plan owns local developer/agent observability parity. It does not own product telemetry policy, production support workflow design, complete portal logging, complete agent-service logging, or all product proof traces.

## Audit and reference-map family

```text
Workpacks:
WP01 Current State and Reference Audit

Owners:
docs/plans/logging-domain-parity reference/current-state docs
read-only games reference inspection

Rule:
Audit proof maps current state and gaps only. It is not implementation parity, MCP readiness, query readiness, or validation closure.
```

## TypeScript logging package family

```text
Workpacks:
WP02 TypeScript Logging Package Parity

Owners:
packages/logging-domain for TypeScript local logging helpers, bridge, NDJSON, DuckDB/query, app-log, and package exports
schema-domain/event-domain only through public neutral contracts already exposed to the package

Rule:
Package parity proof is local TypeScript package proof. It is not production telemetry readiness, repo-wide instrumentation, MCP proof, or proof-trace coverage by itself.
```

## Parent architecture and routing family

```text
Workpacks:
WP03 Parent Logging Architecture and Routing

Owners:
logging-domain-parity docs for route/scope decisions
apps/portal only for selected portal dev-log consumer proof
crates/agent-service only for selected Rust-side mapping proof

Rule:
Portal dev-log proof is not full WP03 closeout while Rust-side agent-service mapping remains open. Snapshot endpoints are snapshots, not primary log stores.
```

## Rust logging-core family

```text
Workpacks:
WP04 Rust Logging Core Crate

Owners:
crates/logging-core for Rust NDJSON, artifact, dev-log, diagnostic, redaction, source/context, and snapshot helpers
crates/agent-service only when selected as a consumer path

Rule:
Rust logging-core proof proves Rust helper behavior only. It is not agent-service-wide adoption, product runtime logging readiness, or full validation closure.
```

## Local validation evidence wrapper family

```text
Workpacks:
WP05 Local Validation Evidence

Owners:
scripts/dev agent-run/query/evidence wrappers
packages/logging-domain query/NDJSON/DuckDB helpers when selected
logging-core only when Rust artifact/diagnostic proof is selected

Rule:
Wrapper proof must show controlled pass/fail command capture, artifact pointers, parsed diagnostics, and compact evidence packets. It must not become raw terminal-log storage or semantic summarization.
```

## Validation and enforcement family

```text
Workpacks:
WP06 Validation and Enforcement

Owners:
root validation scripts and logging-owned checkers
logging-domain parity plan docs for no-claim boundaries
portal/agent-service only through typed handoff when dev-log-routing points to their owner surface

Rule:
Validation proof is not full closure while `validate:logging` fails. Proof-inventory detection is useful, but it does not restore or close missing proof roots.
```

## MCP query interface family

```text
Workpacks:
WP07 MCP Query Interface

Owners:
scripts/dev MCP server and shared query service
packages/logging-domain query/DuckDB/NDJSON surfaces when selected

Rule:
MCP smoke proof proves selected tools and bounded output only. It does not prove all MCP tools, all scopes, all artifact slices, or package parity by itself.
```

## Logger instrumentation and adoption family

```text
Workpacks:
WP08 Logger Instrumentation and Adoption

Owners:
logging-domain/logger APIs
logging-core for Rust helper pattern
selected portal and agent-service paths only when named by the workpack

Rule:
Instrumentation proof is bounded to the selected surfaces. Portal dev logger proof and agent-service startup/dev-log proof do not prove repo-wide adoption or all service diagnostics.
```

## Log control, retention, and bridge lifecycle family

```text
Workpacks:
WP09 Log Control, Retention, and Bridge Lifecycle

Owners:
packages/logging-domain log decision, wipe, retention, bridge lifecycle, local bridge/tunnel mode helpers
scripts/dev only when selected wrapper lifecycle proof is named

Rule:
Control/retention proof must separate console/storage decisions, fresh-run wipe, retention, bridge health, run-start metadata, stale-run handling, local default, and optional tunnel mode. It is not production retention policy.
```

## Proof trace pipeline family

```text
Workpacks:
WP10 Proof Trace Pipeline

Owners:
packages/logging-domain proof-trace/query helpers
scripts/dev proof-trace/MCP helpers
selected portal or runtime path only for the one vertical proof slice named by the workpack

Rule:
Proof-trace proof must stay scoped to one selected vertical path plus reusable helpers. It does not prove every product route, all Playwright ownership, or full UI coverage.
```
