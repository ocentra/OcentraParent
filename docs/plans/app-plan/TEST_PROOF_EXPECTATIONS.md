<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-plan`
> Doc: `Native Apps Plan Test Proof Expectations`
> Kind: command/test selector.
> Read when: selected workpack asks which commands or proof artifacts are expected.
> Stop rule: run focused commands first; do not jump to full validation unless required by the workpack or PR_READY.
> Proves: command expectations only.
> Does not prove: implementation completion without matching artifacts.

<!-- /agent-capsule -->

# Native Apps Plan Test Proof Expectations

## Proof root

```text
output/app-plan-proof/<workpack-file-stem>/
```

## Common command families

Use the subset relevant to the selected workpack:

```bash
# Canonical shared native-app/app-game schema and handoff-shape scope
npm run build --workspace @ocentra-parent/schema-domain
npm run test --workspace @ocentra-parent/schema-domain -- app
npm run type-check --workspace @ocentra-parent/schema-domain

# Rust app observation/event scope
cargo test -p ocentra-app-core app

# Protocol/service scope only when selected workpack touches wire, service handler, read API, or service read model
cargo test -p ocentra-parent-agent-protocol app
cargo test -p ocentra-parent-agent-service app

# Legacy protocol-domain scope only when selected workpack still names it
npm run build --workspace @ocentra-parent/agent-protocol-domain
npm run test --workspace @ocentra-parent/agent-protocol-domain

# UI scope only when selected workpack touches parent portal, child UX preview, or rendered status
npm run test --workspace @ocentra-parent/portal -- app

# Architecture scope: start with touched files; expand only when the workpack requires it
npm run lint:architecture -- --files packages/schema-domain packages/agent-protocol-domain crates/app-core crates/agent-protocol crates/agent-service apps/portal docs/plans/app-plan
```

Run through `npm run agent:run --` when collecting proof if the logging/evidence wrapper is available.

## Command ownership notes

- `packages/schema-domain` owns canonical native-app/app-game shapes when contracts cross package/crate/app/plan boundaries.
- `crates/app-core` proves child-local native-app observation, evidence-event, AI-request, policy-request, and source-readiness behavior when selected.
- `crates/agent-protocol` and `crates/agent-service` are protocol/service proof only when wire, service handler, read API, or service read-model behavior is selected.
- `app-game-plan` owns the shared native app/game evidence spine and most combined runtime/read-model/proof chains. Use this plan only for app-only narrowing or explicit app-plan reconciliation.
- `portal-domain`, `apps/portal`, policy, enforcement, notification, setup/install, and child-runtime are sibling/consumer scopes. Run them only when the selected workpack explicitly touches the handoff or rendered projection.

## Native app E2E meaning

Do not use one proof family to claim the whole native-app path. For this plan, E2E has separate meanings:

```text
contract E2E: schema-domain native-app/app-game shape -> selected helper/projection or protocol consumer -> TypeScript tests.
Rust event E2E: app observation intent -> evidence-recorded event -> optional AI/policy requested event -> Rust tests.
installed inventory E2E: platform app inventory source -> local evidence record -> journal/SQLite row -> service/read-model row.
runtime/process E2E: process/runtime source -> running-now row -> source freshness status.
foreground E2E: foreground-window source -> foreground row -> no-content private title handling -> parent-visible status.
policy/source-readiness E2E: fresh app source status + parent rule target -> dry-run policy preview -> manual-required or ready status.
enforcement/manual-required E2E: source-ready target + policy decision -> adapter preflight/action-result or explicit manual-required block.
portal E2E: service/read-model state -> portal projection -> parent-visible status with source/manual-required labels.
platform E2E: real platform/OS/permission state -> adapter output -> cleanup/rollback/manual-required proof.
```

A workpack can be complete for one tier while other tiers remain open. Record the non-claim instead of broad DONE.

## Structured harness logging expectations

Every native-app implementation/proof slice must preserve both product-safe logging and local harness logging.

Product/runtime-safe logging:

```text
redact private window titles when required, executable paths when policy says opaque refs only, account tokens, store account ids, chat/content data, and child private activity payloads
log evidence refs, source freshness, adapter name/version, platform/permission state, session/action state, degraded reason, and audit reference when safe
separate observe-only, dry-run, manual-required, unsupported, unavailable, permission-limited, and adapter-error states
never treat portal logs, policy logs, AI logs, or notification logs as source evidence
```

Local Codex/MCP/debug harness logging:

```text
prefer npm run agent:run -- <command> when available
store raw stdout/stderr by artifact pointer instead of pasting terminal walls into plan docs
write compact command summaries into 16-validation-commands.log
include run id, command id, workpack id, owner module, exit code, result, artifact pointer, diagnostics summary, source/custody note, platform note, and no-claim note when available
if the wrapper is unavailable, write wrapper: unavailable and keep the same compact command-log shape
```

## Required proof themes

Selected workpacks must state which apply:

```text
contract/schema proof
platform capability proof
service lifecycle proof
permission/degraded-state proof
install/package handoff proof
portal/request proof
negative case proof
manual-required gap proof
source freshness proof
runtime/event proof
adapter-readiness proof
```

## Required negative states

```text
scaffold is not runtime support
package preview is not product readiness
missing OS permission is visible
unsupported platform is visible
stale source blocks readiness
portal cannot bypass service boundary
policy dry-run cannot claim enforcement
child-agent package/runtime claims remain in owning plan
app-game-plan proof cannot close app-plan without named app-only handoff
```
