# MCP Query Interface for Logging Domain Parity

## Purpose

The logging-domain parity plan must include an MCP interface, not only CLI commands.

The CLI commands are still required, but MCP is the Codex-native path:

```text
Codex asks for latest failures / logs / diagnostics
  -> MCP tool queries DuckDB or indexed NDJSON
  -> returns compact structured rows
  -> Codex fixes from precise evidence
```

This keeps the original intent intact: compact, high-density, deterministic evidence instead of terminal dumps or lossy LLM log summaries.

## Reference behavior from games

The games logging-domain README says the dev/test pipeline uses NDJSON plus DuckDB for centralized ingestion and MCP querying. It also says DuckDB files and ingest manifests allow MCP tools and CLI scripts to perform high-performance queries without scanning raw files every time.

Games agent rules expose project MCP tools such as:

```text
get_errors
get_recent_logs
get_logs_by_source
get_logs_by_context
query_logs
```

Parent should implement the same class of interface for logging-domain parity, adapted to parent scopes and local validation evidence.

## Required parent MCP server

Add a local MCP server for parent logging evidence.

Suggested location:

```text
packages/logging-domain/src/mcp/
packages/logging-domain/scripts/mcp-logging-server.ts
```

Alternative acceptable location:

```text
scripts/dev/mcp-logging-server.mjs
```

Preferred package script:

```json
{
  "mcp:logging": "npx tsx scripts/mcp-logging-server.ts"
}
```

Preferred root script:

```json
{
  "mcp:logging": "npm run mcp:logging --workspace @ocentra-parent/logging-domain"
}
```

## Required MCP tools

Minimum tool set:

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

### `get_errors`

Purpose:

```text
Return recent error-level logs for a scope.
```

Input:

```text
scope?: parent-agent | parent-portal | parent-cloudflare | parent-codex | parent-test
since?: ISO timestamp or duration
limit?: number, default 50, max 200
```

### `get_recent_logs`

Purpose:

```text
Return recent logs for general debugging.
```

Input:

```text
scope?
level?
limit?
since?
```

### `get_logs_by_source`

Purpose:

```text
Filter by source, e.g. agent-service, portal, codex, validation.
```

Input:

```text
source: string
scope?
limit?
```

### `get_logs_by_context`

Purpose:

```text
Filter by module/context name.
```

Input:

```text
context: string
scope?
limit?
```

### `query_logs`

Purpose:

```text
Flexible log query with scope, level, source, context, run_id, command_id, text search, and time range.
```

Input:

```text
scope?
level?
source?
context?
runId?
commandId?
contains?
from?
to?
limit?
```

### `get_log_stats`

Purpose:

```text
Return counts by level, source, context, run status, and diagnostic kind.
```

Input:

```text
scope?
from?
to?
```

### `get_latest_failures`

Purpose:

```text
Return compact failed validation runs from agent_runs and agent_diagnostics.
```

Input:

```text
scope?: default parent-codex
limit?: default 10
```

### `get_run_diagnostics`

Purpose:

```text
Return diagnostics for one run_id without raw log spam.
```

Input:

```text
runId: string
scope?: default parent-codex
includeArtifactRefs?: boolean
limit?: default 100
```

### `get_artifact_slice`

Purpose:

```text
Return a bounded line slice from a local artifact.
```

Input:

```text
artifactId?: string
path?: string
startLine?: number
endLine?: number
maxLines?: number, default 80, max 200
```

Hard rule:

```text
Never return whole stdout/stderr by default.
```

## Safety and context-size rules

Every tool must enforce:

```text
limit defaults
max limits
bounded artifact slices
compact rows by default
no raw full logs unless explicitly requested and bounded
local paths only
no network upload
```

Default result shape should be compact:

```text
id
level/status
source/kind
context/signature
file:line:column
message summary
run_id
command_id
artifact ref
```

## DuckDB/NDJSON resolution

MCP must query DuckDB first when available.

Fallback order:

```text
1. DuckDB for selected scope
2. incremental ingest then DuckDB retry
3. bounded NDJSON scan if DuckDB is unavailable
4. clear error with next command if no data exists
```

Do not silently return empty results when ingestion is stale and NDJSON exists.

## Integration with CLI

MCP and CLI must share query code.

Do not implement two different query engines.

Preferred shape:

```text
src/query/logQueryService.ts
scripts/query-test-logs.ts       -> uses logQueryService
scripts/mcp-logging-server.ts    -> uses logQueryService
scripts/dev/agent-query.mjs      -> uses same query service or thin wrapper
```

## Agent guidance

Root or plan agent guidance must tell Codex:

```text
When debugging logs or validation failures, prefer MCP logging tools when available.
Use CLI query commands when MCP is unavailable.
Do not paste raw terminal logs into context by default.
Use artifact slices only when compact diagnostics are insufficient.
```

## Acceptance criteria

```text
MCP server starts locally.
MCP tools query DuckDB for parent scopes.
get_latest_failures returns compact failed validation rows.
get_run_diagnostics returns diagnostics for a run_id.
get_artifact_slice returns bounded artifact lines.
MCP and CLI query paths share the same query service or data access layer.
Limits prevent context spam.
Agent guidance documents MCP-first, CLI-fallback behavior.
```
