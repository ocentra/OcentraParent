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
cargo test -p ocentra-eventing --test unit
cargo test -p ocentra-eventing --test contract
cargo test -p ocentra-eventing --test journal_replay
cargo test -p ocentra-eventing --test integration
cargo test -p ocentra-eventing --test version_skew
npm run test --workspace @ocentra-parent/event-domain
npm run type-check --workspace @ocentra-parent/event-domain
cmd /c npm run test --workspace @ocentra-parent/agent-protocol-domain -- network-runtime-events.test.ts contracts.test.ts
cargo test -p ocentra-parent-agent-protocol child_domain_runtime_events --quiet
npm run lint:architecture -- --files crates/ocentra-eventing crates/agent-protocol packages/event-domain docs/plans/eventing-plan
```

If a command/test path does not exist, record the blocker and keep rows open.

## Proof references

```text
docs/proof/eventing-plan/slice-01-envelope-version.md
docs/proof/eventing-plan/slice-02-ordering-replay.md
docs/proof/eventing-plan/slice-03-consumer-boundary.md
```

## Rollout proof bundle

```text
output/eventing-plan-proof/rollout-proof/proof-summary.json
test-results/eventing-rollout-proof/proof.json
output/eventing-plan-proof/rollout-proof/pr-done-report.md
output/eventing-plan-proof/rollout-proof/command-logs/
```

If the rollout bundle or the `docs/proof/eventing-plan/` slice files are
absent, record the blocker and keep WP12 open.

## DONE rule

One workpack is DONE only after focused commands or blockers are recorded and proof artifacts exist under that workpack root.
