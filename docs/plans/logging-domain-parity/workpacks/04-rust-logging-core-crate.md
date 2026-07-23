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
output/logging-domain-parity-proof/04-rust-logging-core-crate/ (generated locally and intentionally ignored; regenerate from the recorded commands, do not expect it in Git)
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
cargo check -p ocentra-parent-logging-core --features test-support
cargo test -p ocentra-parent-logging-core
cargo test -p ocentra-parent-logging-core --features test-support
cargo clippy -p ocentra-parent-logging-core --all-targets -- -D warnings
cargo clippy -p ocentra-parent-logging-core --all-targets --features test-support -- -D warnings
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

The July 20, 2026 closure merged `origin/main`, preserved explicit
operation-identity semantics, added a real subprocess artifact
publish/replay/conflict test, and expanded normalized secret-key coverage
through persisted NDJSON, artifact content, and artifact metadata. The scoped
Rust, fixture, formatting, clippy, and architecture gates passed. The generated
directory is intentionally ignored, so its existence is local reproducible
evidence rather than a tracked checkout prerequisite.

## Completion record

```text
Workpack id and branch: 04-rust-logging-core-crate; codex/logging-domain-parity-wp04; merged origin/main base dc27a632a852ee5ba5f85dc9188824ca8abe4308; proof-bound source commit 268b1f93d01d8410bf1fa5eccacf79dedad4f9aa
Touched files: logging-core NDJSON tail/operation recovery, artifact publication and custody replay, path normalization, visible unit/fault/subprocess coverage, bounded logging-plan closeout docs, and five local ignored proof artifacts under output/logging-domain-parity-proof/04-rust-logging-core-crate/
Validation commands and results: PASS cargo fmt -p ocentra-parent-logging-core -- --check; PASS cargo check -p ocentra-parent-logging-core with and without `test-support`; PASS cargo test -p ocentra-parent-logging-core (42 normal top-level tests; 48 with `test-support`, including 6 deterministic failure-recovery tests); PASS cargo clippy -p ocentra-parent-logging-core --all-targets with and without `test-support` and `-D warnings`; PASS focused large-tail, corrupted-custody-field, UNC-path, failure-recovery, subprocess-custody, and persisted-redaction tests; PASS cargo test -p ocentra-parent-agent-service dev_log (exit 0, with no matching test names in this revision); PASS npm run test --workspace @ocentra-parent/logging-domain -- dev-log-fixture (2/2); PASS npm run lint:architecture -- --files crates/logging-core; PASS source-shape; PASS git diff --check
Proof artifacts: regenerated locally against source commit 268b1f93d01d8410bf1fa5eccacf79dedad4f9aa as 00-rust-crate-file-map.json, 01-rust-ndjson-writer-proof.json, 02-artifact-writer-proof.json, 03-ts-rust-fixture-parity.json, and 16-validation-commands.log; they are intentionally untracked and must be regenerated from the recorded commands when needed
Product/runtime claims: local Rust logging-core and agent-service dev-log delegation only
Known gaps/manual-required states: no production telemetry, agent-run, or DuckDB-wrapper claim
```
