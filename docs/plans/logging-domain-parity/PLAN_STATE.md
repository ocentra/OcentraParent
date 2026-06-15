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

The initial spec pack exists:

```text
00-current-state-and-reference-audit.md
01-parent-logging-architecture.md
02-rust-logging-core-crate.md
03-local-validation-evidence.md
04-validation-and-enforcement.md
```

These are now routed through workpacks. Do not treat the five source docs as the default execution path.

## Current status

```text
Plan route: added
Workpack route: added by this plan
Implementation: not started in this plan
Source code changes: none from this plan yet
Proof artifacts: none yet
PR-ready: false
```

## What is already understood

- `ocentra-games/packages/logging-domain` is the reference implementation.
- Parent logging-domain currently has live schema/contract usage but does not have games-level local logging pipeline parity.
- Parent needs TypeScript package parity and a Rust `crates/logging-core` because the parent runtime is Rust-heavy.
- Local development observability is separate from production/product safe logging.
- Codex/local agents should consume compact deterministic evidence, not full raw terminal logs.

## Open gaps

```text
- TypeScript test-log/transport/app-log parity modules not implemented.
- Bridge, NDJSON, DuckDB, query scripts not implemented in parent package.
- Rust logging-core crate not implemented.
- Agent-service still needs migration away from one-off dev log writer.
- Portal dev-log route must be implemented or routed through bridge.
- Local validation evidence wrappers are not implemented.
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
production telemetry readiness
product runtime logging readiness
```

## Workpack summary

Workpacks are indexed in `WORKPACK_INDEX.md`.

Current default execution order:

```text
WP01 current-state/reference audit
WP02 TypeScript package parity
WP03 parent architecture/routing fix
WP04 Rust logging-core
WP05 local validation evidence
WP06 validation/enforcement
```

## Health rules

- Do not touch other plan folders while working this plan.
- Do not change source before selecting one workpack.
- Do not claim DONE without focused commands and proof artifacts.
- Do not add more proof-only read models as a substitute for bridge/NDJSON/DuckDB/query implementation.
