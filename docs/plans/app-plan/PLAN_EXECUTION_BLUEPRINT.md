<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-plan`
> Doc: `Native Apps Plan Execution Blueprint`
> Kind: implementation sequence and handoff protocol.
> Read when: a worker needs exact execution order, DONE rules, or handoff sequencing.
> Stop rule: choose one workpack; do not implement multiple workpacks unless explicitly assigned.
> Proves: execution routing only.
> Does not prove: platform runtime readiness, package readiness, child-agent readiness, or PR readiness.

<!-- /agent-capsule -->

# Native Apps Plan Execution Blueprint

## Execution rule

This plan owns native app and local-service app boundaries. Do not use it to absorb account, setup, package-distribution, policy, LAN, remote, data-custody, or child-agent distribution ownership.

Use this loop:

```text
AGENTS.md
  -> PLAN_STATE.md
  -> NEXT_ACTIONS.md
  -> WORKPACK_INDEX.md
  -> exactly one selected workpack
  -> TEST_PROOF_EXPECTATIONS.md
  -> PROOF_INDEX.md
```

## Deterministic proof root

```text
output/app-plan-proof/<workpack-file-stem>/
```

## Pre-edit note

```text
Assigned workpack:
Implementation slice:
Expected source/doc files:
Expected tests/proof files:
Proof root:
Adjacent handoffs that are read-only:
No-claim boundaries:
```

## Focused command policy

Use the subset relevant to the selected workpack:

```bash
cargo test -p ocentra-parent-agent-service
cargo test -p ocentra-parent-agent-protocol
npm run build --workspace @ocentra-parent/agent-protocol-domain
npm run test --workspace @ocentra-parent/agent-protocol-domain
npm run test --workspace @ocentra-parent/portal -- app
npm run lint:architecture -- --files crates/agent-service crates/agent-protocol packages/agent-protocol-domain apps/portal docs/plans/app-plan
```

If a command or test path does not exist, record the missing location and keep the row open.

## Universal proof files

Every selected workpack needs:

```text
00-scope-summary.md
01-negative-case-proof.md
02-no-claim-boundary.md
16-validation-commands.log
```

## Native app no-claim boundaries

Do not claim:

```text
Android child support ready
parent mobile support ready
iOS support ready
permission support ready
service lifecycle ready
package/install readiness
policy/enforcement runtime ready
```

unless the selected proof root proves the claim and the rollout gate aggregates it.
