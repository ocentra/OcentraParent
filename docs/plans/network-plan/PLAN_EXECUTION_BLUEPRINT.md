<!-- agent-capsule -->

> Agent Capsule
> Plan: `network-plan`
> Doc: `Network Plan Execution Blueprint`
> Kind: implementation sequence and handoff protocol.
> Read when: a worker needs exact execution order, DONE rules, or handoff sequencing.
> Stop rule: choose one workpack; do not implement multiple workpacks unless explicitly assigned.
> Proves: execution routing only.
> Does not prove: network observation, attribution, enforcement, or PR readiness.

<!-- /agent-capsule -->

# Network Plan Execution Blueprint

## Execution rule

Network/domain observations must be source-backed and honest about unknown attribution. Do not imply HTTPS content inspection, packet-content monitoring, or process attribution when the selected proof does not show it.

Use this loop:

```text
AGENTS.md -> PLAN_STATE.md -> NEXT_ACTIONS.md -> WORKPACK_INDEX.md -> one workpack -> TEST_PROOF_EXPECTATIONS.md -> PROOF_INDEX.md
```

## Deterministic proof root

```text
output/network-plan-proof/<workpack-file-stem>/
```

## Focused commands

```bash
npm run build --workspace @ocentra-parent/network-domain
npm run test --workspace @ocentra-parent/network-domain
cargo test -p ocentra-parent-agent-protocol network
cargo test -p ocentra-parent-agent-service network
npm run test --workspace @ocentra-parent/portal -- network
npm run lint:architecture -- --files packages/network-domain packages/agent-protocol-domain crates/agent-protocol crates/agent-service apps/portal docs/plans/network-plan
```

If a command/test path does not exist, record the blocker and keep rows open.

## Universal proof files

```text
00-scope-summary.md
01-negative-case-proof.md
02-no-claim-boundary.md
16-validation-commands.log
```

## No-claim boundaries

Do not claim exact domain/process attribution, packet inspection, enforcement readiness, VPN/DNS/adapter readiness, or product network coverage unless the selected proof root proves it.
