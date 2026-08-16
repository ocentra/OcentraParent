<!-- agent-capsule -->

> Agent Capsule
> Plan: `child-agent-runtime-distribution-plan`
> Doc: `Child Agent Runtime Distribution Execution Blueprint`
> Kind: implementation sequence and handoff protocol.
> Read when: a worker needs exact execution order, DONE rules, or handoff sequencing.
> Stop rule: choose one workpack; do not implement multiple workpacks unless explicitly assigned.
> Proves: execution routing only.
> Does not prove: child runtime/package readiness or PR readiness.

<!-- /agent-capsule -->

# Child Agent Runtime Distribution Execution Blueprint

## Execution rule

Child package/runtime work stays separate from parent-client distribution and setup journey proof.

Use this loop:

```text
AGENTS.md -> PLAN_STATE.md -> NEXT_ACTIONS.md -> WORKPACK_INDEX.md -> one workpack -> TEST_PROOF_EXPECTATIONS.md -> PROOF_INDEX.md
```

## Deterministic proof root

```text
output/child-agent-runtime-distribution-plan-proof/<workpack-file-stem>/
```

## Focused commands

```bash
cargo test -p ocentra-parent-agent-service
cargo test -p ocentra-parent-agent-protocol
npm run test:child-android-protocol-package-lifecycle-proof
npm run test:child-android-permission-capability-proof
npm run test:child-android-device-proof-artifact-gate
npm run lint:architecture -- --files crates/agent-service crates/agent-protocol scripts/release docs/plans/child-agent-runtime-distribution-plan
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

Do not claim package/platform/service/readiness for a child runtime unless the selected proof root proves that exact platform and artifact state.
