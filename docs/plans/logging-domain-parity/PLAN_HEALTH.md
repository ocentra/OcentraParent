<!-- agent-capsule -->

> Agent Capsule
> Plan: `logging-domain-parity`
> Doc: `Logging Domain Parity Plan Health`
> Kind: consistency and readiness check.
> Read when: before claiming the plan is complete, stale, blocked, or PR-ready.
> Stop rule: Do not use this as implementation instructions; use assigned workpacks.
> Proves: plan consistency only.
> Does not prove: source implementation or validation completion.

<!-- /agent-capsule -->

# Logging Domain Parity Plan Health

## Current health

```text
route docs: present
workpack index: present
checklist index: present
proof index: present
workpacks: present
implementation: substantial source/test work exists for WP02/WP03/WP04/WP05/WP06/WP07/WP08/WP09/WP10, but some closeout docs overclaim proof-backed completion relative to this checkout
source proof: output/logging-domain-parity-proof/ exists in this checkout with canonical WP03, WP06, WP07, WP08, and WP10 roots
test-results roots: test-results/logging-domain-parity-mcp/ and test-results/logging-domain-parity-proof-trace/ exist; the remaining named test-results/logging-domain-parity-* roots are still absent
PR-ready: false
```

## Consistency checks

Before reporting broad progress, verify:

```text
README.md routes to AGENTS.md, PLAN_STATE.md, NEXT_ACTIONS.md, WORKPACK_INDEX.md.
WORKPACK_INDEX.md lists every workpack under workpacks/.
WORKPACK_FAMILIES.md is used only when owner/proof family is unclear.
CHECKLIST_INDEX.md has rows for every workpack.
PROOF_INDEX.md has proof roots for every workpack.
TEST_PROOF_EXPECTATIONS.md has commands for every workpack.
Every workpack has a Fill-before-DONE section.
Plan-level done/open labels match the actual checklist state and on-disk proof roots.
```

## Known healthy boundaries

This plan intentionally separates:

```text
local development observability
product/runtime safe logging
cloudflare infra logging
log controls / retention / bridge lifecycle
MCP/CLI query interface
source instrumentation pattern
proof trace pipeline
```

Do not remove that split.

## Known incomplete areas

The plan is not implementation-complete until these are done:

```text
remaining proof inventory restored or proof claims removed
WP03 truthful closeout
WP06 enforcement extended if the plan intends to guarantee proof-inventory honesty
WP07 and WP10 checklist/workpack closeout synced to the restored proof roots
```

## Stale-state triggers

Update `PLAN_STATE.md`, `NEXT_ACTIONS.md`, and `WORKPACK_INDEX.md` when:

```text
a workpack is completed
a workpack is blocked
a new workpack is added
a proof root changes
a root script changes
a no-claim boundary changes
```

## Required hygiene before PR_READY

- Update the assigned workpack.
- Update the relevant checklist row/section.
- Update `PLAN_STATE.md`/`NEXT_ACTIONS.md` if the current state changed.
- Update feature/product docs if a product claim, gap, or proof changed.
- Do not claim READY from proof roots alone.
- Do not claim READY from MCP smoke as full MCP interface completion.
- Do not claim READY from proof-trace smoke as product-flow proof coverage.
- Do not claim READY from portal dev logger proof as full portal logging migration.
- Do not claim READY from agent-service startup/dev-log proof as full Rust logging adoption.
- Do not claim READY from logging-domain package parity as production telemetry readiness.
- Do not claim READY while root logging validation still has the delegated route-check failure.
- Do not claim feature completeness until the relevant E2E tier in `TEST_PROOF_EXPECTATIONS.md` is explicitly proven or blocked.

## Rejection conditions

The plan is unhealthy if:

```text
source files changed without assigned workpack
proof/checklist changed before source/tests for implementation work
other plan folders were edited by this plan without explicit user assignment
root validation was wired before target files existed
workpack status says checked but proof artifacts are missing
MCP or instrumentation claims are made without WP07/WP08 proof
proof-trace claims are made without WP10 proof
log lifecycle/control claims are made without WP09 proof
plan-level done/source-proof claims remain after named proof roots are absent in the checkout
workpack boxes imply completion while CHECKLIST_INDEX.md remains unchecked
```

## Agent route walkthrough

- Landing decision: root plan routing selects this plan for local developer/agent observability parity, not product telemetry ownership.
- Scope split: logging-domain, logging-core, scripts/dev wrappers, MCP query, instrumentation pattern, retention/wipe, proof trace, portal dev-log consumer, and agent-service dev-log consumer remain separate proof families.
- Minimum read set: `README.md`, `AGENTS.md`, `PLAN_STATE.md`, `NEXT_ACTIONS.md`, `WORKPACK_INDEX.md`, `WORKPACK_FAMILIES.md` only when owner/proof family is unclear, one workpack, `TEST_PROOF_EXPECTATIONS.md`, and `PROOF_INDEX.md` when validating proof.
- Test/proof decision: require package, Rust core, bridge/NDJSON, DuckDB/query, wrapper, MCP, instrumentation, retention, validation, and proof-trace tiers only where the selected workpack claims them.
- DONE blocker: no logging claim may move unless proof distinguishes local developer evidence, product-safe logging, portal dev route, agent-service route, MCP query, proof-trace mode, validation state, and no-claim boundaries.

## PR-ready rule

The plan as a whole is PR-ready only when all workpacks are checked and proof roots exist.

A partial PR may be ready only when the selected workpack is complete and the report lists remaining open workpacks.
