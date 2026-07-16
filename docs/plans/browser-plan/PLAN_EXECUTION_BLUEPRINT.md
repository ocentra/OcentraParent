<!-- agent-capsule -->

> Agent Capsule
> Plan: `browser-plan`
> Doc: `Browser Plan Execution Blueprint`
> Kind: implementation sequence and handoff protocol.
> Read when: a worker needs exact execution order, DONE rules, or handoff sequencing.
> Stop rule: choose one workpack; do not implement multiple workpacks unless explicitly assigned.
> Proves: execution routing only.
> Does not prove: browser visibility/control readiness or PR readiness.

<!-- /agent-capsule -->

# Browser Plan Execution Blueprint

## Execution rule

Browser visibility/control work is high-risk. Do not claim coverage from mocked browser data, unmanaged browser assumptions, or UI-only proof.

Use this loop:

```text
AGENTS.md -> PLAN_STATE.md -> NEXT_ACTIONS.md -> WORKPACK_INDEX.md -> one workpack -> TEST_PROOF_EXPECTATIONS.md -> PROOF_INDEX.md
```

## Deterministic proof root

```text
output/browser-plan-proof/<workpack-file-stem>/
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

## Likely source ownership map

```text
packages/browser-domain/**
packages/agent-protocol-domain/** when browser events cross protocol
crates/agent-protocol/** browser contract parity
crates/agent-service/** browser service boundary only
apps/portal/** selected browser surface proof
scripts/test/** selected browser proof harnesses
```

## Focused command policy

```bash
npm run build --workspace @ocentra-parent/browser-domain
npm run test --workspace @ocentra-parent/browser-domain
cargo test -p ocentra-parent-agent-protocol browser
cargo test -p ocentra-parent-agent-service browser
npm run test --workspace @ocentra-parent/portal -- browser
npm run lint:architecture -- --files packages/browser-domain packages/agent-protocol-domain crates/agent-protocol crates/agent-service apps/portal docs/plans/browser-plan
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

Do not claim:

```text
exact URL visibility
content inspection
unmanaged browser coverage
browser enforcement
extension/app-store readiness
social/video feed classification
```

unless the selected proof root proves the claim and degraded/manual-required states are visible.
