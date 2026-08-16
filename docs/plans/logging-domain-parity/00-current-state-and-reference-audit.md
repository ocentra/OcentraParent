# Logging Domain Parity — Current-State Audit and Reference Map

## Purpose

This document prevents another hand-waved logging-domain pass.

The reference implementation is:

```text
ocentra-games/packages/logging-domain
```

The target implementation is:

```text
OcentraParent/packages/logging-domain
OcentraParent/crates/logging-core
OcentraParent/scripts/dev/*
```

The goal is **not** to add more metadata-only proof contracts. The goal is to build a real local developer/agent logging pipeline equivalent in capability to the mature games logging system, adapted for OcentraParent's Rust-heavy runtime and TypeScript portal/cloudflare surfaces.

## Non-negotiable interpretation

`logging-domain` has two separate concerns that must not be collapsed:

1. **Product/runtime safe logging**: redaction-safe operational contracts, privacy/legal state, support status, custody boundaries.
2. **Local developer/agent observability**: local-only logs, command runs, test/validation failures, NDJSON, DuckDB, query tools, Codex evidence packets.

The current parent package is mostly concern 1. This parity plan adds concern 2 without weakening concern 1.

## Current reference state: ocentra-games

The games package is a complete logging domain. It includes:

```text
packages/logging-domain/src/core/
  BaseLogger
  MainAppLogger
  CloudflareLogger
  adapters
  stack parsing
  registration
  batching
  redaction

packages/logging-domain/src/transport/
  bridge payload types
  bridge transport
  transport interface

packages/logging-domain/src/test-log/
  NDJSON types
  NDJSON writer
  logs tree
  DuckDB test store
  ingest manifest
  summary formatting
  per-file formatting
  wipe helpers

packages/logging-domain/src/app-log/
  local app NDJSON writer
  app DuckDB store
  app ingest helpers
  createAppLogStorage

packages/logging-domain/scripts/
  log-bridge.ts
  ensure-db.ts
  rebuild-db-from-ndjson.ts
  query-test-logs.ts
  prepare-ndjson-logs.ts
  view-ndjson.ts
```

The games package also has npm scripts for:

```text
bridge
logs:prepare
db:ensure
db:rebuild
db:ingest
test:query
view:ndjson
```

These scripts make the package operational. They are not documentation-only.

## Current target state: OcentraParent

The current parent logging-domain package is not parity.

Observed current shape:

```text
packages/logging-domain/
  src/contracts.ts
  many support/privacy/notification/custody proof contracts
  read models
  package exports
  build/type-check/lint/test scripts only
```

Current parent package does not currently provide equivalent:

```text
- bridge server
- NDJSON writer/tree
- DuckDB store
- ingest manifest
- query CLI
- app-log storage
- test-log storage
- Codex/agent command diagnostics
```

This is not a judgment on the existing proof contracts. Those contracts are useful, but they are not the mature local observability system needed here.

## Current parent live usage

The parent logging-domain package is not dead code.

Known live usage:

```text
apps/portal/src/dev-logger.ts
  imports DevLogEndpoint, DevLogField, DevLogMessage, DevLogEntrySchema,
  LogLevel, LogSource from @ocentra-parent/logging-domain/contracts

apps/portal/src/main.ts
apps/portal/src/transport.ts
  writePortalDevLog(...) on portal start, command send, and websocket event receive

packages/agent-protocol-domain/src/contracts.ts
  imports AgentLogSnapshotSchema, LogFieldsSchema, LogLevelSchema and related
  types from @ocentra-parent/logging-domain/contracts

root package.json
  build:contracts includes @ocentra-parent/logging-domain
  test:contract includes @ocentra-parent/logging-domain
```

Therefore do not delete or gut the existing package. Extend it and separate concerns.

## Current parent broken/misaligned areas

### 1. Portal dev log endpoint is likely not implemented where portal expects it

Parent TypeScript contracts define:

```text
DevLogEndpoint.Write = /__ocentra-parent-dev-log
```

The portal posts dev log entries to that endpoint.

The inspected Rust agent-service router exposes:

```text
/health
/api/browser/intervention/page
/api/dev/log-snapshot
/api/dev/ws
```

No matching POST `/__ocentra-parent-dev-log` route was found in the inspected router path.

Required outcome:

```text
Either implement this endpoint deliberately, or replace portal dev logging with the new bridge/NDJSON path. Do not leave fetch calls silently disappearing.
```

### 2. Agent service has ad hoc file logging outside a coherent pipeline

Current Rust agent dev logging writes directly to `.logs/dev/agent-service-YYYY-MM-DD.ndjson` using local code in `crates/agent-service/src/dev_log.rs`.

That must be moved behind a reusable Rust logging crate:

```text
crates/logging-core
```

The agent service should import and use the crate. It should not own its own one-off logger.

### 3. Dev log snapshot is synthetic, not an index over actual logs

Current `build_dev_log_snapshot()` returns a generated one-entry snapshot. It does not read the agent dev log file or a DuckDB/NDJSON store.

Required outcome:

```text
Keep health/snapshot endpoints if useful, but do not confuse them with the local logging store. Real debugging evidence must live in NDJSON/DuckDB and query tools.
```

### 4. TypeScript and Rust log schema are duplicated

Parent TypeScript defines log schemas in `packages/logging-domain/src/contracts.ts`.

Rust defines equivalent types in `crates/agent-protocol/src/logging.rs`.

Required outcome:

```text
Add cross-language parity tests/golden fixtures or a single-source generation strategy. At minimum, add fixtures that prove TS and Rust serialize/parse the same event shapes.
```

### 5. Production custody language is over-applied to local dev logging

Parent README and contracts emphasize metadata-only proofs, redaction, support workflows, privacy/legal disclosure, provider custody, etc.

That is valid for production/product logs.

It is not the same as local developer observability.

Required outcome:

```text
Add explicit local-dev-observability mode. It is local-only, workspace-owned, and intended for development/test/agent diagnostics. It must not imply Ocentra-hosted custody or production collection.
```

## Reference map: copy/adapt from games

Use games as the reference implementation, but adapt names and scopes.

### TypeScript package parity

Add/adapt these modules under `packages/logging-domain/src/`:

```text
test-log/types.ts
test-log/bridgeConvert.ts
test-log/ndjsonPaths.ts
test-log/ndjsonBrands.ts
test-log/ndjsonLogFileWriter.ts
test-log/logsTree.ts
test-log/wipeNdjsonScope.ts
test-log/ndjsonWriter.ts
test-log/testLogDuckDb.ts
test-log/ingestManifest.ts
test-log/formatRunSummary.ts
test-log/formatPerFileBlock.ts

transport/bridgeLogPayload.ts
transport/bridgeTransport.ts
transport/logTransport.ts

app-log/duckDbHelpers.ts
app-log/appLogDuckDb.ts
app-log/appNdjsonWriter.ts
app-log/appLogIngest.ts
app-log/createAppLogStorage.ts
```

Add/adapt these scripts under `packages/logging-domain/scripts/`:

```text
log-bridge.ts
ensure-db.ts
rebuild-db-from-ndjson.ts
query-test-logs.ts
prepare-ndjson-logs.ts
view-ndjson.ts
```

### Do not blindly copy games hardcodes

The games implementation has Cloudflare-specific defaults and some hardcoded Cloudflare assumptions. Parent must be generic.

Parent scopes must include at least:

```text
parent-agent
parent-portal
parent-cloudflare
parent-codex
parent-test
```

Allowed additional scopes:

```text
parent-desktop
parent-mobile
parent-browser
parent-network
parent-screen
```

No generic parent writer may hardcode `cloudflare` as the default consumer.

## Required parent package scripts

Add scripts equivalent to:

```json
{
  "bridge": "npx tsx scripts/log-bridge.ts",
  "db:ensure": "npx tsx scripts/ensure-db.ts",
  "db:rebuild": "npx tsx scripts/rebuild-db-from-ndjson.ts",
  "db:ingest": "npx tsx scripts/rebuild-db-from-ndjson.ts --no-delete",
  "logs:prepare": "npx tsx scripts/prepare-ndjson-logs.ts",
  "test:query": "npx tsx scripts/query-test-logs.ts",
  "view:ndjson": "npx tsx scripts/view-ndjson.ts"
}
```

Then add root-level wrappers as needed:

```json
{
  "logs:bridge": "npm run bridge --workspace @ocentra-parent/logging-domain",
  "logs:query": "npm run test:query --workspace @ocentra-parent/logging-domain",
  "logs:ingest": "npm run db:ingest --workspace @ocentra-parent/logging-domain"
}
```

## Required acceptance criteria

This phase is complete only when all are true:

```text
1. Parent logging-domain still exports existing production proof contracts.
2. Parent logging-domain also exports test-log, transport, and app-log modules.
3. Parent package has bridge/db/query scripts.
4. Parent bridge can receive logs and write NDJSON.
5. Parent DuckDB can ingest NDJSON and query failed/log/error/search/stats views.
6. Parent does not hardcode generic logging to Cloudflare scope.
7. Existing portal dev logging is either routed into the new bridge/local store or intentionally replaced.
8. Existing Rust agent dev logging is moved toward crates/logging-core, not left as an isolated one-off.
9. Validation catches missing parity.
```

## Explicit non-goals for this doc

Do not implement Codex command wrappers in this doc. That belongs to `03-codex-agent-diagnostics.md`.

Do not implement the Rust crate in this doc. That belongs to `02-rust-logging-core-crate.md`.

Do not weaken production privacy/custody contracts. Preserve them and separate them from local dev observability.
