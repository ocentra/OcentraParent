<!-- agent-capsule -->

> Agent Capsule
> Plan: `logging-domain-parity`
> Doc: `WP06 Validation and Enforcement`
> Kind: assigned implementation workpack.
> Read when: selected by WORKPACK_INDEX.md or explicit assignment.
> Stop rule: Do not wire root validation before target files exist.
> Proves: parity enforcement only after negative checks/proof pass.
> Does not prove: implementation of missing files by itself.
> Proof rule: Before DONE, run focused positive and negative checks and write proof artifacts.

<!-- /agent-capsule -->

# WP06 Validation and Enforcement

## Purpose

Make logging-domain parity enforceable.

Codex must not treat the plan as optional guidance. Required files, exports, route fixes, local evidence wrappers, and validation smoke tests must be checked by scripts.

## Source inputs

```text
docs/plans/logging-domain-parity/04-validation-and-enforcement.md
package.json
packages/logging-domain/package.json
packages/logging-domain/src/**
crates/logging-core/**
crates/agent-service/**
apps/portal/src/dev-logger.ts
scripts/dev/**
scripts/check-*.mjs
```

## Dependency gate

Run this workpack after the implementation files exist.

Do not add root validation first if it will fail because planned files are not implemented yet.

## Target state

Validation scripts exist:

```text
scripts/check-logging-domain-parity.mjs
scripts/check-local-evidence-wrapper.mjs
scripts/check-dev-log-routing.mjs
scripts/check-logging-exports.mjs
```

Root scripts exist:

```json
{
  "lint:logging-parity": "node scripts/check-logging-domain-parity.mjs",
  "lint:local-evidence": "node scripts/check-local-evidence-wrapper.mjs",
  "lint:dev-log-routing": "node scripts/check-dev-log-routing.mjs",
  "lint:logging-exports": "node scripts/check-logging-exports.mjs",
  "validate:logging": "npm run lint:logging-parity && npm run lint:local-evidence && npm run lint:dev-log-routing && npm run lint:logging-exports",
  "test:logging-evidence": "node scripts/test/logging-local-evidence-smoke.mjs"
}
```

## Required proof root

```text
output/logging-domain-parity-proof/06-validation-and-enforcement/
```

Required artifacts:

```text
00-validation-script-map.json
01-negative-checks-proof.json
02-root-script-wiring-proof.json
03-agent-guidance-proof.md
16-validation-commands.log
```

## Checklist rows

- [ ] `check-logging-domain-parity.mjs` added.
- [ ] `check-local-evidence-wrapper.mjs` added.
- [ ] `check-dev-log-routing.mjs` added.
- [ ] `check-logging-exports.mjs` added.
- [ ] Root scripts added.
- [ ] Validation chain updated at safe point.
- [ ] Logging evidence smoke script added.
- [ ] Agent guidance references wrapper usage.
- [ ] Negative/failure checks verified.
- [ ] Focused validation passes.
- [ ] Proof root written.
- [ ] Workpack completion section filled.

## Required checks

`check-logging-domain-parity.mjs` verifies:

```text
bridge/db/query scripts exist
src/test-log exists
src/transport exists
src/app-log exists or explicit deferral exists
no generic Cloudflare default scope
```

`check-local-evidence-wrapper.mjs` verifies:

```text
agent:run / agent:query / codex:evidence root scripts exist
scripts/dev implementation files exist
agent guidance says to use wrappers for validation evidence
```

`check-dev-log-routing.mjs` verifies:

```text
portal dev logger has implemented receiver or bridge path
agent-service delegates to logging-core after migration
snapshot endpoint is not documented as primary log store
```

`check-logging-exports.mjs` verifies:

```text
required logging-domain exports exist
existing production contract exports remain available
```

## Focused commands

```bash
npm run validate:logging
npm run test:logging-evidence
node scripts/check-logging-domain-parity.mjs
node scripts/check-local-evidence-wrapper.mjs
node scripts/check-dev-log-routing.mjs
node scripts/check-logging-exports.mjs
```

Negative checks must use temporary fixtures or script-internal fixtures. Do not mutate real source files to prove failure cases.

## Manual-required gaps

This workpack enforces implemented parity. It does not itself implement missing TypeScript/Rust/wrapper files.

## Fill before DONE or PR-ready

```text
Workpack id and branch:
Touched files:
Validation commands and results:
Proof artifacts:
Product/runtime claims:
Known gaps/manual-required states:
```
