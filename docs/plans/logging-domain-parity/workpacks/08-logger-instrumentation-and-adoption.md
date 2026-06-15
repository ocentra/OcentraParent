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

- [x] Parent TypeScript logger usage pattern implemented or documented at API boundary.
- [x] Parent Rust logger usage pattern implemented through logging-core.
- [x] Portal dev/runtime logging uses parent logger instead of ad hoc fetch/console path.
- [x] Agent-service startup/health/dev diagnostics use logging-core.
- [x] Validation/evidence scripts log run_id and command_id where useful.
- [x] At least one TypeScript runtime path produces source/context fields queryable from storage.
- [x] At least one Rust service path produces source/context fields queryable from storage or fixture output.
- [x] Tests verify registered source/context fields are preserved.
- [x] Checks prevent new raw console logging in touched logging surfaces.
- [x] Checks prevent ad hoc JSON log writers outside logging-domain/logging-core.
- [x] MCP or CLI query proof shows useful source/context values.
- [x] Proof root and workpack completion section filled.

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

## Completion

Workpack id and branch:
WP08 on `codex/tracking-plan-full-continuation-a`

Touched files:
`packages/logging-domain/src/contracts.ts`
`packages/logging-domain/src/core/logConfig.ts`
`packages/logging-domain/src/core/logDecisionProvider.ts`
`packages/logging-domain/src/core/logger.ts`
`packages/logging-domain/src/core/stackTraceParser.ts`
`packages/logging-domain/src/transport/bridgeTransport.ts`
`packages/logging-domain/tests/unit/logger.test.ts`
`packages/logging-domain/tests/unit/agent-evidence-scripts.test.ts`
`packages/parent-domain/tests/logging/parent-domain-logger-consumer.ts`
`packages/parent-domain/tests/logging/parent-domain-logger-consumer.test.ts`
`crates/agent-service/src/dev_log.rs`
`apps/portal/src/dev-logger.ts`
`apps/portal/tests/logging/portal-dev-log-route.test.ts`
`apps/portal/tests/logging/portal-proof-trace.test.ts`
`scripts/dev/lib/agent-summary-format.mjs`
`scripts/dev/lib/log-query-service.d.mts`
`output/logging-domain-parity-proof/08-logger-instrumentation-and-adoption/*.json`
`output/logging-domain-parity-proof/08-logger-instrumentation-and-adoption/16-validation-commands.log`
`docs/plans/logging-domain-parity/NEXT_ACTIONS.md`
`docs/plans/logging-domain-parity/PLAN_STATE.md`
`docs/plans/logging-domain-parity/WORKPACK_INDEX.md`
`docs/plans/logging-domain-parity/workpacks/08-logger-instrumentation-and-adoption.md`

Validation commands and results:
`cmd /c npm run build --workspace @ocentra-parent/logging-domain` passed.
`cmd /c npm run test --workspace @ocentra-parent/logging-domain` passed.
`cargo test -p ocentra-parent-logging-core` passed.
`cargo test -p ocentra-parent-agent-service dev_log` passed.
`cmd /c npx vitest run packages/parent-domain/tests/logging/parent-domain-logger-consumer.test.ts --config packages/parent-domain/vitest.config.ts` passed.
`cmd /c npx vitest run tests/unit/agent-evidence-scripts.test.ts --config packages/logging-domain/vitest.config.ts` passed.
`cmd /c npx vitest run tests/logging/portal-dev-log-route.test.ts tests/logging/portal-proof-trace.test.ts` with `workdir=apps/portal` passed.
`cmd /c npm run lint:architecture -- --files apps/portal/src/dev-logger.ts apps/portal/tests/logging/portal-dev-log-route.test.ts apps/portal/tests/logging/portal-proof-trace.test.ts packages/logging-domain/src/contracts.ts packages/logging-domain/src/core packages/logging-domain/tests/unit packages/parent-domain/tests/logging scripts/dev/agent-run.mjs scripts/dev/agent-query.mjs scripts/dev/codex-evidence.mjs scripts/dev/lib/agent-summary-format.mjs scripts/dev/lib/log-query-service.d.mts` passed.
`cargo lint-architecture crates/agent-service/src/dev_log.rs` passed.
`cmd /c npm run build --workspace @ocentra-parent/portal` still fails, but the remaining errors are outside WP08 ownership and the touched logger adoption path is no longer on the failure list.

Proof artifacts:
`output/logging-domain-parity-proof/08-logger-instrumentation-and-adoption/00-instrumentation-surface-map.json`
`output/logging-domain-parity-proof/08-logger-instrumentation-and-adoption/01-typescript-logger-pattern-proof.json`
`output/logging-domain-parity-proof/08-logger-instrumentation-and-adoption/02-rust-logger-pattern-proof.json`
`output/logging-domain-parity-proof/08-logger-instrumentation-and-adoption/03-storage-observability-proof.json`
`output/logging-domain-parity-proof/08-logger-instrumentation-and-adoption/04-mcp-source-context-proof.json`
`output/logging-domain-parity-proof/08-logger-instrumentation-and-adoption/16-validation-commands.log`

Product/runtime claims:
`packages/logging-domain/src/core/logger.ts` now has a shared parent logger surface that mirrors the games logger pattern closely enough for cross-package adoption: `Logger.instance`, `register(import.meta.url)`, helper wrappers, stack-trace-derived source/context, and bridge queue flush.
`packages/parent-domain/tests/logging/parent-domain-logger-consumer.ts` proves a non-logging domain can use that API directly and emit four real rows (`info`, `warn`, `error`, `debug`) into structured NDJSON.
Those TypeScript rows are queryable through raw NDJSON, DuckDB ingest, the shared query-service module, and the MCP logging server by exact `source` and `context`.
`apps/portal/src/dev-logger.ts` now routes portal dev runtime logs and proof-trace rows through the shared `Logger` API instead of the old ad hoc bridge-entry writer, and the targeted portal tests prove `source=DevLogger`, stable `context`, `file_path`, ordered proof steps, and `correlation_id` preservation.
`packages/logging-domain/tests/unit/agent-evidence-scripts.test.ts` proves the selected evidence scripts preserve `run_id` and `command_id` end to end through real NDJSON rows and human-facing query output.
The Rust `crates/agent-service/src/dev_log.rs` wrapper now proves all four levels through logging-core into scoped NDJSON and preserves `source=agent-service` plus a structured `fields.context` value.
The Rust proof harness now honors an externally supplied `OCENTRA_PARENT_LOG_ROOT`, which makes the real emitted files inspectable from the higher-level cross-package parity test.

Known gaps/manual-required states:
WP08 closes the selected portal and script adoption rows only. It does not claim repo-wide instrumentation coverage or a fully green `@ocentra-parent/portal` workspace build outside the touched logging surfaces.
