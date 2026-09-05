<!-- agent-capsule -->

> Agent Capsule
> Plan: `logging-domain-parity`
> Doc: `Logging Domain Parity Plan State`
> Kind: current state and open gaps.
> Read when: immediately after plan AGENTS.md.
> Stop rule: Do not continue into implementation docs unless this file routes you there.
> Proves: only current plan state and open gap accounting.
> Does not prove: implementation completion, validation, or PR readiness.
> Proof rule: If state changes, update matching workpack, checklist, and proof path.

<!-- /agent-capsule -->

# Logging Domain Parity Plan State

## Scope

This plan upgrades OcentraParent logging from a mostly schema/proof-contract package to a practical local development, agent observability, MCP query, and proof-trace pipeline.

No external `docs/features/*` or `docs/expectations/*` files are routed by this plan today. The numbered plan docs in this folder are the routed source-of-truth inputs.

## Current ownership interpretation

```text
packages/logging-domain:
  TypeScript local logging helpers, bridge transport/server, NDJSON writers, DuckDB/query helpers, log-control/wipe/retention helpers, MCP query helpers, and proof-trace helper surfaces.

crates/logging-core:
  Rust NDJSON/artifact/dev-log/diagnostic/redaction/source/context helper crate,
  plus the WP02-owned local-artifact mutation owner only for this route.

crates/logging-local-artifact-windows-ffi and
crates/logging-local-artifact-provider:
  Dedicated Windows ABI boundary and long-lived provider process required by
  the WP02 local-artifact owner; neither exists in the current workspace.

scripts/dev:
  Local agent wrapper, query, evidence, MCP, and proof-trace entrypoints used by Codex/local development flows.

apps/portal:
  Dev-log producer/consumer path only when selected; portal owns UI/projection and cannot become the logging system owner.

crates/agent-service:
  Rust service producer/consumer path only when selected; agent-service owns service runtime behavior and consumes logging-core helpers.

cloudflare-control-plane-plan:
  Backend/Cloudflare infra logging owner when Cloudflare runtime logging is selected.

Product/support telemetry owners:
  Product-safe logging, support diagnostics, retention policy, and customer-facing telemetry policy remain outside this local parity plan unless a selected handoff names them.
```

## Current coupling risks

```text
- Local developer/agent evidence is not production telemetry readiness.
- MCP smoke proof is not full MCP interface readiness.
- Proof-trace smoke proof is not all product-flow proof coverage.
- Portal dev logger proof is not repo-wide portal instrumentation.
- Agent-service startup/dev-log proof is not full Rust logging adoption.
- Logging-domain package parity is not product runtime logging readiness.
- Proof-inventory query proof detects missing/stale roots; it does not restore or close missing roots by itself.
- Proof roots alone do not close checklist/workpack rows without focused commands and no-claim boundaries.
```

## Current proof interpretation

```text
output/logging-domain-parity-proof/<workpack>/ is the active proof route.
test-results/logging-domain-parity-* roots are supporting result roots and must exist before cited.
WP03, WP06, WP07, WP08, and WP10 have canonical proof roots in this checkout.
WP07 and WP10 proof roots exist, but checklist/workpack closeout remains open.
WP08 is a bounded partial-proof for portal dev logger, logging-domain source/context storage/query, and agent-service startup/dev-log only.
WP06 remains partial-proof while the root routing validation failure is outside this delegated logging-owned slice.
```

## Current source docs

The source docs are routed through workpacks:

```text
00-current-state-and-reference-audit.md
01-parent-logging-architecture.md
02-rust-logging-core-crate.md
03-local-validation-evidence.md
04-validation-and-enforcement.md
05-codex-continuation-plan.md
06-mcp-query-interface.md
07-logger-instrumentation-pattern.md
08-log-control-retention-bridge-lifecycle.md
09-proof-trace-pipeline.md
```

## Current status

### Accepted source-wave truth (2026-08-17)

Accepted source head `735df89de` establishes one redaction authority across
the logging surfaces: Rust owns the exact 18-key sensitive-field policy and
generates `packages/logging-domain/src/generated-log-redaction-policy.ts`;
the TypeScript sanitizer consumes that generated artifact; `Logger` sanitizes
before serialization; and the portal compatibility fallback sanitizes before
serializing its JSON body. No alternate local regex or sensitive-key policy is
part of the accepted source.

This is a source-only reconciliation. Tests, focused validation, proof roots,
checklist closeout, PR/DONE state, and external portal/agent-service/product
composition remain deferred and are not claimed by this source wave.

```text
Plan route: added
Workpack route: added
WP01 audit closeout: audit docs present, but the named proof root is absent in this checkout
WP02 TypeScript package parity: source is hardened with one canonical structured-redaction owner and explicit package exports; the named proof root is absent and the new redaction/export expected tests are not written
WP03 parent architecture/routing: Vite dev/preview middleware now consumes the canonical writer and the live agent-service startup/health/activity callers delegate through crates/logging-core; the real Vite/writer boundary still lacks current expected tests and the broader workpack remains open
WP04 Rust logging core: source/tests present, but the named proof root is absent in this checkout
WP05 local validation evidence: source/tests/smokes present, but the named proof root is absent in this checkout
WP06 validation/enforcement: root checker scripts, wrapper scripts, and local evidence smoke are present; logging-owned proof-inventory query surfaces now detect missing/stale proof roots and stale closeout claims through agent-query/MCP plus focused tests, and the canonical WP06 proof root is present; full focused validation remains open because one root routing check fails against an owning surface outside this delegated slice
WP07 MCP query interface: server and bounded proof roots exist; the query service now enforces realpath/symlink containment and redacted malformed-NDJSON diagnostics, but the corresponding containment/diagnostic expected tests are not written
WP08 logger instrumentation/adoption: the shared logger now sanitizes all structured values through the canonical fail-closed redaction policy; Date/URL/custom-toJSON and unsupported/proxy/getter behavior still require expected tests, and repo-wide adoption is not proved
WP09 log control/retention/bridge lifecycle: source/tests present, but the named proof root is absent in this checkout
WP10 proof trace pipeline: focused portal proof-trace tests pass, the standalone MCP proof-trace smoke is now self-seeding in a clean workspace, and the canonical proof root is present; checklist/workpack closeout is still open
Checklist state: WP03 now reflects its written proof root, WP06 now has 11/12 rows checked against focused proof, WP08 now has 8/12 rows checked against its canonical partial-proof root, and the remaining workpacks stay open as documented in CHECKLIST_INDEX.md
Proof inventory root: output/logging-domain-parity-proof/ now contains canonical WP03, WP06, WP07, WP08, and WP10 roots in this checkout
Test-results roots: test-results/logging-domain-parity-mcp/ and test-results/logging-domain-parity-proof-trace/ now exist; the other named test-results/logging-domain-parity-* roots are still absent
PR-ready: false
```

### Windows local-artifact mutation owner route (2026-08-29)

The 27 reported Windows package failures identify a missing owner seam in
WP02. `packages/logging-domain/src/local-artifact-path.ts` currently exposes
the path, ownership, identity, and explicit mutation-unsupported boundary, but
there is no platform mutation provider behind the local-artifact append, lock,
transaction, retention, bridge, and NDJSON callers. WP02 therefore owns the
absent production root
`packages/logging-domain/src/local-artifact-mutation-provider.ts` and its
consumer wiring.

The routed contract is fail-closed: canonical containment; symlink/reparse
rejection or independently proven safety; directory ownership and identity
currentness; atomic create/write/lock/recovery; and provider-issued opaque
authority that callers cannot mint, widen, replay, or replace. Containment,
canonicalization, reparse, ownership/identity, lock, atomicity, recovery, and
unsupported-provider uncertainty must remain explicit failures. No path-only
fallback, boolean flip, mock, or temporary provider is an accepted outcome.

The route has no hard predecessor because it is package-owned. WP09 remains a
downstream consumer and lifecycle dependent. Graph review and code-map entries
authorize only the absent implementation seam; the source, expected tests,
focused validation, proof root, checklist, normal READY, and DONE remain open.

Dedicated expected test roots are the absent unit and integration files
`packages/logging-domain/tests/unit/local-artifact-mutation-provider.test.ts`
and
`packages/logging-domain/tests/integration/local-artifact-mutation-provider.test.ts`.

### Native adapter expansion (2026-08-29)

The current workspace has no production Node-native binding for this package.
The existing Node child-process usage is limited to dev/test/MCP tooling, and
the existing Windows FFI crate is protected-custody-specific. WP02 therefore
routes the smallest shippable native boundary as two new workspace crates:

```text
crates/logging-local-artifact-windows-ffi/Cargo.toml
crates/logging-local-artifact-windows-ffi/src/lib.rs
crates/logging-local-artifact-provider/Cargo.toml
crates/logging-local-artifact-provider/src/main.rs
```

`crates/logging-core/src/local_artifact_mutation.rs` is the safe owner module;
the Windows FFI crate owns only bounded ABI/handle mechanics, and the provider
binary owns the long-lived framed process/session boundary. The TypeScript
adapter remains
`packages/logging-domain/src/local-artifact-mutation-provider.ts`. This avoids
inventing N-API while keeping callers unable to mint filesystem authority.

The implementation must add both crates to the root `Cargo.toml`, add the
Windows-only dependency from `crates/logging-core/Cargo.toml`, and extend
`packages/logging-domain/package.json` to resolve a pinned built provider
executable. A parent-desktop/release owner must separately stage and
integrity-check that executable; no installer or release artifact is claimed
here. The route must fail closed for containment/canonicalization, reparse or
symlink substitution, directory ownership/currentness, atomic
create/write/lock/recovery, protocol/process loss, and provider-authority
provenance.

The native Rust integration roots
`crates/logging-core/tests/integration/local_artifact_mutation.rs`,
`crates/logging-local-artifact-windows-ffi/tests/integration/local_artifact_windows.rs`,
and
`crates/logging-local-artifact-provider/tests/integration/local_artifact_provider.rs`,
plus the TypeScript integration root
`packages/logging-domain/tests/integration/local-artifact-mutation-provider.test.ts`,
are required and currently absent. This expands implementation authorization
only; source, tests, focused validation, proof, checklist, review, normal
READY, and DONE remain open.

## What is already understood

- `ocentra-games/packages/logging-domain` is the reference implementation.
- Games also had log-query tools exposed to agents and SQL-vs-tool validation evidence.
- Games required source files to register with the shared logger pattern, not merely define a logging package.
- Games used log decision controls, fresh-run wipe, bridge run-start, retention, and optional tunnel routing.
- Parent logging-domain currently has live schema/contract usage but does not have games-level local logging pipeline parity.
- WP01 completed the local MCP audit and confirmed no reusable parent MCP framework was found, so WP07 can implement the parent logging MCP layer if that remains true.
- Parent now has substantial source/test coverage for TypeScript package parity, Rust logging core, log-control/retention/bridge lifecycle, local validation evidence, MCP query interface support, logger instrumentation/adoption support, proof trace pipeline support, and validation/enforcement support, but the plan docs overclaim proof-backed closeout for those slices in this checkout.
- Local development observability is separate from production/product safe logging.
- Codex/local agents should consume compact deterministic evidence through CLI and MCP, not full raw terminal logs.
- The same log pipeline should also collect proof traces for Playwright/service/runtime paths.

## Open gaps

```text
- Recreate or remove the remaining claimed proof roots under output/logging-domain-parity-proof/*
- Recreate or remove the remaining claimed test-results/logging-domain-parity-* roots
- Reconcile the remaining WP07/WP10 checklist closeout and keep WP08 scoped to its canonical partial-proof boundary instead of inflating it to repo-wide adoption
- Decide whether done in this plan means source present, proof present, or both; the current docs mix those states
- The WP03 Rust-side mapping is source-present: app::health, service_runtime::run_agent_service, and activity_capture call agent-service::dev_log, which converts protocol fields and invokes logging-core::DevLogger. Keep focused validation/proof deferred in this code-only pass, and hand off the separate root dev-log-routing failure before claiming full WP06 focused-validation closure
- Accepted source through integration `3fec0793a` materially advances WP02/WP03/WP04/WP07/WP08: Rust owns the exact 18-key sensitive-key policy and generated TypeScript artifact; the TypeScript sanitizer is fail-closed and JSON-safe for unsupported/reflection failures; Vite uses the canonical writer; Logger and the portal compatibility fallback sanitize before serialization; and query reads enforce realpath/symlink containment without absolute-path diagnostics. Independent review found no remaining P0/P1 source defect, but no expected-test source was added or executed; focused validation, proof, external composition, and every DONE claim remain deferred.
- WP02's Windows local-artifact route still lacks its native owner, package-specific Windows FFI boundary, provider process/build integration, and real Rust/TypeScript integration tests. No Node-native binding is present to reuse; do not substitute N-API, a caller callback, a path-only CLI, or a temporary/mock provider.
```

## No-claim boundaries

Until implemented and validated, do not claim:

```text
logging-domain parity complete
full WP03 parent architecture/routing closure
repo-wide agent-service logging migrated
proof trace coverage for product flows
production telemetry readiness
product runtime logging readiness
full MCP logging interface completion
repo-wide instrumentation adoption
```

## Workpack summary

Workpacks are indexed in `WORKPACK_INDEX.md`.

Current default execution order:

```text
1. remaining proof-inventory restoration or claim reduction for WP01/WP02/WP04/WP05/WP09 now that WP08 has a canonical partial-proof root
2. retain the WP03 Rust-side mapping as source-present and defer its focused validation/proof row; no additional production mapping slice is indicated by the live callers
3. hand off the root lint:dev-log-routing failure to the owning portal/agent-service slice before claiming full WP06 focused-validation closure
```

## Health rules

- Do not touch other plan folders while working this plan.
- Do not change source before selecting one workpack.
- Do not claim DONE without focused commands and proof artifacts.
- Do not add more proof-only read models as a substitute for bridge/NDJSON/DuckDB/query/MCP/instrumentation/proof-trace implementation.
- Use `WORKPACK_FAMILIES.md` only when the selected workpack owner/proof family is unclear.
