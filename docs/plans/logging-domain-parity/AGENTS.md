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
5. Open exactly one assigned workpack.
6. Use `CHECKLIST_INDEX.md` only for exact rows.
7. Use `PROOF_INDEX.md` only for proof paths.

Do not scan unrelated plan folders.
Do not touch `docs/plans/tracking-plan/` unless explicitly assigned by the user.

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
