<!-- agent-capsule -->

> Agent Capsule
> Plan: `tracking-plan`
> Doc: `Tracking Plan Execution Blueprint`
> Kind: implementation sequence and handoff protocol.
> Read when: a worker needs exact execution order, DONE rules, or handoff sequencing.
> Stop rule: choose one active workpack; do not implement multiple workpacks unless explicitly assigned.
> Proves: execution routing only.
> Does not prove: location/product readiness or PR readiness.

<!-- /agent-capsule -->

# Tracking Plan Execution Blueprint

> **2026-08-15 routing correction:** current source and test ownership is the
> 42-row map in `CODE_AUDIT.md` and `docs/engineering-graph/code-map.json`.
> Commands or ownership paths below that target `packages/tracking-domain` or
> `scripts/test/tracking-*.mjs` are historical expectations, not runnable
> present-checkout gates. Restore or replace them only in the owning Phase 1
> workpack before using them for Phase 2 or proof.

## Execution rule

Tracking is mature but large. Do not execute the full plan. Select one workpack from `WORKPACK_INDEX.md` and use its named proof root, proof ids, no-claim boundaries, and focused commands.

Use this loop:

```text
AGENTS.md -> PLAN_STATE.md -> NEXT_ACTIONS.md -> WORKPACK_INDEX.md -> one selected workpack -> TEST_PROOF_EXPECTATIONS.md -> PROOF_INDEX.md
```

## Proof root rule

Prefer proof roots explicitly named inside the selected workpack.

If a workpack lacks a proof root, derive:

```text
output/tracking-plan-proof/<workpack-file-stem>/
```

## Active-workpack rule

Checked rows in `WORKPACK_INDEX.md` stay closed unless the assignment is
audit/regression/reopen. This plan currently has audit-reopened checked rows:
`WP25`, `WP27`, `WP28`, `WP29`, and `WP33`.

Treat on-disk `WP34-WP39` as active workpacks even though older generated
index/state docs omitted them.

Large reference docs under `workpacks/` that have `0/0` boxes are source/reference material, not executable workpacks, unless a selected workpack names them.

## Focused command policy

Use only commands relevant to the selected workpack, typically from:

```bash
npm run build --workspace @ocentra-parent/tracking-domain
npm run test --workspace @ocentra-parent/tracking-domain
cargo test -p ocentra-tracking-core
cargo test -p ocentra-parent-agent-protocol tracking
npm run test --workspace @ocentra-parent/portal -- tracking
npm run lint:architecture -- --files packages/tracking-domain crates/tracking-core packages/agent-protocol-domain crates/agent-protocol apps/portal docs/plans/tracking-plan
```

If focused Rust commands are blocked by unrelated crates, record the blocker and keep the row open unless the selected workpack already documents the blocker.

## Proof update rule

Each completed row needs:

```text
exact command
exit code
proof file path
test/proof id
negative case status
remaining gaps/no-claim boundary
```

## No-claim boundaries

Do not claim physical-device proof, background platform behavior, notification delivery, authority proof, production adapter dispatch, or product-ready tracking unless the selected workpack explicitly proves it.
