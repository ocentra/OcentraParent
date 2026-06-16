<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-ai-pipeline-plan`
> Doc: `Screen AI Pipeline Test Proof Expectations`
> Kind: command/test selector.
> Read when: selected workpack asks which commands or proof artifacts are expected.
> Stop rule: run focused commands first; do not jump to full validation unless required by the workpack or PR_READY.
> Proves: command expectations only.
> Does not prove: implementation completion without matching artifacts.

<!-- /agent-capsule -->

# Screen AI Pipeline Test Proof Expectations

## Proof root

```text
output/screen-ai-pipeline-plan-proof/<workpack-file-stem>/
```

## Common commands

```bash
npm run build --workspace @ocentra-parent/screen-domain
npm run test --workspace @ocentra-parent/screen-domain
npm run build --workspace @ocentra-parent/ai-domain
npm run test --workspace @ocentra-parent/ai-domain
cargo test -p ocentra-parent-agent-protocol screen_ai
cargo test -p ocentra-parent-agent-service screen_ai
npm run test --workspace @ocentra-parent/portal -- screen
npm run lint:architecture -- --files packages/screen-domain packages/ai-domain packages/evidence-domain crates/agent-protocol crates/agent-service apps/portal docs/plans/screen-ai-pipeline-plan
```

## Required states

```text
source evidence reference
schema validation
unavailable state
manual-required state
redacted output
mock proof not product proof
```
