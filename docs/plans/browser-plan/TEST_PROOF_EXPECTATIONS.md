<!-- agent-capsule -->

> Agent Capsule
> Plan: `browser-plan`
> Doc: `Browser Plan Test Proof Expectations`
> Kind: command/test selector.
> Read when: selected workpack asks which commands or proof artifacts are expected.
> Stop rule: run focused commands first; do not jump to full validation unless required by the workpack or PR_READY.
> Proves: command expectations only.
> Does not prove: implementation completion without matching artifacts.

<!-- /agent-capsule -->

# Browser Plan Test Proof Expectations

## Proof root

```text
output/browser-plan-proof/<workpack-file-stem>/
```

## Common commands

Use the subset relevant to the selected workpack:

```bash
# Canonical shared browser schema and handoff-shape scope
npm run build --workspace @ocentra-parent/schema-domain
npm run test --workspace @ocentra-parent/schema-domain -- browser
npm run type-check --workspace @ocentra-parent/schema-domain

# Browser helper/projection scope
npm run build --workspace @ocentra-parent/browser-domain
npm run test --workspace @ocentra-parent/browser-domain

# Rust browser observation/event scope
cargo test -p ocentra-browser-core browser

# Protocol/service scope only when selected workpack touches wire, service handler, read API, or service read model
cargo test -p ocentra-parent-agent-protocol browser
cargo test -p ocentra-parent-agent-service browser

# UI scope only when selected workpack touches parent portal or rendered status
npm run test --workspace @ocentra-parent/portal -- browser

# Architecture scope: start with touched files; expand only when the workpack requires it
npm run lint:architecture -- --files packages/schema-domain packages/browser-domain packages/agent-protocol-domain crates/browser-core crates/agent-protocol crates/agent-service apps/portal docs/plans/browser-plan
```

Run through `npm run agent:run --` when collecting proof if the logging/evidence wrapper is available.

## Command ownership notes

- `packages/schema-domain` owns canonical browser/evidence/read-model/intervention shapes when contracts cross package/crate/app/plan boundaries.
- `packages/browser-domain` proves helper/projection behavior only. It must not re-own shared shapes or aggregate policy, notification, family, AI, portal, or enforcement runtime behavior.
- `crates/browser-core` proves child-local browser observation, evidence-event, AI-request, policy-request, and source-readiness behavior when selected.
- `crates/agent-protocol` and `crates/agent-service` are protocol/service proof only when wire, service handler, read API, or service read-model behavior is selected.
- `apps/portal` and portal-domain prove parent-visible status projection only; they do not prove browser source capture.
- AI, policy, enforcement, network, screen, app-game, tracking, LAN, and remote remain sibling/consumer scopes. Run them only when the selected browser workpack explicitly touches the handoff or rendered projection.

## Browser E2E meaning

Do not use one proof family to claim the whole browser path. For this plan, E2E has separate meanings:

```text
contract E2E: schema-domain browser shape -> browser-domain helper/projection -> TypeScript tests.
Rust event E2E: browser observation intent -> evidence-recorded event -> optional AI/policy requested event -> Rust tests.
inventory E2E: browser inventory/support source -> local evidence/status row -> service/read-model row.
managed profile E2E: managed profile/launcher/bridge state -> custody/redaction/restart proof -> service status row.
CDP target E2E: managed bridge target list -> target evidence rows -> unknown active state unless activation proof exists.
active-tab E2E: source-backed active tab signal -> exact URL/title/domain evidence -> freshness/custody labels.
journal/read-model E2E: browser evidence event -> encrypted journal -> SQLite/read model -> service projection.
policy-target E2E: browser evidence/source readiness + parent rule target -> dry-run policy preview -> manual-required or ready status.
managed intervention E2E: source-ready target + policy decision -> browser intervention/action ref -> audit/child-delivery/portal proof.
unmanaged fallback E2E: unmanaged browser detection -> report/warn/terminate/manual-required row, without exact unmanaged URL claim.
portal E2E: service/read-model state -> portal projection -> parent-visible status with source/custody/manual-required labels.
platform E2E: real platform/browser/permission state -> adapter output -> cleanup/rollback/manual-required proof.
```

A workpack can be complete for one tier while other tiers remain open. Record the non-claim instead of broad DONE.

## WP07/WP09 deferred test-source debt (2026-08-19)

Canonical Browser source is intentionally manual-required. Before running any
Browser test command, repair these stale compile surfaces without restoring
caller-mintable authority:

```text
crates/agent-service/tests/unit/browser_runtime_status.rs
  old bridge_disconnected_status and connected_status arities
crates/agent-service/tests/unit/browser_runtime_tests.rs
  old disconnected helper arity and private BrowserManagedLaunch construction
crates/agent-service/tests/unit/browser_inventory_read_model_tests.rs
  private BrowserManagedLaunch construction without private CDP authority
```

Then create the missing workpack roots:

```text
crates/agent-service/tests/integration/browser_managed_runtime.rs
crates/agent-core/tests/integration/browser_bridge_managed_launch.rs
```

Required coverage includes manual-required status with no DEV/env custody,
owner-issued start/stop, retained launch identity, pre/post I/O revalidation,
confirmed teardown, restart/expiry/process exit, same-port replacement,
malformed/oversized/timeout target lists, target disappearance/navigation,
same-launch target authority, active-tab remaining Unknown, and unavailable
Screen handoff. Fixture constructors or public authority shims are forbidden.

## Structured harness logging expectations

Every browser implementation/proof slice must preserve both product-safe logging and local harness logging.

Product/runtime-safe logging:

```text
redact cookies, tokens, local storage, page bodies, form values, chat content, screenshots, decrypted payloads, child private activity payloads, and unmanaged exact URLs unless a selected expectation explicitly allows the field
log evidence refs, browser family/channel, source/custody label, managed/unmanaged state, adapter id, bridge state, target/tab/window ids when safe, freshness, degraded reason, and audit reference when safe
separate managed source proof, unmanaged detection, policy preview, intervention action, and portal display states
never treat portal logs, AI logs, network logs, or process/window logs as exact browser URL/tab source evidence
```

Local Codex/MCP/debug harness logging:

```text
prefer npm run agent:run -- <command> when available
store raw stdout/stderr by artifact pointer instead of pasting terminal walls into plan docs
write compact command summaries into 16-validation-commands.log
include run id, command id, workpack id, owner module, exit code, result, artifact pointer, diagnostics summary, source/custody note, browser-boundary note, platform note, and no-claim note when available
if the wrapper is unavailable, write wrapper: unavailable and keep the same compact command-log shape
```

## Required negative states

```text
unsupported browser visible
unmanaged browser visible
missing extension/app bridge visible
stale evidence visible
private content not exposed by default
mock data not product proof
UI cannot claim exact URL/feed visibility without source proof
CDP target list cannot claim active tab
process/window/network evidence cannot claim exact URL
target URL proof cannot claim policy/enforcement action
managed intervention harness cannot claim product blocking without policy/action/audit/delivery proof
```

## Failure conditions

- Do not mark DONE or PR_READY from happy-path-only proof.
- Do not store proof inventories inside this plan folder.
- Do not claim browser enforcement or content inspection unless the selected proof root proves it.
- Do not claim feature completeness until the relevant E2E tier above is explicitly proven or blocked.
