<!-- agent-capsule -->

> Agent Capsule
> Plan: `data-custody-storage-plan`
> Doc: `Data Custody Storage Plan Execution Blueprint`
> Kind: implementation sequence and handoff protocol.
> Read when: a worker needs exact execution order, DONE rules, or handoff sequencing.
> Stop rule: choose one workpack; do not implement multiple workpacks unless explicitly assigned.
> Proves: execution routing only.
> Does not prove: implementation completion or PR readiness.

<!-- /agent-capsule -->

# Data Custody Storage Plan Execution Blueprint

## Execution order

```text
1. WP01 Custody Source Of Truth
2. WP02 Encryption Key Custody
3. WP03 Parent Owned Cloud Sync
4. WP04 Retention Delete Tombstone
5. WP05 Export Import Backup Recovery
6. WP06 Report Query Custody
7. WP08 Parent Storage Settings Apply Flow
8. WP07 Rollout Proof And Route Gate
```

## Codex startup prompt

```text
You are working in OcentraParent on data-custody-storage-plan.
Read only:
- docs/plans/data-custody-storage-plan/AGENTS.md
- docs/plans/data-custody-storage-plan/PLAN_STATE.md
- docs/plans/data-custody-storage-plan/NEXT_ACTIONS.md
- docs/plans/data-custody-storage-plan/WORKPACK_INDEX.md
Then open exactly one assigned workpack.
Do not read sibling plan folders unless the selected workpack names a handoff.
Do not implement eventing internals while eventing-plan is active in another lane.
Do not claim storage, sync, export, restore, report, or delete readiness without proof artifacts.
```

## Pre-edit note

Before editing source or docs, write:

```text
Assigned workpack:
Implementation slice:
Expected source/doc files:
Expected tests/proof files:
Proof root:
Adjacent handoffs that are read-only:
No-claim boundaries:
```

## Source ownership map

Likely owned paths:

```text
crates/storage-custody-core/**
crates/ocentra-evidence/** when custody references are touched
packages/production-domain/src/parent-owned-sync-export.ts
packages/production-domain/tests/** selected parent-owned sync/export tests
packages/portal-domain/src/** selected parent storage settings/read-model text
apps/portal/src/** selected storage/settings surfaces only
scripts/test/** selected proof harnesses only
docs/plans/data-custody-storage-plan/**
```

Read-only or handoff-only paths:

```text
docs/plans/eventing-plan/**
docs/plans/account-identity-family-plan/**
docs/plans/payment-subscription-plan/**
docs/plans/remote-access-plan/**
docs/plans/portal-ux-household-surfaces-plan/**
docs/plans/setup-install-provisioning-plan/**
docs/plans/device-trust-bootstrap-plan/**
```

## Focused command policy

Use relevant commands only:

```bash
cargo test -p ocentra-parent-storage-custody-core
cargo test -p ocentra-evidence
npm run build --workspace @ocentra-parent/production-domain
npm run test --workspace @ocentra-parent/production-domain -- custody
npm run test --workspace @ocentra-parent/portal -- storage
npm run lint:architecture -- --files crates/storage-custody-core crates/ocentra-evidence packages/production-domain apps/portal docs/plans/data-custody-storage-plan
```

If a command or test path does not exist, record the missing location and keep the row open.

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

Proof roots are under:

```text
output/data-custody-storage-plan-proof/<workpack-id>/
```

Test result roots are under:

```text
test-results/data-custody-storage-plan-<workpack-id>/
```

## DONE / PR_READY criteria

DONE for one workpack requires:

```text
source/docs/tests updated
focused commands run or blocker recorded
negative cases covered or explicitly open
proof artifacts written
CHECKLIST_INDEX.md rows updated
selected workpack Fill-before-DONE section updated
PLAN_STATE.md open gaps updated if state changed
```

PR_READY for the whole plan requires WP07 route gate proof and all earlier workpack proof roots.

## Global no-touch rule

Do not edit active policy/eventing work from this plan unless the user explicitly assigns that route-sync after active lanes finish.
