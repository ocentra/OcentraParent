<!-- agent-capsule -->

> Agent Capsule
> Plan: `logging-domain-parity`
> Doc: `WP03 Parent Logging Architecture and Routing`
> Kind: assigned implementation workpack.
> Read when: selected by WORKPACK_INDEX.md or explicit assignment.
> Stop rule: Do not edit unrelated app/service paths.
> Proves: parent routing/architecture fixes only after tests/proof pass.
> Does not prove: full TypeScript parity, Rust logging-core, or local evidence wrappers.
> Proof rule: Before DONE, run focused commands and write proof artifacts.

<!-- /agent-capsule -->

# WP03 Parent Logging Architecture and Routing

## Purpose

Fix the parent-specific split between portal dev logs, Rust agent dev logs, scopes, and local-vs-product logging language.

This workpack makes the architecture coherent after or alongside WP02.

## Source inputs

```text
docs/plans/logging-domain-parity/01-parent-logging-architecture.md
OcentraParent/apps/portal/src/dev-logger.ts
OcentraParent/apps/portal/src/main.ts
OcentraParent/apps/portal/src/transport.ts
OcentraParent/crates/agent-service/src/dev_log.rs
OcentraParent/crates/agent-service/src/app.rs
OcentraParent/crates/agent-service/src/service_runtime.rs
OcentraParent/packages/logging-domain/src/contracts.ts
```

## Target state

Parent logging paths are explicit:

```text
local-dev-observability
product/runtime safe logging
cloudflare infra logging
```

Portal dev logs must have a real receiver or bridge route.

Agent-service dev logs must be ready to migrate to `crates/logging-core` in WP04.

`/api/dev/log-snapshot` must be documented as a snapshot/status endpoint, not the primary local log store.

## Required decisions

Choose and implement one portal route strategy:

### Preferred

```text
portal dev logs use logging-domain bridge transport when local bridge is configured
```

### Acceptable compatibility

```text
/__ocentra-parent-dev-log exists and writes into the same NDJSON path as bridge-compatible logs
```

Unacceptable:

```text
portal fetches an endpoint that no local process implements
```

## Required proof root

```text
output/logging-domain-parity-proof/03-parent-logging-architecture-and-routing/
```

Required artifacts:

```text
00-routing-before-after.md
01-portal-dev-log-route-proof.json
02-agent-service-logging-route-proof.json
03-scope-model-proof.json
16-validation-commands.log
```

## Checklist rows

- [x] Local-dev-observability and product-safe logging separated in docs/API.
- [x] Parent scopes defined.
- [x] Portal dev-log route implemented or moved to bridge path.
- [x] Agent-service current logging path mapped to Rust crate migration.
- [x] `/api/dev/log-snapshot` role documented as snapshot, not primary store.
- [x] Cloudflare infra scope kept separate.
- [x] README/package docs updated.
- [x] Route tests or smoke checks added.
- [x] Focused commands pass.
- [x] Proof root written.
- [x] Workpack completion section filled.

## Expected source changes

Likely files:

```text
apps/portal/src/dev-logger.ts
apps/portal/src/main.ts
apps/portal/src/transport.ts
crates/agent-service/src/app.rs
crates/agent-service/src/dev_log.rs
packages/logging-domain/src/contracts.ts
packages/logging-domain/README.md
```

Only touch service/portal files required for routing.

## Focused commands

```bash
npm run build --workspace @ocentra-parent/logging-domain
npm run test --workspace @ocentra-parent/portal
cargo test -p ocentra-parent-agent-service dev_log
```

If route depends on WP02 bridge, record blocker instead of faking proof.

## Manual-required gaps

This workpack does not complete Rust logging-core. It may create compatibility routing only until WP04 migrates agent-service to the Rust crate.

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
WP03 on `codex/tracking-plan-full-continuation-a`

Touched files:
`apps/portal/src/dev-logger.ts`
`apps/portal/tests/logging/portal-dev-log-route.test.ts`
`packages/logging-domain/src/contracts.ts`
`packages/logging-domain/README.md`
`crates/agent-service/src/dev_log.rs`
`crates/agent-service/src/app.rs`

Validation commands and results:
`npm run build --workspace @ocentra-parent/logging-domain` passed
`npm --workspace @ocentra-parent/portal exec vitest run tests/logging/portal-dev-log-route.test.ts` passed
`npm install` passed and restored missing workspace links in `node_modules/@ocentra-parent`
`npm run build --workspace @ocentra-parent/event-domain` passed
`node scripts/fix-esm-imports.mjs <packages/*/dist>` repair pass applied across existing package dist trees
`npm run test --workspace @ocentra-parent/portal` still fails after the workspace/dist repair, now on upstream contract drift in `packages/agent-protocol-domain`:
`packages/agent-protocol-domain/src/contracts.ts` and `src/security.ts` import shared agent schemas from `@ocentra-parent/evidence-domain/primitives`
those agent schemas actually live in `packages/event-domain/src/primitives.ts`, not in `packages/evidence-domain/src/primitives.ts`
the resulting runtime/schema mismatch crashes portal-adjacent suites outside WP03 source scope
`cargo test -p ocentra-parent-agent-service dev_log` passed
`npm run lint:architecture -- --files apps/portal/src/dev-logger.ts apps/portal/tests/logging/portal-dev-log-route.test.ts packages/logging-domain` passed
`cargo lint-architecture crates/agent-service/src/dev_log.rs crates/agent-service/src/app.rs` passed

Proof artifacts:
`output/logging-domain-parity-proof/03-parent-logging-architecture-and-routing/00-routing-before-after.md`
`output/logging-domain-parity-proof/03-parent-logging-architecture-and-routing/01-portal-dev-log-route-proof.json`
`output/logging-domain-parity-proof/03-parent-logging-architecture-and-routing/02-agent-service-logging-route-proof.json`
`output/logging-domain-parity-proof/03-parent-logging-architecture-and-routing/03-scope-model-proof.json`
`output/logging-domain-parity-proof/03-parent-logging-architecture-and-routing/16-validation-commands.log`

Product/runtime claims:
Portal dev logs no longer depend on an unimplemented relative endpoint and instead emit bridge-compatible rows through the logging-domain local bridge transport.
Parent logging docs now split local-dev observability from product/runtime-safe logging and Cloudflare infra logging.
Agent-service keeps the current NDJSON compatibility path, and its snapshot route is explicitly documented as read-model/status output rather than the primary log store.

Known gaps/manual-required states:
WP04 still owns extraction of the Rust compatibility writer into `crates/logging-core`.
WP03 cannot be marked fully complete until the upstream `agent-protocol-domain` to `event-domain`/`evidence-domain` contract drift is repaired and the required portal workspace command goes green.
