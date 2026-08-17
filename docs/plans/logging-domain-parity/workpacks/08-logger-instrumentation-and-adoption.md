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

## Accepted source-wave reconciliation (2026-08-17)

The accepted source head `735df89de` confirms that `Logger.serializeData`
sanitizes structured data before JSON serialization, and the portal
compatibility fallback sanitizes its entry fields before its JSON body is
serialized. Both paths consume the generated Rust-owned 18-key policy through
`redactStructuredLogValue`; neither path owns an alternate local regex/policy.
This does not establish repo-wide instrumentation adoption, test/proof
closure, or external product/runtime composition.

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

## Partial-proof boundary

The current proof root is intentionally narrow. It proves selected surfaces only:

```text
portal_dev_logger_path
logging_domain_source_context_storage_query_path
agent_service_startup_dev_log_path
```

It does not prove:

```text
repo_wide_instrumentation_adoption
all_portal_logging
all_agent_service_diagnostics
all_validation_evidence_script_run_id_command_id_adoption
all_raw_console_checks
all_ad_hoc_json_writer_checks
WP06 root validation green
product runtime logging readiness
production telemetry readiness
```

Required partial-proof fields:

```text
surface_id
owner
source_context_state
storage_query_state
rust_logging_core_state
portal_dev_logger_state
agent_service_dev_log_state
query_proof_state
validation_gate_state
repo_wide_adoption_state
no_claim
```

If proof exists but adoption/checker rows remain open, keep the workpack at `partial-proof`.

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
sensitive credential material
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

## Current audit note

This workpack now has a canonical partial-proof root under `output/logging-domain-parity-proof/08-logger-instrumentation-and-adoption/`.

The current bounded proof is real:

- `packages/logging-domain/tests/unit/logger.test.ts` proves a registered TypeScript source survives bridge storage with source/context/file path metadata.
- `apps/portal/tests/logging/portal-dev-log-route.test.ts` proves portal dev logging emits bridge-compatible rows with source/context/file metadata through the shared parent logger path.
- `crates/agent-service/src/service_runtime.rs` now emits structured startup fields through the existing logging-core-backed `dev_log` writer.
- `crates/agent-service/tests/unit/dev_log.rs` is now mounted and proved by the exact `write_agent_info_writes_dev_log_ndjson_line` unit test instead of existing only as dead layout inventory.
- the shared query service and MCP server now have a canonical source/context proof against a temporary local bridge root.

Treat WP08 as honest `partial-proof`, not as full repo instrumentation completion. The proof root narrows the claim to the portal dev logger path, the logging-domain storage/query path, and the agent-service startup/dev-log path.

The accepted `720609306` source delta additionally routes shared logger data
through the canonical fail-closed sanitizer. Unsupported primitives/objects,
cycles, throwing accessors/proxies, and failed serializers are converted into
JSON-safe markers; Date/URL/custom-`toJSON` values retain bounded native
semantics. No new expected tests were written. The later test wave must prove
bridge serialization never throws or silently drops these values and that
custom serializers receive root/property/array keys exactly once.

## Current completion block

```text
Workpack id and branch:
WP08 / codex/tracking-plan-full-continuation-a

Touched files:
- crates/agent-service/src/service_runtime.rs
- crates/agent-service/src/dev_log.rs
- crates/agent-service/tests/unit/service_runtime.rs
- crates/agent-service/tests/unit/dev_log.rs
- output/logging-domain-parity-proof/08-logger-instrumentation-and-adoption/00-instrumentation-surface-map.json
- output/logging-domain-parity-proof/08-logger-instrumentation-and-adoption/01-typescript-logger-pattern-proof.json
- output/logging-domain-parity-proof/08-logger-instrumentation-and-adoption/02-rust-logger-pattern-proof.json
- output/logging-domain-parity-proof/08-logger-instrumentation-and-adoption/03-storage-observability-proof.json
- output/logging-domain-parity-proof/08-logger-instrumentation-and-adoption/04-mcp-source-context-proof.json
- output/logging-domain-parity-proof/08-logger-instrumentation-and-adoption/16-validation-commands.log

Validation commands and results:
- pass: cargo test -p ocentra-parent-agent-service startup_log_fields_include_context_and_bound_port --lib
- pass: cargo test -p ocentra-parent-agent-service write_agent_info_writes_dev_log_ndjson_line --lib
- pass: cargo lint-architecture crates/agent-service/src/service_runtime.rs crates/agent-service/src/dev_log.rs crates/agent-service/tests/unit/service_runtime.rs crates/agent-service/tests/unit/dev_log.rs
- pass: npm run test --workspace @ocentra-parent/logging-domain -- tests/unit/logger.test.ts
- pass: npm run test --workspace @ocentra-parent/portal -- tests/logging/portal-dev-log-route.test.ts
- pass: inline node proof harness for source/context storage and MCP query proof

Proof artifacts:
- output/logging-domain-parity-proof/08-logger-instrumentation-and-adoption/00-instrumentation-surface-map.json
- output/logging-domain-parity-proof/08-logger-instrumentation-and-adoption/01-typescript-logger-pattern-proof.json
- output/logging-domain-parity-proof/08-logger-instrumentation-and-adoption/02-rust-logger-pattern-proof.json
- output/logging-domain-parity-proof/08-logger-instrumentation-and-adoption/03-storage-observability-proof.json
- output/logging-domain-parity-proof/08-logger-instrumentation-and-adoption/04-mcp-source-context-proof.json
- output/logging-domain-parity-proof/08-logger-instrumentation-and-adoption/16-validation-commands.log

Product/runtime claims:
- portal dev logging is proved on the shared parent logger path
- one TypeScript storage/query path is proved for source/context metadata
- agent-service startup/dev-log path is proved through logging-core-backed output

Known gaps/manual-required states:
- repo-wide instrumentation adoption is still not proved
- agent-service startup/health/dev diagnostics are only partially covered here
- validation/evidence script run_id and command_id adoption is still open
- WP06 root lint:dev-log-routing remains outside this bounded WP08 proof
```
