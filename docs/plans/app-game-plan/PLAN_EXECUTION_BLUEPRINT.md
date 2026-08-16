<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `App Game Plan Execution Blueprint`
> Kind: implementation sequence and handoff protocol.
> Read when: a worker needs exact execution order, DONE rules, or handoff sequencing.
> Stop rule: choose one active workpack; do not implement multiple workpacks unless explicitly assigned.
> Proves: execution routing only.
> Does not prove: app/game control readiness or PR readiness.

<!-- /agent-capsule -->

# App Game Plan Execution Blueprint

## Execution rule

This plan is large and contains many historical/generated workpacks. Do not read or execute the whole plan.

Use this loop:

```text
AGENTS.md -> PLAN_STATE.md -> CODE_AUDIT.md -> NEXT_ACTIONS.md -> WORKPACK_INDEX.md -> one selected workpack -> TEST_PROOF_EXPECTATIONS.md -> PROOF_INDEX.md
```

## Active-workpack rule

When a workpack row is marked `checked`, do not reopen it unless the assignment says audit/regression/reopen.

When a row is marked `possibly done`, treat it as **not execution-ready** until proof artifacts and validation commands are named.

Source/reference inventory files are not workpacks and must not be assigned as implementation slices.

## Deterministic proof root

```text
output/app-game-plan-proof/<workpack-file-stem>/
```

## Focused commands

```bash
cargo test -p ocentra-app-game-core app_game
cargo test -p ocentra-parent-agent-core app_game
cargo test -p ocentra-parent-agent-protocol app_game
cargo test -p ocentra-parent-agent-service app_game
npm run test --workspace @ocentra-parent/portal -- app
npm run lint:architecture -- --files packages/schema-domain crates/app-game-core crates/agent-protocol crates/agent-core crates/agent-service crates/parent-runtime-core apps/portal platforms/android/agent docs/plans/app-game-plan
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

Do not claim inventory, runtime duration, foreground app, launcher evidence, policy readiness, app install, app block/allow, notification, timer, or parent UI readiness unless the selected proof root proves that exact slice.

## Cleanup requirement

Before assigning broad app/game work, reduce the selected slice to one owner package/crate/UI route. Do not let long generated workpack names or checked historical rows drive new implementation without a fresh proof target.
