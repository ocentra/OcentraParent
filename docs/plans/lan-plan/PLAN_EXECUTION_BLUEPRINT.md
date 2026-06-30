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
cargo test -p ocentra-lan-core lan
cargo test -p ocentra-parent-agent-protocol lan
cargo test -p ocentra-parent-agent-service lan
npm run test --workspace @ocentra-parent/portal -- lan
npm run lint:architecture -- --files crates/lan-core crates/agent-protocol crates/agent-service apps/portal docs/plans/lan-plan
cargo lint-architecture crates/lan-core crates/agent-protocol crates/agent-service
```

Use real organized test folders/crates only. Do not count inline source-owned tests, placeholder directories, or mock-only coverage as workpack closure.

## Proof files

```text
00-scope-summary.md
01-negative-case-proof.md
02-no-claim-boundary.md
16-validation-commands.log
```

## DONE rule

One workpack is DONE only after focused commands or blockers are recorded and proof artifacts exist under that workpack root.
