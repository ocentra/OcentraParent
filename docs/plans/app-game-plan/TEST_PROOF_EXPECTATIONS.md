<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `App Game Plan Test Proof Expectations`
> Kind: command/test selector.
> Read when: selected workpack asks which commands or proof artifacts are expected.
> Stop rule: run focused commands first; do not jump to full validation unless required by the workpack or PR_READY.
> Proves: command expectations only.
> Does not prove: implementation completion without matching artifacts.

<!-- /agent-capsule -->

# App Game Plan Test Proof Expectations

## Proof root

```text
output/app-game-plan-proof/<workpack-file-stem>/
```

## Common commands

Use the subset relevant to the selected workpack:

```bash
# Canonical shared app/game schema and handoff-shape scope
npm run build --workspace @ocentra-parent/schema-domain
npm run test --workspace @ocentra-parent/schema-domain -- app-game
npm run type-check --workspace @ocentra-parent/schema-domain

# Rust app/game runtime/event scope
cargo test -p ocentra-app-game-core app_game

# Windows evidence/session/source scope when selected
cargo test -p ocentra-parent-agent-core app_game

# Protocol/service scope only when selected workpack touches wire, service handler, read API, or service read model
cargo test -p ocentra-parent-agent-protocol app_game
cargo test -p ocentra-parent-agent-service app_game

# UI scope only when selected workpack touches parent portal, child UX preview, or rendered status
npm run test --workspace @ocentra-parent/portal -- app

# Architecture scope: start with touched files; expand only when the workpack requires it
npm run lint:architecture -- --files packages/schema-domain crates/app-game-core crates/agent-protocol crates/agent-core crates/agent-service crates/parent-runtime-core apps/portal platforms/android/agent docs/plans/app-game-plan
```

Run through `npm run agent:run --` when collecting proof if the logging/evidence wrapper is available.

## Command ownership notes

- The owning Rust crate owns canonical shared app/game behavior/contracts;
  `packages/schema-domain` is a generated validation/decoder edge.
- Removed `app-game-domain`, `activity-domain`, `parent-domain`,
  `agent-protocol-domain`, and `text-domain` paths are not test owners.
- `crates/app-game-core` proves child-local app/game observation, sessionization, event handoff, and source-readiness runtime behavior when selected.
- `crates/agent-protocol` and `crates/agent-service` are protocol/service proof only when wire, service handler, read API, or service read-model behavior is selected.
- `parent-domain`, `policy-domain`, `enforcement-domain`, `notification-domain`, `portal-domain`, and `apps/portal` are sibling/consumer scopes. Run them only when the selected workpack explicitly touches the handoff or rendered projection.
- AI consumes stored evidence or structured digest refs. Do not test AI runtime as app/game proof unless the selected workpack is an AI classifier/digest handoff.

## App/game E2E meaning

Do not use one proof family to claim the whole app/game path. For this plan, E2E has separate meanings:

```text
contract E2E: Rust-owned app/game shape -> generated schema-domain edge -> Rust/TypeScript contract tests.
Rust event E2E: app/game observation intent -> evidence-recorded event -> optional AI/policy requested event -> Rust tests.
inventory E2E: platform inventory source -> local evidence record -> journal/SQLite row -> service/read-model row.
runtime E2E: process/runtime source -> running-now row -> session/duration summary -> source freshness status.
foreground E2E: foreground-window source -> foreground row -> no-content private title handling -> parent-visible status.
classifier digest E2E: stored evidence/digest -> AI/classifier result handoff -> validated result ref, without AI scanning OS state.
policy preview E2E: source readiness + parent rule target -> dry-run policy preview -> manual-required or ready status.
timer/budget E2E: session summary + schedule/budget/bonus state -> timer read model -> audit/rollback proof.
enforcement E2E: source-ready target + policy decision -> adapter preflight/action-result -> rollback/manual-required status.
portal E2E: service/read-model state -> portal projection -> parent-visible status with source/custody/manual-required labels.
child UX/notification E2E: child-facing status or request -> local outbox/receipt/audit handoff -> notification/UX proof.
platform E2E: real platform/OS/permission state -> adapter output -> cleanup/rollback/manual-required proof.
```

A workpack can be complete for one tier while other tiers remain open. Record the non-claim instead of broad DONE.

## WP197 Linux Docker host preflight

Run the executable proof runner after the WP197 source and real test roots are
closed:

```text
npm run proof:app-game-wp197 -- --cargo-target-dir=<isolated-target>
```

The runner must retain exactly the scope summary, negative-case proof,
no-claim boundary, compact validation log, and raw command logs under
`output/app-game-plan-proof/197-app-game-linux-docker-host-preflight/`. A green
result proves the protocol shape, bounded/redacted count projection,
unavailable and malformed fail-closed states, cache behavior, path-security
negatives, cleanup-owner degradation, and route rejection. It does not prove
Docker policy execution, enforcement, provider delivery, child delivery, or
private Docker identifier custody.

## Structured harness logging expectations

Every app/game implementation/proof slice must preserve both product-safe logging and local harness logging.

Product/runtime-safe logging:

```text
redact private window titles when required, executable paths when policy says opaque refs only, launcher account tokens, store account ids, chat/content data, and child private activity payloads
log evidence refs, source freshness, adapter name/version, platform/permission state, session id, action/result state, degraded reason, and audit reference when safe
separate observe-only, dry-run, manual-required, unavailable, adapter-error, and enforcement-result states
never treat portal logs, AI logs, or notification logs as source evidence
```

Local Codex/MCP/debug harness logging:

```text
prefer npm run agent:run -- <command> when available
store raw stdout/stderr by artifact pointer instead of pasting terminal walls into plan docs
write compact command summaries into 16-validation-commands.log
include run id, command id, workpack id, owner module, exit code, result, artifact pointer, diagnostics summary, source/custody note, platform note, and no-claim note when available
if the wrapper is unavailable, write wrapper: unavailable and keep the same compact command-log shape
```

The logging evidence should let a human, Codex, or project MCP ask what failed, where it failed, which artifact contains raw output, which source/evidence refs were involved, and what the result proves without reading an entire terminal dump.

## Required negative states

```text
unsupported platform visible
unknown app/game state visible
stale evidence visible
manual-required state visible
permission-limited source blocks readiness
adapter-error blocks readiness
mock evidence not product proof
historical checked row not used as new proof
UI cannot claim runtime action without service/protocol proof
policy dry-run cannot claim enforcement
AI classifier digest cannot claim AI scanned the machine
enforcement adapter cannot run without source-ready target and authority proof
```

## Failure conditions

- Do not mark DONE or PR_READY from happy-path-only proof.
- Do not store proof inventories inside this plan folder.
- Do not use generated long-name handoff rows as implementation scope without a fresh selected proof target.
- Do not claim feature completeness until the relevant E2E tier above is explicitly proven or blocked.
