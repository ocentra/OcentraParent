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
docs/proof/policy-control-plane-plan/
```

## Focused commands

```bash
npm run build --workspace @ocentra-parent/policy-domain
npm run test --workspace @ocentra-parent/policy-domain
cargo test -p ocentra-policy-control-core
cargo test -p ocentra-parent-agent-protocol policy
npm run test --workspace @ocentra-parent/agent-protocol-domain -- tests/unit/policy-preview-contracts.test.ts tests/unit/policy-control-delivery-read-model.test.ts tests/unit/policy-control-audit-redaction.test.ts tests/unit/parent-assistant-adapter.test.ts
cd apps/portal && npx vitest run tests/policy-preview-route-panel.test.ts tests/policy-preview-live-activity-state.test.ts
npm run lint:architecture -- --files packages/policy-domain crates/policy-control-core packages/agent-protocol-domain crates/agent-protocol apps/portal docs/plans/policy-control-plane-plan
```

If a command/test path does not exist, record the blocker and keep rows open.
If a workspace script is broader than the selected proof slice, prefer a direct scoped command and record why.

## Platform proof rule

- Real iOS/macOS proof is an external-platform constraint on this Windows host.
- Windows, Android, WSL, and Docker proof remain expected where relevant and should not be reported as blocked unless a real dependency prevents them.

## Proof files

```text
00-scope-summary.md
01-negative-case-proof.md
02-no-claim-boundary.md
16-validation-commands.log
```

## DONE rule

One workpack is DONE only after focused commands or blockers are recorded and proof artifacts exist under that workpack root.
