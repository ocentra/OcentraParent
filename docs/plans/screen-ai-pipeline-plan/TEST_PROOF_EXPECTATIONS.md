<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-ai-pipeline-plan`
> Doc: `Screen AI Pipeline Test Proof Expectations`
> Kind: command/test selector.
> Read when: selected workpack asks which commands or proof artifacts are expected.
> Stop rule: run focused commands first; do not jump to full validation unless required by the workpack or PR readiness.
> Proves: command expectations only.
> Does not prove: implementation completion without matching artifacts.

<!-- /agent-capsule -->

# Screen AI Pipeline Test Proof Expectations

## Current audited truth

- No retained `output/screen-ai-pipeline-proof/` proof root currently exists in this checkout.
- Use real focused validation and real retained artifacts for the assigned workpack; do not close rows with mock-only or placeholder proof.
- `docs/proof/screen-ai-pipeline-plan/PLAN_PROOF_MANIFEST.md` is required before any slice-level closure claim.

## Proof root

```text
output/screen-ai-pipeline-proof/
```

## Host and platform expectations

- Windows proof is expected where the assigned workpack touches Windows-owned runtime or portal behavior.
- Android proof is expected where the assigned workpack includes Android scope; use the emulator and the already-synced Samsung device when needed.
- Linux proof via WSL is expected where the assigned workpack includes Linux scope; missing Docker CLI on PATH is a local execution gap if Docker-backed proof is required.
- macOS and iOS proof are external-platform constraints from this Windows host.

## Common focused commands

Use the subset relevant to the selected workpack:

```bash
npm run build --workspace @ocentra-parent/screen-domain
npm run test --workspace @ocentra-parent/screen-domain
npm run build --workspace @ocentra-parent/ai-domain
npm run test --workspace @ocentra-parent/ai-domain
cargo test -p ocentra-parent-agent-protocol screen_ai
cargo test -p ocentra-parent-agent-service screen_ai
npm run test --workspace @ocentra-parent/portal -- screen
npm run lint:architecture -- --files packages/screen-domain packages/ai-domain packages/evidence-domain crates/agent-protocol crates/agent-service apps/portal docs/plans/screen-ai-pipeline-plan
node --check scripts/test/screen-ai-final-product-path-proof.mjs
node --check scripts/test/screen-ai-live-operator-artifact-gate.mjs
node --check scripts/test/screen-ai-service-winrt-ocr-proof.mjs
node --check scripts/test/screen-ai-household-mesh-proof.mjs
```

Run through `npm run agent:run --` when collecting proof if the wrapper is available.

## Command ownership notes

- `screen-ai-pipeline-plan` owns cross-hop scenario proof and no-claim boundaries.
- `screen-plan` owns raw capture mechanics and protected-surface behavior.
- `screen-domain` owns screen capture/evidence/OCR/VLM/disclosure/settings contracts.
- `ai-plan` owns provider/runtime/model behavior; canonical shared AI contracts live in `schema-domain`, not `ai-domain` package identity.
- `policy-control-plane-plan` owns policy authority and parent-rule precedence.
- `v0-8-enforcement-control-plan` owns adapter execution and rollback.
- `data-custody-storage-plan` owns retention/export/delete/custody policy.
- `portal-ux-household-surfaces-plan` owns rendered parent-visible UI proof.

## Screen-AI E2E meaning

Do not use one proof family to claim the whole pipeline. For this plan, E2E has separate meanings:

```text
trigger-to-capture E2E: real trigger or structured skip -> queued capture job -> deletion/custody state; no AI claim.
capture-to-AI E2E: capture ref -> AI context/router -> OCR/VLM/text/deterministic result -> queue deletion; no policy authority claim.
AI-result-to-policy E2E: schema-valid AI result with evidence refs -> deterministic policy handoff -> invalid output rejected; no action authority claim.
policy-action-dry-run E2E: policy decision -> observe/allow/warn/ask-parent/time-limit/block dry-run -> audit refs; no adapter execution claim.
journal/read-model/portal E2E: pipeline event/journal row -> read model -> portal projection; no raw capture/model/runtime claim.
custody/deletion E2E: encrypted queue/raw path -> delete success/TTL/delete failure/retention state -> no remote upload by default.
live-operator E2E: operator manifest -> real URL/app capture -> local AI/policy artifacts -> deletion and screenshots; artifact gate is not rerun.
performance/backpressure E2E: cadence/queue pressure -> backpressure/degraded/manual-required state -> no classification/policy claim.
final rollout E2E: retained proof roots + manifest + known gaps + validation logs -> allowed/blocked claims.
```

A workpack can be complete for one tier while other tiers remain open. Record the non-claim instead of broad DONE.

## Current known gate failure

- `npm run lint:architecture -- --files packages/screen-domain/src/screen-evidence.ts packages/portal-domain/src/contracts.ts packages/parent-domain/src/local-ai-runtime.ts` is currently red because those files still use banned re-export patterns.

## Structured harness logging expectations

Product/runtime-safe logging:

```text
redact raw images, raw screenshot paths, OCR text unless fixture-scoped, VLM prompt/output unless fixture-scoped, child-private text, account/session secrets, provider payloads, and support-private diagnostics
log workpack, scenario id, source evidence ref, capture reason, platform, provider route, model route, policy decision state, action dry-run state, queue state, deletion state, retention state, portal state, artifact shape, command id, blocker, and no-claim boundary when safe
separate capture source, AI context, model result, policy decision, action dry-run, enforcement, custody, portal projection, and live-operator artifact-gate states
never treat source-only, mock-only, happy-path-only, local-capture-only, artifact-gate-only, or screenshot-only proof as product readiness without selected retained proof root and no-claim boundary
```

Local Codex/MCP/debug harness logging:

```text
prefer npm run agent:run -- <command> when available
store raw stdout/stderr by artifact pointer instead of pasting terminal walls into plan docs
write compact command summaries into 16-validation-commands.log
include run id, command id, workpack id, scenario id, artifact shape, platform, exit code, result, artifact pointer, diagnostics summary, blocker note, and no-claim note when available
if the wrapper is unavailable, write wrapper: unavailable and keep the same compact command-log shape
```

## Required states

```text
source evidence reference
schema validation
unavailable state
manual-required state
redacted output
deletion/custody
no direct AI-to-policy authority
mock proof not product proof
source-only proof not product proof
AI result not policy authority
policy dry-run not enforcement proof
live-operator artifact gate not live rerun proof
```
