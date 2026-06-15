<!-- agent-capsule -->

> Agent Capsule
> Plan: `logging-domain-parity`
> Doc: `WP07 MCP Query Interface`
> Kind: assigned implementation workpack.
> Read when: selected by WORKPACK_INDEX.md or explicit assignment.
> Stop rule: Do not build MCP before the underlying query/DuckDB path exists.
> Proves: logging MCP query interface only after smoke tests/proof pass.
> Does not prove: TypeScript package parity, Rust logging-core, or validation wrapper completion by itself.
> Proof rule: Before DONE, run MCP smoke tests and write proof artifacts.

<!-- /agent-capsule -->

# WP07 MCP Query Interface

## Purpose

Add or upgrade a Codex-native MCP query interface over the local logging evidence store.

CLI is useful, but MCP is the intended agent interface:

```text
Codex asks for latest failures or logs
  -> MCP tool queries DuckDB/NDJSON
  -> bounded structured result returns
  -> Codex fixes using precise evidence
```

This workpack preserves the core intent of the plan: high-density deterministic evidence instead of terminal dumps.

## Source inputs

```text
docs/plans/logging-domain-parity/06-mcp-query-interface.md
docs/plans/logging-domain-parity/03-local-validation-evidence.md
ocentra-games/AGENTS.md MCP tool list
ocentra-games/.cursor/rules/ocentra-games-rules.mdc MCP_USE_GUIDELINE
ocentra-games/packages/logging-domain/README.md storage/query sections
packages/logging-domain/src/test-log/**
packages/logging-domain/src/app-log/**
packages/logging-domain/scripts/**
scripts/dev/agent-query.mjs
scripts/dev/codex-evidence.mjs
```

## Existing MCP audit gate

Before implementing, check whether OcentraParent already has an MCP framework from earlier roadmap work.

Run locally:

```bash
git grep -ni "mcp\|model context protocol\|modelcontextprotocol" -- .
find . -iname '*mcp*' -o -iname '*modelcontext*'
```

Remote pre-check found no obvious parent MCP server or config in:

```text
root package scripts
packages/logging-domain package scripts
.mcp.json
mcp.json
.cursor/mcp.json
packages/mcp-domain
packages/mcp-server
apps/mcp
apps/mcp-server
scripts/dev/mcp-server.mjs
scripts/dev/mcp-logging-server.mjs
packages/logging-domain/scripts/mcp-logging-server.ts
```

Decision rule:

```text
If existing MCP exists: upgrade/adapt it and do not create a second MCP framework.
If existing MCP does not exist: implement the logging MCP server described here.
```

Record the audit result in:

```text
output/logging-domain-parity-proof/07-mcp-query-interface/00-existing-mcp-audit.json
```

## Dependency gate

Run this after the query/DuckDB path exists.

Required before implementation:

```text
DuckDB query service exists or is implemented in same slice.
NDJSON ingest path exists.
Parent scopes exist.
At least one local evidence run can be inserted or fixture-loaded.
```

If these are missing, route to WP02/WP05 first.

## Target state

MCP server exists and can query local logging evidence.

Preferred package files when no parent MCP framework exists:

```text
packages/logging-domain/src/mcp/**
packages/logging-domain/scripts/mcp-logging-server.ts
```

Acceptable root implementation:

```text
scripts/dev/mcp-logging-server.mjs
```

Preferred scripts:

```json
{
  "mcp:logging": "npm run mcp:logging --workspace @ocentra-parent/logging-domain"
}
```

## Required MCP tools

Minimum tools:

```text
get_errors
get_recent_logs
get_logs_by_source
get_logs_by_context
query_logs
get_log_stats
get_latest_failures
get_run_diagnostics
get_artifact_slice
```

## Required result rules

All tools must enforce:

```text
compact rows by default
bounded limits
max limits
bounded artifact slices
no full stdout/stderr by default
local-only file access
clear stale-ingest or missing-data errors
```

## Shared query service rule

MCP and CLI must share query/data-access code.

Do not implement two divergent query engines.

Preferred shape:

```text
packages/logging-domain/src/query/logQueryService.ts
packages/logging-domain/scripts/query-test-logs.ts      -> uses logQueryService
packages/logging-domain/scripts/mcp-logging-server.ts   -> uses logQueryService
scripts/dev/agent-query.mjs                             -> uses same service or thin wrapper
scripts/dev/codex-evidence.mjs                          -> uses same service or thin wrapper
```

## Required proof root

```text
output/logging-domain-parity-proof/07-mcp-query-interface/
```

Required artifacts:

```text
00-existing-mcp-audit.json
01-mcp-tool-list.json
02-mcp-latest-failures-smoke.json
03-mcp-run-diagnostics-smoke.json
04-mcp-artifact-slice-smoke.json
05-cli-mcp-query-parity-proof.json
16-validation-commands.log
```

## Checklist rows

- [x] Existing parent MCP framework audited.
- [x] Existing MCP reused/upgraded or absence recorded.
- [x] MCP source/query service designed around existing DuckDB/NDJSON query path.
- [x] MCP server script added or existing server extended.
- [x] `mcp:logging` package/root script added.
- [x] `get_errors` implemented.
- [x] `get_recent_logs` implemented.
- [x] `get_logs_by_source` implemented.
- [x] `get_logs_by_context` implemented.
- [x] `query_logs` implemented.
- [x] `get_log_stats` implemented.
- [x] `get_latest_failures` implemented.
- [x] `get_run_diagnostics` implemented.
- [x] `get_artifact_slice` implemented with bounded output.
- [x] MCP and CLI share query/data-access code.
- [x] MCP smoke tests pass.
- [x] Agent guidance documents MCP-first, CLI-fallback behavior.
- [x] Proof root and workpack completion section filled.

## Focused commands

Expected commands depend on final MCP framework, but must include equivalents of:

```bash
npm run mcp:logging -- --list-tools
npm run mcp:logging -- --smoke latest-failures
npm run mcp:logging -- --smoke run-diagnostics
npm run mcp:logging -- --smoke artifact-slice
npm run test --workspace @ocentra-parent/logging-domain -- mcp
```

If the MCP server is only testable through an MCP client harness, add that harness and record the exact command.

## Negative tests

Required negative coverage:

```text
unknown scope returns a clear error
missing DuckDB with existing NDJSON triggers ingest or clear stale-ingest message
artifact slice refuses unbounded full-file output
artifact path traversal is rejected
limit above max is clamped or rejected
```

## Manual-required gaps

This workpack does not implement the raw logging writers or command wrappers. It exposes the query interface over data created by WP02/WP05.

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
WP07 on `codex/tracking-plan-full-continuation-a`

Touched files:
`package.json`
`packages/logging-domain/README.md`
`scripts/dev/agent-query.mjs`
`scripts/dev/codex-evidence.mjs`
`scripts/dev/lib/log-query-service.mjs`
`scripts/dev/mcp-logging-server.mjs`
`docs/plans/logging-domain-parity/CHECKLIST_INDEX.md`
`docs/plans/logging-domain-parity/NEXT_ACTIONS.md`
`docs/plans/logging-domain-parity/PLAN_STATE.md`
`docs/plans/logging-domain-parity/WORKPACK_INDEX.md`
`docs/plans/logging-domain-parity/workpacks/07-mcp-query-interface.md`

Validation commands and results:
`git grep -ni "mcp\|model context protocol\|modelcontextprotocol" -- .` completed; no reusable earlier parent MCP framework was found in scope before the WP07 logging MCP files.
`Get-ChildItem -Recurse -File | Where-Object { $_.Name -match 'mcp|modelcontext' -or $_.FullName -match 'mcp|modelcontext' }` completed; matches were plan docs, the new logging MCP server, vendor dependencies, prior proof, and unrelated build artifacts.
`npm run lint:architecture -- --files package.json scripts/dev/agent-query.mjs scripts/dev/codex-evidence.mjs scripts/dev/mcp-logging-server.mjs scripts/dev/lib/log-query-service.mjs` passed.
`npm run --silent mcp:logging -- --list-tools` passed.
`npm run --silent mcp:logging -- --smoke latest-failures` passed.
`npm run --silent mcp:logging -- --smoke run-diagnostics` passed.
`npm run --silent mcp:logging -- --smoke artifact-slice` passed.
`node --input-type=module -` MCP protocol and negative-path harness passed for `initialize`, `tools/list`, `get_latest_failures`, `get_run_diagnostics`, `get_artifact_slice`, unknown-scope errors, path-traversal rejection, bounded artifact slices, limit clamping, and DuckDB-to-NDJSON fallback.
`npm run --silent agent:query -- latest-failures` passed.
`npm run --silent agent:query -- stats` passed.
`npm run --silent codex:evidence -- latest-failures` passed.
`$env:LEDGER_LANE='codex-a'; npm run --silent codex:evidence -- current-lane` passed.

Proof artifacts:
`output/logging-domain-parity-proof/07-mcp-query-interface/00-existing-mcp-audit.json`
`output/logging-domain-parity-proof/07-mcp-query-interface/01-mcp-tool-list.json`
`output/logging-domain-parity-proof/07-mcp-query-interface/02-mcp-latest-failures-smoke.json`
`output/logging-domain-parity-proof/07-mcp-query-interface/03-mcp-run-diagnostics-smoke.json`
`output/logging-domain-parity-proof/07-mcp-query-interface/04-mcp-artifact-slice-smoke.json`
`output/logging-domain-parity-proof/07-mcp-query-interface/05-cli-mcp-query-parity-proof.json`
`output/logging-domain-parity-proof/07-mcp-query-interface/16-validation-commands.log`

Product/runtime claims:
Parent local evidence now has a dedicated MCP server surface at `npm run mcp:logging` backed by the same shared query/data-access layer used by `agent:query` and `codex:evidence`.
The MCP server exposes bounded tools for latest failures, diagnostics, stats, source/context queries, flexible queries, and local artifact slices.
The shared query service prefers DuckDB for `parent-codex` evidence and falls back to indexed NDJSON when the DuckDB file is unavailable.
Package guidance now documents MCP-first query usage for Codex/local agents with CLI fallback when MCP wiring is unavailable.

Known gaps/manual-required states:
The proof shows no reusable earlier parent MCP framework in scope for logging; this workpack proves the logging MCP surface only, not broader repo MCP standardization outside this plan.
Structured non-`parent-codex` source/context queries still depend on populated `output/logging-domain/test-logs/<scope>` or `output/logging-domain/app-logs/<scope>` data; empty scopes correctly return a clear error.
WP08 still owns broader source/context instrumentation adoption, and WP10/WP06 still own proof-trace and enforcement coverage.
