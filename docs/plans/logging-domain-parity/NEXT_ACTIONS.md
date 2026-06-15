<!-- agent-capsule -->

> Agent Capsule
> Plan: `logging-domain-parity`
> Doc: `Logging Domain Parity Next Actions`
> Kind: resume queue and highest-open work.
> Read when: starting or resuming after PLAN_STATE.md.
> Stop rule: Pick one workpack; do not broaden into unrelated plans.
> Proves: next-action routing only.
> Does not prove: implementation completion or PR readiness.
> Proof rule: Update this file only when queue state changes.

<!-- /agent-capsule -->

# Logging Domain Parity Next Actions

## How to use

1. Confirm the branch is `codex/tracking-plan-full-continuation-a`.
2. Pick one workpack from `WORKPACK_INDEX.md`.
3. Open that workpack only.
4. Fill the workpack pre-edit note.
5. Implement, test, run, proof, then update docs.

## Highest-priority queue

### 1. WP01 Current State and Reference Audit

Confirm current parent logging and MCP state before code edits.

Expected result:

```text
current state confirmed
reference files mapped
existing parent MCP found/reused or absence recorded
no source code changes unless audit finds a blocking route mismatch
```

### 2. WP02 TypeScript Package Parity

First implementation slice.

Expected result:

```text
packages/logging-domain gains bridge/test-log/app-log parity modules and scripts
existing contract exports remain intact
no generic Cloudflare hardcode
```

### 3. WP03 Parent Architecture and Routing Fix

Expected result:

```text
portal dev logs have a receiver or bridge path
parent scopes are explicit
package README explains local-dev-observability separately from product safe logging
```

### 4. WP04 Rust Logging Core

Expected result:

```text
crates/logging-core exists
agent-service can delegate dev logging to it
Rust/TS fixture parity tests exist
```

### 5. WP05 Local Validation Evidence

Expected result:

```text
agent:run
agent:query
codex:evidence
local artifacts
agent_run / diagnostics / artifacts rows
```

### 6. WP07 MCP Query Interface

Expected result:

```text
MCP logging server or existing parent MCP extension exists
Codex can query latest failures, run diagnostics, stats, and bounded file slices
MCP and CLI share query/data-access code
```

### 7. WP08 Logger Instrumentation and Adoption

Expected result:

```text
source files use shared logger patterns
source/context fields are queryable
checks prevent raw console/ad hoc log writers in touched logging surfaces
```

### 8. WP06 Validation and Enforcement

Expected result:

```text
validate:logging and lint checks fail when parity files/routes/wrappers/MCP/instrumentation are missing
```

## PR readiness guard

A PR-ready slice should close a named workpack or explicitly list remaining rows.

Do not create a PR that only:

```text
updates checklist text
adds proof prose
renames docs
adds TODO comments
```

unless the assigned workpack is explicitly proof-routing-only.

## Actioned completion tracker

- [ ] Re-check this plan route from `README.md`, `AGENTS.md`, and `PLAN_STATE.md`.
- [ ] Select one workpack from `WORKPACK_INDEX.md`.
- [ ] Implement at least one real source/test behavior before proof/doc updates.
- [ ] Record focused commands and evidence path before reporting progress.
