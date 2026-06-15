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

This plan upgrades OcentraParent logging from a mostly schema/proof-contract package to a practical local development and agent observability pipeline.

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
```

## Current status

```text
Plan route: added
Workpack route: added
Implementation: not started in this plan
Source code changes: none from this plan yet
Proof artifacts: none yet
PR-ready: false
```

## What is already understood

- `ocentra-games/packages/logging-domain` is the reference implementation.
- Games also had log-query tools exposed to agents and SQL-vs-tool validation evidence.
- Games required source files to register with the shared logger pattern, not merely define a logging package.
- Parent logging-domain currently has live schema/contract usage but does not have games-level local logging pipeline parity.
- Remote inspection did not find an obvious parent MCP implementation; WP01 must still run a local audit before WP07 implements or upgrades it.
- Parent needs TypeScript package parity and a Rust `crates/logging-core` because the parent runtime is Rust-heavy.
- Local development observability is separate from production/product safe logging.
- Codex/local agents should consume compact deterministic evidence through CLI and MCP, not full raw terminal logs.

## Open gaps

```text
- TypeScript test-log/transport/app-log parity modules not implemented.
- Bridge, NDJSON, DuckDB, query scripts not implemented in parent package.
- Rust logging-core crate not implemented.
- Agent-service still needs migration away from one-off dev log writer.
- Portal dev-log route must be implemented or routed through bridge.
- Local validation evidence wrappers are not implemented.
- MCP logging query interface is not implemented or proven through local audit.
- Logger instrumentation/adoption pattern is not implemented in parent surfaces.
- Validation/enforcement scripts are not implemented.
- Root scripts are not wired.
- Proof roots are not generated.
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
WP04 Rust logging-core
WP05 local validation evidence
WP07 MCP query interface
WP08 logger instrumentation and adoption
WP06 validation/enforcement
```

## Health rules

- Do not touch other plan folders while working this plan.
- Do not change source before selecting one workpack.
- Do not claim DONE without focused commands and proof artifacts.
- Do not add more proof-only read models as a substitute for bridge/NDJSON/DuckDB/query/MCP/instrumentation implementation.
