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
| source incomplete / runtime composition blocked | [WP05 Export Import Backup Recovery](workpacks/05-export-import-backup-recovery.md) | 12/12 recorded | `BUNDLE_PROTOCOL.md`, `KEY_CUSTODY_MODEL.md` | bounded schema/storage/parent-runtime source packet present; external composition/reachability, expected tests, focused validation, and proof remain open |
| validation / source accepted, tests open | [WP06 Report Query Custody](workpacks/06-report-query-custody.md) | 13/13 recorded | `EVENT_MODEL.md`, `UI_EXPECTATIONS.md` | no runtime consumer; stale/unwritten expected tests and historical ignored `output/` root remain |
| validation / source incomplete | [WP08 Parent Storage Settings Apply Flow](workpacks/08-parent-storage-settings-apply-flow.md) | 12/12 recorded | `PARENT_SAVE_RETRIEVE_APPLY_FLOW.md`, `UI_EXPECTATIONS.md` | confirmation authority and reachable Applied/Partial path, expected tests, and clean-checkout proof remain open |
| blocked / source reachable, Account composition and tests open | [WP07 Rollout Proof And Route Gate](workpacks/07-rollout-proof-and-route-gate.md) | 2/14 | integrated child custody command/effect/tombstone lifecycle | Account WP04/WP05 plus missing clean-checkout aggregate root |
| planned / source route / waiting on Account WP05 source | [WP09 Parent Local Bundle Provider Runtime](workpacks/09-parent-local-bundle-provider-runtime.md) | 0/0 | `BUNDLE_PROTOCOL.md`, `PARENT_STORAGE_PROVIDER_MATRIX.md` | no source, tests, or proof yet; source-phase edges consume reviewed Data foundations but remain blocked on Account WP05's missing durable participant/CAS source; WP05 owns durable scheduler/job state; WP11 composition is downstream |
| planned / source route / waiting on Account WP05 source | [WP10 Restore Orchestration And Producer Handoffs](workpacks/10-restore-orchestration-and-producer-handoffs.md) | 0/0 | `PARENT_SAVE_RETRIEVE_APPLY_FLOW.md`, `EVENT_MODEL.md` | no source, tests, or proof yet; source-phase edges consume reviewed Data/Account foundations but remain blocked on Account WP05's missing durable participant/CAS source; WP05 owns durable restore/migration ledger; WP11 composition is downstream |
| planned / dependency-waiting source route | [WP11 Runtime Composition And Custody Mount](workpacks/11-runtime-composition-and-custody-mount.md) | 0/0 | `PARENT_SAVE_RETRIEVE_APPLY_FLOW.md`, `BUNDLE_PROTOCOL.md`, `EVENT_MODEL.md` | planned parent-runtime composition/mount roots are absent; implementation-only authorization remains blocked until Account WP05, WP09, and WP10 production source exists; normal tests/proof/DONE remain blocked |
| source | [Migrated Data And AI UI Plan](workpacks/data and AI Ui plan.md) | 0/0 | source evidence only | n/a |

## Default execution order

```text
WP01 -> WP02 -> WP03 -> WP04 -> WP05 -> WP06 -> WP08 -> Account WP05 participant/CAS source -> (WP09, WP10) -> WP11 -> WP07
```

## Dependency rules

```text
WP01 establishes data classes and owners.
WP02 establishes key custody.
WP03 uses WP01/WP02 storage and key boundaries.
WP04 uses WP01 event and retention classes.
WP05 uses WP02/WP04 bundle, key, and retention rules.
WP06 uses WP01/WP04 derived data and deletion behavior.
WP08 uses WP03/WP05/WP06 states for parent-visible settings and depends on
Account WP05's durable opaque-effect CAS/recovery handoff before confirmation
staging/consume can reach `Applied` or `Partial`.
WP05 owns the remaining source packet in three legal layers: schema durable
backup/schedule/job/migration/rollback contracts; pure
storage-custody-core decisions/orchestration; and parent-runtime-core durable
scheduler/job and restore/migration ledgers, restart reconciliation,
executor/rollback mount, and Eventing/outbox composition. It consumes only
opaque Account/family authority, key/decrypt capability, provider-neutral
adapter, and producer ports.
WP09 consumes the WP05 base plus WP02/WP03/WP04, Account WP05, and exact
Device Trust/Eventing handoffs; it remains a downstream pure byte-custody/
provider-port route and does not own a second scheduler/job ledger or depend on
WP11.
WP10 consumes the WP05 base plus WP02/WP03/WP04, Account WP05/WP08, and exact
Device Trust/Eventing/data-class producer handoffs; it remains a downstream
pure producer-handoff route and does not own a second restore/migration ledger,
fabricate receipts, or depend on WP09/WP11.
WP11 consumes WP05 base, WP09 provider operation capability, WP10 outcomes,
Account WP05 true authority transaction/CAS, key/import custody, and the
producer-owned artifact-custody handoff. It is blocked until those owners are
available and never becomes a prerequisite of WP09 or WP10.
WP07 is last and consumes all previous proof roots.
```

## Production-code audit note (2026-08-17, historical pre-packet checkpoint `a78d8f831`)

Recorded boxes and old `output/` references are not current acceptance. The
source wave closed WP02 cross-scope decrypt authority, WP03 manifest custody,
WP05 import integrity, and WP06's full Rust/generated request/row custody edge. WP04/WP07 now place
the durable tombstone/effect owner in `crates/child-runtime` and expose a real
internal service path from `submit_storage_custody_action` through dispatch and
`ChildStorageCustodyRuntime::execute`.

That command path remains fail-closed in shipped composition: default startup
uses a manual-required custody authority, no Account/family trusted adapter or
external upstream caller supplies the opaque handle, and Device Trust remains
an independent outer readiness gate. At this historical checkpoint WP05 still
lacked its durable backup/migration runtime source; the bounded source packet
now supplies that owner, while external composition/reachability remains open.
WP06 has no shipped report/query consumer; its Rust harnesses are stale and its
expected TypeScript contract test is unwritten. These tests belong to the later
expected-test wave and must not be repaired by restoring a core re-export or
inventing a consumer.

Do not treat the graph's validation/completion state as a substitute for this
source audit. Graph topology is updated from the integrated source; DONE still
requires current tests, retained proof, checklist, and required handoffs.

## Do not select

Do not implement adjacent plan internals from this plan. Keep eventing, account, payment, remote, portal shell, setup, device-trust, Cloudflare, notification, report producer, and AI implementation in their owning plans.

Do not use the source-only migrated UI plan as executable implementation scope by default. Do not raise status from docs/checklist/proof text alone, source presence alone, schema proof alone, sync manifest proof, portal UI proof, or a proof root for another workpack.

## 2026-08-17 missing runtime ownership routing (historical pre-packet route)

The live-code audit found that WP05 had typed bundle/preflight/integrity/manual
readiness but no production local/provider writer or retriever, scheduler,
cryptographic byte verifier, restore/migration/apply/rollback/idempotency
runtime. Child-runtime owns local data/tombstone durability and Account owns
authority. The reviewed source packet now supplies the routed schema contracts,
pure storage-custody-core decisions, and parent-runtime-core durable owner;
external composition and production reachability remain open. WP09 and WP10
remain downstream source routes, not completion rows or permission to add a
fake provider.

## 2026-08-18 reviewed WP05 ownership route

The graph-visible WP05 implementation route is deliberately acyclic:

```text
schema contracts
    -> storage-custody-core pure decisions/orchestration
    -> parent-runtime-core durable scheduler/job + restore/migration ledgers
       + restart reconciliation + executor/rollback mount + Eventing/outbox
    -> WP09 provider-neutral byte/adapter-port handoff --┐
    -> WP10 producer-handoff orchestration --------------┴-> WP11 runtime composition/custody mount
```

The exact production and deferred expected-test roots are recorded in
`workpacks/05-export-import-backup-recovery.md` and
`docs/engineering-graph/code-map.json`. The parent-runtime source paths are
now present in the bounded source packet, while dependency-owned mounts,
tests, proof, and runtime composition remain open; this route does not mark
source or plan completion.
