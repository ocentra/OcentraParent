# Local Validation Evidence

## Purpose

Coding agents and developers need compact validation evidence, not raw terminal walls.

Target flow:

```text
validation command
  -> local wrapper
  -> output saved as local files
  -> diagnostics extracted deterministically
  -> NDJSON rows written
  -> DuckDB ingested
  -> compact evidence packet returned
```

This is deterministic evidence extraction, not model summarization.

## Required scripts

Add root scripts:

```json
{
  "agent:run": "node scripts/dev/agent-run.mjs",
  "agent:query": "node scripts/dev/agent-query.mjs",
  "codex:evidence": "node scripts/dev/codex-evidence.mjs"
}
```

Create:

```text
scripts/dev/agent-run.mjs
scripts/dev/agent-query.mjs
scripts/dev/codex-evidence.mjs
scripts/dev/lib/agent-log-paths.mjs
scripts/dev/lib/agent-diagnostic-parsers.mjs
scripts/dev/lib/agent-artifacts.mjs
scripts/dev/lib/agent-summary-format.mjs
```

## `agent-run` contract

`agent-run` records local evidence for one validation run.

It must:

```text
1. create run_id and command_id
2. detect lane_id from environment when present
3. detect machine/hostname without hardcoding machine names
4. capture cwd and workspace root
5. run the requested validation command
6. capture stdout and stderr separately
7. write full stdout/stderr to local files
8. parse known diagnostic formats
9. write agent_run, agent_diagnostic, and artifact_ref NDJSON rows
10. trigger incremental DuckDB ingest
11. print a compact summary
12. return the validation command result code
```

Default output must be compact. Full output requires explicit flags:

```text
--raw
--include-stdout
--include-stderr
```

## `agent-query` contract

Minimum commands:

```text
latest-failures
by-run <run_id>
diagnostics --run-id <run_id>
artifact <artifact_id>
stats
```

Default output must show:

```text
run_id
command_id
status
exit_code
duration
unique diagnostics
file:line:column
short message
local artifact path
next query command
```

Default output must not print full raw files.

## `codex:evidence` contract

Minimum commands:

```text
latest-failures
by-run <run_id>
current-lane
```

Output format:

```text
# Evidence Packet
run_id: <id>
status: failed
command: <command>

## Diagnostics
- [signature] file:line:column — short message

## Local artifacts
- stdout: <path>
- stderr: <path>

## Next action
Use listed diagnostics first. Query local raw artifacts only when compact evidence is insufficient.
```

## Local artifact layout

```text
.logs/parent-codex/artifacts/<run_id>/<command_id>/stdout.log
.logs/parent-codex/artifacts/<run_id>/<command_id>/stderr.log
.logs/parent-codex/artifacts/<run_id>/<command_id>/metadata.json
```

## NDJSON streams

```text
.logs/parent-codex/ndjson/agent-run/YYYY-MM-DD.ndjson
.logs/parent-codex/ndjson/diagnostics/YYYY-MM-DD.ndjson
.logs/parent-codex/ndjson/artifacts/YYYY-MM-DD.ndjson
```

Each line is one JSON object.

## Diagnostic parsers

Start with deterministic parsers for:

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

Each parser emits:

```text
diagnostic_id
run_id
command_id
kind
severity
signature
file
line
column
message
raw_artifact
raw_start_line
raw_end_line
hit_count
```

De-dupe within one command by:

```text
kind + signature + file + line + column
```

## DuckDB tables

Add or extend logging-domain DuckDB support for:

```text
agent_runs
agent_diagnostics
agent_artifacts
```

Required indexes:

```text
run_id
status
started_at
signature
file
kind
severity
```

## Agent usage rule

Add root guidance:

```text
When validating this repository, use npm run agent:run -- <command> where possible.
Do not paste full raw logs into context.
Use npm run agent:query or npm run codex:evidence for failure evidence.
Use raw local artifacts only when compact diagnostics are insufficient.
```

## Acceptance criteria

```text
agent:run records failed and passed runs
agent:query latest-failures shows latest failed run
codex:evidence latest-failures prints compact evidence
stdout/stderr local files exist for each run
DuckDB has agent_runs and agent_diagnostics rows
agent:run preserves the validation result code
root guidance requires evidence wrappers for validation
```
