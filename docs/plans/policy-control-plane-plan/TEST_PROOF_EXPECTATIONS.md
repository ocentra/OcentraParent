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
docs/proof/policy-control-plane-plan/
```

## Common commands

```bash
npm run build --workspace @ocentra-parent/policy-domain
npm run test --workspace @ocentra-parent/policy-domain
cargo test -p ocentra-policy-control-core
cargo test -p ocentra-parent-agent-protocol policy
npm run test --workspace @ocentra-parent/agent-protocol-domain -- tests/unit/policy-preview-contracts.test.ts tests/unit/policy-control-delivery-read-model.test.ts tests/unit/policy-control-audit-redaction.test.ts tests/unit/parent-assistant-adapter.test.ts
cd apps/portal && npx vitest run tests/policy-preview-route-panel.test.ts tests/policy-preview-live-activity-state.test.ts
npm run lint:architecture -- --files packages/policy-domain crates/policy-control-core packages/agent-protocol-domain crates/agent-protocol apps/portal docs/plans/policy-control-plane-plan
```

## Platform proof rule

- Real iOS/macOS proof is an external-platform constraint on this Windows host.
- Windows proof is expected where relevant.
- Android proof is expected where relevant, including emulator and synced-device paths when selected work requires them.
- Linux proof via WSL and/or Docker is expected where relevant.
- Do not treat feasible Windows/Android/Linux proof paths as blockers.

## Blocker reporting rule

- Real dependency blockers: report missing dependency-owned surfaces or integration handoffs.
- External platform constraints: report host/device limits such as iOS/macOS on this Windows host.
- Avoidable local execution gaps: report stale docs, missing proof files, broken scoped commands, or local validation debt separately from real blockers.

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
