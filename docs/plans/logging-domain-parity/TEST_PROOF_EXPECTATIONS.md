<!-- agent-capsule -->

> Agent Capsule
> Plan: `logging-domain-parity`
> Doc: `Logging Domain Parity Test Proof Expectations`
> Kind: command/test selector.
> Read when: a workpack asks which commands or proof artifacts are expected.
> Stop rule: Run focused commands first; do not jump to full validate unless required.
> Proves: command expectations only.
> Does not prove: implementation completion without matching artifacts.

<!-- /agent-capsule -->

# Logging Domain Parity Test Proof Expectations

## General rule

Use focused commands first. Broader validation is allowed only after focused commands pass or a precise blocker is recorded.

Run through `npm run agent:run --` when collecting proof if the wrapper is available.

## Command ownership notes

- `packages/logging-domain` owns TypeScript local logging helpers, bridge, NDJSON, DuckDB/query, log-control/wipe/retention, MCP query, and proof-trace helper surfaces.
- `crates/logging-core` owns Rust NDJSON/artifact/dev-log/diagnostic/redaction/source/context helper behavior.
- `scripts/dev` owns local agent wrapper/query/evidence/MCP/proof-trace entrypoints.
- `apps/portal` is a selected dev-log producer/consumer only when the workpack names that path.
- `crates/agent-service` is a selected Rust producer/consumer only when the workpack names that path.
- Product telemetry, production support workflows, Cloudflare infra logging, and repo-wide instrumentation are not owned by this plan unless a selected handoff explicitly names them.

## Logging E2E meaning

Do not use one proof family to claim the whole logging path. For this plan, E2E has separate meanings:

```text
TypeScript package E2E: logging-domain package exports -> bridge/NDJSON/DuckDB/query tests -> local package proof only.
Rust logging-core E2E: logging-core event/artifact/dev-log/redaction helpers -> cargo tests -> Rust helper proof only.
bridge/NDJSON E2E: producer row -> bridge conversion -> NDJSON file -> local artifact proof.
DuckDB/query E2E: NDJSON ingest -> DuckDB/query service -> bounded structured result.
agent wrapper E2E: agent:run controlled pass/fail -> artifact row -> latest failure/evidence query.
MCP query E2E: MCP tool -> shared query service -> bounded result and CLI parity.
instrumentation E2E: selected source path -> source/context fields -> storage/query proof -> no repo-wide adoption claim.
retention/wipe E2E: selected scope/run/file -> wipe/retention action -> stale data absent and current rows preserved.
validation/enforcement E2E: checker script -> positive/negative fixture -> explicit pass/fail/blocker.
proof-trace E2E: enable selected proof trace -> exercise one path -> ingest/query ordered trace -> disable/cleanup.
```

A workpack can be complete for one tier while other tiers remain open. Record the non-claim instead of broad DONE.

## Structured harness logging expectations

Product/runtime-safe logging:

```text
redact credential material, private payloads, raw command dumps, raw screenshots, full browser URLs, customer data, support-private diagnostics, and sensitive environment data unless the selected proof explicitly allows a bounded field
log scope, run_id, command_id, proof_id, correlation_id, log source, log context, artifact ref, bridge state, NDJSON state, DuckDB state, MCP tool, retention state, validation state, and no-claim boundary when safe
separate local development evidence, product-safe logging, proof-trace mode, MCP query, wrapper command capture, and production telemetry states
never treat raw logs, proof roots, or query smoke output as checklist/workpack closure without matching commands and no-claim boundaries
```

Local Codex/MCP/debug harness logging:

```text
prefer npm run agent:run -- <command> when available
store raw stdout/stderr by artifact pointer instead of pasting terminal walls into plan docs
write compact command summaries into 16-validation-commands.log
include run id, command id, workpack id, owner module, scope, exit code, result, artifact pointer, diagnostics summary, blocker note, and no-claim note when available
if the wrapper is unavailable, write wrapper: unavailable and keep the same compact command-log shape
```

## WP01 Current State and Reference Audit

Expected commands:

```bash
node -e "console.log('audit-only: no source validation required')"
git grep -ni "mcp\|model context protocol\|modelcontextprotocol" -- .
find . -iname '*mcp*' -o -iname '*modelcontext*'
```

Expected proof:

```text
reference file map
parent current state map
live usage map
gap summary
existing MCP audit result
```

## WP02 TypeScript Logging Package Parity

Expected focused commands:

```bash
npm run build --workspace @ocentra-parent/logging-domain
npm run test --workspace @ocentra-parent/logging-domain
npm run test:query --workspace @ocentra-parent/logging-domain -- stats --scope=parent-test
```

After those focused commands pass, the dedicated retained-artifact command is:

```bash
npm run proof:logging-domain-wp02 -- --base=<reviewed-base-ref>
```

The runner writes only the five WP02 artifacts routed by `PROOF_INDEX.md` and
records explicit failed, blocked, and local-package-only no-claim states. Its
presence is source/test evidence only; do not check the proof row until the
command has actually run successfully against the reviewed base.

Expected TypeScript coverage:

```text
bridge transport serializes and sends valid payloads
bridge handles invalid payloads without crashing
NDJSON writer writes one object per line
test-log ingest can rebuild and incrementally ingest
app-log retention deletes old local sessions as configured
query script returns stats/failure/search output from DuckDB
existing production-safe contract exports still parse existing fixtures/read models
```

## WP03 Parent Logging Architecture and Routing

Expected focused commands:

```bash
npm run build --workspace @ocentra-parent/logging-domain
npm run test --workspace @ocentra-parent/portal
cargo test -p ocentra-parent-agent-service dev_log
```

Expected routing coverage:

```text
portal dev-log path has an implemented receiver or bridge route
missing endpoint behavior is explicit and tested
agent-service route into logging-core is mapped or implemented
snapshot endpoint is not documented as the primary log store
```

## WP09 Log Control, Retention, and Bridge Lifecycle

Expected focused commands:

```bash
npm run build --workspace @ocentra-parent/logging-domain
npm run test --workspace @ocentra-parent/logging-domain -- log-decision
npm run test --workspace @ocentra-parent/logging-domain -- wipe
npm run test --workspace @ocentra-parent/logging-domain -- retention
npm run test --workspace @ocentra-parent/logging-domain -- bridge
```

If scripts are implemented:

```bash
npm run logs:wipe -- --scope=parent-test
npm run logs:retention -- --scope=parent-codex --keep=10
```

Expected lifecycle coverage:

```text
error/warn always stored
info/debug/log controlled by environment/source/file/run selection
console and storage decisions can differ
fresh-run wipe deletes selected scope/run/suite/file only
retention keeps configured number of recent sessions/files
bridge health check fails loudly when missing unless explicitly skipped
run-start records run metadata and can wipe selected scope
stale run info is rejected or warned
local bridge is default
tunnel mode is optional and condition-gated
```

## WP04 Rust Logging Core Crate

Expected focused commands:

```bash
cargo check -p ocentra-parent-logging-core
cargo test -p ocentra-parent-logging-core
cargo clippy -p ocentra-parent-logging-core --all-targets -- -D warnings
cargo test -p ocentra-parent-agent-service dev_log
npm run test --workspace @ocentra-parent/logging-domain -- dev-log-fixture
```

Expected Rust coverage:

```text
new crate compiles with cargo check
new crate unit tests pass
agent-service direct consumer tests pass
NDJSON writer writes one JSON object per line
artifact writer writes file metadata, sha256, byte length, and line count
sensitive fields are redacted by default helpers
```

## WP05 Local Validation Evidence

Expected focused commands:

```bash
npm run agent:run -- node -e "process.exit(0)"
npm run agent:run -- node -e "process.exit(2)"
npm run agent:query -- latest-failures
npm run codex:evidence -- latest-failures
```

Expected local smoke proof:

```text
controlled passing command is recorded as passed
controlled failing command is recorded as failed
logs are written locally
NDJSON is ingested
query command returns the latest failed run
codex evidence command returns a compact evidence packet
```

## WP07 MCP Query Interface

Expected focused commands depend on the final MCP framework, but must include equivalents of:

```bash
npm run mcp:logging -- --list-tools
npm run mcp:logging -- --smoke latest-failures
npm run mcp:logging -- --smoke run-diagnostics
npm run mcp:logging -- --smoke artifact-slice
npm run test --workspace @ocentra-parent/logging-domain -- mcp
```

Expected MCP coverage:

```text
existing parent MCP audit completed
MCP server starts locally
MCP tools query DuckDB for parent scopes
latest failures query returns compact rows
run diagnostics query returns diagnostics without raw log spam
bounded file slice query returns limited local file lines
MCP and CLI share query/data-access code
limits prevent context spam
```

## WP08 Logger Instrumentation and Adoption

Expected focused commands:

```bash
npm run build --workspace @ocentra-parent/logging-domain
npm run test --workspace @ocentra-parent/logging-domain -- logger
cargo test -p ocentra-parent-logging-core
cargo test -p ocentra-parent-agent-service dev_log
npm run validate:logging
```

If MCP exists:

```bash
npm run mcp:logging -- --smoke source-context
```

Expected instrumentation coverage:

```text
TypeScript logger pattern preserves source/context fields
Rust logging-core pattern preserves source/context fields
portal or script path emits queryable structured rows
agent-service path emits queryable or fixture-proven structured rows
checks reject new raw console logging in touched logging surfaces
checks reject ad hoc JSON writers outside logging-domain/logging-core
```

## WP10 Proof Trace Pipeline

Expected focused commands depend on final test ownership, but must include equivalents of:

```bash
npm run test --workspace @ocentra-parent/logging-domain -- proof-trace
npm run test:e2e --workspace @ocentra-parent/portal -- proof-trace
npm run logs:query -- proof-trace <proof-id>
npm run mcp:logging -- --smoke proof-trace
```

Expected proof-trace coverage:

```text
proof mode can be enabled for one run/test
proof mode can be disabled after the run/test
proof rows include proof_id and correlation_id
ordered trace query matches expected steps
missing/out-of-order trace query reports gaps
Playwright or equivalent smoke proves one click-to-result path
proof artifact records matched/missing/out-of-order steps
retention/wipe cleans stale proof traces
```

## WP06 Validation and Enforcement

Expected focused commands:

```bash
npm run validate:logging
npm run test:logging-evidence
node scripts/check-logging-domain-parity.mjs
node scripts/check-local-evidence-wrapper.mjs
node scripts/check-dev-log-routing.mjs
node scripts/check-logging-exports.mjs
```

Expected negative checks:

```text
missing bridge script fails
missing required export fails
missing agent wrapper fails
portal dev-log route without receiver fails
invalid bridge payload is rejected without corrupting stored logs
proof trace mode cannot stay globally enabled after tests
```

Use temporary fixtures or script-internal fixtures. Do not mutate the real branch for negative checks.

## Required negative states

```text
local dev evidence not used as production telemetry proof
MCP smoke not used as full MCP readiness proof
proof-trace smoke not used as product-flow coverage proof
portal dev logger proof not used as repo-wide portal instrumentation proof
agent-service startup proof not used as full Rust logging adoption proof
proof-inventory query proof not used as missing-proof closure
```
