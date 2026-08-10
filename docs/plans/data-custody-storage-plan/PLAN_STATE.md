<!-- agent-capsule -->

> Agent Capsule
> Plan: `data-custody-storage-plan`
> Doc: `PLAN_STATE.md`
> Kind: plan state and current gap summary.
> Read when: After this plan is selected and before opening workpacks.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If state changes, update NEXT_ACTIONS.md, WORKPACK_INDEX.md, CHECKLIST_INDEX.md, and feature/checklist rows as needed.

<!-- /agent-capsule -->

# Data Custody Storage Plan State

## Current Product Scope

This plan owns data custody guarantees, encrypted storage, evidence retention, export/import/restore, sync, deletion/tombstones, no-stolen-data boundaries, cloud/relay custody, report/query custody, and parent storage settings/apply flow.

Route status: execution-grade architecture, UI docs, and test/proof inventory now exist. WP01 custody source of truth, WP02 encryption key custody, WP03 parent-owned cloud sync, WP04 retention/delete/tombstone, WP05 export/import/backup/recovery, WP06 report/query custody, and WP08 parent storage settings/apply flow now have green implementation-plus-proof roots. WP07 rollout proof and route gate is refreshed with a focused child-runtime lifecycle, a reusable startup-recovery entry point, and tracked command-evidence pointer; its ignored aggregate proof roots remain unavailable in a clean checkout, so WP07 is still not accepted as aggregate route truth.

## Current ownership interpretation

```text
crates/schema:
  Canonical shared custody/export/sync/restore/report/query/assistant-citation/provider/retention/tombstone/parent-storage-setting shapes when they cross package, crate, app, or plan boundaries.

storage-custody-core:
  Rust generic custody/delete/export decision logic and custody action-plan events.

ocentra-evidence:
  Evidence references, evidence identity, and evidence custody ref semantics.

ocentra-eventing:
  Event journal/replay/idempotency spine. This plan consumes eventing contracts; it does not re-own bus implementation.

production-domain:
  Legacy package identity unless a selected public export is named. Current parent-owned sync/export contract proof routes through the Rust owner and generated TS edge surfaces.

portal-domain and apps/portal:
  Parent-visible custody projection, storage settings, preview, confirmation, and status UI only.

Account, device-trust, Cloudflare, payment, setup, remote, LAN, notification, report producers, and AI plans:
  Adjacent sibling owners or handoff consumers. They must not re-own data custody truth.
```

## Current coupling risks

```text
- Active proof roots are under output/data-custody-storage-plan-proof/<workpack>/; legacy docs/proof/data-custody-storage-plan references are stale and must not raise status.
- `packages/production-domain/src/parent-owned-sync-export.ts` is stale as a source-of-truth path; WP03 now routes the canonical contract through `crates/schema/src/parent_owned_sync_export.rs`, runtime/read-model truth through `crates/storage-custody-core/src/parent_owned_sync_export.rs`, and only thin/generated edge validation through `packages/schema-domain/src/parent-owned-sync-export.ts`.
- Contract/schema proof is not runtime custody proof.
- Sync manifest proof is not provider OAuth/upload/delete runtime proof.
- WP05 export/import/restore proof now covers the shared bundle contract and restore/apply state machine only; it is not provider adapter runtime proof.
- WP06 report/query proof now covers assistant/report citation allowlists at the shared contract boundary; it is not AI runtime answer proof.
- Parent storage settings UI proof is not applied custody state.
- Eventing internals, portal UI internals, account authority, device trust material, Cloudflare runtime, payment semantics, setup journey, remote transport, notification delivery, report rendering, and AI runtime must stay in owning plans unless a selected handoff explicitly touches them.
```

## Current proof interpretation

```text
Source presence is not custody readiness.
Schema/domain contract proof is not storage runtime proof.
Provider status proof is not readable-payload or key-access proof.
WP02 key-custody proof root now covers the shared key hierarchy contract, explicit platform decrypt authority, wrong-household/wrong-device/revoked-key/lost-key fail-closed states, linux manual-required state, mobile proof-gated limits, and no universal decrypt root boundary at the shared contract/runtime layer.
WP04 retention/delete proof now covers the retention matrix, delete state machine, tombstone idempotency, offline replay protection, explicit expiry failure, and restore-no-revival boundary at the shared contract/runtime layer; it is not provider-runtime execution proof.
WP05 export/import proof root now covers the versioned bundle manifest, per-class encrypted payload sections, manifest/payload integrity checks, redacted human summary, non-mutating preview, version/household/key/tombstone/duplicate/migration preflight, partial restore, fail-closed negatives, idempotent apply state, and no default support decrypt path at the shared contract/runtime layer.
Import preview remains non-mutating; WP05 restore/apply proof covers only the shared bundle/preflight/apply state machine and not provider-side retrieval or child-device filesystem execution.
Restore/apply proof must prove tombstone preservation and reject resurrection.
WP06 report/query proof root now proves source refs, citation allowlists, redaction, deletion/expiry behavior, stable pagination, stale/conflict honesty, and rate-limit boundaries at the shared contract/runtime layer.
WP08 parent storage settings/apply flow proof root now proves explicit storage mode labels, preview-before-apply, separate disconnect/delete states, manual-required visibility, and no-claim portal/provider-runtime boundaries at the shared contract/runtime layer; it is not final portal rendering, host wiring, or provider execution proof.
WP07 can aggregate only accepted proof roots plus exact carried blockers. Its
`ChildRuntimeTombstoneEventFlow::recover_pending` method is an explicit
service-startup seam; until a concrete child-service owner invokes it and proves
restart behavior there, it is not live service lifecycle completion.
```

## Current Route Status

- Status: execution-grade route established; no product completion claim is made.
- Default action: choose one workpack from [WORKPACK_INDEX.md](WORKPACK_INDEX.md), then choose required proof from [TEST_PROOF_EXPECTATIONS.md](TEST_PROOF_EXPECTATIONS.md).
- Current limitation: this plan defines ownership, expected proof, and handoff boundaries. It does not claim implementation is complete.

## What Is Already Present

- `crates/storage-custody-core` already owns generic custody/delete/export decision logic.
- `crates/ocentra-evidence` already carries custody-scoped evidence reference semantics.
- `crates/ocentra-eventing` already provides the journal/replay building blocks this plan must not duplicate.
- WP01 custody source of truth is now implemented and proved through `crates/schema/src/data_custody_source_of_truth.rs`, `crates/schema/src/data_custody_source_of_truth_ts.rs`, `packages/schema-domain/src/generated-data-custody-source-of-truth-contracts.ts`, and `output/data-custody-storage-plan-proof/01-custody-source-of-truth/`.
- WP03 parent-owned sync is now implemented at the shared Rust boundary through `crates/schema/src/parent_owned_sync_export.rs`, `crates/storage-custody-core/src/parent_owned_sync_export.rs`, and the proof root `output/data-custody-storage-plan-proof/03-parent-owned-cloud-sync/`; the `packages/schema-domain` files remain generated/edge validation only.
- WP02 encryption key custody is now implemented and proved through `crates/schema/src/encryption_key_custody.rs`, `crates/storage-custody-core/src/encryption_key_custody.rs`, and `output/data-custody-storage-plan-proof/02-encryption-key-custody/`.
- WP04 retention/delete/tombstone is now implemented and proved through `crates/schema/src/retention_delete_tombstone.rs`, `crates/storage-custody-core/src/retention_delete_tombstone.rs`, and `output/data-custody-storage-plan-proof/04-retention-delete-tombstone/`.
- WP05 export/import/backup/recovery is now implemented and proved through `crates/schema/src/export_import_backup_recovery.rs`, `crates/storage-custody-core/src/export_import_backup_recovery.rs`, and `output/data-custody-storage-plan-proof/05-export-import-backup-recovery/`.
- WP06 report/query custody is now implemented and proved through `crates/schema/src/report_query_custody.rs`, `crates/storage-custody-core/src/report_query_custody.rs`, `packages/schema-domain/src/generated/report-query-custody-contracts.ts`, and `output/data-custody-storage-plan-proof/06-report-query-custody/`.
- WP07 has a focused, real retention lifecycle through the Rust event journal,
  child-runtime durable outbox, reusable startup recovery, and explicit terminal
  acknowledgement. Its cited aggregate `output/` proof root is not present in a
  clean checkout because `output/` is ignored, and no concrete child-service
  startup owner invokes the recovery seam yet, so WP07 cannot be used for
  aggregate route truth.
- WP08 parent storage settings/apply flow is now implemented and proved through `crates/schema/src/parent_storage_settings_apply_flow.rs`, `crates/schema/src/parent_storage_settings_apply_flow_ts.rs`, `crates/storage-custody-core/src/parent_storage_settings_apply_flow.rs`, `packages/schema-domain/src/generated/parent-storage-settings-apply-flow-contracts.ts`, `packages/schema-domain/src/parent-storage-settings-apply-flow.ts`, and `output/data-custody-storage-plan-proof/08-parent-storage-settings-apply-flow/`.

## Open Product Gaps

- Zero-knowledge versus recoverable support mode is still a product decision beyond WP02's explicit manual-required and no-universal-key boundary.
- Parent-owned cloud default, provider choice defaults, and visible versus app-specific folder policy are still open.
- Provider sync runtime and provider/file retrieval execution remain open. WP03 now has a green shared sync/provider/tombstone proof surface and no-claim boundary, but it still does not claim provider-side OAuth/upload/delete/retrieval execution.
- AI runtime custody and support diagnostics remain open.
- Proof artifacts must be created by implementation work; this plan only defines expected proof.
- Adjacent implementation plans must be updated only when their workpack is selected.

## No-Read Boundary

Do not read adjacent plans or source trees until a workpack names the exact handoff.

Use `WORKPACK_FAMILIES.md` only when the selected workpack owner/proof family is unclear; do not use it as permission to scan a whole family.

## HID Execution Guard (added 2026-06-12)

- Scope and completion source:
  - follow [PLAN_EXECUTION_BLUEPRINT.md](PLAN_EXECUTION_BLUEPRINT.md) execution slices, then this plan's assigned WORKPACK_INDEX.md and NEXT_ACTIONS.md.
  - do not mark this plan complete from checklist deltas alone.
- Active proof-root route:
  - use `output/data-custody-storage-plan-proof/<workpack-file-stem>/` plus the selected workpack's required artifacts from [PROOF_INDEX.md](PROOF_INDEX.md).
  - legacy `docs/proof/data-custody-storage-plan/` references are stale for new proof and should be removed as touched rather than treated as current proof truth.
- Before any checked update, attach:
  - a real test run log or explicit blocker from the assigned implementation boundary,
  - a proof artifact under the selected output proof root,
  - negative cases, no-claim language, and manual-required notes where applicable.
- Failure rule: no PR-ready claim until replay/idempotency, authZ/replay, deletion/tombstone, and rollback/teardown proofs are present or carried as exact blockers for the assigned slice.

## Execution Blueprint

Continue execution from: [PLAN_EXECUTION_BLUEPRINT.md](PLAN_EXECUTION_BLUEPRINT.md).
Update this plan only via the blueprint and matching workpack proof rows.
