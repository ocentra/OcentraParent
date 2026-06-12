<!-- agent-capsule -->

> Agent Capsule
> Plan: `data-custody-storage-plan`
> Doc: `WORKPACK_INDEX.md`
> Kind: workpack chooser; do not read all workpacks.
> Read when: After PLAN_STATE.md/NEXT_ACTIONS.md and before opening any workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this index changes, update PLAN_STATE.md and ROUTE_INDEX.md.

<!-- /agent-capsule -->

# Data Custody Storage Plan Workpack Index

Choose one workpack. Do not open all workpacks.

| Workpack                                                                          | Purpose                                                                                                                                                  | Status  |
| --------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------- | ------- |
| [01-custody-source-of-truth](workpacks/01-custody-source-of-truth.md)             | Define data categories, ownership, custody truth, and no-stolen-data boundary.                                                                           | Planned |
| [02-encryption-key-custody](workpacks/02-encryption-key-custody.md)               | Define encryption, key ownership, recovery, rotation, and unreadable-at-rest guarantees.                                                                 | Planned |
| [03-parent-owned-cloud-sync](workpacks/03-parent-owned-cloud-sync.md)             | Define parent-owned provider sync states, encryption-before-upload, conflicts, revocation, tombstones, and no-Ocentra-default-store proof.               | Planned |
| [04-retention-delete-tombstone](workpacks/04-retention-delete-tombstone.md)       | Define retention classes, delete/tombstone protocol, offline replay protection, report/export effects, and minimal audit boundaries.                     | Planned |
| [05-export-import-backup-recovery](workpacks/05-export-import-backup-recovery.md) | Define encrypted export/import bundles, household binding, migration, wrong-key/wrong-household rejection, partial restore, and support recovery limits. | Planned |
| [06-report-query-custody](workpacks/06-report-query-custody.md)                   | Define report/query derived truth, citations, notification payload boundaries, assistant references, stale/conflict states, and abuse limits.            | Planned |
| [07-rollout-proof-and-route-gate](workpacks/07-rollout-proof-and-route-gate.md)   | Define privacy/custody proof pack and route/index sync.                                                                                                  | Planned |
| [data and AI Ui plan](workpacks/data and AI Ui plan.md)                           | Migrated source doc. Use as source evidence only when a selected workpack names it.                                                                      | Source  |
