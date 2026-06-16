<!-- agent-capsule -->

> Agent Capsule
> Plan: `eventing-plan`
> Doc: `Eventing Plan Execution Blueprint`
> Kind: implementation sequence and handoff protocol.
> Read when: a worker needs exact execution order, DONE rules, or handoff sequencing.
> Stop rule: choose one workpack; do not implement multiple workpacks unless explicitly assigned.
> Proves: execution routing only.
> Does not prove: implementation completion or PR readiness.

<!-- /agent-capsule -->

# Eventing Plan Execution Blueprint

## Execution rule

Use this loop:

```text
AGENTS.md -> PLAN_STATE.md -> NEXT_ACTIONS.md -> WORKPACK_INDEX.md -> one workpack -> TEST_PROOF_EXPECTATIONS.md -> PROOF_INDEX.md
```

## Proof root

```text
output/eventing-plan-proof/<workpack-file-stem>/
```

## Focused commands

```bash
cargo test -p ocentra-parent-eventing-core
cargo test -p ocentra-parent-agent-protocol event
npm run build --workspace @ocentra-parent/event-domain
npm run test --workspace @ocentra-parent/event-domain
npm run lint:architecture -- --files crates/eventing-core crates/agent-protocol packages/event-domain docs/plans/eventing-plan
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
