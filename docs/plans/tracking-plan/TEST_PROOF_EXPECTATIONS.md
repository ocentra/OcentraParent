<!-- agent-capsule -->

> Agent Capsule
> Plan: `tracking-plan`
> Doc: `Tracking Plan Test Proof Expectations`
> Kind: command/test selector.
> Read when: selected workpack asks which commands or proof artifacts are expected.
> Stop rule: run focused commands first; do not jump to full validation unless required by the workpack or PR_READY.
> Proves: command expectations only.
> Does not prove: implementation completion without matching artifacts.

<!-- /agent-capsule -->

# Tracking Plan Test Proof Expectations

## General rule

Use the selected workpack's named proof artifacts first. If missing, derive:

```text
output/tracking-plan-proof/<workpack-file-stem>/
```

## Common command families

Use the subset relevant to the selected workpack:

```bash
npm run build --workspace @ocentra-parent/tracking-domain
npm run test --workspace @ocentra-parent/tracking-domain
cargo test -p ocentra-parent-tracking-core
cargo test -p ocentra-parent-agent-protocol tracking
npm run test --workspace @ocentra-parent/portal -- tracking
npm run lint:architecture -- --files packages/tracking-domain crates/tracking-core packages/agent-protocol-domain crates/agent-protocol apps/portal docs/plans/tracking-plan
```

## Required negative states

```text
low accuracy produces ambiguous/check-in state
wrong household/device denied
stale evidence visible
offline state visible
manual-required state visible
single-machine proof not physical-device proof
UI read-only proof not delivery/runtime proof
```

## Failure conditions

- Do not mark DONE or PR_READY from happy-path-only proof.
- Do not store proof inventories inside this plan folder.
- Do not claim physical platform behavior unless the selected workpack explicitly proves it.
