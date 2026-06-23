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

## External route-check handoff contract

The remaining root validation failure is not proof that the logging-owned proof-inventory and wrapper work failed. It is a delegated route-check gap.

Required handoff fields:

```text
handoff_id
blocking_check
owning_surface
expected_owner_slice
logging_owned_state
external_owner_state
why_this_does_not_prove_wp06_completion
what_wp06_can_claim_meanwhile
next_validation_command
no_claim
```

Current interpretation:

```text
owning_surface: portal/agent-service route behavior
expected_owner_slice: owning portal/agent-service slice, not this delegated logging proof-inventory slice
logging_owned_state: validation scripts, root wrappers, local evidence smoke, proof-inventory query, and export checks have focused proof
external_owner_state: route-check remains open until the owning slice resolves or narrows it
what_wp06_can_claim_meanwhile: bounded validation/enforcement proof for logging-owned scripts and proof-inventory queries
no_claim: no full validate:logging green claim, no full WP06 closure, no product/runtime logging readiness
```

## Manual-required gaps

This workpack enforces implemented parity. It does not itself implement missing TypeScript/Rust/wrapper files or external portal/agent-service route behavior.

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

Focused checks observed in this checkout:

```text
- npm run validate:logging -> fail at lint:dev-log-routing (`portal dev logger must not post to an unimplemented endpoint`)
- npm run test:logging-evidence -> pass
- npm run lint:logging-exports -> pass
- npm run agent:query -- proof-inventory -> pass
- npm run mcp:logging -- --smoke proof-inventory -> pass
- npx vitest run packages/logging-domain/tests/unit/logging-scripts.test.ts packages/logging-domain/tests/integration/mcp-query-interface.test.ts -> pass
- npm run lint:architecture -- --files scripts/dev/lib/log-query-service.mjs scripts/dev/lib/agent-log-paths.mjs scripts/dev/agent-query.mjs scripts/dev/mcp-logging-server.mjs packages/logging-domain/tests/unit/logging-scripts.test.ts packages/logging-domain/tests/integration/mcp-query-interface.test.ts -> pass
```

What this actually proves:

```text
- the validation scripts and root wrappers exist and run
- local evidence smoke works for controlled pass/fail runs
- logging export shape checks pass
- agent-query and MCP proof-inventory wrappers detect missing/stale proof roots and stale closeout claims through a shared query surface
- focused unit/integration tests verify fixture-based negative cases for missing proof roots, stale checklist/workpack claims, and CLI/MCP parity
- a canonical WP06 proof root now exists in this checkout
```

What this does not yet prove:

```text
- test-results/logging-domain-parity-validation/ exists in this checkout
- full dev-log-routing closure for the portal endpoint expectation
- focused-validation green for the whole workpack while lint:dev-log-routing still fails
- missing proof-root closeout for WP01/WP02/WP04/WP05/WP08/WP09
```

Required next step for truthful closeout:

```text
- hand off lint:dev-log-routing to the owning portal/agent-service slice or narrow that expectation there
- restore or reduce remaining stale proof-inventory claims so proof-inventory wrappers report only real remaining gaps
```

## Workpack completion section

```text
Workpack id and branch:
WP06 Validation and Enforcement
codex/tracking-plan-full-continuation-a

Touched files:
docs/plans/logging-domain-parity/PLAN_STATE.md
docs/plans/logging-domain-parity/NEXT_ACTIONS.md
docs/plans/logging-domain-parity/WORKPACK_INDEX.md
docs/plans/logging-domain-parity/CHECKLIST_INDEX.md
docs/plans/logging-domain-parity/workpacks/06-validation-and-enforcement.md
scripts/dev/lib/agent-log-paths.mjs
scripts/dev/lib/log-query-service.mjs
scripts/dev/agent-query.mjs
scripts/dev/mcp-logging-server.mjs
packages/logging-domain/tests/unit/logging-scripts.test.ts
packages/logging-domain/tests/integration/mcp-query-interface.test.ts
output/logging-domain-parity-proof/06-validation-and-enforcement/*

Validation commands and results:
- npm run validate:logging -> fail at lint:dev-log-routing (`portal dev logger must not post to an unimplemented endpoint`)
- npm run test:logging-evidence -> pass
- npm run lint:logging-exports -> pass
- npm run agent:query -- proof-inventory -> pass
- npm run mcp:logging -- --smoke proof-inventory -> pass
- npx vitest run packages/logging-domain/tests/unit/logging-scripts.test.ts packages/logging-domain/tests/integration/mcp-query-interface.test.ts -> pass
- npm run lint:architecture -- --files scripts/dev/lib/log-query-service.mjs scripts/dev/lib/agent-log-paths.mjs scripts/dev/agent-query.mjs scripts/dev/mcp-logging-server.mjs packages/logging-domain/tests/unit/logging-scripts.test.ts packages/logging-domain/tests/integration/mcp-query-interface.test.ts -> pass

Proof artifacts:
output/logging-domain-parity-proof/06-validation-and-enforcement/00-validation-script-map.json
output/logging-domain-parity-proof/06-validation-and-enforcement/01-negative-checks-proof.json
output/logging-domain-parity-proof/06-validation-and-enforcement/02-root-script-wiring-proof.json
output/logging-domain-parity-proof/06-validation-and-enforcement/03-agent-guidance-proof.md
output/logging-domain-parity-proof/06-validation-and-enforcement/16-validation-commands.log

Product/runtime claims:
- no product/runtime logging readiness claim
- no full logging-domain parity completion claim
- no portal endpoint closure claim

Known gaps/manual-required states:
- lint:dev-log-routing still fails outside this delegated logging-owned boundary
- stale proof-inventory claims remain until their roots are restored or status is reduced
```
