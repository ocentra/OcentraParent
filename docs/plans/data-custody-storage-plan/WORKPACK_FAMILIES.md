<!-- agent-capsule -->

> Agent Capsule
> Plan: `data-custody-storage-plan`
> Doc: `Data Custody Storage Workpack Families`
> Kind: owner-path classifier for selected workpacks.
> Read when: only after `WORKPACK_INDEX.md` selects or names a workpack and the owner/proof family is unclear.
> Stop rule: classify the selected workpack only; do not use this file as permission to scan every workpack in the family.
> Proves: routing and owner-path classification only.
> Does not prove: storage readiness, sync readiness, export readiness, delete readiness, report/query custody readiness, parent settings readiness, or PR readiness.
> Proof rule: if this file changes route/status claims, update `AGENTS.md`, `PLAN_STATE.md`, and any affected selected workpack route.

<!-- /agent-capsule -->

# Data Custody Storage Workpack Families

Use this file to classify a selected workpack before opening source. This plan owns custody rules and proof boundaries. It does not own every producer, consumer, transport, UI, account, key-material, payment, or cloud runtime that touches custody state.

## Custody source-of-truth family

```text
Workpacks:
WP01 Custody Source Of Truth

Owners:
docs/plans/data-custody-storage-plan/DATA_CLASSIFICATION.md
docs/plans/data-custody-storage-plan/DECISIONS.md
schema-domain for shared custody/source-of-truth shapes
storage-custody-core for generic Rust custody decision proof when selected

Rule:
A data-class/source-of-truth matrix proves classification and routing only. It does not prove storage runtime, key custody, provider sync, report/query safety, or parent settings apply.
```

## Key custody and platform wrapper family

```text
Workpacks:
WP02 Encryption Key Custody

Owners:
docs/plans/data-custody-storage-plan/KEY_CUSTODY_MODEL.md
docs/plans/data-custody-storage-plan/PLATFORM_KEY_CUSTODY_MATRIX.md
device-trust-bootstrap-plan when trusted-device key material is selected
schema-domain for shared key/custody state shapes

Rule:
Key-custody proof must name key owner, wrapping location, recovery/manual-required state, wrong-key and revoked-device negatives, and no universal Ocentra key. Key model proof is not provider sync or restore/apply proof.
```

## Parent-owned cloud sync and provider-state family

```text
Workpacks:
WP03 Parent Owned Cloud Sync

Owners:
schema-domain parent-owned sync/export contracts
provider capability matrix docs
selected provider adapter only when the workpack explicitly selects runtime implementation
account/device-trust owners only through typed handoff when provider account/device authority is selected

Rule:
Parent-owned sync proof must prove provider capability, encrypted-before-upload state, connector status, cursor/conflict state, provider revocation, offline retry, partial outage, quota/corruption, tombstone propagation, and no automatic Ocentra fallback. Provider status proof is not OAuth runtime, upload runtime, delete runtime, or readable payload proof.
```

## Retention, delete, and tombstone family

```text
Workpacks:
WP04 Retention Delete Tombstone

Owners:
storage-custody-core for generic delete/export/action-plan decision proof when selected
ocentra-eventing for journal/replay/idempotency primitives
data-custody plan docs for retention and tombstone policy

Rule:
Delete proof must distinguish delete request, confirmed delete, tombstone write, idempotent replay, offline replay, retention expiry, and restore-blocking state. Delete proof alone is not tombstone propagation proof unless the selected proof root proves it.
```

## Export, import, backup, and restore family

```text
Workpacks:
WP05 Export Import Backup Recovery

Owners:
docs/plans/data-custody-storage-plan/BUNDLE_PROTOCOL.md
schema-domain bundle/export/import/restore shapes
selected restore/apply runtime only when the workpack names it

Rule:
Export proof is not restore/apply proof. Import preview must be non-mutating. Restore/apply proof must preserve tombstones, reject wrong household/key/corrupt bundles, be idempotent, and name partial-restore/manual-required states.
```

## Report, query, notification, and assistant custody family

```text
Workpacks:
WP06 Report Query Custody

Owners:
report/query producer packages only through public contracts when selected
portal-domain/apps/portal for projection only
notification-domain for notification payload semantics when selected
AI plan for AI runtime when selected
schema-domain for shared report/query/citation/custody shapes

Rule:
Report/query proof must carry source refs, deletion/expiry behavior, cursor/pagination, rate-limit/misuse boundaries, notification payload allow/deny, portal cache custody, assistant citation allowlist, stale/conflict states, and redaction proof. Report/query proof is not assistant-safe output proof unless the selected proof root proves citation/ref constraints.
```

## Parent storage settings and apply-flow family

```text
Workpacks:
WP08 Parent Storage Settings Apply Flow

Owners:
portal-domain/apps/portal for UI/projection only
schema-domain for parent storage setting/apply contract shapes
provider/account/device-trust owners only through typed handoff when selected

Rule:
Parent storage settings UI is not applied custody state. Apply proof must show storage choice state machine, export status, import preview, apply confirmation, provider disconnect/delete state, no automatic fallback, portal cache status, and visible manual-required state.
```

## Rollout, route gate, and PR gate family

```text
Workpacks:
WP07 Rollout Proof And Route Gate

Owners:
selected proof roots under output/data-custody-storage-plan-proof/<workpack>/
PLAN_STATE, ROUTE_INDEX, WORKPACK_INDEX, CHECKLIST_INDEX, PROOF_INDEX, and TEST_PROOF_EXPECTATIONS when status changes
adjacent plans only when a typed handoff claim changes

Rule:
WP07 is last. It may aggregate only accepted proof roots or exact carried blockers. No PR_READY from route/docs/checklist changes alone, source presence alone, or one proof family standing in for another.
```

## Source-only migrated UI plan family

```text
Workpacks:
Migrated Data And AI UI Plan

Owners:
source evidence only; not executable implementation scope by default
portal/AI/data-custody selected workpacks only after exact route selection

Rule:
The migrated source-only plan is context, not closure proof. Do not use it to claim portal UI, AI custody, report/query, or data-custody implementation readiness.
```
