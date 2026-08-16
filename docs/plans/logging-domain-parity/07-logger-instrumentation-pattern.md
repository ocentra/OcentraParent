# Logger Instrumentation Pattern for Logging Domain Parity

## Purpose

The logging-domain parity plan must define how source files use the logger, not only how the logging package, NDJSON, DuckDB, CLI, and MCP are built.

The useful end state is:

```text
source file uses shared logger pattern
  -> logger captures context/source/stack
  -> bridge or Rust writer stores structured rows
  -> NDJSON/DuckDB index rows
  -> CLI/MCP query exact evidence
```

Without consistent instrumentation, MCP and DuckDB will exist but contain weak evidence.

## Reference behavior from games

The games repo has a dedicated Cloudflare logging rule requiring every new handler, Durable Object, service, and test file to use a shared logger pattern. The rule requires:

```text
- shared Logger import
- getStackTrace import
- log.register(import.meta.url)
- helper methods: logInfo, logWarn, logError, logDebug
- getStackTrace() passed to each helper call
- logs at entry, branch points, errors, and important successes
```

Parent should adapt this pattern to its own runtime boundaries.

## Parent target surfaces

Apply the pattern to new or touched files in these parent surfaces:

```text
apps/portal/src/**
packages/logging-domain/src/**
packages/*-domain/src/** when the package emits operational logs
crates/agent-service/src/**
crates/logging-core/src/**
future parent Cloudflare infra files
scripts/dev/** validation/evidence wrappers
```

Do not add noisy logs everywhere. Add logs at useful evidence points.

## TypeScript pattern

Use a shared logging-domain import path once implemented.

Preferred shape for module-level TypeScript files:

```ts
const log = Logger.instance;
log.register(import.meta.url);

const logInfo = (message: string, stackTrace: StackTrace, data?: unknown, enabled: boolean = false) => {
  log.logInfo(message, stackTrace, data, enabled);
};

const logWarn = (message: string, stackTrace: StackTrace, data?: unknown, enabled: boolean = false) => {
  log.logWarn(message, stackTrace, data, enabled);
};

const logError = (message: string, stackTrace: StackTrace, data?: unknown) => {
  log.logError(message, stackTrace, data);
};

const logDebug = (message: string, stackTrace: StackTrace, data?: unknown, enabled: boolean = false) => {
  log.logDebug(message, stackTrace, data, enabled);
};
```

Preferred shape for class-based TypeScript runtime files:

```ts
private readonly log = Logger.instance;

constructor(/* args */) {
  this.log.register(import.meta.url);
}

private logInfo = (message: string, stackTrace: StackTrace, data?: unknown, enabled: boolean = false) => {
  this.log.logInfo(message, stackTrace, data, enabled);
};

private logWarn = (message: string, stackTrace: StackTrace, data?: unknown, enabled: boolean = false) => {
  this.log.logWarn(message, stackTrace, data, enabled);
};

private logError = (message: string, stackTrace: StackTrace, data?: unknown) => {
  this.log.logError(message, stackTrace, data);
};

private logDebug = (message: string, stackTrace: StackTrace, data?: unknown, enabled: boolean = false) => {
  this.log.logDebug(message, stackTrace, data, enabled);
};
```

Use the actual parent logger import names once WP02/WP03 define them. Do not invent a second logger API.

## Rust pattern

Rust files should not duplicate the TypeScript helper shape. Use the Rust logging-core API from WP04.

Preferred shape:

```rust
let logger = DevLogger::from_env(LogSource::AgentService);
let _ = logger.info("agent service started", LogFields::new());
```

For command/evidence wrappers or service paths, include:

```text
source
message
fields
correlation_id when available
run_id / command_id when available
file / line when available
```

Do not write raw `println!`, `eprintln!`, or ad hoc JSON log lines for runtime diagnostics once logging-core exists.

## Where to log

Log at high-signal locations only:

```text
entry points: request/command/test/run begins
branch points: selected route, validator branch, degraded/manual-required branch
warnings: invalid input, missing optional dependency, stale ingest, unavailable DB
errors: exceptions, failed writes, invalid payloads, rejected artifact access
successes: important persisted row, completed run, completed ingest, query result count
```

Do not log:

```text
loops on every iteration
large raw payloads
full command lines with secrets
raw browser URLs
message contents
screenshots
private activity payloads
```

## Data shape

Use small structured data objects.

Good:

```text
{ runId, commandId, scope, status, durationMs, diagnosticCount }
{ source, context, level, count }
{ route, method, statusCode }
```

Bad:

```text
full stdout
full stderr
raw screenshot
full request body
full browser URL
full command line with flags/secrets
```

Raw output belongs in local artifacts with references, not log fields.

## Test instrumentation

Tests should use the same logging pattern when logs are part of proof or debugging.

For test wrappers and validation scripts, logs should include:

```text
run_id
command_id
suite/test name when available
phase
status
artifact refs
```

## Validation expectations

Add or extend architecture/logging validation to check:

```text
new touched TypeScript runtime files use the parent logger or explicitly justify no logging
new touched Rust service files use logging-core or explicitly justify no logging
no new console.log/error/warn in runtime code
no ad hoc JSON log writer outside logging-core/logging-domain
no raw full stdout/stderr fields in structured log rows
```

## Acceptance criteria

```text
Parent logger usage pattern is documented.
At least one portal/service/script path demonstrates the pattern.
Rust service logging delegates to logging-core.
Validation catches raw console/ad hoc log usage in touched logging surfaces.
MCP/query results show useful source/context fields because files register correctly.
```
