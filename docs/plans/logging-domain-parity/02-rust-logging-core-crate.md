# Rust Logging Core Crate

## Purpose

OcentraParent is Rust-heavy. TypeScript-only logging-domain parity is not enough.

Add a Rust crate:

```text
crates/logging-core
package name: ocentra-parent-logging-core
```

This crate is the Rust-side local logging primitive for agent-service, Rust tests, validation wrappers, and future Rust runtime modules.

## Current problem

`crates/agent-service/src/dev_log.rs` currently owns a one-off writer:

```text
write_agent_info(message, fields)
  -> create DevLogEntry
  -> resolve .logs/dev
  -> append agent-service-YYYY-MM-DD.ndjson
```

That is not reusable. It is not connected to command-run diagnostics. It is not aligned with the parent TypeScript logging-domain or the future DuckDB query surface.

## Required crate layout

Create:

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

Add it to the workspace members.

## Cargo setup

Use minimal dependencies.

```toml
[package]
name = "ocentra-parent-logging-core"
version.workspace = true
edition.workspace = true
license.workspace = true

[lints]
workspace = true

[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
sha2 = "0.10"
```

Optional only if already approved in the workspace:

```toml
uuid = { version = "1", features = ["v4"] }
time = { version = "0.3", features = ["formatting"] }
```

If dependency policy blocks these, use existing timestamp/id helpers until an approved helper exists.

## Hard rules

```text
- forbid unsafe code
- no panic/unwrap/expect in library code
- no stdout/stderr writes from library code
- all file writes return Result
- paths are local by default
- no network transport in this crate
- no global mutable singleton unless protected and justified
- no product-policy decisions in this crate
```

This crate is local infrastructure. It writes local development and validation artifacts. It does not make production telemetry decisions.

## Core event types

### Log level

```rust
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}
```

### Log source

```rust
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum LogSource {
    #[serde(rename = "agent-service")]
    AgentService,
    #[serde(rename = "dev-server")]
    DevServer,
    #[serde(rename = "local-api")]
    LocalApi,
    #[serde(rename = "portal")]
    Portal,
    #[serde(rename = "codex")]
    Codex,
    #[serde(rename = "validation")]
    Validation,
    #[serde(rename = "rust-test")]
    RustTest,
}
```

### Log fields

```rust
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LogFieldValue {
    String(String),
    Number(f64),
    Boolean(bool),
    Null(()),
}

pub type LogFields = std::collections::BTreeMap<String, LogFieldValue>;
```

### ParentLogEvent

```rust
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentLogEvent {
    pub schema_version: u16,
    pub id: String,
    pub timestamp: String,
    pub level: LogLevel,
    pub source: LogSource,
    pub message: String,
    pub fields: LogFields,
    pub run_id: Option<String>,
    pub lane_id: Option<String>,
    pub command_id: Option<String>,
    pub correlation_id: Option<String>,
    pub file: Option<String>,
    pub line: Option<u32>,
    pub column: Option<u32>,
}
```

### AgentRunEvent

```rust
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentRunEvent {
    pub schema_version: u16,
    pub event_type: String,
    pub run_id: String,
    pub command_id: String,
    pub lane_id: Option<String>,
    pub machine: Option<String>,
    pub workspace: String,
    pub cwd: String,
    pub command: Vec<String>,
    pub started_at: String,
    pub ended_at: String,
    pub duration_ms: u64,
    pub status: RunStatus,
    pub exit_code: Option<i32>,
    pub stdout_artifact: Option<String>,
    pub stderr_artifact: Option<String>,
    pub summary: Option<String>,
}
```

### RunStatus

```rust
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RunStatus {
    Passed,
    Failed,
    Timeout,
    Cancelled,
    Unknown,
}
```

### AgentDiagnostic

```rust
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentDiagnostic {
    pub schema_version: u16,
    pub event_type: String,
    pub diagnostic_id: String,
    pub run_id: String,
    pub command_id: String,
    pub kind: DiagnosticKind,
    pub severity: DiagnosticSeverity,
    pub signature: String,
    pub file: Option<String>,
    pub line: Option<u32>,
    pub column: Option<u32>,
    pub message: String,
    pub raw_artifact: Option<String>,
    pub raw_start_line: Option<u32>,
    pub raw_end_line: Option<u32>,
}
```

### DiagnosticKind

```rust
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DiagnosticKind {
    Rustc,
    Clippy,
    CargoTest,
    TypeScript,
    Eslint,
    NpmScript,
    ArchitecturePolicy,
    NoReexportPolicy,
    Unknown,
}
```

### ArtifactRef

```rust
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtifactRef {
    pub schema_version: u16,
    pub event_type: String,
    pub artifact_id: String,
    pub run_id: String,
    pub command_id: String,
    pub path: String,
    pub kind: ArtifactKind,
    pub sha256: String,
    pub byte_length: u64,
    pub line_count: u64,
    pub created_at: String,
}
```

## Writer API

Expose a generic append writer:

```rust
pub struct NdjsonWriter {
    root: PathBuf,
}

impl NdjsonWriter {
    pub fn new(root: impl Into<PathBuf>) -> Self;
    pub fn append_event<T: Serialize>(&self, scope: &str, stream: &str, event: &T) -> std::io::Result<PathBuf>;
}
```

Path format:

```text
<root>/<scope>/ndjson/<stream>/<YYYY-MM-DD>.ndjson
```

Examples:

```text
.logs/parent-agent/ndjson/dev-log/2026-06-14.ndjson
.logs/parent-codex/ndjson/agent-run/2026-06-14.ndjson
.logs/parent-codex/ndjson/diagnostics/2026-06-14.ndjson
```

## Artifact API

Expose a local artifact writer:

```rust
pub struct ArtifactWriter {
    root: PathBuf,
}

impl ArtifactWriter {
    pub fn new(root: impl Into<PathBuf>) -> Self;
    pub fn write_text_artifact(
        &self,
        scope: &str,
        run_id: &str,
        command_id: &str,
        kind: ArtifactKind,
        content: &str,
    ) -> std::io::Result<ArtifactRef>;
}
```

Path format:

```text
<root>/<scope>/artifacts/<runId>/<commandId>/stdout.log
<root>/<scope>/artifacts/<runId>/<commandId>/stderr.log
<root>/<scope>/artifacts/<runId>/<commandId>/metadata.json
```

## Environment variables

Define or reuse constants for:

```text
OCENTRA_PARENT_LOG_ROOT
OCENTRA_PARENT_LOG_SCOPE
OCENTRA_PARENT_DEV_LOG_DIR        # compatibility only; prefer LOG_ROOT
OCENTRA_PARENT_CODEX_RUN_ID
OCENTRA_PARENT_CODEX_LANE_ID
```

Log root resolution order:

```text
1. OCENTRA_PARENT_LOG_ROOT
2. repo-local .logs
3. current working directory .logs
```

Do not default to user-global paths.

## Migration from agent-service

Current:

```rust
crate::dev_log::write_agent_info(...)
```

Target:

```rust
use ocentra_parent_logging_core::{DevLogger, LogFields, LogSource};

let logger = DevLogger::from_env(LogSource::AgentService);
let _ = logger.info(constants::dev_log_message::AGENT_SERVICE_STARTED, LogFields::new());
```

Then remove or shrink `crates/agent-service/src/dev_log.rs` to a compatibility wrapper only.

## Snapshot behavior

The crate may provide snapshot helpers, but snapshots are not the primary log store.

Allowed:

```text
/api/dev/log-snapshot returns current process status + recent state
```

Not allowed:

```text
snapshot endpoint replaces NDJSON/DuckDB query history
```

## Tests required

Add tests for:

```text
- writes one JSON object per line
- creates directories recursively
- rejects or normalizes invalid path segments
- preserves append order
- serializes LogLevel and LogSource exactly as TypeScript contracts expect
- writes artifact and computes sha256/line count
- redacts obvious secret-like field names
- agent-service can write startup log through logging-core
- golden fixture: Rust DevLogEntry JSON parses with TypeScript DevLogEntrySchema
```

## Golden fixture parity

Create fixtures:

```text
packages/logging-domain/fixtures/dev-log-entry.json
crates/logging-core/tests/fixtures/dev-log-entry.json
```

Required tests:

```text
TypeScript: parse Rust fixture through DevLogEntrySchema.
Rust: deserialize TypeScript fixture through Rust structs.
```

## Acceptance criteria

```text
cargo test -p ocentra-parent-logging-core passes
cargo test -p ocentra-parent-agent-service passes
npm run build --workspace @ocentra-parent/logging-domain passes
npm run test --workspace @ocentra-parent/logging-domain passes
agent-service no longer owns an isolated full dev-log writer
Rust/TypeScript log fixture parity tests exist
```
