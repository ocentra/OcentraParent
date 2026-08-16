<!-- agent-capsule -->

> Agent Capsule
> Plan: `logging-domain-parity`
> Doc: `WP10 Proof Trace Pipeline`
> Kind: assigned implementation workpack.
> Read when: selected by WORKPACK_INDEX.md or explicit assignment.
> Stop rule: Do not mass-instrument all tests; implement one proof-trace path and reusable helpers first.
> Proves: proof-trace capability only after focused proof tests pass.
> Does not prove: product feature completion or full UI coverage.
> Proof rule: Before DONE, run a proof-trace smoke test and write proof artifacts.

<!-- /agent-capsule -->

# WP10 Proof Trace Pipeline

## Purpose

Use structured logs as proof traces.

A test should enable proof logging, exercise a path, then query the resulting logs to prove the expected sequence happened.

## Source inputs

```text
docs/plans/logging-domain-parity/09-proof-trace-pipeline.md
docs/plans/logging-domain-parity/07-logger-instrumentation-pattern.md
docs/plans/logging-domain-parity/06-mcp-query-interface.md
apps/portal/src/**
crates/agent-service/src/**
crates/logging-core/**
packages/logging-domain/src/query/**
packages/logging-domain/src/test-log/**
scripts/dev/**
```

## Dependency gate

Required before implementation:

```text
log decision controls exist or are implemented in same slice
source/context instrumentation exists for at least one path
query service exists
fresh-run wipe/retention exists or is stubbed with explicit proof gap
```

If these are missing, route to WP09/WP08/WP02 first.

## Target state

A proof trace mode exists:

```text
start proof trace
  -> enable selected source/level logging
  -> run UI/API/runtime test
  -> flush/ingest
  -> query ordered trace
  -> write proof artifact
  -> disable proof trace
```

## Required proof root

```text
output/logging-domain-parity-proof/10-proof-trace-pipeline/
```

Required artifacts:

```text
00-proof-trace-mode-proof.json
01-playwright-click-trace-proof.json
02-proof-trace-query-proof.json
03-proof-trace-gap-proof.json
04-mcp-proof-trace-proof.json
16-validation-commands.log
```

## Vertical-slice proof boundary

WP10 proves one selected proof-trace vertical path plus reusable helpers. It does not prove all product proof-trace routes, all Playwright ownership, full UI coverage, or product feature completion.

Required proof fields:

```text
proof_trace_id
selected_path
source_context_state
proof_mode_enable_state
proof_mode_disable_state
flush_ingest_state
ordered_trace_state
missing_step_state
out_of_order_state
cli_query_state
mcp_query_state
retention_wipe_state
product_flow_scope
no_claim
```

If an equivalent non-Playwright UI-to-result smoke is used, record that explicitly and do not claim Playwright-specific coverage.

## Checklist rows

- [ ] Proof trace mode controls added.
- [ ] Proof rows include proof_id and correlation_id.
- [ ] Proof rows include source/context/action/event fields.
- [ ] Query service can fetch a proof trace by proof_id.
- [ ] Query service can validate ordered expected steps.
- [ ] Query service reports missing/out-of-order steps.
- [ ] One Playwright or equivalent UI-to-result proof trace smoke exists.
- [ ] Proof trace can be flushed/ingested before assertion.
- [ ] Proof trace can be queried through CLI.
- [ ] Proof trace can be queried through MCP or has explicit MCP-followup blocker.
- [ ] Proof mode is disabled/cleaned after the test.
- [ ] Retention/wipe prevents stale proof traces from polluting normal evidence.
- [ ] Proof root and workpack completion section filled.

## Expected source changes

Possible files:

```text
packages/logging-domain/src/query/**
packages/logging-domain/src/proof-trace/**
packages/logging-domain/scripts/query-proof-trace.ts
packages/logging-domain/scripts/mcp-logging-server.ts
apps/portal/src/** selected proof route only
crates/agent-service/src/** selected proof path only
scripts/test/**
tests/e2e/** or apps/portal tests
```

Do not instrument every UI route in this workpack. Prove one vertical slice and reusable helpers.

## Focused commands

Use final package names once implemented. Expected equivalents:

```bash
npm run test --workspace @ocentra-parent/logging-domain -- proof-trace
npm run test:e2e --workspace @ocentra-parent/portal -- proof-trace
npm run logs:query -- proof-trace <proof-id>
```

If MCP proof trace is implemented:

```bash
npm run mcp:logging -- --smoke proof-trace
```

## Example expected trace

```text
portal.route.opened
portal.action.clicked
portal.command.sent
local-api.command.received
agent-service.command.validated
runtime.decision.completed
local-api.response.sent
portal.read-model.received
portal.ui.rendered
```

## Manual-required gaps

This workpack proves the proof-trace pipeline, not every product feature route.

## Fill before DONE or PR-ready

```text
Workpack id and branch:
Touched files:
Validation commands and results:
Proof artifacts:
Product/runtime claims:
Known gaps/manual-required states:
```

## Current audit note

Focused checks observed in this checkout:

```text
- cmd /c npx vitest run apps/portal/tests/logging/portal-proof-trace-pipeline.test.ts apps/portal/tests/logging/portal-proof-trace.test.ts apps/portal/tests/logging/portal-dev-log-route.test.ts -> pass
- node --import tsx scripts/dev/logging-proof-trace-smoke.mjs --root=test-results/logging-domain-parity-proof-trace --keep-root -> pass
- node scripts/dev/mcp-logging-server.mjs --smoke proof-trace --smoke-root test-results/logging-domain-parity-proof-trace -> pass
```

What this actually proves:

```text
- the portal proof-trace path is implemented and passes when the Vitest flow seeds its own structured log data
- the standalone proof-trace smoke now self-seeds deterministic data, wipes stale proof rows, and passes in a clean workspace
- CLI/MCP proof-trace query helpers exist in source
- output/logging-domain-parity-proof/10-proof-trace-pipeline/ and test-results/logging-domain-parity-proof-trace/ now exist in this checkout
```

What this does not yet prove:

```text
- every product proof-trace route
- Playwright-specific ownership for the current equivalent UI-to-result smoke artifact
- full WP10 checklist closeout
- full product UI coverage
- product feature completion
```

Required next step for truthful closeout:

```text
- use the restored WP10 proof roots when closing WP03 and WP06 so downstream docs stop implying missing proof data
- keep WP10 claims scoped to one selected vertical proof-trace path plus reusable helpers
- either mark checklist rows with proof references or keep them open as partial-proof
```
