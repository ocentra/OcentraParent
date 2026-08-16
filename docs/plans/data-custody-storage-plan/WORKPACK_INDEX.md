<!-- agent-capsule -->

> Agent Capsule
> Plan: `data-custody-storage-plan`
> Doc: `Data Custody Storage Plan Workpack Index`
> Kind: workpack selector.
> Read when: after PLAN_STATE.md and NEXT_ACTIONS.md.
> Stop rule: open exactly one selected workpack; do not read every workpack.
> Proves: workpack routing only.
> Does not prove: implementation completion or PR readiness.
> Proof rule: update counts/status only after matching checklist rows and proof artifacts exist.

<!-- /agent-capsule -->

# Data Custody Storage Plan Workpack Index

Choose one workpack. Do not open all workpacks.

Use `WORKPACK_FAMILIES.md` only when the selected workpack owner/proof family is unclear. Do not use it as permission to scan multiple workpacks.

| Status | Workpack | Boxes | Primary source docs | Proof root |
| --- | --- | ---: | --- | --- |
| done | [WP01 Custody Source Of Truth](workpacks/01-custody-source-of-truth.md) | 12/12 | `DATA_CLASSIFICATION.md`, `DECISIONS.md` | `output/data-custody-storage-plan-proof/01-custody-source-of-truth/` |
| done | [WP02 Encryption Key Custody](workpacks/02-encryption-key-custody.md) | 12/12 | `KEY_CUSTODY_MODEL.md`, `PLATFORM_KEY_CUSTODY_MATRIX.md` | `output/data-custody-storage-plan-proof/02-encryption-key-custody/` |
| done | [WP03 Parent Owned Cloud Sync](workpacks/03-parent-owned-cloud-sync.md) | 13/13 | `PARENT_STORAGE_PROVIDER_MATRIX.md`, `PARENT_SAVE_RETRIEVE_APPLY_FLOW.md` | `output/data-custody-storage-plan-proof/03-parent-owned-cloud-sync/` |
| done | [WP04 Retention Delete Tombstone](workpacks/04-retention-delete-tombstone.md) | 13/13 | `DECISIONS.md`, `EVENT_MODEL.md` | `output/data-custody-storage-plan-proof/04-retention-delete-tombstone/` |
| done | [WP05 Export Import Backup Recovery](workpacks/05-export-import-backup-recovery.md) | 12/12 | `BUNDLE_PROTOCOL.md`, `KEY_CUSTODY_MODEL.md` | `output/data-custody-storage-plan-proof/05-export-import-backup-recovery/` |
| done | [WP06 Report Query Custody](workpacks/06-report-query-custody.md) | 13/13 | `EVENT_MODEL.md`, `UI_EXPECTATIONS.md` | `output/data-custody-storage-plan-proof/06-report-query-custody/` |
| done | [WP08 Parent Storage Settings Apply Flow](workpacks/08-parent-storage-settings-apply-flow.md) | 12/12 | `PARENT_SAVE_RETRIEVE_APPLY_FLOW.md`, `UI_EXPECTATIONS.md` | `output/data-custody-storage-plan-proof/08-parent-storage-settings-apply-flow/` |
| in progress / limited lifecycle proven | [WP07 Rollout Proof And Route Gate](workpacks/07-rollout-proof-and-route-gate.md) | 3/14 | prior proof roots plus Rust retention lifecycle | `output/data-custody-storage-plan-proof/07-rollout-proof-and-route-gate/` |
| source | [Migrated Data And AI UI Plan](workpacks/data and AI Ui plan.md) | 0/0 | source evidence only | n/a |

## Default execution order

```text
WP01 -> WP02 -> WP03 -> WP04 -> WP05 -> WP06 -> WP08 -> WP07
```

## Dependency rules

```text
WP01 establishes data classes and owners.
WP02 establishes key custody.
WP03 uses WP01/WP02 storage and key boundaries.
WP04 uses WP01 event and retention classes.
WP05 uses WP02/WP04 bundle, key, and retention rules.
WP06 uses WP01/WP04 derived data and deletion behavior.
WP08 uses WP03/WP05/WP06 states for parent-visible settings.
WP07 is last and consumes all previous proof roots.
```

## Production-code audit note (2026-08-16)

The `done` rows below describe their recorded contract/proof state, not
shipped runtime reachability. Source inspection found no non-test caller for
the WP01/WP02/WP03/WP04/WP05/WP06/WP08 custody derivation APIs. WP07 is
different: the real `ocentra-child-agent-service` composition opens the
durable journal and `RetentionDeleteTombstoneStore`, then invokes
`ChildRuntimeTombstoneEventFlow::recover_pending()` before readiness. Its
`publish_action` and `publish_action_and_require_journal` methods have no
non-test caller, so the missing production slice is the trusted custody-action
producer/handoff, not another storage adapter or proof surface.

Do not treat the graph's current validation/completion state as a substitute
for this source audit. `npm run graph:validate` currently reports checked-in
graph drift and was not repaired here.

## Do not select

Do not implement adjacent plan internals from this plan. Keep eventing, account, payment, remote, portal shell, setup, device-trust, Cloudflare, notification, report producer, and AI implementation in their owning plans.

Do not use the source-only migrated UI plan as executable implementation scope by default. Do not raise status from docs/checklist/proof text alone, source presence alone, schema proof alone, sync manifest proof, portal UI proof, or a proof root for another workpack.
