<!-- agent-capsule -->

> Agent Capsule
> Plan: `logging-domain-parity`
> Doc: `WP04 Rust Logging Core Crate`
> Kind: assigned implementation workpack.
> Read when: selected by WORKPACK_INDEX.md or explicit assignment.
> Stop rule: Do not implement TypeScript package parity here except fixture tests needed for parity.
> Proves: Rust logging-core implementation only after tests/proof pass.
> Does not prove: local validation evidence wrapper or full parent parity.
> Proof rule: Before DONE, run focused commands and write proof artifacts.

<!-- /agent-capsule -->

# WP04 Rust Logging Core Crate

## Purpose

Add the Rust-side logging primitive required by the parent repo.

OcentraParent is Rust-heavy. The parent logging parity plan is incomplete without:

```text
crates/logging-core
package: ocentra-parent-logging-core
```

## Source inputs

```text
docs/plans/logging-domain-parity/02-rust-logging-core-crate.md
docs/plans/logging-domain-parity/05-codex-continuation-plan.md
crates/agent-service/src/dev_log.rs
crates/agent-service/src/service_runtime.rs
crates/agent-protocol/src/logging.rs
packages/logging-domain/src/contracts.ts
Cargo.toml
```

## Target state

Rust crate exists and owns:

```text
log event structs
log level/source/fields values
NDJSON writer
artifact writer
redaction helpers
agent run structs
diagnostic structs
local path resolution
```

Agent-service delegates dev logging to it.

TS/Rust JSON fixture parity exists.

## Required crate layout

```text
crates/logging-core/Cargo.toml
crates/logging-core/src/lib.rs
crates/logging-core/src/event.rs
crates/logging-core/src/level.rs
crates/logging-core/src/source.rs
crates/logging-core/src/field.rs
crates/logging-core/src/redaction.rs
crates/logging-core/src/path.rs
crates/logging-core/src/ndjson_writer.rs
crates/logging-core/src/artifact.rs
crates/logging-core/src/dev_log.rs
crates/logging-core/src/agent_run.rs
crates/logging-core/src/diagnostic.rs
crates/logging-core/src/snapshot.rs
```

## Required proof root

```text
output/logging-domain-parity-proof/04-rust-logging-core-crate/
```

Required artifacts:

```text
00-rust-crate-file-map.json
01-rust-ndjson-writer-proof.json
02-artifact-writer-proof.json
03-ts-rust-fixture-parity.json
16-validation-commands.log
```

## Checklist rows

- [x] `crates/logging-core` created.
- [x] Workspace manifest updated.
- [x] Rust log event types added.
- [x] NDJSON writer added.
- [x] Artifact writer added.
- [x] Redaction helpers added.
- [x] Agent run/diagnostic structs added.
- [x] Agent-service delegates dev logging to logging-core.
- [x] Rust tests added.
- [x] TS/Rust fixture parity tests added.
- [x] Focused cargo/npm commands pass.
- [x] Proof root and workpack completion filled.

## Expected source changes

```text
Cargo.toml
crates/logging-core/**
crates/agent-service/Cargo.toml
crates/agent-service/src/dev_log.rs
crates/agent-service/src/service_runtime.rs
packages/logging-domain/fixtures/**
packages/logging-domain/tests/**
```

## Additional validation from continuation note

`05-codex-continuation-plan.md` adds the missing Rust validation detail: use `cargo check` as well as unit tests.

Required Rust coverage:

```text
cargo check for the new crate
unit tests for logging-core
unit tests or focused tests for direct consumers
NDJSON writer behavior
artifact writer behavior
TS/Rust JSON fixture parity
```

## Focused commands

```bash
cargo check -p ocentra-parent-logging-core
cargo test -p ocentra-parent-logging-core
cargo clippy -p ocentra-parent-logging-core --all-targets -- -D warnings
cargo test -p ocentra-parent-agent-service dev_log
npm run test --workspace @ocentra-parent/logging-domain -- dev-log-fixture
```

## Hard rules

```text
no unsafe code
no panic/unwrap/expect in library code
no stdout/stderr writes from logging-core library
all file writes return Result
no network transport in logging-core
paths are local by default
```

## Manual-required gaps

This workpack does not implement `agent:run` or DuckDB query wrappers. That belongs to WP05.

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
WP04 on `codex/tracking-plan-full-continuation-a`

Touched files:
`Cargo.toml`
`crates/agent-service/Cargo.toml`
`crates/agent-service/src/dev_log.rs`
`crates/logging-core/Cargo.toml`
`crates/logging-core/src/lib.rs`
`crates/logging-core/src/event.rs`
`crates/logging-core/src/level.rs`
`crates/logging-core/src/source.rs`
`crates/logging-core/src/field.rs`
`crates/logging-core/src/redaction.rs`
`crates/logging-core/src/path.rs`
`crates/logging-core/src/ndjson_writer.rs`
`crates/logging-core/src/artifact.rs`
`crates/logging-core/src/dev_log.rs`
`crates/logging-core/src/agent_run.rs`
`crates/logging-core/src/diagnostic.rs`
`crates/logging-core/src/snapshot.rs`
`crates/logging-core/tests/logging_core.rs`
`crates/logging-core/tests/fixtures/dev-log-entry.json`
`packages/logging-domain/fixtures/dev-log-entry.json`
`packages/logging-domain/tests/unit/dev-log-fixture.test.ts`

Validation commands and results:
`cargo check -p ocentra-parent-logging-core` passed
`cargo test -p ocentra-parent-logging-core` passed
`cargo clippy -p ocentra-parent-logging-core --all-targets -- -D warnings` passed
`cargo test -p ocentra-parent-agent-service dev_log` passed
`npm run test --workspace @ocentra-parent/logging-domain -- dev-log-fixture` passed
`cargo lint-architecture crates/logging-core crates/agent-service/src/dev_log.rs` passed
`npm run lint:architecture -- --files packages/logging-domain/tests/unit/dev-log-fixture.test.ts` passed

Proof artifacts:
`output/logging-domain-parity-proof/04-rust-logging-core-crate/00-rust-crate-file-map.json`
`output/logging-domain-parity-proof/04-rust-logging-core-crate/01-rust-ndjson-writer-proof.json`
`output/logging-domain-parity-proof/04-rust-logging-core-crate/02-artifact-writer-proof.json`
`output/logging-domain-parity-proof/04-rust-logging-core-crate/03-ts-rust-fixture-parity.json`
`output/logging-domain-parity-proof/04-rust-logging-core-crate/16-validation-commands.log`

Product/runtime claims:
Parent now has a dedicated `ocentra-parent-logging-core` crate that owns Rust log event contracts, local path resolution, NDJSON writing, artifact writing, redaction, agent-run structs, diagnostic structs, and dev-log compatibility behavior.
`agent-service` no longer owns the dev-log writer implementation; it delegates to logging-core through a narrow compatibility wrapper that preserves the legacy `OCENTRA_PARENT_DEV_LOG_DIR` file layout when that environment variable is set.
TS/Rust fixture parity now exists in both directions: Rust deserializes the package fixture and the TypeScript logging-domain test parses the Rust fixture through `DevLogEntrySchema`.

Known gaps/manual-required states:
WP05 still owns higher-level local validation evidence, `agent:run`, and DuckDB/query wrappers.
WP04 intentionally left `crates/agent-service/src/service_runtime.rs` unchanged because the existing call site already routes through `crate::dev_log::write_agent_info`, and that wrapper now delegates into logging-core.
