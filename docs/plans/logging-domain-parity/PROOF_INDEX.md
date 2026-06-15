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
```

## Required proof files per workpack

### WP01

```text
00-reference-file-map.json
01-parent-current-state.json
02-live-usage-map.json
03-gap-summary.md
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

### WP06

```text
00-validation-script-map.json
01-negative-checks-proof.json
02-root-script-wiring-proof.json
03-agent-guidance-proof.md
16-validation-commands.log
```

### WP07

```text
00-mcp-tool-list.json
01-mcp-latest-failures-smoke.json
02-mcp-run-diagnostics-smoke.json
03-mcp-file-slice-smoke.json
04-cli-mcp-query-parity-proof.json
16-validation-commands.log
```

## Command log format

Every proof root must include a command log:

```text
command: <exact command>
exit: <code>
result: pass | fail | blocked
notes: <short note>
```

If blocked, include:

```text
blocker:
required environment:
why this does not prove completion:
next command:
```

## No-claim language

Proof files must not claim:

```text
production telemetry readiness
product runtime logging readiness
full repo validation
logging-domain parity complete
Codex evidence wrapper complete
MCP logging interface complete
```

unless the assigned workpack acceptance criteria and validation scripts prove it.

Use narrower wording:

```text
local bridge smoke passed
package export parity passed
Rust NDJSON writer test passed
agent-run controlled failure recorded
MCP latest-failures smoke passed
validation script negative check passed
```
