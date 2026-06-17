<!-- agent-capsule -->

> Agent Capsule
> Plan: `logging-domain-parity`
> Doc: `Logging Domain Parity Next Actions`
> Kind: resume queue and highest-open work.
> Read when: starting or resuming after PLAN_STATE.md.
> Stop rule: Pick one workpack; do not broaden into unrelated plans.
> Proves: next-action routing only.
> Does not prove: implementation completion or PR readiness.
> Proof rule: Update this file only when queue state changes.

<!-- /agent-capsule -->

# Logging Domain Parity Next Actions

## How to use

1. Confirm the branch is `codex/tracking-plan-full-continuation-a`.
2. Confirm the claimed proof roots and test-results roots actually exist before trusting any done claim.
3. Pick one honest next slice from `WORKPACK_INDEX.md`.
4. Open that workpack only.
5. Fill the workpack pre-edit note.
6. Implement or reconcile, test, run, proof, then update docs.

## Highest-priority queue

### 1. Plan-State and Proof-Inventory Reconciliation

Current status:

```text
the plan docs still overclaim missing-vs-present proof state, but canonical WP07/WP10 proof roots now exist under output/logging-domain-parity-proof/ and test-results/logging-domain-parity-{mcp,proof-trace}/
```

Expected result:

```text
PLAN_STATE.md, WORKPACK_INDEX.md, PLAN_HEALTH.md, and the affected workpack docs match the restored WP07/WP10 proof truth
done claims do not outrun on-disk proof
the plan stops implying missing proof roots for WP07/WP10 while keeping the remaining roots explicitly open
```

### 2. WP03 Parent Architecture and Routing Truthful Closeout

Current status:

```text
portal bridge routing, snapshot-language separation, and agent-service delegation already exist in source/tests, but the workpack still reads as open/unproved
```

Expected result:

```text
WP03 either gets rebuilt proof and an honest closeout
or it records the exact remaining blocker without implying missing implementation
```

### 3. WP06 Validation / Enforcement Honest Closeout

Current status:

```text
WP07/WP10 proof roots are now present, but enforcement still does not verify proof inventory or reject stale closeout claims
```

Expected result:

```text
validation scripts and workpack language reflect the restored proof inventory
proof-backed closeout stays honest when roots are missing or stale
```

## PR readiness guard

A PR-ready slice should close a named workpack or explicitly list remaining rows.

Do not create a PR that only:

```text
updates checklist text
adds proof prose
renames docs
adds TODO comments
```

unless the assigned workpack is explicitly proof-routing-only.

## Actioned completion tracker

- [x] Re-check this plan route from `README.md`, `AGENTS.md`, and `PLAN_STATE.md`.
- [x] Audit the current source/test/proof state against the plan claims.
- [ ] Reconcile plan-state and workpack docs with the restored WP07/WP10 proof truth.
- [ ] Rebuild the remaining missing proof roots or remove the claims that say they already exist.
- [x] Fix the standalone proof-trace smoke claim with a self-seeding clean-workspace harness and canonical proof roots.
