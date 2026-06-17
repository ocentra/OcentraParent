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

## Common commands

```bash
npm run build --workspace @ocentra-parent/enforcement-domain
npm run test --workspace @ocentra-parent/enforcement-domain
cargo test -p ocentra-parent-agent-protocol enforcement
cargo test -p ocentra-parent-agent-service enforcement
npm run test --workspace @ocentra-parent/portal -- enforcement
npm run lint:architecture -- --files packages/enforcement-domain packages/policy-domain crates/agent-protocol crates/agent-service apps/portal docs/plans/v0-8-enforcement-control-plan
node scripts/test/v0-8-enforcement-control-plan-proof.mjs
```

## Required negative states

```text
policy missing -> no effect-ready claim
parent authority missing -> no effect-ready claim
device authority missing -> no effect-ready claim
platform unsupported -> manual-required
observe-only and dry-run cannot be treated as active effect
rollback/manual override missing -> no ready claim
audit missing -> no ready claim
```
