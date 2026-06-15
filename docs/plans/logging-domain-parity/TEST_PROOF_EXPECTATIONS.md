<!-- agent-capsule -->

> Agent Capsule
> Plan: `logging-domain-parity`
> Doc: `Logging Domain Parity Test and Proof Expectations`
> Kind: command expectation router.
> Read when: validating a selected workpack or auditing plan completeness.
> Stop rule: Use the selected workpack first; this file summarizes expected command families only.
> Proves: validation routing only.
> Does not prove: completion by itself.

<!-- /agent-capsule -->

# Logging Domain Parity Test and Proof Expectations

Use this file to map each workpack to its minimum command families and expected proof root.

## WP01

Expected commands:

```text
games-reference file inspection
parent current-state inspection
existing MCP audit commands
```

Expected proof root:

```text
output/logging-domain-parity-proof/01-current-state-and-reference-audit/
```

## WP02

Expected commands:

```text
npm run build --workspace @ocentra-parent/logging-domain
npm run test --workspace @ocentra-parent/logging-domain
npm run lint:architecture -- --files <touched logging-domain files>
```

Expected proof root:

```text
output/logging-domain-parity-proof/02-typescript-logging-package-parity/
```

## WP03

Expected commands:

```text
portal route-focused tests or smokes
agent-service route-focused tests or smokes
npm run lint:architecture -- --files <touched portal/logging files>
```

Expected proof root:

```text
output/logging-domain-parity-proof/03-parent-logging-architecture-and-routing/
```

## WP09

Expected commands:

```text
npm run test --workspace @ocentra-parent/logging-domain -- retention
npm run test --workspace @ocentra-parent/logging-domain -- bridge
npm run lint:architecture -- --files <touched logging-domain lifecycle files>
```

Expected proof root:

```text
output/logging-domain-parity-proof/09-log-control-retention-bridge-lifecycle/
```

## WP04

Expected commands:

```text
cargo check -p ocentra-parent-logging-core
cargo test -p ocentra-parent-logging-core
cargo clippy -p ocentra-parent-logging-core --all-targets -- -D warnings
cargo test -p ocentra-parent-agent-service dev_log
npm run test --workspace @ocentra-parent/logging-domain -- dev-log-fixture
```

Expected proof root:

```text
output/logging-domain-parity-proof/04-rust-logging-core-crate/
```

## WP05

Expected commands:

```text
npm run agent:run -- <validation command>
npm run agent:query -- <query>
npm run codex:evidence -- <query>
npm run test:logging-evidence
```

Expected proof root:

```text
output/logging-domain-parity-proof/05-local-validation-evidence/
```

## WP07

Expected commands:

```text
npm run --silent mcp:logging -- --list-tools
npm run --silent mcp:logging -- --smoke latest-failures
npm run --silent mcp:logging -- --smoke run-diagnostics
npm run --silent mcp:logging -- --smoke artifact-slice
npm run --silent agent:query -- latest-failures
npm run --silent codex:evidence -- latest-failures
```

Expected proof root:

```text
output/logging-domain-parity-proof/07-mcp-query-interface/
```

## WP08

Expected commands:

```text
npm run build --workspace @ocentra-parent/logging-domain
npm run test --workspace @ocentra-parent/logging-domain
cargo test -p ocentra-parent-logging-core
cargo test -p ocentra-parent-agent-service dev_log
npx vitest run packages/parent-domain/tests/logging/parent-domain-logger-consumer.test.ts --config packages/parent-domain/vitest.config.ts
npx vitest run tests/logging/portal-dev-log-route.test.ts tests/logging/portal-proof-trace.test.ts
```

Expected proof root:

```text
output/logging-domain-parity-proof/08-logger-instrumentation-and-adoption/
```

## WP10

Expected commands:

```text
npx vitest run tests/logging/portal-proof-trace-pipeline.test.ts tests/logging/portal-proof-trace.test.ts tests/logging/portal-dev-log-route.test.ts
MCP proof-trace smoke through scripts/dev/mcp-logging-server.mjs or an equivalent client harness
CLI proof-trace query through scripts/dev/agent-query.mjs
```

Expected proof root:

```text
output/logging-domain-parity-proof/10-proof-trace-pipeline/
```

## WP06

Expected commands:

```text
npm run validate:logging
npm run test:logging-evidence
node scripts/check-logging-domain-parity.mjs
node scripts/check-local-evidence-wrapper.mjs
node scripts/check-dev-log-routing.mjs
node scripts/check-logging-exports.mjs
negative fixture checks for missing bridge, missing endpoint, and invalid payload rejection
```

Expected proof root:

```text
output/logging-domain-parity-proof/06-validation-and-enforcement/
```
