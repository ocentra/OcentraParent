<!-- agent-capsule -->

> Agent Capsule
> Plan: `logging-domain-parity`
> Doc: `Logging Domain Parity Plan State`
> Kind: current state and open gaps.
> Read when: immediately after plan AGENTS.md.
> Stop rule: Do not continue into implementation docs unless this file routes you there.
> Proves: only current plan state and open gap accounting.
> Does not prove: implementation completion, validation, or PR readiness.
> Proof rule: If state changes, update matching workpack, checklist, and proof path.

<!-- /agent-capsule -->

# Logging Domain Parity Plan State

## Scope

This plan upgrades OcentraParent logging from a mostly schema/proof-contract package to a practical local development, agent observability, MCP query, and proof-trace pipeline.

## Current source docs

The source docs are routed through workpacks:

```text
00-current-state-and-reference-audit.md
01-parent-logging-architecture.md
02-rust-logging-core-crate.md
03-local-validation-evidence.md
04-validation-and-enforcement.md
05-codex-continuation-plan.md
06-mcp-query-interface.md
07-logger-instrumentation-pattern.md
08-log-control-retention-bridge-lifecycle.md
09-proof-trace-pipeline.md
```

## Current status

```text
Plan route: added
Workpack route: added
Implementation: complete at workpack scope and re-verified on the current branch with live MCP, TS, Rust, portal proof-trace, and validation evidence
Completed workpacks in this checkout: WP01, WP02, WP03, WP09, WP04, WP05, WP07, WP08, WP10, WP06
Current forward execution focus: no open workpacks remain inside this plan; preserve the explicit boundary that the unrelated portal workspace build is still red
Proof artifacts: written for WP01, WP02, WP03, WP04, WP05, WP06, WP07, WP08, WP09, and WP10, and WP06, WP08, and WP10 proof now reflect the current portal logger contract
PR-ready: false
```

## What is already understood

- `ocentra-games/packages/logging-domain` is the reference implementation.
- Games also had log-query tools exposed to agents and SQL-vs-tool validation evidence.
- Games required source files to register with the shared logger pattern, not merely define a logging package.
- Games used log decision controls, fresh-run wipe, bridge run-start, retention, and optional tunnel routing.
- Parent logging-domain currently has live schema/contract usage but does not have games-level local logging pipeline parity.
- Local MCP audit found no reusable preexisting parent MCP framework in scope before the WP07 logging MCP server path.
- Parent needs TypeScript package parity and a Rust `crates/logging-core` because the parent runtime is Rust-heavy.
- Local development observability is separate from production/product safe logging.
- Codex/local agents should consume compact deterministic evidence through CLI and MCP, not full raw terminal logs.
- The same log pipeline should also collect proof traces for Playwright/service/runtime paths.
- A parent-domain consumer now proves the games-style logger API in a non-logging package and emits four real rows into `parent-test` storage.
- Those TypeScript rows are queryable through NDJSON, DuckDB, the shared query-service module, and the MCP logging server by exact `source` and `context`.
- The Rust agent-service dev-log fixture now proves all four levels under `parent-agent` with `source=agent-service` and a structured `fields.context=hello-world` value through logging-core output.

## Open gaps

```text
- The full `@ocentra-parent/portal` workspace build is still red for unrelated non-logging portal type errors, so plan completion cannot be framed as a portal-wide green build.
```

## No-claim boundaries

Until implemented and validated, do not claim:

```text
logging-domain parity complete
local validation evidence complete
Rust logging-core complete
portal dev-log routing fixed
agent-service logging migrated
Codex evidence wrapper available
MCP logging query interface available
logger instrumentation adopted repo-wide
proof trace coverage for product flows
production telemetry readiness
product runtime logging readiness
```

## Workpack summary

Workpacks are indexed in `WORKPACK_INDEX.md`.

Current default execution order:

```text
WP01 current-state/reference audit, including existing MCP audit
WP02 TypeScript package parity
WP03 parent architecture/routing fix
WP09 log control, retention, and bridge lifecycle
WP04 Rust logging-core
WP05 local validation evidence
WP07 MCP query interface
WP08 logger instrumentation and adoption
WP10 proof trace pipeline
WP06 validation/enforcement
```

## Health rules

- Do not touch other plan folders while working this plan.
- Do not change source before selecting one workpack.
- Do not claim DONE without focused commands and proof artifacts.
- Do not add more proof-only read models as a substitute for bridge/NDJSON/DuckDB/query/MCP/instrumentation/proof-trace implementation.
