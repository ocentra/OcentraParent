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
docs/plans/logging-domain-parity/05-codex-continuation-plan.md
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

- [x] `check-logging-domain-parity.mjs` added.
- [x] `check-local-evidence-wrapper.mjs` added.
- [x] `check-dev-log-routing.mjs` added.
- [x] `check-logging-exports.mjs` added.
- [x] Root scripts added.
- [x] Validation chain updated at safe point.
- [x] Logging evidence smoke script added.
- [x] Agent guidance references wrapper usage.
- [x] Negative/failure checks verified.
- [x] Focused validation passes.
- [x] Proof root written.
- [x] Workpack completion section filled.

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
missing endpoint behavior is explicit and covered by smoke/negative tests
```

`check-logging-exports.mjs` verifies:

```text
required logging-domain exports exist
existing production contract exports remain available
```

## Additional negative coverage from continuation note

`05-codex-continuation-plan.md` adds three explicit negative checks. Include them in WP06:

```text
missing bridge is detected
missing endpoint is detected
invalid payload is rejected or reported without corrupting stored logs
```

These checks must use temporary fixtures or script-internal fixtures, not destructive edits to the real branch.

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

## Completion

Workpack id and branch:
WP06 on `codex/tracking-plan-full-continuation-a`

Touched files:
`package.json`
`packages/logging-domain/package.json`
`packages/logging-domain/src/test-log/bridgeConvert.ts`
`packages/logging-domain/src/test-log/logsTree.ts`
`scripts/check-logging-domain-parity.mjs`
`scripts/check-local-evidence-wrapper.mjs`
`scripts/check-dev-log-routing.mjs`
`scripts/check-logging-exports.mjs`
`scripts/test/logging-local-evidence-smoke.mjs`
`docs/plans/logging-domain-parity/CHECKLIST_INDEX.md`
`docs/plans/logging-domain-parity/NEXT_ACTIONS.md`
`docs/plans/logging-domain-parity/PLAN_STATE.md`
`docs/plans/logging-domain-parity/WORKPACK_INDEX.md`
`docs/plans/logging-domain-parity/workpacks/06-validation-and-enforcement.md`

Validation commands and results:
`npm run build --workspace @ocentra-parent/logging-domain` passed.
`npm run lint:dev-log-routing` initially failed during the re-audit because `scripts/check-dev-log-routing.mjs` still assumed the old direct `sendToBridge(` portal path.
`npm run validate:logging` passed after updating `scripts/check-dev-log-routing.mjs` to accept either the direct bridge path or the current shared logger bridge path (`sendPortalLoggerMessage(` plus `portalLogger.register(import.meta.url)` and `portalLogger.flush()`).
`npm run test:logging-evidence` passed.
`npm run lint:architecture -- --files package.json packages/logging-domain/package.json packages/logging-domain/src/test-log/bridgeConvert.ts packages/logging-domain/src/test-log/logsTree.ts scripts/check-logging-domain-parity.mjs scripts/check-local-evidence-wrapper.mjs scripts/check-dev-log-routing.mjs scripts/check-logging-exports.mjs scripts/test/logging-local-evidence-smoke.mjs` passed.
`npx vitest run packages/parent-domain/tests/logging/parent-domain-logger-consumer.test.ts --config packages/parent-domain/vitest.config.ts` passed.
Temporary negative fixtures proved missing bridge detection and missing endpoint detection, and a live bridge instance proved invalid payload rejection without stored-log corruption.

Proof artifacts:
`output/logging-domain-parity-proof/06-validation-and-enforcement/00-validation-script-map.json`
`output/logging-domain-parity-proof/06-validation-and-enforcement/01-negative-checks-proof.json`
`output/logging-domain-parity-proof/06-validation-and-enforcement/02-root-script-wiring-proof.json`
`output/logging-domain-parity-proof/06-validation-and-enforcement/03-agent-guidance-proof.md`
`output/logging-domain-parity-proof/06-validation-and-enforcement/16-validation-commands.log`

Product/runtime claims:
Root validation now has dedicated logging-parity, local-evidence, dev-log-routing, and export checks plus a reusable `validate:logging` entrypoint.
The root `validate` chain now includes both `validate:logging` and `test:logging-evidence` at a safe point after the broader lint gate.
Parent local-evidence smoke now proves `agent:run`, `agent:query`, and `codex:evidence` work together end to end with artifact files, NDJSON streams, DuckDB ingest, and compact evidence output.
WP06 also closed one real parity gap by adding and exporting `packages/logging-domain/src/test-log/bridgeConvert.ts` and `packages/logging-domain/src/test-log/logsTree.ts`, which the new export check now enforces.
The WP06 re-audit also closed a stale enforcement gap: `scripts/check-dev-log-routing.mjs` now validates the current portal `DevLogger` contract instead of only the older direct `sendToBridge(` path.

Known gaps/manual-required states:
WP06 enforces the currently implemented logging parity surfaces only; it does not imply a full `@ocentra-parent/portal` workspace build because unrelated non-logging portal type errors still exist outside this plan scope.
The negative proofs use temporary fixtures and a live temp bridge runtime by design; they do not mutate branch source files to demonstrate failure cases.
