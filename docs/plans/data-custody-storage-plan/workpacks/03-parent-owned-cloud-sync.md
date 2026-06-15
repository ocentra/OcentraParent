# Workpack 03: Parent-Owned Cloud Sync

Goal: define sync to parent-owned storage without turning Ocentra into the data store.

Context to read:

- `docs/plans/data-custody-storage-plan/DECISIONS.md`
- `docs/plans/data-custody-storage-plan/PARENT_STORAGE_PROVIDER_MATRIX.md`
- `docs/expectations/cloud.md`
- `docs/expectations/sync-export.md`
- `docs/features/remote-lan-mobile-platforms.md`

In scope:

- Google Drive, OneDrive, iCloud, local folder, NAS, and later provider adapter criteria.
- Sync manifest, cursor, conflict, tombstone, retry, partial outage, quota, and permission loss states.
- Parent-visible provider status.
- Encryption-before-upload, manifest integrity, provider permission scopes, provider revocation, and parent-controlled disconnect.
- No Ocentra-hosted default storage for child activity or evidence.

Out of scope:

- Provider SDK implementation code.
- Report rendering.
- LAN transport internals.
- Restore/apply-back flow beyond provider custody and sync behavior.

Acceptance:

- Provider mode split is explicit.
- Sync states are explicit and claim-safe.
- Encryption-before-upload is explicit.
- Provider success never implies readable payloads without keys.
- Ocentra never becomes the default evidence store.

Expected artifacts:

- Provider capability matrix.
- Sync state machine.
- Conflict and tombstone rules.
- Provider permission and revocation matrix.
- Encrypted manifest and bundle contract.

Expected proof names:

- `data-custody.sync.provider-capability-matrix`
- `data-custody.sync.encrypted-before-upload`
- `data-custody.sync.offline-retry-partial-outage`
- `data-custody.sync.quota-conflict-corruption`
- `data-custody.sync.permission-revoked`
- `data-custody.sync.tombstone-propagation`
- `data-custody.sync.no-ocentra-default-store-negative`

Failure conditions:

- Ocentra cloud silently becomes the default evidence store.
- Sync marks success before encrypted payload, manifest, and tombstone state are consistent.
- Sync resurrects deleted evidence.
- Provider receives readable child activity without explicit parent-approved custody proof.

