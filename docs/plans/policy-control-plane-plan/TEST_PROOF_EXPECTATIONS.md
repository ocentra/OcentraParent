<!-- agent-capsule -->

> Agent Capsule
> Plan: `policy-control-plane-plan`
> Doc: `Policy Control Plane Test Proof Expectations`
> Kind: command/test selector.
> Read when: selected workpack asks which commands or proof artifacts are expected.
> Stop rule: run focused commands first; do not jump to full validation unless required by the workpack or PR_READY.
> Proves: command expectations only.
> Does not prove: implementation completion without matching artifacts.

<!-- /agent-capsule -->

# Policy Control Plane Test Proof Expectations

## Proof root

```text
docs/proof/policy-control-plane-plan/
```

## Common commands

Use the subset relevant to the selected workpack:

```bash
npm run build --workspace @ocentra-parent/policy-domain
npm run test --workspace @ocentra-parent/policy-domain
cargo test -p ocentra-policy-control-core
cargo test -p ocentra-parent-agent-protocol policy
npm run test --workspace @ocentra-parent/agent-protocol-domain -- tests/unit/policy-preview-contracts.test.ts tests/unit/policy-control-delivery-read-model.test.ts tests/unit/policy-control-audit-redaction.test.ts tests/unit/parent-assistant-adapter.test.ts
cd apps/portal && npx vitest run tests/policy-preview-route-panel.test.ts tests/policy-preview-live-activity-state.test.ts
npm run lint:architecture -- --files packages/policy-domain crates/policy-control-core packages/agent-protocol-domain crates/agent-protocol apps/portal docs/plans/policy-control-plane-plan
```

Run through `npm run agent:run --` when collecting proof if the wrapper is available.

## Command ownership notes

- `schema-domain` owns canonical shared policy shapes when cross-boundary.
- `policy-domain` is a legacy/package anchor or TypeScript proof-consumer surface unless explicit public exports exist.
- `policy-control-core` owns Rust source, authority, compiler, conflict, delivery, event, preview, request, and source helper behavior.
- `agent-protocol` and `agent-protocol-domain` prove delivery/read-model/audit/assistant seams only when selected.
- Portal, account, device-trust, data-custody, eventing, domain, AI, notification, and enforcement scopes run only when the selected workpack names the handoff.

## Policy E2E meaning

Do not use one proof family to claim the whole policy path. For this plan, E2E has separate meanings:

```text
source-of-truth E2E: parent-authorized policy source -> versioned source document -> audit reference.
schedule/timezone/DST E2E: schedule input -> timezone/DST normalization -> conflict/time-budget decision -> deterministic precedence.
parent authoring/preview E2E: parent draft/template -> preview/conflict/manual-required state -> parent confirmation or cancellation.
domain compiler E2E: source policy version -> deterministic domain artifact -> unsupported/manual-required/rollback ref -> no runtime mutation.
delivery/ack/audit E2E: confirmed policy -> per child/device/domain delivery -> ack/degraded/retry/rollback state -> audit proof.
ask-parent/override E2E: child request -> parent confirmation -> scoped override with expiry/replay defense -> audit proof.
event/idempotency/replay E2E: policy event -> idempotency/replay/journal/audit linkage -> no delivery claim by itself.
rollout gate E2E: accepted proof roots + carried blockers -> route sync -> no-claim boundary.
```

A workpack can be complete for one tier while other tiers remain open. Record the non-claim instead of broad DONE.

## Platform proof rule

- Real iOS/macOS proof is an external-platform constraint on this Windows host.
- Windows proof is expected where relevant.
- Android proof is expected where relevant, including emulator and synced-device paths when selected work requires them.
- Linux proof via WSL and/or Docker is expected where relevant.
- Do not treat feasible Windows/Android/Linux proof paths as blockers.

## Blocker reporting rule

- Real dependency blockers: report missing dependency-owned surfaces or integration handoffs.
- External platform constraints: report host/device limits such as iOS/macOS on this Windows host.
- Avoidable local execution gaps: report stale docs, missing proof files, broken scoped commands, or local validation debt separately from real blockers.

## Structured harness logging expectations

Product/runtime-safe logging:

```text
redact child private data, raw policy payloads when not fixture-scoped, account/session secrets, device-trust proof material, assistant chat content, and support-private diagnostics
log workpack, actor role, source policy version, target domain, schedule state, conflict state, compiler state, delivery state, ack state, override state, event idempotency state, audit redaction state, manual-required note, and no-claim boundary when safe
separate source truth, preview, compiler, delivery, eventing, ask-parent, portal, account, device-trust, data-custody, domain effect, and enforcement states
never treat UI preview, compiler output, event model, assistant draft, or focused contract logs as full policy readiness without selected proof roots
```

Local Codex/MCP/debug harness logging:

```text
prefer npm run agent:run -- <command> when available
store raw stdout/stderr by artifact pointer instead of pasting terminal walls into plan docs
write compact command summaries into 16-validation-commands.log
include run id, command id, workpack id, owner module, policy version, target domain, exit code, result, artifact pointer, diagnostics summary, blocker note, and no-claim note when available
if the wrapper is unavailable, write wrapper: unavailable and keep the same compact command-log shape
```

## Required states

```text
source of truth
schedule/timezone/DST
parent authoring preview
approval/override
manual-required
policy authority
compiler handoff
read-model proof
delivery ack
event replay/idempotency
audit redaction
negative cases
```

## Required negative states

```text
UI preview not used as applied policy proof
compiler output not used as source truth
compiler tests not used as runtime domain effect proof
event model not used as delivery proof
assistant draft not used as parent approval
child request not used as parent approval
single-domain ack not used as global active policy
policy delivery not used as enforcement authority
focused contract passes not used as full plan completion
```
