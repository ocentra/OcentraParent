<!-- agent-capsule -->

> Agent Capsule
> Plan: `logging-domain-parity`
> Doc: `WP05 Local Validation Evidence`
> Kind: assigned implementation workpack.
> Read when: selected by WORKPACK_INDEX.md or explicit assignment.
> Stop rule: Do not implement base logging primitives here if WP02/WP04 are missing.
> Proves: local validation evidence wrappers only after smoke tests/proof pass.
> Does not prove: TypeScript package parity, Rust logging-core, or full validation enforcement.
> Proof rule: Before DONE, run controlled pass/fail command smoke tests and write proof artifacts.

<!-- /agent-capsule -->

# WP05 Local Validation Evidence

## Purpose

Add local command evidence wrappers so Codex and developers consume compact validation evidence instead of raw terminal walls.

This is deterministic extraction, not model summarization.

## Source inputs

```text
docs/plans/logging-domain-parity/03-local-validation-evidence.md
docs/plans/logging-domain-parity/01-parent-logging-architecture.md
packages/logging-domain/src/test-log/**
packages/logging-domain/src/app-log/**
crates/logging-core/**
scripts/dev/**
```

## Dependency gate

Before implementing this workpack, confirm enough of WP02 and WP04 exists:

```text
NDJSON writer available
DuckDB ingest/query path available or planned in same slice
local artifact writer available or implemented in script helper
```

If not available, stop and route to WP02/WP04 first.

## Target state

Root scripts exist:

```json
{
  "agent:run": "node scripts/dev/agent-run.mjs",
  "agent:query": "node scripts/dev/agent-query.mjs",
  "codex:evidence": "node scripts/dev/codex-evidence.mjs"
}
```

Scripts write:

```text
.logs/parent-codex/artifacts/<run_id>/<command_id>/stdout.log
.logs/parent-codex/artifacts/<run_id>/<command_id>/stderr.log
.logs/parent-codex/artifacts/<run_id>/<command_id>/metadata.json
.logs/parent-codex/ndjson/agent-run/YYYY-MM-DD.ndjson
.logs/parent-codex/ndjson/diagnostics/YYYY-MM-DD.ndjson
.logs/parent-codex/ndjson/artifacts/YYYY-MM-DD.ndjson
```

## Required proof root

```text
output/logging-domain-parity-proof/05-local-validation-evidence/
```

Required artifacts:

```text
00-agent-run-smoke.json
01-agent-query-smoke.json
02-codex-evidence-smoke.txt
03-diagnostic-parser-proof.json
04-local-artifact-proof.json
16-validation-commands.log
```

## Checklist rows

- [x] `agent:run` root script added.
- [x] `agent:query` root script added.
- [x] `codex:evidence` root script added.
- [x] `scripts/dev/agent-run.mjs` implemented.
- [x] `scripts/dev/agent-query.mjs` implemented.
- [x] `scripts/dev/codex-evidence.mjs` implemented.
- [x] Local artifact layout implemented.
- [x] Agent run/diagnostics/artifacts NDJSON streams written.
- [x] DuckDB tables/indexes added.
- [x] Diagnostic parsers added for first supported toolchain set.
- [x] Smoke test passes.
- [x] Proof root and workpack completion filled.

## Required diagnostic parser set

Initial deterministic parsers:

```text
rustc
clippy
cargo test
tsc
eslint
npm script failure
architecture policy validators
no-reexport validator
```

Every diagnostic should include:

```text
kind
severity
signature
file
line
column
message
raw artifact path
raw line span
hit count
```

## Focused smoke commands

```bash
npm run agent:run -- node -e "process.exit(0)"
npm run agent:run -- node -e "process.exit(2)"
npm run agent:query -- latest-failures
npm run codex:evidence -- latest-failures
```

`agent:run` must return the wrapped command exit code.

## Output rule

Default output is compact.

Full stdout/stderr can be printed only with explicit flags such as:

```text
--raw
--include-stdout
--include-stderr
```

## Manual-required gaps

This workpack does not enforce every repo validation command. It creates the evidence path and proves controlled pass/fail runs.

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
WP05 on `codex/tracking-plan-full-continuation-a`

Touched files:
`package.json`
`scripts/dev/agent-run.mjs`
`scripts/dev/agent-query.mjs`
`scripts/dev/codex-evidence.mjs`
`scripts/dev/lib/agent-log-paths.mjs`
`scripts/dev/lib/agent-artifacts.mjs`
`scripts/dev/lib/agent-diagnostic-parsers.mjs`
`scripts/dev/lib/agent-evidence-db.mjs`
`scripts/dev/lib/agent-summary-format.mjs`

Validation commands and results:
`npm run agent:run -- node -e "process.exit(0)"` passed and wrote a passed `agent-run` event
`node scripts/dev/agent-run.mjs node -e "process.exit(2)"` returned `node_exit=2`
`npm run agent:run -- node -e "process.exit(2)"` returned `npm_exit=2` and wrote failed run evidence
`npm run agent:query -- latest-failures` passed
`npm run agent:query -- by-run run-20260615024414-eacf1ee1` passed
`npm run agent:query -- diagnostics --run-id=run-20260615024414-eacf1ee1` passed
`npm run agent:query -- stats` passed
`npm run codex:evidence -- latest-failures` passed
`LEDGER_LANE=codex-a npm run codex:evidence -- current-lane` passed on Windows PowerShell via `$env:LEDGER_LANE='codex-a'`
`npm run lint:architecture -- --files package.json scripts/dev/agent-run.mjs scripts/dev/agent-query.mjs scripts/dev/codex-evidence.mjs scripts/dev/lib/agent-log-paths.mjs scripts/dev/lib/agent-artifacts.mjs scripts/dev/lib/agent-diagnostic-parsers.mjs scripts/dev/lib/agent-evidence-db.mjs scripts/dev/lib/agent-summary-format.mjs` passed

Proof artifacts:
`output/logging-domain-parity-proof/05-local-validation-evidence/00-agent-run-smoke.json`
`output/logging-domain-parity-proof/05-local-validation-evidence/01-agent-query-smoke.json`
`output/logging-domain-parity-proof/05-local-validation-evidence/02-codex-evidence-smoke.txt`
`output/logging-domain-parity-proof/05-local-validation-evidence/03-diagnostic-parser-proof.json`
`output/logging-domain-parity-proof/05-local-validation-evidence/04-local-artifact-proof.json`
`output/logging-domain-parity-proof/05-local-validation-evidence/16-validation-commands.log`

Product/runtime claims:
Root validation wrappers now exist as `npm run agent:run`, `npm run agent:query`, and `npm run codex:evidence`.
Validation runs now write compact local evidence under `.logs/parent-codex` with full stdout/stderr artifacts, metadata, NDJSON event streams, a DuckDB query surface, and deterministic diagnostics.
The wrapper path preserves the wrapped command exit code for both direct `node scripts/dev/agent-run.mjs ...` execution and `npm run agent:run -- ...` execution on this checkout.
Lane-scoped evidence packets are now available through `codex:evidence current-lane` when `LEDGER_LANE` is present in the environment.

Known gaps/manual-required states:
DuckDB is single-writer on Windows, so WP05 query and evidence commands must run sequentially rather than as parallel processes against the same `.logs/parent-codex/db/agent-evidence.duckdb` file.
WP06 still owns broader validation-enforcement adoption across the repo rather than just the local evidence path implemented here.
