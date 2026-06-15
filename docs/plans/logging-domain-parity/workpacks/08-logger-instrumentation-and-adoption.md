<!-- agent-capsule -->

> Agent Capsule
> Plan: `logging-domain-parity`
> Doc: `WP08 Logger Instrumentation and Adoption`
> Kind: assigned implementation workpack.
> Read when: selected by WORKPACK_INDEX.md or explicit assignment.
> Stop rule: Do not mass-edit unrelated files; instrument only assigned surfaces.
> Proves: logger usage/adoption only after focused tests and evidence pass.
> Does not prove: logging package parity, Rust logging-core, MCP, or full validation completion by itself.
> Proof rule: Before DONE, run focused instrumentation checks and write proof artifacts.

<!-- /agent-capsule -->

# WP08 Logger Instrumentation and Adoption

## Purpose

Make parent source files actually use the logging pipeline.

A logging-domain, NDJSON store, DuckDB store, CLI, and MCP are not enough if source files do not register and log useful source/context evidence.

## Source inputs

```text
docs/plans/logging-domain-parity/07-logger-instrumentation-pattern.md
ocentra-games/.cursor/rules/ocentra-cloudflare-logging.mdc
ocentra-games infra/cloudflare Durable Object logger usage examples
packages/logging-domain/src/**
crates/logging-core/**
crates/agent-service/src/**
apps/portal/src/**
scripts/dev/**
```

## Dependency gate

Run this after the relevant logger primitives exist.

Required before broad adoption:

```text
TypeScript logger API exists from WP02/WP03.
Rust logging-core exists from WP04 for Rust service code.
At least one bridge/NDJSON path exists for proving logs land in storage.
```

If not available, implement only documentation/validation stubs and route back to WP02/WP04.

## Target state

New or touched parent logging surfaces follow a shared instrumentation pattern:

```text
register source identity
use helper methods
pass stack/source context where available
log entry/branch/error/success points
emit small structured fields
avoid raw dumps in log fields
```

## Required proof root

```text
output/logging-domain-parity-proof/08-logger-instrumentation-and-adoption/
```

Required artifacts:

```text
00-instrumentation-surface-map.json
01-typescript-logger-pattern-proof.json
02-rust-logger-pattern-proof.json
03-storage-observability-proof.json
04-mcp-source-context-proof.json
16-validation-commands.log
```

## Checklist rows

- [ ] Parent TypeScript logger usage pattern implemented or documented at API boundary.
- [ ] Parent Rust logger usage pattern implemented through logging-core.
- [ ] Portal dev/runtime logging uses parent logger instead of ad hoc fetch/console path.
- [ ] Agent-service startup/health/dev diagnostics use logging-core.
- [ ] Validation/evidence scripts log run_id and command_id where useful.
- [ ] At least one TypeScript runtime path produces source/context fields queryable from storage.
- [ ] At least one Rust service path produces source/context fields queryable from storage or fixture output.
- [ ] Tests verify registered source/context fields are preserved.
- [ ] Checks prevent new raw console logging in touched logging surfaces.
- [ ] Checks prevent ad hoc JSON log writers outside logging-domain/logging-core.
- [ ] MCP or CLI query proof shows useful source/context values.
- [ ] Proof root and workpack completion section filled.

## Expected source changes

Possible files:

```text
packages/logging-domain/src/core/**
packages/logging-domain/src/transport/**
crates/logging-core/**
crates/agent-service/src/dev_log.rs
crates/agent-service/src/service_runtime.rs
apps/portal/src/dev-logger.ts
scripts/dev/agent-run.mjs
scripts/dev/agent-query.mjs
scripts/dev/codex-evidence.mjs
scripts/check-*.mjs
```

Do not mass-edit the repo just to add helper methods. Instrument the paths selected by this workpack and add validation for future touched files.

## Focused commands

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

## Logging call rules

Log at:

```text
entry
branch/degraded/manual-required path
warning
error
important success
```

Do not log:

```text
full stdout/stderr
full request body
raw screenshots
raw browser URLs
message contents
secrets/tokens
loop spam
```

## Manual-required gaps

This workpack does not require every parent source file to be instrumented. It creates the pattern, proves it, and enforces it for touched logging surfaces.

## Fill before DONE or PR-ready

```text
Workpack id and branch:
Touched files:
Validation commands and results:
Proof artifacts:
Product/runtime claims:
Known gaps/manual-required states:
```
