<!-- agent-capsule -->

> Agent Capsule
> Plan: `network-plan`
> Doc: `Network Plan Test Proof Expectations`
> Kind: command/test selector.
> Read when: selected workpack asks which commands or proof artifacts are expected.
> Stop rule: run focused commands first; do not jump to full validation unless required by the workpack or PR_READY.
> Proves: command expectations only.
> Does not prove: implementation completion without matching artifacts.

<!-- /agent-capsule -->

# Network Plan Test Proof Expectations

## Proof root

```text
output/network-plan-proof/<workpack-file-stem>/
```

## Common commands

```bash
npm run build --workspace @ocentra-parent/network-domain
npm run test --workspace @ocentra-parent/network-domain
cargo test -p ocentra-parent-agent-protocol network
cargo test -p ocentra-parent-agent-service network
npm run test --workspace @ocentra-parent/portal -- network
npm run lint:architecture -- --files packages/network-domain packages/agent-protocol-domain crates/agent-protocol crates/agent-service apps/portal docs/plans/network-plan
```

## Required negative states

```text
unknown domain attribution visible
unknown process attribution visible
adapter unsupported visible
permission missing visible
stale evidence visible
mock evidence not product proof
private network-content claim blocked without explicit proof
```
