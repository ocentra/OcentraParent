<!-- agent-capsule -->

> Agent Capsule
> Plan: `logging-domain-parity`
> Doc: `WP02 TypeScript Logging Package Parity`
> Kind: assigned implementation workpack.
> Read when: selected by WORKPACK_INDEX.md or explicit assignment.
> Stop rule: Do not open sibling workpacks unless a dependency is named here.
> Proves: TypeScript package parity only after tests/proof pass.
> Does not prove: Rust logging-core, local validation wrappers, or production readiness.
> Proof rule: Before DONE, run focused commands and write proof artifacts.

<!-- /agent-capsule -->

# WP02 TypeScript Logging Package Parity

## Purpose

Bring `packages/logging-domain` toward the mature games TypeScript package shape:

```text
test-log
transport
app-log
scripts
exports
scopes
```

This workpack is the first real implementation slice.

## Source inputs

```text
docs/plans/logging-domain-parity/00-current-state-and-reference-audit.md
docs/plans/logging-domain-parity/01-parent-logging-architecture.md
docs/plans/logging-domain-parity/05-codex-continuation-plan.md
ocentra-games/packages/logging-domain/package.json
ocentra-games/packages/logging-domain/src/test-log/**
ocentra-games/packages/logging-domain/src/transport/**
ocentra-games/packages/logging-domain/src/app-log/**
ocentra-games/packages/logging-domain/scripts/**
OcentraParent/packages/logging-domain/package.json
OcentraParent/packages/logging-domain/src/**
```

## Target state

Parent logging-domain has TypeScript parity modules and scripts adapted to parent scopes.

## Accepted source-wave reconciliation (2026-08-17)

The accepted source head `735df89de` establishes the TypeScript edge of the
redaction contract without closing this workpack. Rust owns the canonical
18-entry sensitive-key policy in
`crates/logging-core/src/redaction_policy.rs`; its checked-in generated output
is `packages/logging-domain/src/generated-log-redaction-policy.ts`, and
`packages/logging-domain/src/core/log-redaction.ts` consumes that artifact.
There is no alternate TypeScript-local sensitive-key list or regex policy.

This is source evidence only. The parity test, focused validation, proof-root,
checklist, and completion rows remain deferred and unchecked.

Required module groups:

```text
packages/logging-domain/src/test-log/
packages/logging-domain/src/transport/
packages/logging-domain/src/app-log/
packages/logging-domain/scripts/
```

Required scripts:

```text
bridge
db:ensure
db:rebuild
db:ingest
logs:prepare
test:query
view:ndjson
```

## Scope rules

Required parent scopes:

```text
parent-agent
parent-portal
parent-cloudflare
parent-codex
parent-test
```

Do not hardcode generic parent logging to `cloudflare`.

Cloudflare can exist only as explicit `parent-cloudflare` scope.

## Required proof root

```text
output/logging-domain-parity-proof/02-typescript-logging-package-parity/
```

Required artifacts:

```text
00-package-export-before-after.json
01-typescript-parity-file-map.json
02-scope-defaults-proof.json
03-query-script-smoke.json
16-validation-commands.log
```

## Checklist rows

- [ ] `src/test-log` parity modules added/adapted.
- [ ] `src/transport` parity modules added/adapted.
- [ ] `src/app-log` parity modules added/adapted or explicit deferral recorded.
- [ ] `scripts/log-bridge.ts` added.
- [ ] DB ensure/rebuild/ingest/query/view scripts added.
- [ ] Package exports updated explicitly.
- [ ] Existing proof/contract exports preserved.
- [ ] Parent scopes added without generic Cloudflare default.
- [ ] TypeScript tests added/updated.
- [ ] Focused package build/test commands pass.
- [ ] Proof root written.
- [ ] Workpack completion section filled.

## Expected source changes

Likely files:

```text
packages/logging-domain/package.json
packages/logging-domain/src/test-log/**
packages/logging-domain/src/transport/**
packages/logging-domain/src/app-log/**
packages/logging-domain/scripts/**
packages/logging-domain/tests/**
```

Do not remove existing exports unless migration is documented and tests prove no consumers break.

## Additional validation from continuation note

`05-codex-continuation-plan.md` adds useful TypeScript validation details. Include them in this workpack:

```text
transport serialization and send behavior
invalid bridge payload handling
app/test-log retention behavior
NDJSON serialization shape
DuckDB ingest/query smoke
existing production-safe contract exports still parse
```

Do not treat this as a new workpack. It strengthens WP02 validation.

## Focused commands

```bash
npm run build --workspace @ocentra-parent/logging-domain
npm run test --workspace @ocentra-parent/logging-domain
```

If new query scripts are ready:

```bash
npm run test:query --workspace @ocentra-parent/logging-domain -- stats --scope=parent-test
```

## Manual-required gaps

This workpack does not implement Rust logging-core or validation wrappers. Those belong to WP04/WP05.

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

`packages/logging-domain` now contains the TypeScript package surface this
workpack expected, including the `core`, `test-log`, `transport`, and
`app-log` areas plus targeted unit and integration tests. The June 16, 2026
audit also re-verified focused package coverage through
`packages/logging-domain/tests/integration/mcp-query-interface.test.ts`,
`packages/logging-domain/tests/unit/logger.test.ts`, and
`packages/logging-domain/tests/unit/dev-log-fixture.test.ts`.

The appended completion block was still overstated, because the named proof root
`output/logging-domain-parity-proof/02-typescript-logging-package-parity/` is
absent in this checkout and the plan checklist remains unchecked. Treat WP02 as
source-present but not durably proved complete from current workspace evidence.

## Accepted source and expected-test delta (2026-08-17)

The reviewed source at `720609306` adds one canonical structured-redaction
policy, explicit package exports, JSON-safe markers for unsupported values,
cycle/reflection failure containment, and native Date/URL/custom-`toJSON`
handling. No test file changed. Before focused execution, add dedicated
redaction coverage for nested secrets, arrays, cycles, unsupported primitives
and objects, throwing getters/proxies/`toJSON`, root/property/index key
semantics, one-call behavior, and package-export import resolution. This is
local logging package hardening, not product telemetry readiness.
