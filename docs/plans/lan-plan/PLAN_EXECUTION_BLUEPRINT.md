<!-- agent-capsule -->

> Agent Capsule
> Plan: `lan-plan`
> Doc: `LAN Plan Execution Blueprint`
> Kind: implementation sequence and handoff protocol.
> Read when: a worker needs exact execution order, DONE rules, or handoff sequencing.
> Stop rule: choose one workpack; do not implement multiple workpacks unless explicitly assigned.
> Proves: execution routing only.
> Does not prove: implementation completion or PR readiness.

<!-- /agent-capsule -->

# LAN Plan Execution Blueprint

## Execution rule

Use this loop:

```text
AGENTS.md -> PLAN_STATE.md -> NEXT_ACTIONS.md -> WORKPACK_INDEX.md -> one workpack -> TEST_PROOF_EXPECTATIONS.md -> PROOF_INDEX.md
```

## Proof root

```text
output/lan-plan-proof/<workpack-file-stem>/
```

## Focused commands

```bash
npm run build --workspace @ocentra-parent/lan-domain
npm run test --workspace @ocentra-parent/lan-domain
cargo test -p ocentra-parent-agent-protocol lan
cargo test -p ocentra-parent-agent-service lan
npm run test --workspace @ocentra-parent/portal -- lan
npm run lint:architecture -- --files packages/lan-domain packages/agent-protocol-domain crates/agent-protocol crates/agent-service apps/portal docs/plans/lan-plan
```

## Proof files

```text
00-scope-summary.md
01-negative-case-proof.md
02-no-claim-boundary.md
16-validation-commands.log
```

## DONE rule

One workpack is DONE only after focused commands or blockers are recorded and proof artifacts exist under that workpack root.
