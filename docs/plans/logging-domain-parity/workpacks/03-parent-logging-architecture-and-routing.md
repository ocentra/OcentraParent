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

- [ ] Local-dev-observability and product-safe logging separated in docs/API.
- [ ] Parent scopes defined.
- [ ] Portal dev-log route implemented or moved to bridge path.
- [ ] Agent-service current logging path mapped to Rust crate migration.
- [ ] `/api/dev/log-snapshot` role documented as snapshot, not primary store.
- [ ] Cloudflare infra scope kept separate.
- [ ] README/package docs updated.
- [ ] Route tests or smoke checks added.
- [ ] Focused commands pass.
- [ ] Proof root written.
- [ ] Workpack completion section filled.

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

## Current audit note

Observed current source/test state in this checkout:

```text
- apps/portal/src/dev-logger.ts already routes portal dev logs through the shared logger/bridge path instead of the legacy endpoint
- packages/logging-domain/README.md already documents /api/dev/log-snapshot as a snapshot/status endpoint, not the primary local log store
- crates/agent-service/src/dev_log.rs already delegates to ocentra-parent-logging-core
- focused checks passed:
  * cmd /c npx vitest run apps/portal/tests/logging/portal-dev-log-route.test.ts apps/portal/tests/logging/portal-proof-trace.test.ts apps/portal/tests/logging/portal-proof-trace-pipeline.test.ts
  * cargo test -p ocentra-parent-agent-service dev_log
```

Remaining blocker for full-workpack closeout:

```text
- the portal dev-log consumer slice is now proved in output/logging-domain-parity-proof/03-parent-logging-architecture-and-routing/
- the remaining open row is the Rust-side agent-service-to-logging-core mapping, which this delegated slice intentionally did not widen into
```

## Fill before DONE or PR-ready

```text
Workpack id and branch:
Touched files:
Validation commands and results:
Proof artifacts:
Product/runtime claims:
Known gaps/manual-required states:
```

## Slice completion: logging-wp03-portal-dev-log-consumer-closeout

```text
Workpack id and branch:
- WP03 Parent Logging Architecture and Routing
- codex/tracking-plan-full-continuation-a

Touched files:
- docs/plans/logging-domain-parity/PLAN_STATE.md
- docs/plans/logging-domain-parity/NEXT_ACTIONS.md
- docs/plans/logging-domain-parity/WORKPACK_INDEX.md
- docs/plans/logging-domain-parity/CHECKLIST_INDEX.md
- docs/plans/logging-domain-parity/workpacks/03-parent-logging-architecture-and-routing.md
- output/logging-domain-parity-proof/03-parent-logging-architecture-and-routing/00-routing-before-after.md
- output/logging-domain-parity-proof/03-parent-logging-architecture-and-routing/01-portal-dev-log-route-proof.json
- output/logging-domain-parity-proof/03-parent-logging-architecture-and-routing/02-agent-service-logging-route-proof.json
- output/logging-domain-parity-proof/03-parent-logging-architecture-and-routing/03-scope-model-proof.json
- output/logging-domain-parity-proof/03-parent-logging-architecture-and-routing/16-validation-commands.log

Validation commands and results:
- npm run build --workspace @ocentra-parent/logging-domain -> pass
- cmd /c npx vitest run apps/portal/tests/logging/portal-dev-log-route.test.ts apps/portal/tests/logging/portal-proof-trace.test.ts apps/portal/tests/logging/portal-proof-trace-pipeline.test.ts -> pass
- npm run lint:architecture -- --files apps/portal/src/dev-logger.ts apps/portal/src/main.ts apps/portal/src/transport.ts packages/logging-domain/src/contracts.ts packages/logging-domain/src/test-log/types.ts apps/portal/tests/logging/portal-dev-log-route.test.ts apps/portal/tests/logging/portal-proof-trace.test.ts apps/portal/tests/logging/portal-proof-trace-pipeline.test.ts -> pass

Proof artifacts:
- output/logging-domain-parity-proof/03-parent-logging-architecture-and-routing/00-routing-before-after.md
- output/logging-domain-parity-proof/03-parent-logging-architecture-and-routing/01-portal-dev-log-route-proof.json
- output/logging-domain-parity-proof/03-parent-logging-architecture-and-routing/02-agent-service-logging-route-proof.json
- output/logging-domain-parity-proof/03-parent-logging-architecture-and-routing/03-scope-model-proof.json
- output/logging-domain-parity-proof/03-parent-logging-architecture-and-routing/16-validation-commands.log

Product/runtime claims:
- none
- this slice proves only the portal dev-log consumer path, parent scope model, and snapshot-endpoint documentation boundary

Known gaps/manual-required states:
- the agent-service current logging path mapped to Rust crate migration row remains open
- this slice intentionally did not read or validate crates/agent-service/** or crates/logging-core/**
- full WP03 closeout must happen in a later Rust-owned slice or by reducing that row to the correct owner boundary
```
