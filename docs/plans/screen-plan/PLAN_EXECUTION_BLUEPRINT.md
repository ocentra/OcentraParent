<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-plan`
> Doc: `Screen Plan Execution Blueprint`
> Kind: implementation sequence and handoff protocol.
> Read when: a worker needs exact execution order, DONE rules, or handoff sequencing.
> Stop rule: choose one workpack; do not implement multiple workpacks unless explicitly assigned.
> Proves: execution routing only.
> Does not prove: screen capture, screen analysis, enforcement, or PR readiness.

<!-- /agent-capsule -->

# Screen Plan Execution Blueprint

## Execution rule

Screen evidence is high-risk. Do not claim capture, analysis, retention, or enforcement readiness from screenshots, UI mockups, or local-only fixtures alone.

Use this loop:

```text
AGENTS.md -> PLAN_STATE.md -> NEXT_ACTIONS.md -> WORKPACK_INDEX.md -> one workpack -> TEST_PROOF_EXPECTATIONS.md -> PROOF_INDEX.md
```

## Deterministic proof root

```text
output/screen-plan-proof/<workpack-file-stem>/
```

## Focused commands

```bash
npm run build --workspace @ocentra-parent/screen-domain
npm run test --workspace @ocentra-parent/screen-domain
cargo test -p ocentra-parent-agent-protocol screen
cargo test -p ocentra-parent-agent-service screen
npm run test --workspace @ocentra-parent/portal -- screen
npm run lint:architecture -- --files packages/screen-domain packages/agent-protocol-domain crates/agent-protocol crates/agent-service apps/portal docs/plans/screen-plan
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

Do not claim raw image storage, content analysis, local AI readiness, cloud processing, deletion/retention, remote view, or enforcement readiness unless the selected proof root proves the claim and custody/privacy states are explicit.
