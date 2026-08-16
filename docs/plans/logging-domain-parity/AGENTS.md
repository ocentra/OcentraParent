<!-- agent-capsule -->

> Agent Capsule
> Plan: `logging-domain-parity`
> Doc: `Logging Domain Parity Agent Rules`
> Kind: plan-specific agent instructions.
> Read when: before touching files for this plan.
> Stop rule: Follow only this plan route and the assigned workpack.
> Proves: routing and execution constraints only.
> Does not prove: implementation, validation, or PR readiness.

<!-- /agent-capsule -->

# Logging Domain Parity Agent Rules

## Required route

1. Read `README.md`.
2. Read `PLAN_STATE.md`.
3. Read `NEXT_ACTIONS.md`.
4. Read `WORKPACK_INDEX.md`.
5. Use `WORKPACK_FAMILIES.md` only when the selected workpack owner/proof family is unclear.
6. Open exactly one assigned workpack.
7. Use `CHECKLIST_INDEX.md` only for exact rows.
8. Use `PROOF_INDEX.md` only for proof paths.

Do not scan unrelated plan folders.
Do not touch `docs/plans/tracking-plan/` unless explicitly assigned by the user.

## Ownership, Import, And Boundary Contract

This plan owns local developer/agent observability parity. It does not own product telemetry policy, production support workflow design, all portal logging, all agent-service logging, Cloudflare infra logging, or every product feature that emits logs.

Module roles:

```text
packages/logging-domain: TypeScript local logging helpers, bridge transport/server, NDJSON writers, DuckDB/query helpers, log-control/wipe/retention helpers, MCP query helpers, and proof-trace helper surfaces.
crates/logging-core: Rust NDJSON/artifact/dev-log/diagnostic/redaction/source/context helper crate.
scripts/dev: local agent wrapper, query, evidence, MCP, and proof-trace entrypoints used by Codex/local development flows.
apps/portal: dev-log producer/consumer path only when selected; portal owns UI/projection and cannot become the logging system owner.
crates/agent-service: Rust service producer/consumer path only when selected; agent-service owns service runtime behavior and consumes logging-core helpers.
cloudflare-control-plane-plan: backend/Cloudflare infra logging owner when Cloudflare runtime logging is selected.
product/support telemetry owners: product-safe logging, support diagnostics, retention policy, and customer-facing telemetry policy remain outside this local parity plan unless a selected handoff names them.
```

Direct imports are allowed only for explicit public helper surfaces:

```text
logging-domain public exports for TypeScript local logging, bridge, NDJSON, DuckDB/query, MCP, and proof-trace helpers
logging-core public Rust helpers for dev-log, NDJSON/artifact writing, diagnostics, fields, levels, redaction, source/context, and snapshots
scripts/dev wrapper entrypoints when the selected workpack owns agent-run/query/evidence/MCP/proof-trace proof
selected portal/agent-service public surfaces only when the assigned workpack names that path
schema-domain/event-domain only for neutral shared shapes already exposed by their public package surfaces
pure common helpers that do not own product telemetry or runtime side effects
```

Forbidden direct imports and claims:

```text
portal, agent-service, Cloudflare, product telemetry, support, or feature-runtime internals imported to bypass typed logging handoffs
local dev evidence upgraded into production telemetry readiness
MCP smoke upgraded into complete MCP interface readiness
proof-trace smoke upgraded into full product-flow proof coverage
portal dev logger proof upgraded into repo-wide portal logging adoption
agent-service startup/dev-log proof upgraded into full Rust logging adoption
logging-domain package parity upgraded into product runtime logging readiness
proof-inventory query proof upgraded into missing-proof closure
proof roots alone treated as checklist/workpack closeout without matching commands and no-claim boundaries
```

If logging work needs portal, agent-service, Cloudflare, product telemetry, support diagnostics, proof trace, MCP, wrapper, or runtime behavior, it must use typed commands, artifacts, proof roots, structured local rows, and explicit handoffs. Do not solve cross-plan behavior by importing another feature owner's runtime internals.

## Reference implementation rule

The games reference is:

```text
ocentra-games/packages/logging-domain
```

When implementation starts, inspect the reference repo read-only and adapt the pattern. Do not blindly copy Cloudflare-specific defaults.

## Scope rule

This plan owns local developer/agent observability parity:

```text
bridge
NDJSON
DuckDB
query tools
Rust logging-core
local validation evidence wrappers
portal/agent dev log routing
validation enforcement
MCP query interface
proof trace pipeline
```

It does not own product-wide telemetry policy or production support workflow design.

## No proof-only churn

Do not start by changing proof JSON, checklist status, or status docs.

For implementation slices, use:

```text
PLAN -> CODE -> TEST -> RUN/FIX -> PROOF -> DOC
```

Proof-only work is allowed only if the assigned workpack says it is proof-routing-only.

## No semantic log compression

Do not build an LLM log summarizer.

Build deterministic evidence extraction:

```text
command output -> local artifacts -> parsed diagnostics -> NDJSON -> DuckDB -> compact evidence packet
```

## No new barrel/re-export expansion

Do not add new aggregate `export *` files.

If existing package exports require changes, use explicit exports and update validation.

## Completion report

Use this exact format:

```text
Assigned workpack:
Real source behavior added:
Files changed:
Tests added/changed:
Focused commands run:
Proof artifacts:
Checklist/docs updated:
No-claim boundaries preserved:
Remaining gaps:
```
