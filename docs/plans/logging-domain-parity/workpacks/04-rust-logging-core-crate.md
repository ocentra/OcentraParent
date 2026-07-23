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

- [ ] `crates/logging-core` created.
- [ ] Workspace manifest updated.
- [ ] Rust log event types added.
- [ ] NDJSON writer added.
- [ ] Artifact writer added.
- [ ] Redaction helpers added.
- [ ] Agent run/diagnostic structs added.
- [ ] Agent-service delegates dev logging to logging-core.
- [ ] Rust tests added.
- [ ] TS/Rust fixture parity tests added.
- [ ] Focused cargo/npm commands pass.
- [ ] Proof root and workpack completion filled.

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

## Current audit note

`crates/logging-core` is now a real crate with source, tests, fixture parity
assets, and downstream use from `crates/agent-service/src/dev_log.rs`. The June
16, 2026 audit re-verified the live delegation path with
`cargo test -p ocentra-parent-agent-service dev_log`, which passed.

The prior completion block should still not be trusted as durable proof. The
named proof root
`output/logging-domain-parity-proof/04-rust-logging-core-crate/` is absent in
this checkout, and this audit pass did not re-run the full WP04 command set.
Treat WP04 as source-present with partial live re-check, not as fully proved
complete.

## Proof restoration (2026-07-23)

The canonical WP04 proof root now contains a bounded Rust crate map, NDJSON and
artifact/redaction test evidence, TypeScript/Rust fixture parity evidence, and
the exact focused command log. All required WP04 commands passed:
`cargo check`, `cargo test`, and all-target `cargo clippy -D warnings` for
`ocentra-parent-logging-core`; the direct
`ocentra-parent-agent-service dev_log` consumer test; and the named
`@ocentra-parent/logging-domain` `dev-log-fixture` test.

This closes the WP04 local Rust-helper and direct-consumer proof slice only. It
does not claim local validation-wrapper completion, full agent-service logging
migration, production telemetry readiness, product-runtime logging readiness,
or repository-wide instrumentation adoption.
