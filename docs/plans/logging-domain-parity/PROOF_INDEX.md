<!-- agent-capsule -->

> Agent Capsule
> Plan: `logging-domain-parity`
> Doc: `Logging Domain Parity Proof Index`
> Kind: proof artifact router.
> Read when: selected workpack needs proof paths.
> Stop rule: Use only the proof root for the assigned workpack.
> Proves: proof location routing only.
> Does not prove: implementation completion by itself.
> Proof rule: Proof artifacts are valid only after focused commands run or a precise blocker is recorded.

<!-- /agent-capsule -->

# Logging Domain Parity Proof Index

## Proof roots

Use one proof root per workpack:

```text
output/logging-domain-parity-proof/01-current-state-and-reference-audit/
output/logging-domain-parity-proof/02-typescript-logging-package-parity/
output/logging-domain-parity-proof/03-parent-logging-architecture-and-routing/
output/logging-domain-parity-proof/04-rust-logging-core-crate/
output/logging-domain-parity-proof/05-local-validation-evidence/
output/logging-domain-parity-proof/06-validation-and-enforcement/
output/logging-domain-parity-proof/07-mcp-query-interface/
output/logging-domain-parity-proof/08-logger-instrumentation-and-adoption/
output/logging-domain-parity-proof/09-log-control-retention-bridge-lifecycle/
output/logging-domain-parity-proof/10-proof-trace-pipeline/
```

Test result roots:

```text
test-results/logging-domain-parity-current-state-audit/
test-results/logging-domain-parity-typescript-package/
test-results/logging-domain-parity-routing/
test-results/logging-domain-parity-rust-core/
test-results/logging-domain-parity-local-evidence/
test-results/logging-domain-parity-validation/
test-results/logging-domain-parity-mcp/
test-results/logging-domain-parity-instrumentation/
test-results/logging-domain-parity-control-lifecycle/
test-results/logging-domain-parity-proof-trace/
```

## Required proof files per workpack

### WP01

```text
00-reference-file-map.json
01-parent-current-state.json
02-live-usage-map.json
03-gap-summary.md
04-existing-mcp-audit.json
16-validation-commands.log
```

### WP02

```text
00-package-export-before-after.json
01-typescript-parity-file-map.json
02-scope-defaults-proof.json
03-query-script-smoke.json
16-validation-commands.log
```

### WP03

```text
00-routing-before-after.md
01-portal-dev-log-route-proof.json
02-agent-service-logging-route-proof.json
03-scope-model-proof.json
16-validation-commands.log
```

### WP09

```text
00-log-decision-provider-proof.json
01-wipe-scope-proof.json
02-retention-cleanup-proof.json
03-bridge-run-lifecycle-proof.json
04-tunnel-mode-decision-proof.json
16-validation-commands.log
```

### WP04

```text
00-rust-crate-file-map.json
01-rust-ndjson-writer-proof.json
02-artifact-writer-proof.json
03-ts-rust-fixture-parity.json
16-validation-commands.log
```

### WP05

```text
00-agent-run-smoke.json
01-agent-query-smoke.json
02-codex-evidence-smoke.txt
03-diagnostic-parser-proof.json
04-local-artifact-proof.json
16-validation-commands.log
```

### WP07

```text
00-existing-mcp-audit.json
01-mcp-tool-list.json
02-mcp-latest-failures-smoke.json
03-mcp-run-diagnostics-smoke.json
04-mcp-artifact-slice-smoke.json
05-cli-mcp-query-parity-proof.json
16-validation-commands.log
```

### WP08

```text
00-instrumentation-surface-map.json
01-typescript-logger-pattern-proof.json
02-rust-logger-pattern-proof.json
03-storage-observability-proof.json
04-mcp-source-context-proof.json
16-validation-commands.log
```

### WP10

```text
00-proof-trace-mode-proof.json
01-playwright-click-trace-proof.json
02-proof-trace-query-proof.json
03-proof-trace-gap-proof.json
04-mcp-proof-trace-proof.json
16-validation-commands.log
```

### WP06

```text
00-validation-script-map.json
01-negative-checks-proof.json
02-root-script-wiring-proof.json
03-agent-guidance-proof.md
16-validation-commands.log
```

## Command log format

Every proof root must include a command log:

```text
command: <exact command>
exit: <code>
result: pass | fail | blocked
artifact: <path or n/a>
notes: <short note>
```

If blocked, include:

```text
blocker:
required environment:
why this does not prove completion:
next command:
```

## Structured proof metadata

For new proof artifacts and new command-log entries, include structured metadata when available:

```text
plan: logging-domain-parity
workpack: <workpack id and name>
owner: logging-domain | logging-core | scripts-dev | portal | agent-service | mcp | proof-trace | validation | docs-only
scope: parent-test | parent-codex | parent-portal | parent-agent-service | proof-trace | mcp | local-dev | n/a
run_id: <wrapper run id or n/a>
command_id: <wrapper command id or n/a>
artifact_ref: <artifact pointer or n/a>
log_source: <source id or n/a>
log_context: <context id or n/a>
bridge_state: not-tested | sent | received | rejected | unavailable | blocked | n/a
ndjson_state: not-tested | written | ingested | malformed-rejected | stale | n/a
duckdb_state: not-tested | ensured | rebuilt | ingested | queried | stale | missing | n/a
mcp_tool: latest-failures | run-diagnostics | artifact-slice | proof-trace | source-context | proof-inventory | other | n/a
proof_trace_id: <proof trace id or n/a>
correlation_id: <correlation id or n/a>
retention_state: not-tested | kept | wiped | expired | stale-rejected | n/a
validation_state: not-tested | passed | failed | blocked | external-owner | n/a
manual_required_note: <manual-required gap or n/a>
command: <exact command>
exit: <code>
result: pass | fail | blocked
artifact: <artifact pointer, proof file, test result path, or n/a>
diagnostics_summary: <short unique failure or proof summary>
no_claim: <what this result does not prove>
```

The command log is a compact index. Store long command output, proof JSON, test reports, MCP smoke output, query output, or long failure dumps under artifact paths and reference them by pointer. If no wrapper exists, write `run_id: n/a` and `command_id: n/a`; do not omit the proof row.

## No-claim language

Proof files must not claim:

```text
production telemetry readiness
product runtime logging readiness
full repo validation
logging-domain parity complete
Codex evidence wrapper complete
MCP logging interface complete
logger instrumentation complete for the whole repo
log retention policy complete for all products
proof-trace coverage for all product flows
portal logging fully migrated
agent-service logging fully migrated
missing proof roots restored by query proof alone
```

unless the assigned workpack acceptance criteria and validation scripts prove it.

Use narrower wording:

```text
local bridge smoke passed
package export parity passed
Rust NDJSON writer test passed
agent-run controlled failure recorded
MCP existing-framework audit completed
MCP latest-failures smoke passed
source/context query proof passed
log decision provider proof passed
wipe/retention smoke passed
one proof-trace smoke passed
validation script negative check passed
```
