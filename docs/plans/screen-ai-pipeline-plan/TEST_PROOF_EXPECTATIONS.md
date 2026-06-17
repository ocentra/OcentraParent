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

## Current known gate failure

- `npm run lint:architecture -- --files packages/screen-domain/src/screen-evidence.ts packages/portal-domain/src/contracts.ts packages/parent-domain/src/local-ai-runtime.ts` is currently red because those files still use banned re-export patterns.

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
```
