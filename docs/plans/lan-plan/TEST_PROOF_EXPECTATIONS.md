<!-- agent-capsule -->

> Agent Capsule
> Plan: `lan-plan`
> Doc: `LAN Plan Test Proof Expectations`
> Kind: command/test selector.
> Read when: selected workpack asks which commands or proof artifacts are expected.
> Stop rule: run focused commands first; do not jump to full validation unless required by the workpack or PR_READY.
> Proves: command expectations only.
> Does not prove: implementation completion without matching artifacts.

<!-- /agent-capsule -->

# LAN Plan Test Proof Expectations

## Proof root

```text
output/lan-plan-proof/<workpack-file-stem>/
```

## Common commands

```bash
npm run build --workspace @ocentra-parent/lan-domain
npm run test --workspace @ocentra-parent/lan-domain
cargo test -p ocentra-parent-agent-protocol lan
cargo test -p ocentra-parent-agent-service lan
npm run test --workspace @ocentra-parent/portal -- lan
npm run lint:architecture -- --files packages/lan-domain packages/agent-protocol-domain crates/agent-protocol crates/agent-service apps/portal docs/plans/lan-plan
```

## Required negative states

```text
unsupported route visible
stale state visible
offline state visible
wrong household/device state visible
manual-required state visible
single-machine proof not used for multi-device claim
```
