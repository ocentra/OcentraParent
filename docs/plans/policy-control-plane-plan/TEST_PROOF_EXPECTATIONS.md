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
output/policy-control-plane-plan-proof/<workpack-file-stem>/
```

## Common commands

```bash
npm run build --workspace @ocentra-parent/policy-domain
npm run test --workspace @ocentra-parent/policy-domain
cargo test -p ocentra-parent-policy-control-core
cargo test -p ocentra-parent-agent-protocol policy
npm run test --workspace @ocentra-parent/portal -- policy
npm run lint:architecture -- --files packages/policy-domain crates/policy-control-core packages/agent-protocol-domain crates/agent-protocol apps/portal docs/plans/policy-control-plane-plan
```

## Required states

```text
source of truth
schedule/timezone/DST
approval/override
manual-required
policy authority
read-model proof
negative cases
```
