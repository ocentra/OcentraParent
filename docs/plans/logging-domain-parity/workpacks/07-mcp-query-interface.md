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

- [ ] Existing parent MCP framework audited.
- [ ] Existing MCP reused/upgraded or absence recorded.
- [ ] MCP source/query service designed around existing DuckDB/NDJSON query path.
- [ ] MCP server script added or existing server extended.
- [ ] `mcp:logging` package/root script added.
- [ ] `get_errors` implemented.
- [ ] `get_recent_logs` implemented.
- [ ] `get_logs_by_source` implemented.
- [ ] `get_logs_by_context` implemented.
- [ ] `query_logs` implemented.
- [ ] `get_log_stats` implemented.
- [ ] `get_latest_failures` implemented.
- [ ] `get_run_diagnostics` implemented.
- [ ] `get_artifact_slice` implemented with bounded output.
- [ ] MCP and CLI share query/data-access code.
- [ ] MCP smoke tests pass.
- [ ] Agent guidance documents MCP-first, CLI-fallback behavior.
- [ ] Proof root and workpack completion section filled.

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

## Current audit note

Focused checks observed in this checkout:

```text
- npm run mcp:logging -- --list-tools -> pass
- npm run mcp:logging -- --smoke run-diagnostics -> pass
- npm run mcp:logging -- --smoke artifact-slice -> pass
- cmd /c npx vitest run packages/logging-domain/tests/integration/mcp-query-interface.test.ts -> pass
```

What this actually proves:

```text
- the MCP server starts locally
- the tool list is present
- run-diagnostics and bounded artifact-slice paths work against local evidence
- the package has at least one dedicated MCP integration test
```

What this does not yet prove:

```text
- output/logging-domain-parity-proof/07-mcp-query-interface/ exists in this checkout
- test-results/logging-domain-parity-mcp/ exists in this checkout
- the standalone proof-trace smoke path works from a clean workspace
- negative coverage for unknown scope, path traversal, stale-ingest recovery, and limit-clamp behavior is represented by the named proof inventory
```

Required next step for truthful closeout:

```text
- recreate the missing proof root or remove the completion claim
- either make the proof-trace smoke self-seeding or stop implying that WP07 already proves it as a standalone MCP smoke
```
