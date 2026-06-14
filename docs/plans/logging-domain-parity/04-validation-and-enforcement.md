# Validation and Enforcement

## Purpose

Make logging-domain parity enforceable.

Codex must not treat the parity docs as optional guidance. The repository must fail validation when the required local logging pipeline is missing, dead, or bypassed.

## Required validation scripts

Create:

```text
scripts/check-logging-domain-parity.mjs
scripts/check-local-evidence-wrapper.mjs
scripts/check-dev-log-routing.mjs
scripts/check-logging-exports.mjs
```

Add root scripts:

```json
{
  "lint:logging-parity": "node scripts/check-logging-domain-parity.mjs",
  "lint:local-evidence": "node scripts/check-local-evidence-wrapper.mjs",
  "lint:dev-log-routing": "node scripts/check-dev-log-routing.mjs",
  "lint:logging-exports": "node scripts/check-logging-exports.mjs"
}
```

Then include these in an existing validation chain.

Preferred:

```json
{
  "lint:schema-boundaries": "npm run lint:logging-parity && npm run lint:local-evidence && npm run lint:dev-log-routing && npm run lint:logging-exports && <existing checks>"
}
```

If ordering is risky, add a new root check first:

```json
{
  "validate:logging": "npm run lint:logging-parity && npm run lint:local-evidence && npm run lint:dev-log-routing && npm run lint:logging-exports"
}
```

and add it to `validate` after build dependencies are available.

## `check-logging-domain-parity.mjs`

This check verifies package shape.

Required checks:

```text
1. packages/logging-domain/package.json has bridge/db/query scripts.
2. packages/logging-domain/src/test-log exists.
3. packages/logging-domain/src/transport exists.
4. packages/logging-domain/src/app-log exists or is explicitly deferred with a TODO marker in this plan.
5. packages/logging-domain/scripts/log-bridge.ts exists.
6. packages/logging-domain/scripts/rebuild-db-from-ndjson.ts exists.
7. packages/logging-domain/scripts/query-test-logs.ts exists.
8. package exports expose test-log and transport modules.
9. existing production contract exports still exist.
10. generic parent logging code does not hardcode Cloudflare as default scope.
```

Failure message must be direct:

```text
logging-domain parity failed: missing packages/logging-domain/scripts/log-bridge.ts
```

No soft warnings for required files.

## `check-local-evidence-wrapper.mjs`

This check verifies local validation evidence tooling.

Required checks:

```text
1. root package.json has agent:run.
2. root package.json has agent:query.
3. root package.json has codex:evidence.
4. scripts/dev/agent-run.mjs exists.
5. scripts/dev/agent-query.mjs exists.
6. scripts/dev/codex-evidence.mjs exists.
7. scripts/dev/lib/agent-diagnostic-parsers.mjs exists.
8. scripts/dev/lib/agent-artifacts.mjs exists.
9. docs or AGENTS reference the wrapper usage rule.
```

Required rule text, or equivalent:

```text
Use npm run agent:run -- <command> for validation where possible.
Use npm run agent:query or npm run codex:evidence for failure evidence.
Do not paste full raw logs into agent context by default.
```

## `check-dev-log-routing.mjs`

This check prevents the current portal/agent split-brain from returning.

Required checks:

```text
1. If DevLogEndpoint.Write is exported, a receiver exists or portal dev logger uses bridge transport.
2. apps/portal/src/dev-logger.ts must not post to an unimplemented endpoint.
3. crates/agent-service must not own a full standalone dev log writer after logging-core migration.
4. crates/agent-service depends on ocentra-parent-logging-core after the Rust crate is added.
5. /api/dev/log-snapshot is not documented as the primary logging store.
```

Allowed temporary state:

```text
A compatibility wrapper may remain in crates/agent-service/src/dev_log.rs only if it delegates to ocentra-parent-logging-core.
```

Disallowed temporary state:

```text
portal posts dev logs to an endpoint with no implementation
agent writes local files through ad hoc code while logging-core exists
```

## `check-logging-exports.mjs`

This check verifies package exports.

Required parent logging-domain exports:

```text
./contracts
./test-log/types
./test-log/bridgeConvert
./test-log/ndjsonPaths
./test-log/ndjsonWriter
./test-log/testLogDuckDb
./test-log/logsTree
./test-log/wipeNdjsonScope
./transport/bridgeLogPayload
./transport/bridgeTransport
./app-log/createAppLogStorage
```

Existing production-proof exports must remain available.

The check must fail if any existing export is removed without an explicit migration note in this plan directory.

## Rust validation

When `crates/logging-core` is added, update root validation to include:

```text
cargo test -p ocentra-parent-logging-core
cargo clippy -p ocentra-parent-logging-core --all-targets -- -D warnings
```

Also add logging-core to workspace-level checks.

Required Rust tests:

```text
- NDJSON writer writes one JSON object per line
- writer creates parent directories
- artifact writer writes content and metadata
- artifact writer computes sha256 and line count
- event structs serialize in camelCase where required
- agent-service startup logging delegates to logging-core
```

## TypeScript validation

Required TypeScript tests:

```text
- bridge accepts log payload and writes NDJSON
- query script can read failed run from DuckDB
- ingest script supports incremental mode
- generic scope defaults are parent scopes, not Cloudflare
- existing contracts still parse existing read models
- Rust fixture parses through TypeScript DevLogEntrySchema
```

## End-to-end smoke test

Add a smoke script:

```text
scripts/test/logging-local-evidence-smoke.mjs
```

It must:

```text
1. start or use the logging-domain local bridge if needed
2. run a controlled failing validation through agent:run
3. confirm stdout/stderr files exist
4. confirm NDJSON rows exist
5. ingest into DuckDB
6. query latest-failures
7. assert compact evidence output contains run_id and at least one diagnostic
```

Add root script:

```json
{
  "test:logging-evidence": "node scripts/test/logging-local-evidence-smoke.mjs"
}
```

Add to `validate` after implementation is stable.

## Agent instruction enforcement

Update root agent guidance to include:

```text
Logging and validation evidence:
- Use npm run agent:run -- <command> for repository validation where possible.
- Use npm run agent:query or npm run codex:evidence for compact failure evidence.
- Do not paste full raw logs into context unless explicitly needed.
- If a wrapper is missing, add/fix the wrapper rather than bypassing the evidence pipeline permanently.
```

Validation should check that this text or equivalent exists.

## Implementation order

Codex must implement in this order:

```text
1. TypeScript package parity files and exports.
2. Bridge + NDJSON + DuckDB scripts.
3. Rust logging-core crate.
4. Agent-service migration to logging-core.
5. Portal dev-log route fix or bridge migration.
6. Local evidence wrapper scripts.
7. Query/evidence commands.
8. Validation scripts.
9. Root validation integration.
10. Documentation updates.
```

Do not start with root validation integration before the target files exist, or the branch becomes noisy and hard to repair.

## Acceptance criteria

The full parity round is complete only when:

```text
npm run build --workspace @ocentra-parent/logging-domain passes
npm run test --workspace @ocentra-parent/logging-domain passes
cargo test -p ocentra-parent-logging-core passes
cargo test -p ocentra-parent-agent-service passes
npm run validate:logging passes
npm run test:logging-evidence passes
npm run agent:run -- <controlled passing command> records a passed run
npm run agent:run -- <controlled failing command> records a failed run
npm run agent:query -- latest-failures returns the failed run
npm run codex:evidence -- latest-failures returns compact evidence
portal dev logs are received or intentionally routed through bridge
agent-service dev logs go through logging-core
production contract exports remain intact
```

## Explicit failure cases

Reject the implementation if any are true:

```text
- parent logging-domain only adds more proof/read-model contracts
- no bridge script exists
- no DuckDB query path exists
- generic logging defaults to Cloudflare scope
- portal dev log fetch has no receiver
- agent-service keeps a full ad hoc dev logger after logging-core exists
- Codex instructions say to inspect raw logs by default
- validation wrapper prints full stdout/stderr by default
- existing production contract exports are broken
```
