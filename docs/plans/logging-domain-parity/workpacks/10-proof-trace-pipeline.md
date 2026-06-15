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

## Checklist rows

- [x] Proof trace mode controls added.
- [x] Proof rows include proof_id and correlation_id.
- [x] Proof rows include source/context/action/event fields.
- [x] Query service can fetch a proof trace by proof_id.
- [x] Query service can validate ordered expected steps.
- [x] Query service reports missing/out-of-order steps.
- [x] One Playwright or equivalent UI-to-result proof trace smoke exists.
- [x] Proof trace can be flushed/ingested before assertion.
- [x] Proof trace can be queried through CLI.
- [x] Proof trace can be queried through MCP or has explicit MCP-followup blocker.
- [x] Proof mode is disabled/cleaned after the test.
- [x] Retention/wipe prevents stale proof traces from polluting normal evidence.
- [x] Proof root and workpack completion section filled.

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

## Completion

Workpack id and branch:
WP10 on `codex/tracking-plan-full-continuation-a`

Touched files:
`apps/portal/src/dev-logger.ts`
`apps/portal/tests/logging/portal-proof-trace.test.ts`
`apps/portal/tests/logging/portal-proof-trace-pipeline.test.ts`
`scripts/dev/agent-query.mjs`
`scripts/dev/mcp-logging-server.mjs`
`scripts/dev/lib/log-query-service.mjs`
`docs/plans/logging-domain-parity/NEXT_ACTIONS.md`
`docs/plans/logging-domain-parity/PLAN_STATE.md`
`docs/plans/logging-domain-parity/workpacks/10-proof-trace-pipeline.md`

Validation commands and results:
`cmd /c npx vitest run tests/logging/portal-proof-trace-pipeline.test.ts tests/logging/portal-proof-trace.test.ts tests/logging/portal-dev-log-route.test.ts` passed in `apps/portal`.
`cmd /c npm run lint:architecture -- --files apps/portal/src/dev-logger.ts apps/portal/tests/logging/portal-proof-trace-pipeline.test.ts apps/portal/tests/logging/portal-proof-trace.test.ts apps/portal/tests/logging/portal-dev-log-route.test.ts scripts/dev/agent-query.mjs scripts/dev/mcp-logging-server.mjs scripts/dev/lib/log-query-service.mjs scripts/dev/lib/log-query-service.d.mts` passed.
`cmd /c npm run build --workspace @ocentra-parent/portal` still fails, but the remaining errors are outside WP10 ownership and the touched logging proof-trace files are no longer on the failure list.

Proof artifacts:
`output/logging-domain-parity-proof/10-proof-trace-pipeline/00-proof-trace-mode-proof.json`
`output/logging-domain-parity-proof/10-proof-trace-pipeline/01-playwright-click-trace-proof.json`
`output/logging-domain-parity-proof/10-proof-trace-pipeline/02-proof-trace-query-proof.json`
`output/logging-domain-parity-proof/10-proof-trace-pipeline/03-proof-trace-gap-proof.json`
`output/logging-domain-parity-proof/10-proof-trace-pipeline/04-mcp-proof-trace-proof.json`
`output/logging-domain-parity-proof/10-proof-trace-pipeline/16-validation-commands.log`

Product/runtime claims:
Portal proof-trace controls can now be enabled through portal globals or env-derived config, while direct proof-id calls remain usable for focused tests.
Portal proof rows now carry `proofId`, `traceStep`, `eventType`, optional `action` and `artifactRef`, and proof-specific `correlation_id` data that remain queryable from stored rows.
The shared query service can fetch one proof trace by `proofId`, validate ordered expected steps, and report missing-step gaps from structured local logs.
The same proof trace is queryable through the human CLI surface and the logging MCP server without a separate proof-only data path.
`apps/portal/tests/logging/portal-proof-trace-pipeline.test.ts` now provides durable repo-backed proof for run-start wipe, flush plus DuckDB ingest, query-service lookup, CLI proof-trace rendering, and MCP proof-trace rendering against the current `DevLogger` source/context contract.
WP10 proves one equivalent portal UI-to-result slice, not a full browser-driven Playwright route.

Known gaps/manual-required states:
The `01-playwright-click-trace-proof.json` artifact is an allowed equivalent portal proof-trace smoke, not a literal Playwright browser session.
The shared proof-trace query path currently reads structured NDJSON directly; DuckDB ingest is still executed before assertion in the test to prove flush/ingest readiness rather than to satisfy the trace query itself.
WP06 still owns the final validation/enforcement re-audit for the broader plan, and the full `@ocentra-parent/portal` workspace build remains red for unrelated non-WP10 issues.
