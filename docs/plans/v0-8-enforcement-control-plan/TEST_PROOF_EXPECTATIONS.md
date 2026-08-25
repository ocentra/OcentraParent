<!-- agent-capsule -->

> Agent Capsule
> Plan: `v0-8-enforcement-control-plan`
> Doc: `V0.8 Enforcement Control Test Proof Expectations`
> Kind: command/test selector.
> Read when: selected workpack asks which commands or proof artifacts are expected.
> Stop rule: run focused commands first; do not jump to full validation unless required by the workpack or PR_READY.
> Proves: command expectations only.
> Does not prove: implementation completion without matching artifacts.

<!-- /agent-capsule -->

# V0.8 Enforcement Control Test Proof Expectations

## Proof root

```text
output/v0-8-enforcement-control-plan-proof/<workpack-file-stem>/
```

Narrative proof notes remain under:

```text
docs/proof/v0-8-enforcement-control-plan/
```

## WP11 durable-journal prerequisite for WP04

Before WP11 can claim durable-journal proof or WP04 can be scheduled, retain
Eventing WP06's
`output/eventing-plan-proof/06-journal-replay-and-lineage/00-enforcement-wp11-handoff.md`
with its journal/topology proof, then retain WP11's enforcement-specific proof
under `output/v0-8-enforcement-control-plan-proof/11-audit-journal-events/`.
If either side lacks required proof, record the exact blocker in its command
log: WP11 remains blocked and WP04 remains unscheduled/manual-required. A
precise blocker records the gap; it does not satisfy either prerequisite.

For the completed-command retry/recovery slice, focused validation must cover:

- the 7-test `enforcement_eventing_retry_production_tests` family, including
  exact persisted-payload replay, real completion time, identity mismatch,
  before-only state, after-without-store, incomplete report, incomplete V3
  replay, and corrupted report-journal identity without adapter reexecution;
- the exact ActivityStore enforcement-audit replacement and missing-row
  regressions;
- focused Rust compile/format, architecture, routed Enforcer checks, diff
  check, and hub guard for the touched files.

Passing this focused slice does not satisfy the absent WP11 proof root or the
remaining approval/denial/expiry/override transition-family obligations.

## Common commands

Use the subset relevant to the selected workpack:

```bash
npm run build --workspace @ocentra-parent/enforcement-domain
npm run test --workspace @ocentra-parent/enforcement-domain
npm run test --workspace @ocentra-parent/agent-protocol-domain -- enforcement
cargo test -p ocentra-parent-agent-protocol enforcement
cargo test -p ocentra-parent-agent-core enforcement
cargo test -p ocentra-parent-agent-service enforcement
npm run test --workspace @ocentra-parent/portal -- enforcement
npm run lint:architecture -- --files packages/schema-domain packages/enforcement-domain packages/agent-protocol-domain crates/agent-protocol crates/agent-core crates/agent-service apps/portal docs/plans/v0-8-enforcement-control-plan
node scripts/test/v0-8-enforcement-control-plan-proof.mjs
```

Run through `npm run agent:run --` when collecting proof if the wrapper is
available.

## Command ownership notes

- `schema-domain` owns canonical shared enforcement schemas when cross-boundary.
- `enforcement-domain` owns TypeScript helper, proof, and read-model consumer
  surfaces. It does not silently replace `schema-domain` as canonical owner.
- `policy-control-plane-plan` owns policy source truth, schedule/budget, and
  ask-parent/override authority semantics.
- `agent-protocol` and `agent-protocol-domain` prove transport and read-model
  parity when selected.
- `app-game`, `browser`, `network`, `screen`, `tracking`, AI/evidence, and
  portal scopes run only when the selected workpack explicitly names the
  handoff.

## Enforcement E2E meaning

Do not use one proof family to claim the whole enforcement path. For this plan,
E2E has separate meanings:

```text
contract boundary E2E: canonical schema -> protocol parity -> helper/read-model consumer -> no silent ownership drift
policy decision ref E2E: deterministic policy decision -> typed target/evidence refs -> eligible/dry-run/manual-required/rejected state
adapter execution E2E: authority + capability + target identity -> dispatch/preflight -> adapter result/no-op/mismatch/unavailable -> audit
managed browser E2E: managed bridge/session -> managed intervention/manual-required state -> visible result with no exact-URL claim
network/domain E2E: evidence visibility -> report-only/manual-required state -> no blocking claim without adapter proof
approval/override E2E: child request -> parent approval/denial/expiry -> scoped action state -> audit trail
read-model/surface E2E: service read model -> parent/child visible state -> no UI-authored authority
integrity E2E: heartbeat/install/permission state -> degraded/manual-required visibility -> no anti-tamper claim
rollout gate E2E: accepted proof roots + carried blockers -> route sync -> PR-ready or not-ready state
```

A workpack can be complete for one tier while other tiers remain open. Record
the non-claim instead of broad DONE.

## Platform proof rule

- Windows proof is expected where relevant.
- Real iOS/macOS proof is an external-platform constraint on this Windows host.
- Android, WSL, and Docker proof paths remain expected where the selected
  workpack names them.
- Do not treat feasible Windows-only proof as a blocker unless a real
  dependency-owned surface is missing.

## Blocker reporting rule

- Real dependency blockers: missing selected sibling-plan or owner-path surfaces.
- External platform blockers: host/device limits such as iOS/macOS proof on this
  Windows host.
- Avoidable local blockers: stale docs, missing proof files, broken scoped
  commands, or local validation debt. Keep these separate from true dependency
  blockers.

## Structured harness logging expectations

Product/runtime-safe logging:

```text
redact child private data, raw policy payloads when not fixture-scoped, exact browser/network content, device secrets, and support-private diagnostics
log workpack, policy decision ref state, actor/device authority state, target identity state, adapter capability state, execution state, rollback state, audit state, parent-visible state, manual-required state, and no-claim boundary when safe
separate policy authority, evidence input, adapter capability, execution, rollback, audit, read-model, portal, and platform-health state
never treat UI preview, browser evidence, tracking evidence, app/game evidence, AI results, or focused contract logs as full enforcement readiness without selected proof roots
```

Local Codex/MCP/debug harness logging:

```text
prefer npm run agent:run -- <command> when available
store raw stdout/stderr by artifact pointer instead of pasting terminal walls into plan docs
write compact command summaries into 16-validation-commands.log
include run id, command id, workpack id, owner module, exit code, result, artifact pointer, diagnostics summary, blocker note, and no-claim note when available
if the wrapper is unavailable, write wrapper: unavailable and keep the same compact command-log shape
```

## Required states

```text
contract boundary
policy decision refs
authority state
target identity
adapter capability
execution result
rollback or recovery
approval or override
audit or journal
read-model visibility
manual-required or degraded
negative cases
```

## Required negative states

```text
policy missing -> no effect-ready claim
parent authority missing -> no effect-ready claim
device authority missing -> no effect-ready claim
adapter capability missing or stale -> manual-required or rejected
platform unsupported -> manual-required
observe-only and dry-run cannot be treated as active effect
rollback/manual override missing -> no ready claim
audit missing -> no ready claim
AI result not used as enforcement authority
portal click not used as enforcement authority
screen/browser/app-game/network/tracking evidence not used as adapter execution proof
managed-browser proof not used as exact-URL proof
network visibility not used as network blocking proof
heartbeat or install visibility not used as anti-tamper proof
focused contract passes not used as full plan completion
```
