<!-- agent-capsule -->

> Agent Capsule
> Plan: `device-trust-bootstrap-plan`
> Doc: `Device Trust Bootstrap Plan Test Proof Expectations`
> Kind: command/test selector.
> Read when: selected workpack asks which commands or proof artifacts are expected.
> Stop rule: run focused commands first; do not jump to full validation unless required by the workpack or PR_READY.
> Proves: command expectations only.
> Does not prove: implementation completion without matching artifacts.

<!-- /agent-capsule -->

# Device Trust Bootstrap Plan Test Proof Expectations

## Proof root

```text
output/device-trust-bootstrap-plan-proof/<workpack-file-stem>/
```

## Common commands

```bash
cargo test -p ocentra-parent-agent-protocol device_trust
cargo test -p ocentra-parent-agent-service device_trust
npm run test --workspace @ocentra-parent/portal -- trust
npm run lint:architecture -- --files crates/agent-protocol crates/agent-service packages/family-domain packages/parent-domain apps/portal docs/plans/device-trust-bootstrap-plan
```

## Required proof states

```text
trust source-of-truth
local key custody
parent approval step
phone approval bridge
entitlement snapshot
recovery/reset/re-pair
child-device removal/tamper state
dependency adoption review
route gate
```

## Required negative states

```text
login alone not trust proof
license alone not unlock proof
wrong household/device blocked
revoked/expired state visible
manual-required state visible
mock proof not product proof
```
