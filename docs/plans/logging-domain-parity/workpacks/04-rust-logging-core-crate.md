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

The July 22, 2026 review closeout preserved explicit operation-identity
semantics and added atomic marker repair, committed-record verification,
bounded compacted operation state, full artifact custody including createdAt,
hard-link and crash-safe copy fallback, real Windows publication durability,
and extended UNC coverage. The agent-service dev-log target now runs three
tests, and checked-in CI requires the logging-core all-features gate. The
scoped Rust, fixture, formatting, clippy, architecture, and source-shape gates
passed. The generated directory is intentionally ignored, so its existence is
local reproducible evidence rather than a tracked checkout prerequisite.

The July 23, 2026 exact-head follow-up makes compacted-commit creation and
deletion directory-durable, re-syncs recovered compacted rows before deleting
standalone commits, keeps its `commits.state` journal out of NDJSON discovery,
and indexes compacted bytes with bounded generation-aware process state.
Non-locking producer rows survive both atomic append and failed-sync rollback;
library append and cleanup lifecycle operations share a persistent sidecar
lock. The follow-up also directory-syncs cleanup, recovers a stale primary
artifact temporary on the first attempt, and re-syncs existing metadata
directory entries before accepting artifact replay. The normal gate passes 45
top-level tests; the all-features gate passes 59, including 14 deterministic
recovery and operation-state tests.

The final exact-head repair keeps random one-shot dev-log entries on the
ordinary locked append path instead of retaining operation journals, creates
every fresh artifact hierarchy level with an immediate parent-directory sync,
uses a deterministic lock-owned primary temporary so later processes recover
crash leftovers, and runs health-request durability I/O through Tokio's
blocking boundary. The agent-service Clippy gate and its full test matrix pass.

## Completion record

```text
Workpack id and branch: 04-rust-logging-core-crate; codex/logging-domain-parity-wp04; merged origin/main base dc27a632a852ee5ba5f85dc9188824ca8abe4308; proof-bound source commit dba6df3ab05db3186068ffaf4d57d6c66fcfd83f
Touched files: logging-core NDJSON tail/operation recovery, directory-durable hidden compacted state lifecycle with recovered-row re-sync, indexed bounded generation-aware compacted-journal cache, mixed-producer atomic append custody through failed-sync rollback, persistent-sidecar-locked and directory-synced cleanup, one-shot dev logging without retained operation journals, artifact publication/durability and custody replay including deterministic cross-process stale-primary-temporary recovery plus fully synced hierarchy creation and existing-metadata re-sync, path normalization, visible unit/fault/subprocess coverage, agent-service health blocking-boundary logging and Clippy-clean dev-log tests, checked-in all-features CI gate, bounded logging-plan closeout docs, and five local ignored proof artifacts under output/logging-domain-parity-proof/04-rust-logging-core-crate/
Validation commands and results: PASS cargo fmt for logging-core and agent-service with `--check`; PASS cargo check -p ocentra-parent-logging-core with and without `test-support`; PASS cargo test -p ocentra-parent-logging-core --all-targets (45 top-level tests); PASS cargo test -p ocentra-parent-logging-core --all-targets --all-features (59 top-level tests, including 14 deterministic recovery and operation-state tests); PASS cargo clippy -p ocentra-parent-logging-core --all-targets --all-features -- -D warnings; PASS focused large-tail, committed-record custody, marker repair, bounded operation-state lifecycle and generation-aware cache, recovered-row re-sync, indexed hidden-journal lookup, mixed-producer failed-sync preservation, persistent-sidecar-locked and directory-synced cleanup, deterministic stale-primary-temporary recovery, fully synced artifact hierarchy creation, corrupted-custody-field, UNC-path, crash-safe artifact fallback, subprocess-custody, and persisted-redaction tests; PASS cargo test -p ocentra-parent-agent-service --test dev_log (3/3); PASS cargo test -p ocentra-parent-agent-service --all-targets; PASS cargo clippy -p ocentra-parent-agent-service --all-targets -- -D warnings; PASS npm run test --workspace @ocentra-parent/logging-domain -- dev-log-fixture (2/2); PASS npm run lint:architecture -- --files crates/logging-core crates/agent-service; PASS source-shape; PASS git diff --check
Proof artifacts: regenerated locally against source commit dba6df3ab05db3186068ffaf4d57d6c66fcfd83f as 00-rust-crate-file-map.json, 01-rust-ndjson-writer-proof.json, 02-artifact-writer-proof.json, 03-ts-rust-fixture-parity.json, and 16-validation-commands.log; they are intentionally untracked and must be regenerated from the recorded commands when needed
Product/runtime claims: local Rust logging-core and agent-service dev-log delegation only
Known gaps/manual-required states: no production telemetry, agent-run, or DuckDB-wrapper claim
```
