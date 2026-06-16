<!-- agent-capsule -->

> Agent Capsule
> Plan: `policy-control-plane-plan`
> Doc: `Policy Control Plane Execution Blueprint`
> Kind: implementation sequence and handoff protocol.
> Read when: a worker needs exact execution order, DONE rules, or handoff sequencing.
> Stop rule: choose one workpack; do not implement multiple workpacks unless explicitly assigned.
> Proves: execution routing only.
> Does not prove: policy runtime readiness or PR readiness.

<!-- /agent-capsule -->

# Policy Control Plane Execution Blueprint

## Execution rule

Use this loop:

```text
AGENTS.md -> PLAN_STATE.md -> NEXT_ACTIONS.md -> WORKPACK_INDEX.md -> one workpack -> TEST_PROOF_EXPECTATIONS.md -> PROOF_INDEX.md
```

## Proof root

```text
output/policy-control-plane-plan-proof/<workpack-file-stem>/
```

## Focused commands

```bash
npm run build --workspace @ocentra-parent/policy-domain
npm run test --workspace @ocentra-parent/policy-domain
cargo test -p ocentra-parent-policy-control-core
cargo test -p ocentra-parent-agent-protocol policy
npm run test --workspace @ocentra-parent/portal -- policy
npm run lint:architecture -- --files packages/policy-domain crates/policy-control-core packages/agent-protocol-domain crates/agent-protocol apps/portal docs/plans/policy-control-plane-plan
```

If a command/test path does not exist, record the blocker and keep rows open.

## Proof files

```text
00-scope-summary.md
01-negative-case-proof.md
02-no-claim-boundary.md
16-validation-commands.log
```

## DONE rule

One workpack is DONE only after focused commands or blockers are recorded and proof artifacts exist under that workpack root.
