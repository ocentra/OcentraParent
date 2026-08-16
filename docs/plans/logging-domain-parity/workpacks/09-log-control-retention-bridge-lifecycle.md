<!-- agent-capsule -->

> Agent Capsule
> Plan: `logging-domain-parity`
> Doc: `WP09 Log Control, Retention, and Bridge Lifecycle`
> Kind: assigned implementation workpack.
> Read when: selected by WORKPACK_INDEX.md or explicit assignment.
> Stop rule: Do not implement MCP/query tools here except query smoke needed for proof.
> Proves: logging control, wipe/retention, and bridge lifecycle only after tests/proof pass.
> Does not prove: source instrumentation, MCP, Rust logging-core, or full parity by itself.
> Proof rule: Before DONE, run focused control/lifecycle tests and write proof artifacts.

<!-- /agent-capsule -->

# WP09 Log Control, Retention, and Bridge Lifecycle

## Purpose

Implement the controls that make logging usable long-term:

```text
log level / source / file enable controls
store-vs-console decisions
fresh-run wipe
retention cleanup
bridge health / run-start / stale-run lifecycle
local-vs-tunnel bridge mode
```

Without this workpack, parent can produce logs but will become noisy, stale, and expensive for agents to query.

## Source inputs

```text
docs/plans/logging-domain-parity/08-log-control-retention-bridge-lifecycle.md
ocentra-games/packages/logging-domain/src/core/logDecisionProvider.ts
ocentra-games/packages/logging-domain/src/core/adapters/cloudflareLogDecisionProvider.ts
ocentra-games/infra/cloudflare/src/logging/log-config.ts
ocentra-games/packages/logging-domain/src/test-log/wipeNdjsonScope.ts
ocentra-games/packages/logging-domain/scripts/log-bridge.ts
ocentra-games/infra/cloudflare/scripts/run-suite-helper.ts
ocentra-games/infra/cloudflare/test-runner/script/report/summary-reporter.ts
ocentra-games/packages/logging-domain/src/app-log/appNdjsonWriter.ts
```

## Dependency gate

Run this after the base TypeScript logging package files exist.

Required before implementation:

```text
parent scopes exist
bridge script exists or is being implemented in same slice
NDJSON path helpers exist
app/test log writer exists
```

If these are missing, route to WP02 first.

## Target state

Parent logging has explicit controls and lifecycle:

```text
log decision provider
local/dev/test default policy
source/file/run debug selection
fresh-run wipe
retention cleanup
bridge health check
run-start metadata
stale-run rejection or warning
local bridge default
tunnel bridge optional
```

## Required proof root

```text
output/logging-domain-parity-proof/09-log-control-retention-bridge-lifecycle/
```

Required artifacts:

```text
00-log-decision-provider-proof.json
01-wipe-scope-proof.json
02-retention-cleanup-proof.json
03-bridge-run-lifecycle-proof.json
04-tunnel-mode-decision-proof.json
16-validation-commands.log
```

## Checklist rows

- [ ] Log decision provider implemented.
- [ ] Error/warn are always stored.
- [ ] Info/debug/log are controlled by environment/source/file/run selection.
- [ ] Console and storage decisions are separate.
- [ ] Fresh-run wipe can wipe selected scope/run/suite/file.
- [ ] Retention cleanup keeps configurable recent local sessions/files.
- [ ] Bridge health check exists.
- [ ] Bridge run-start endpoint records current run metadata.
- [ ] Stale run info is rejected or warned.
- [ ] Local bridge is default.
- [ ] Tunnel bridge mode is optional and condition-gated.
- [ ] Tests cover controls, wipe, retention, and bridge lifecycle.
- [ ] Proof root and workpack completion section filled.

## Expected source changes

Likely files:

```text
packages/logging-domain/src/core/logDecisionProvider.ts
packages/logging-domain/src/core/logConfig.ts
packages/logging-domain/src/test-log/wipeNdjsonScope.ts
packages/logging-domain/src/app-log/appNdjsonWriter.ts
packages/logging-domain/scripts/log-bridge.ts
packages/logging-domain/scripts/wipe-logs.ts
packages/logging-domain/scripts/retention-logs.ts
packages/logging-domain/tests/**
```

Do not hardcode `cloudflare` as the generic parent default.

## Required controls

Suggested environment variables:

```text
OCENTRA_PARENT_LOG_LEVEL
OCENTRA_PARENT_LOG_ENABLED
OCENTRA_PARENT_LOG_CONSOLE
OCENTRA_PARENT_LOG_STORE
OCENTRA_PARENT_DEBUG_SOURCES
OCENTRA_PARENT_DEBUG_FILES
OCENTRA_PARENT_TEST_MODE
OCENTRA_PARENT_LOG_BRIDGE_URL
OCENTRA_PARENT_LOG_BRIDGE_MODE=local|tunnel|disabled
OCENTRA_PARENT_LOG_BRIDGE_SKIP_HEALTH
```

## Focused commands

```bash
npm run build --workspace @ocentra-parent/logging-domain
npm run test --workspace @ocentra-parent/logging-domain -- log-decision
npm run test --workspace @ocentra-parent/logging-domain -- wipe
npm run test --workspace @ocentra-parent/logging-domain -- retention
npm run test --workspace @ocentra-parent/logging-domain -- bridge
```

If scripts are implemented:

```bash
npm run logs:wipe -- --scope=parent-test
npm run logs:retention -- --scope=parent-codex --keep=10
```

## Bridge/tunnel rule

Use localhost by default.

Use tunnel only when the runtime cannot reach localhost directly:

```text
wrangler/miniflare worker process
pooled worker process outside host process
mobile/emulator host mapping
remote CI/developer bridge
```

Do not require tunnel for normal local Node/Vitest/Rust/portal execution.

## Manual-required gaps

This workpack does not require MCP. MCP reads the output of this lifecycle through WP07.

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

The log-control and retention lifecycle source surface is present in the current
tree, including `logConfig`, `logDecisionProvider`, `wipeNdjsonScope`,
`testLogRetention`, the app log writer, and the bridge/retention scripts plus
their unit-test counterparts.

What is missing is durable proof. The appended completion block named
`output/logging-domain-parity-proof/09-log-control-retention-bridge-lifecycle/`,
but that proof root is absent in this checkout, and this audit pass did not
re-run the dedicated wipe/retention/bridge command set. Treat WP09 as
implemented-in-source but unproved from current workspace evidence.
