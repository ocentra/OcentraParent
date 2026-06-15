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
test/proof expectations: present
workpacks: present
implementation: complete at workpack scope
source proof: generated for WP01 through WP10
PR-ready: false
```

## Consistency checks

Before reporting broad progress, verify:

```text
README.md routes to AGENTS.md, PLAN_STATE.md, NEXT_ACTIONS.md, WORKPACK_INDEX.md.
WORKPACK_INDEX.md lists every workpack under workpacks/.
CHECKLIST_INDEX.md has rows for every workpack.
PROOF_INDEX.md has proof roots for every workpack.
TEST_PROOF_EXPECTATIONS.md has commands for every workpack.
Every workpack has a Fill-before-DONE section.
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

## Current completion boundary

The workpacks are complete, but broad completion claims must still preserve these boundaries:

```text
the full @ocentra-parent/portal workspace build is still red for unrelated non-logging type errors
repo-wide logging instrumentation is not claimed
production telemetry readiness is not claimed
product runtime logging readiness is not claimed
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
```

## PR-ready rule

The plan as a whole is PR-ready only when all workpacks are checked and proof roots exist.

A partial PR may be ready only when the selected workpack is complete and the report lists remaining open workpacks.
