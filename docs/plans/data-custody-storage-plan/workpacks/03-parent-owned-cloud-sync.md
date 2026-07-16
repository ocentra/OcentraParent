# Workpack 03: Parent-Owned Cloud Sync

Goal: define sync to parent-owned storage without turning Ocentra into the data store.

Context to read:

- `docs/plans/data-custody-storage-plan/DECISIONS.md`
- `docs/plans/data-custody-storage-plan/PARENT_STORAGE_PROVIDER_MATRIX.md`
- `docs/plans/data-custody-storage-plan/PARENT_SAVE_RETRIEVE_APPLY_FLOW.md`
- `docs/plans/data-custody-storage-plan/KEY_CUSTODY_MODEL.md`
- `docs/expectations/cloud.md`
- `docs/expectations/sync-export.md`
- `docs/features/remote-lan-mobile-platforms.md`

In scope:

- Google Drive, OneDrive, iCloud, local folder, NAS, and later provider adapter criteria.
- Sync manifest, cursor, conflict, tombstone, retry, partial outage, quota, and permission loss states.
- Parent-visible provider status.
- Encryption-before-upload, manifest integrity, provider permission scopes, provider revocation, and parent-controlled disconnect.
- Provider delete/disconnect and tombstone propagation state.
- No Ocentra-hosted default storage for child activity or evidence.

Out of scope:

- Provider SDK implementation code unless a selected implementation slice names it.
- Provider OAuth runtime unless selected and proven.
- Upload/delete runtime unless selected and proven.
- Report rendering.
- LAN transport internals.
- Restore/apply-back flow beyond provider custody and sync behavior.

## Ownership boundary

```text
schema-domain owns parent-owned sync/export contract shapes.
data-custody-storage-plan owns provider custody policy and no-claim boundaries.
selected provider adapter owners own provider SDK/runtime behavior only when explicitly selected.
account-identity-family-plan owns provider account/actor authority when selected.
device-trust-bootstrap-plan owns trusted-device/key state when selected.
cloudflare-control-plane-plan owns Cloudflare runtime/storage bindings only through custody handoff.
remote-access-plan and LAN own their transport internals.
```

## Must prove

- Provider mode split is explicit.
- Sync states are explicit and claim-safe.
- Encryption-before-upload is explicit.
- Provider success never implies readable payloads without keys.
- Ocentra never becomes the default evidence store.
- Provider OAuth/upload/delete runtime is not claimed from contract or manifest proof alone.
- Tombstone propagation is named separately from export/sync success.
- Provider disconnect/delete state is visible and does not silently retain stale custody state.
- Provider revoked/wrong-account/folder-unavailable/partial-upload states remain visible.

## Expected artifacts

- Provider capability matrix.
- Sync state machine.
- Conflict and tombstone rules.
- Provider permission and revocation matrix.
- Encrypted manifest and bundle contract.
- Provider disconnect/delete state table.
- No-automatic-Ocentra-fallback negative proof.

## Expected proof names

- `data-custody.sync.provider-capability-matrix`
- `data-custody.sync.encrypted-before-upload`
- `data-custody.sync.offline-retry-partial-outage`
- `data-custody.sync.quota-conflict-corruption`
- `data-custody.sync.permission-revoked`
- `data-custody.sync.tombstone-propagation`
- `data-custody.sync.provider-disconnect-delete-state`
- `data-custody.sync.no-ocentra-default-store-negative`

## Required proof files

```text
output/data-custody-storage-plan-proof/03-parent-owned-cloud-sync/00-provider-capability-matrix-proof.md
output/data-custody-storage-plan-proof/03-parent-owned-cloud-sync/01-encrypted-before-upload-proof.md
output/data-custody-storage-plan-proof/03-parent-owned-cloud-sync/02-provider-revoked-state-proof.md
output/data-custody-storage-plan-proof/03-parent-owned-cloud-sync/03-quota-conflict-corruption-proof.md
output/data-custody-storage-plan-proof/03-parent-owned-cloud-sync/04-offline-retry-partial-outage-proof.md
output/data-custody-storage-plan-proof/03-parent-owned-cloud-sync/05-tombstone-propagation-proof.md
output/data-custody-storage-plan-proof/03-parent-owned-cloud-sync/06-no-automatic-ocentra-fallback-proof.md
output/data-custody-storage-plan-proof/03-parent-owned-cloud-sync/16-validation-commands.log
```

Failure conditions:

- Ocentra cloud silently becomes the default evidence store.
- Sync marks success before encrypted payload, manifest, and tombstone state are consistent.
- Sync resurrects deleted evidence.
- Provider receives readable child activity without explicit parent-approved custody proof.
- Provider status is used to claim OAuth/upload/delete runtime without selected proof.
- Provider disconnect/delete does not produce visible custody state.
- Parent storage provider choice overrides key custody or retention constraints.

## Completion

- Status: complete for WP03 only; no broader plan, provider-runtime, or PR readiness claim is made.
- Proof root: `output/data-custody-storage-plan-proof/03-parent-owned-cloud-sync/`
- Canonical owners: `crates/schema` for the shared sync/export contract and `crates/storage-custody-core` for the claim-safe sync/provider/tombstone runtime derivation.
- TS/shared edge note: `packages/schema-domain` is thin/generated validation only for WP03 and does not own the runtime or shared contract truth.

## What is actually proved

- Provider mode split is explicit across Google Drive, OneDrive, iCloud, Dropbox, NAS, local-folder, and disabled paths.
- Provider states are explicit for `ready`, `manual-required`, `revoked`, `wrong-account`, `folder-unavailable`, `partial-upload`, `disconnected`, and `disabled`.
- Sync states are explicit for `synced`, `stale`, `missing`, `conflict`, `offline-retry-pending`, `partial-outage`, `manual-required`, and `not-started`.
- Encryption-before-upload remains explicit for machine-readable payloads, while human-readable parent-authorized exports stay separate.
- Manifest integrity, checksum ref, and signature ref remain explicit for honest synced/stale claims.
- Tombstone propagation remains separate from sync success with explicit `not-requested`, `pending`, `propagated`, `blocked`, and `manual-required` states.
- Non-claims are explicit for no default Ocentra custody and no OAuth/upload/delete runtime claim from contract proof alone.

## Manual-required and no-claim truth

- `cmd /c cargo test -q -p ocentra-schema --test contract parent_owned_sync_export` is green.
- `cmd /c cargo test -q -p ocentra-storage-custody-core parent_owned_sync_export` is green.
- `cmd /c npm run build --workspace @ocentra-parent/schema-domain` is green on the cleared WP01 baseline.
- `node scripts/test/parent-owned-sync-export-manifest-proof.mjs` is green and emitted a fresh `parent-owned-sync-export-manifest-proof.json`.
- Provider OAuth runtime, provider upload runtime, provider delete runtime, and provider retrieval runtime remain manual-required/unproven in this lane.

## Proof artifacts

- `00-provider-capability-matrix-proof.md`
- `01-encrypted-before-upload-proof.md`
- `02-provider-revoked-state-proof.md`
- `03-quota-conflict-corruption-proof.md`
- `04-offline-retry-partial-outage-proof.md`
- `05-tombstone-propagation-proof.md`
- `06-no-automatic-ocentra-fallback-proof.md`
- `16-validation-commands.log`

## Focused validations

- `cmd /c cargo test -q -p ocentra-storage-custody-core parent_owned_sync_export`
- `cmd /c npm run test --workspace @ocentra-parent/schema-domain -- tests/contract/parent-owned-sync-export.test.ts`
- `cmd /c npm run lint:architecture -- --files packages/schema-domain/src/parent-owned-sync-export.ts packages/schema-domain/src/parent-owned-sync-export-validation.ts packages/schema-domain/tests/contract/parent-owned-sync-export.test.ts scripts/test/parent-owned-sync-export-manifest-proof.mjs`
- `cargo lint-architecture crates/schema/src/parent_owned_sync_export.rs crates/schema/src/parent_owned_sync_export_ts.rs crates/schema/src/bin/export_parent_owned_sync_export_contract_types.rs crates/storage-custody-core/src/parent_owned_sync_export.rs crates/storage-custody-core/tests/unit/parent_owned_sync_export.rs crates/schema/tests/contract/parent_owned_sync_export.rs`
- `cmd /c cargo test -q -p ocentra-schema --test contract parent_owned_sync_export`
- `cmd /c npm run build --workspace @ocentra-parent/schema-domain`
- `node scripts/test/parent-owned-sync-export-manifest-proof.mjs`

## No-claim boundary

- No provider OAuth readiness claim is made.
- No provider upload/delete/retrieval runtime claim is made.
- No Ocentra-hosted default child-evidence store claim is made.
- No portal, LAN, or sibling workpack completion claim is made.
