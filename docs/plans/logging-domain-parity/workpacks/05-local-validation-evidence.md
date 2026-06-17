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

- [ ] `agent:run` root script added.
- [ ] `agent:query` root script added.
- [ ] `codex:evidence` root script added.
- [ ] `scripts/dev/agent-run.mjs` implemented.
- [ ] `scripts/dev/agent-query.mjs` implemented.
- [ ] `scripts/dev/codex-evidence.mjs` implemented.
- [ ] Local artifact layout implemented.
- [ ] Agent run/diagnostics/artifacts NDJSON streams written.
- [ ] DuckDB tables/indexes added.
- [ ] Diagnostic parsers added for first supported toolchain set.
- [ ] Smoke test passes.
- [ ] Proof root and workpack completion filled.

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

## Current audit note

The local evidence wrapper surface is present in the current source tree,
including `scripts/dev/agent-run.mjs`, `scripts/dev/agent-query.mjs`,
`scripts/dev/codex-evidence.mjs`, and shared query/evidence helpers under
`scripts/dev/lib/`. During the June 16, 2026 audit, the cheap focused checks
`npm run validate:logging` and `npm run test:logging-evidence` both passed,
showing the slice is at least partially wired.

The appended completion block remained stale because the named proof root
`output/logging-domain-parity-proof/05-local-validation-evidence/` is absent in
this checkout, and this audit pass did not recreate the original proof pack.
Treat WP05 as source-present with focused validation signal, but not fully
proved complete.
