<!-- agent-capsule -->

> Agent Capsule
> Plan: `remote-access-plan`
> Doc: `Remote Access Plan Execution Blueprint`
> Kind: implementation sequence and handoff protocol.
> Read when: a worker needs exact execution order, DONE rules, or handoff sequencing.
> Stop rule: choose one workpack; do not implement multiple workpacks unless explicitly assigned.
> Proves: execution routing only.
> Does not prove: remote access readiness or PR readiness.

<!-- /agent-capsule -->

# Remote Access Plan Execution Blueprint

## Execution rule

Remote access is high-risk. Do not claim live view/control readiness from UI-only proof, LAN-only proof, or mocked sessions.

Use this loop:

```text
AGENTS.md -> PLAN_STATE.md -> NEXT_ACTIONS.md -> WORKPACK_INDEX.md -> one workpack -> TEST_PROOF_EXPECTATIONS.md -> PROOF_INDEX.md
```

## Deterministic proof root

```text
output/remote-access-plan-proof/<workpack-file-stem>/
```

## Focused commands

```bash
npm run build --workspace @ocentra-parent/screen-domain
npm run test --workspace @ocentra-parent/screen-domain
cargo test -p ocentra-parent-agent-protocol remote
cargo test -p ocentra-parent-agent-service remote
npm run test --workspace @ocentra-parent/portal -- remote
npm run lint:architecture -- --files packages/screen-domain packages/agent-protocol-domain crates/agent-protocol crates/agent-service apps/portal docs/plans/remote-access-plan
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

Do not claim remote view/control/session/grant readiness unless the selected proof root proves account authority, device trust handoff, explicit parent approval, session expiry, revocation, audit, and degraded states.
