<!-- agent-capsule -->

> Agent Capsule
> Plan: `ai-plan`
> Doc: `AI Plan Test Proof Expectations`
> Kind: command/test selector.
> Read when: selected workpack asks which commands or proof artifacts are expected.
> Stop rule: run focused commands first; do not jump to full validation unless required by the workpack or PR_READY.
> Proves: command expectations only.
> Does not prove: implementation completion without matching artifacts.

<!-- /agent-capsule -->

# AI Plan Test Proof Expectations

## General rule

Use focused commands first. Broader validation is allowed only after focused commands pass or a precise blocker is recorded.

For a selected workpack, use proof root:

```text
output/ai-plan-proof/<workpack-file-stem>/
```

## Common command families

Use the subset relevant to the selected workpack:

```bash
# Canonical shared AI schema and encoded-shape contract scope
cargo test -p ocentra-schema
cargo lint-architecture crates/schema

# Transitional TypeScript validation edge only when the selected workpack still touches it
npm run build --workspace @ocentra-parent/schema-domain
npm run type-check --workspace @ocentra-parent/schema-domain

# AI helper/projection scope
npm run build --workspace @ocentra-parent/ai-domain
npm run test --workspace @ocentra-parent/ai-domain

# Text/prompt/display-token scope only when selected workpack touches text-domain
npm run build --workspace @ocentra-parent/text-domain
npm run test --workspace @ocentra-parent/text-domain

# Rust runtime/parity scope only when selected workpack touches Rust AI runtime or parity
cargo test -p ocentra-child-ai-core ai
cargo test -p ocentra-screen-ai-core ai
cargo test -p ocentra-parent-agent-protocol ai
cargo test -p ocentra-parent-agent-service ai

# Parent-visible UI scope only when selected workpack touches portal projection or route rendering
npm run test --workspace @ocentra-parent/portal -- ai

# Architecture scope: start with touched files; expand only when the workpack requires it
npm run lint:architecture -- --files packages/schema-domain packages/ai-domain packages/text-domain crates/child-ai-core crates/screen-ai-core crates/agent-protocol crates/agent-service apps/portal docs/plans/ai-plan
```

Run through `npm run agent:run --` when collecting proof if the logging/evidence wrapper is available.

## Command ownership notes

- `crates/schema` owns canonical AI context, runtime, primitive, reference, model artifact, memory, graph, prompt, provider, result, and bridge DTO shapes when they cross packages/crates/apps/plans.
- `packages/schema-domain` is transitional only. Use it when a temporary generated-validation or edge-decoder surface still exists and changes.
- `packages/ai-domain` proves helper/projection behavior only. It must not re-own canonical shared AI contracts that belong in `crates/schema` or the owning Rust crate.
- `crates/child-ai-core` proves child-local AI runtime/evaluator behavior only when runtime work is selected.
- `crates/screen-ai-core` proves screen AI worker/router behavior only when screen AI work is selected.
- `crates/agent-protocol` and `crates/agent-service` are protocol/service proof only when wire or service behavior is selected.
- `apps/portal` and `portal-domain` prove parent-visible status/explanation projection only; they do not prove child-local safety execution.
- Browser, screen, tracking, network, app/game, policy, enforcement, LAN, and remote plans remain sibling owners. Do not validate or edit their implementation unless the selected AI workpack explicitly names a handoff proof.

## AI E2E meaning

Do not use one proof family to claim the whole AI product path. For this plan, E2E has separate meanings:

```text
contract E2E: Rust-owned AI shape -> generated DTO or temporary edge decoder -> ai-domain helper/projection -> TypeScript tests.
Rust parity E2E: canonical AI shape/protocol expectation -> child-ai-core or screen-ai-core behavior -> Rust tests.
context-builder E2E: stored evidence refs + parent rules + runtime refs + memory/graph refs -> validated AI context build result.
provider/runtime E2E: AI job request -> provider selection/lease/result -> child-agent validation -> accepted or rejected AI result.
policy-handoff E2E: schema-valid AI result -> deterministic policy input -> audit event, without AI owning enforcement.
portal explanation E2E: accepted AI result/read model -> portal projection -> UI proof with evidence/source/status labels.
remote assistant E2E: parent-authorized source bundle -> redacted request/result -> cited answer outside the normal blocking path.
```

A workpack can be complete for one tier while the other tiers remain open. Record the non-claim instead of broad DONE.

## Structured harness logging expectations

Every AI implementation/proof slice must preserve both product-safe logging and local harness logging.

Product/runtime-safe logging:

```text
redact prompt payloads, provider raw output, model tokens, source secrets, report bodies, and child activity content unless the selected expectation explicitly allows the field
log evidence refs, model/runtime refs, prompt/template version, provider id, decision state, degraded reason, validation result, and audit reference when safe
separate local child-safety evaluation from parent-report or remote-assistant activity
never use remote/API AI logs as default child-activity storage
```

Local Codex/MCP/debug harness logging:

```text
prefer npm run agent:run -- <command> when available
store raw stdout/stderr by artifact pointer instead of pasting terminal walls into plan docs
write compact command summaries into 16-validation-commands.log
include run id, command id, workpack id, owner module, exit code, result, artifact pointer, diagnostics summary, redaction note, and no-claim note when available
if the wrapper is unavailable, write wrapper: unavailable and keep the same compact command-log shape
```

The logging evidence should let a human, Codex, or project MCP ask what failed, where it failed, which artifact contains raw output, which evidence refs were involved, and what the result proves without reading an entire terminal dump.

## Required proof themes

Any selected workpack must state which of these apply:

```text
contract/schema proof
fixture/replay proof
local no-model deterministic proof
provider adapter proof
timeout/degraded proof
invalid output proof
redaction/custody proof
source-reference/citation proof
journal/replay proof
parent-visible UI proof
performance/resource proof
security/privacy proof
```

## Required negative states

```text
invalid provider output rejected
timeout returns degraded/manual-required state
missing evidence refs block claim
private payload not included in prompt/output/logs
mock provider does not prove product readiness
memory/reference without source proof is rejected
assistant action cannot bypass policy/account/device authority
model unavailable state is visible
```

## Failure conditions

- Do not mark DONE or PR_READY until code, tests, validation, and proof are complete for the selected slice.
- Do not store proof inventories inside this plan folder.
- Do not claim AI product readiness from docs-only work, mock-only proof, or happy-path-only tests.
