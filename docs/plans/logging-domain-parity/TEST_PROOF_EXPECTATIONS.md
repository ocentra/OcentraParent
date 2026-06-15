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

Expected negative checks:

```text
missing bridge script fails parity check
missing export fails export check
Cloudflare default in generic parent scope fails scope check
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

## WP04 Rust Logging Core Crate

Expected focused commands:

```bash
cargo test -p ocentra-parent-logging-core
cargo clippy -p ocentra-parent-logging-core --all-targets -- -D warnings
cargo test -p ocentra-parent-agent-service dev_log
npm run test --workspace @ocentra-parent/logging-domain -- dev-log-fixture
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
```

Do not mutate the real branch for negative checks. Use temporary fixtures or script-internal test fixtures.
