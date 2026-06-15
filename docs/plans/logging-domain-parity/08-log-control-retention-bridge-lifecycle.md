# Logging Controls, Retention, and Bridge Lifecycle

## Purpose

Logging parity is not only logger calls and query tools. The system must also control when logs are recorded, how much is retained, when fresh runs wipe old evidence, and when bridge/tunnel transport is required.

Without this layer, the local DB becomes noisy and agents waste tokens reading stale evidence.

## Reference behavior from games

Games had these important patterns:

```text
LogDecisionProvider controls shouldLog / shouldLogToConsole / shouldStoreLog.
Cloudflare log config keeps error/warn always on, stores lower levels mainly in dev/test or request-selected debug modules.
Test helper preflight checks the log bridge before running.
Test helper wipes the selected NDJSON scope before fresh runs.
Bridge /__run_started__ records current run info and can wipe all or scoped test files.
Reporter rejects stale bridge run info to avoid writing raw vitest runs into old run IDs.
Tunnel URL exists for cross-process/wrangler/cloudflare-style routing; default local bridge is still localhost.
App-log retention keeps only the most recent local sessions.
```

Parent must preserve these ideas while adapting names and scopes.

## Required log decision model

Add a parent log decision provider equivalent.

Required decisions:

```text
shouldLog(source, level, requestDebugSources?)
shouldLogToConsole(source, level, requestDebugSources?)
shouldStoreLog(source, level, requestDebugSources?)
isDevOrTestEnvironment()
```

Required policy:

```text
error and warn are always stored
error and warn are normally visible in console during local/dev/test
info/debug/log are stored only when local/dev/test, explicitly enabled, or source-selected
console output can be stricter than storage
production/product-safe runtime defaults must avoid noisy low-level logging
local validation mode can store more but should still use retention/wipe
```

## Controls required

Support these control surfaces:

```text
whole logging off/on for low-level logs
minimum level
per-source or per-file debug enable list
per-run debug enable list
store-vs-console distinction
local validation mode override
```

Suggested environment variables:

```text
OCENTRA_PARENT_LOG_LEVEL=error|warn|info|debug|trace
OCENTRA_PARENT_LOG_ENABLED=true|false
OCENTRA_PARENT_LOG_CONSOLE=true|false
OCENTRA_PARENT_LOG_STORE=true|false
OCENTRA_PARENT_DEBUG_SOURCES=agent-service,portal,codex
OCENTRA_PARENT_DEBUG_FILES=apps/portal/src/dev-logger.ts,crates/agent-service/src/dev_log.rs
OCENTRA_PARENT_TEST_MODE=true|false
```

Do not let file/source debug selection bypass redaction rules.

## Fresh-run wipe and retention

Implement explicit fresh-run behavior.

Required modes:

```text
fresh run: wipe selected scope/run before execution
append run: preserve existing rows and append new run
retention cleanup: keep only latest N local sessions/files
manual clean: explicit command to clear a scope
```

Required commands:

```text
npm run logs:wipe -- --scope=parent-codex
npm run logs:wipe -- --scope=parent-test --run-type=single-pool --suite=unit
npm run logs:retention -- --scope=parent-codex --keep=10
```

`agent:run` default should be fresh per run_id for command artifacts while retaining configurable recent history. It should never append a new run into an old run_id.

`test` flows may wipe selected test scope before run, matching games.

## Bridge lifecycle

Bridge endpoints should include:

```text
GET /__health__
POST /__logs__
POST /__flush__
GET /__flush__?runId=...
POST /__run_started__
GET /__run_info__
POST /__reporter__
```

`/__run_started__` should:

```text
record current run_id/run_type/suite/scope
wipe selected scope if requested
wipe received-temp diagnostics for the new run
reject or warn on stale run info
```

Reporter/test harness should reject stale run info so direct raw test commands do not write into a previous run.

## Tunnel / bridge decision

Default parent local bridge should be localhost:

```text
http://127.0.0.1:<port>
```

Use a tunnel only when required by runtime topology, such as:

```text
wrangler/miniflare/cloudflare worker process cannot reach host localhost directly
pooled test runner runs outside the process that owns the local bridge
mobile/emulator process must call host bridge via mapped address
remote worker or CI job must report to a developer-owned bridge endpoint
```

Do not make tunnel mandatory for normal local Node/Vitest/Rust/portal flows.

Required config:

```text
OCENTRA_PARENT_LOG_BRIDGE_URL
OCENTRA_PARENT_LOG_BRIDGE_MODE=local|tunnel|disabled
OCENTRA_PARENT_LOG_BRIDGE_SKIP_HEALTH=true|false
```

Health check must fail loudly unless explicitly skipped.

## MCP relation

MCP reads indexed evidence. It should not be the write path.

Correct:

```text
logger/bridge/agent-run writes NDJSON and artifacts
ingest updates DuckDB
MCP queries DuckDB or bounded NDJSON fallback
```

Incorrect:

```text
source files send logs to MCP directly
MCP server owns raw logging storage
MCP returns entire artifact files by default
```

## Acceptance criteria

```text
log decision provider exists
error/warn are always stored
info/debug/log can be source/file/run-enabled
console and storage decisions are separate
fresh-run wipe exists for selected scopes
retention cleanup exists for local sessions
bridge run-start lifecycle exists
stale run info is rejected or warned
local bridge is default
tunnel is optional and documented by condition
MCP queries indexed data, not direct logger writes
```
