<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-plan`
> Doc: `Screen Plan Test Proof Expectations`
> Kind: command/test selector.
> Read when: selected workpack asks which commands or proof artifacts are expected.
> Stop rule: run focused commands first; do not jump to full validation unless required by the workpack or PR_READY.
> Proves: command expectations only.
> Does not prove: implementation completion without matching artifacts.

<!-- /agent-capsule -->

# Screen Plan Test Proof Expectations

## Proof root

```text
output/screen-plan-proof/<workpack-file-stem>/
```

## Common commands

```bash
npm run build --workspace @ocentra-parent/screen-domain
npm run test --workspace @ocentra-parent/screen-domain
cargo test -p ocentra-parent-agent-protocol screen
cargo test -p ocentra-parent-agent-service screen
npm run test --workspace @ocentra-parent/portal -- screen
npm run lint:architecture -- --files packages/screen-domain packages/agent-protocol-domain crates/agent-protocol crates/agent-service apps/portal docs/plans/screen-plan
```

## Required negative states

```text
permission missing visible
unsupported platform visible
capture disabled visible
image deleted/expired state visible
private raw image not exposed by default
mock screenshot not product proof
AI analysis handoff remains separate unless selected workpack owns it
```
