<!-- agent-capsule -->

> Agent Capsule
> Plan: `v0-8-enforcement-control-plan`
> Doc: `V0.8 Enforcement Control Execution Blueprint`
> Kind: implementation sequence and handoff protocol.
> Read when: a worker needs exact execution order, DONE rules, or handoff sequencing.
> Stop rule: choose one workpack; do not implement multiple workpacks unless explicitly assigned.
> Proves: execution routing only.
> Does not prove: enforcement readiness or PR readiness.

<!-- /agent-capsule -->

# V0.8 Enforcement Control Execution Blueprint

## Execution rule

Enforcement work must be explicitly parent-authorized, platform-proven, reversible where applicable, and visible as observe-only/dry-run/enforcement-eligible before runtime claims.

Use this loop:

```text
AGENTS.md -> PLAN_STATE.md -> NEXT_ACTIONS.md -> WORKPACK_INDEX.md -> one workpack -> TEST_PROOF_EXPECTATIONS.md -> PROOF_INDEX.md
```

## Deterministic proof root

```text
output/v0-8-enforcement-control-plan-proof/<workpack-file-stem>/
```

## Focused commands

```bash
npm run build --workspace @ocentra-parent/enforcement-domain
npm run test --workspace @ocentra-parent/enforcement-domain
cargo test -p ocentra-parent-agent-protocol enforcement
cargo test -p ocentra-parent-agent-service enforcement
npm run test --workspace @ocentra-parent/portal -- enforcement
npm run lint:architecture -- --files packages/enforcement-domain packages/policy-domain crates/agent-protocol crates/agent-service apps/portal docs/plans/v0-8-enforcement-control-plan
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

Do not claim enforcement runtime readiness unless policy authority, account/device authority, platform capability, rollback/manual override, audit, and parent-visible state are proven for the selected slice.
