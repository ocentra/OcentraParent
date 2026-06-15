<!-- agent-capsule -->

> Agent Capsule
> Plan: `logging-domain-parity`
> Doc: `Logging Domain Parity Test Proof Expectations`
> Kind: command/test selector.
> Read when: a workpack asks which commands or proof artifacts are expected.
> Stop rule: Run focused commands first; do not jump to full validate unless required.
> Proves: command expectations only.
> Does not prove: implementation completion without matching artifacts.

<!-- /agent-capsule -->

# Logging Domain Parity Test Proof Expectations

## General rule

Use focused commands first. Broader validation is allowed only after focused commands pass or a precise blocker is recorded.

The continuation note `05-codex-continuation-plan.md` adds four useful validation themes that are now folded into this file:

```text
TypeScript: transports, app/test-log retention, and log serialization tests.
Rust: cargo check plus unit tests for the new crate and direct consumers.
Local smoke: prove logs are written, ingested, and queryable end to end.
Negative coverage: missing bridge, missing endpoint, and invalid payload handling.
```

## WP01 Current State and Reference Audit

Expected commands:

```bash
node -e "console.log('audit-only: no source validation required')"
```

Expected proof:

```text
reference file map
parent current state map
live usage map
gap summary
```

## WP02 TypeScript Logging Package Parity

Expected focused commands:

```bash
npm run build --workspace @ocentra-parent/logging-domain
npm run test --workspace @ocentra-parent/logging-domain
npm run test:query --workspace @ocentra-parent/logging-domain -- stats --scope=parent-test
```

If package scripts are not wired yet, run direct script commands and record the transition.

Expected TypeScript coverage:

```text
bridge transport serializes and sends valid payloads
bridge handles invalid payloads without crashing
NDJSON writer writes one object per line
test-log ingest can rebuild and incrementally ingest
app-log retention deletes old local sessions as configured
query script returns stats/failure/search output from DuckDB
existing production-safe contract exports still parse existing fixtures/read models
```

Expected negative checks:

```text
missing bridge script fails parity check
missing export fails export check
Cloudflare default in generic parent scope fails scope check
invalid bridge payload is rejected or reported as invalid without corrupting NDJSON
```

## WP03 Parent Logging Architecture and Routing

Expected focused commands:

```bash
npm run build --workspace @ocentra-parent/logging-domain
npm run test --workspace @ocentra-parent/portal
cargo test -p ocentra-parent-agent-service dev_log
```

Allowed blocker:

```text
portal route receiver cannot be fully tested until WP02 bridge exists
```

If blocked, record the exact missing dependency and do not claim routing complete.

Expected routing coverage:

```text
portal dev-log path has an implemented receiver or bridge route
missing endpoint behavior is explicit and tested
agent-service route into logging-core is mapped or implemented
snapshot endpoint is not documented as the primary log store
```

## WP04 Rust Logging Core Crate

Expected focused commands:

```bash
cargo check -p ocentra-parent-logging-core
cargo test -p ocentra-parent-logging-core
cargo clippy -p ocentra-parent-logging-core --all-targets -- -D warnings
cargo test -p ocentra-parent-agent-service dev_log
npm run test --workspace @ocentra-parent/logging-domain -- dev-log-fixture
```

Expected Rust coverage:

```text
new crate compiles with cargo check
new crate unit tests pass
agent-service direct consumer tests pass
NDJSON writer writes one JSON object per line
artifact writer writes file metadata, sha256, byte length, and line count
secret-like fields are redacted by default helpers
```

Expected parity proof:

```text
Rust fixture deserializes in TypeScript.
TypeScript fixture deserializes in Rust.
```

## WP05 Local Validation Evidence

Expected focused commands:

```bash
npm run agent:run -- node -e "process.exit(0)"
npm run agent:run -- node -e "process.exit(2)"
npm run agent:query -- latest-failures
npm run codex:evidence -- latest-failures
```

Expected artifacts:

```text
stdout.log
stderr.log
metadata.json
agent-run NDJSON
diagnostics NDJSON
artifacts NDJSON
DuckDB rows
compact evidence packet
```

Expected local smoke proof:

```text
controlled passing command is recorded as passed
controlled failing command is recorded as failed
logs are written locally
NDJSON is ingested
query command returns the latest failed run
codex evidence command returns a compact evidence packet
```

## WP06 Validation and Enforcement

Expected focused commands:

```bash
npm run validate:logging
npm run test:logging-evidence
node scripts/check-logging-domain-parity.mjs
node scripts/check-local-evidence-wrapper.mjs
node scripts/check-dev-log-routing.mjs
node scripts/check-logging-exports.mjs
```

Expected negative checks:

```text
remove/rename bridge script in temp copy -> check fails
remove/rename required export in temp copy -> check fails
remove agent wrapper script in temp copy -> check fails
portal dev-log route without receiver -> check fails
missing bridge endpoint -> smoke test reports clear failure
invalid bridge payload -> bridge rejects/reports invalid payload without corrupting stored logs
```

Do not mutate the real branch for negative checks. Use temporary fixtures or script-internal test fixtures.
