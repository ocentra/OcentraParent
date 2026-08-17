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
| validation / source complete, expected tests open | [WP01 Custody Source Of Truth](workpacks/01-custody-source-of-truth.md) | 12/12 recorded | `DATA_CLASSIFICATION.md`, `DECISIONS.md` | Rust invariant-test family and clean-checkout proof acceptance remain open |
| validation / source accepted, tests open | [WP02 Encryption Key Custody](workpacks/02-encryption-key-custody.md) | 12/12 recorded | `KEY_CUSTODY_MODEL.md`, `PLATFORM_KEY_CUSTODY_MATRIX.md` | historical ignored `output/` root; refresh later |
| validation / source accepted, tests open | [WP03 Parent Owned Cloud Sync](workpacks/03-parent-owned-cloud-sync.md) | 13/13 recorded | `PARENT_STORAGE_PROVIDER_MATRIX.md`, `PARENT_SAVE_RETRIEVE_APPLY_FLOW.md` | historical ignored `output/` root; refresh later |
| validation / source accepted, test migration open | [WP04 Retention Delete Tombstone](workpacks/04-retention-delete-tombstone.md) | 13/13 recorded | `DECISIONS.md`, `EVENT_MODEL.md` | historical ignored `output/` root; refresh later |
| source incomplete | [WP05 Export Import Backup Recovery](workpacks/05-export-import-backup-recovery.md) | 12/12 recorded | `BUNDLE_PROTOCOL.md`, `KEY_CUSTODY_MODEL.md` | historical ignored `output/` root; refresh later |
| source edge incomplete / tests open | [WP06 Report Query Custody](workpacks/06-report-query-custody.md) | 13/13 recorded | `EVENT_MODEL.md`, `UI_EXPECTATIONS.md` | historical ignored `output/` root; refresh later |
| validation / source incomplete | [WP08 Parent Storage Settings Apply Flow](workpacks/08-parent-storage-settings-apply-flow.md) | 12/12 recorded | `PARENT_SAVE_RETRIEVE_APPLY_FLOW.md`, `UI_EXPECTATIONS.md` | confirmation authority and reachable Applied/Partial path, expected tests, and clean-checkout proof remain open |
| blocked / source reachable, Account composition and tests open | [WP07 Rollout Proof And Route Gate](workpacks/07-rollout-proof-and-route-gate.md) | 2/14 | integrated child custody command/effect/tombstone lifecycle | Account WP04/WP05 plus missing clean-checkout aggregate root |
| source route only / not implemented | [WP09 Parent Local Bundle Provider Runtime](workpacks/09-parent-local-bundle-provider-runtime.md) | 0/0 | `BUNDLE_PROTOCOL.md`, `PARENT_STORAGE_PROVIDER_MATRIX.md` | no source, tests, or proof yet; owns parent-local encrypted bytes and provider-neutral runtime |
| source route only / not implemented | [WP10 Restore Orchestration And Producer Handoffs](workpacks/10-restore-orchestration-and-producer-handoffs.md) | 0/0 | `PARENT_SAVE_RETRIEVE_APPLY_FLOW.md`, `EVENT_MODEL.md` | no source, tests, or proof yet; owns durable orchestration and producer handoffs |
| source | [Migrated Data And AI UI Plan](workpacks/data and AI Ui plan.md) | 0/0 | source evidence only | n/a |

## Default execution order

```text
WP01 -> WP02 -> WP03 -> WP04 -> WP05 -> WP06 -> WP08 -> WP09 -> WP10 -> WP07
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
WP09 consumes WP02/WP03/WP04/WP05 plus Account WP05 and exact Device Trust/Eventing handoffs; it owns parent-local/provider-neutral byte runtime.
WP10 consumes WP02/WP03/WP04/WP05 plus Account WP05/WP08 and exact Device Trust/Eventing/data-class producer handoffs; it owns restore orchestration and receipts, not mutation.
WP07 is last and consumes all previous proof roots.
```

## Production-code audit note (2026-08-17, source checkpoint `7a1e1c389`)

Recorded boxes and old `output/` references are not current acceptance. The
source wave closed WP02 cross-scope decrypt authority, WP03 manifest custody,
WP05 import integrity, and WP06 request/row authority gaps. WP04/WP07 now place
the durable tombstone/effect owner in `crates/child-runtime` and expose a real
internal service path from `submit_storage_custody_action` through dispatch and
`ChildStorageCustodyRuntime::execute`.

That command path remains fail-closed in shipped composition: default startup
uses a manual-required custody authority, no Account/family trusted adapter or
external upstream caller supplies the opaque handle, and Device Trust remains
an independent outer readiness gate. WP05 still lacks backup cadence/manual
backup and migration execution/rollback source. WP06 still needs its declared
thin TypeScript adapter/rules edge. Stale moved-store tests belong to the later
expected-test wave and must not be repaired by restoring a core re-export.

Do not treat the graph's validation/completion state as a substitute for this
source audit. Graph topology is updated from the integrated source; DONE still
requires current tests, retained proof, checklist, and required handoffs.

## Do not select

Do not implement adjacent plan internals from this plan. Keep eventing, account, payment, remote, portal shell, setup, device-trust, Cloudflare, notification, report producer, and AI implementation in their owning plans.

Do not use the source-only migrated UI plan as executable implementation scope by default. Do not raise status from docs/checklist/proof text alone, source presence alone, schema proof alone, sync manifest proof, portal UI proof, or a proof root for another workpack.

## 2026-08-17 missing runtime ownership routing

The live-code audit found that WP05 has typed bundle/preflight/integrity/manual
readiness but no production local/provider writer or retriever, scheduler,
cryptographic byte verifier, restore/migration/apply/rollback/idempotency
runtime. Child-runtime owns local data/tombstone durability and Account owns
authority, but no current workpack owns the parent-local/provider runtime or
restore orchestration. WP09 and WP10 are therefore explicit source routes,
not completion rows or permission to add a fake provider.
