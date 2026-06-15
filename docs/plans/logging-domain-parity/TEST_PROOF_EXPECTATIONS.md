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
secret-like fields are redacted by default helpers
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
```

Use temporary fixtures or script-internal fixtures. Do not mutate the real branch for negative checks.

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
