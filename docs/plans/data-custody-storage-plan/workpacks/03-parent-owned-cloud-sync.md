# Workpack 03: Parent-Owned Cloud Sync

Goal: define sync to parent-owned storage without turning Ocentra into the data store.

Context to read:

- `docs/expectations/cloud.md`
- `docs/expectations/sync-export.md`
- `docs/expectations/data-custody.md`
- `docs/features/remote-lan-mobile-platforms.md`

In scope:

- Google Drive, iCloud, OneDrive, local folder, and later provider adapter criteria.
- Sync manifest, cursor, conflict, tombstone, retry, partial outage, quota, and permission loss states.
- Parent-visible provider status.
- Ocentra relay boundary for optional remote access and account metadata.
- Encryption-before-upload, manifest integrity, provider permission scopes, provider revocation, and parent-controlled disconnect.

Out of scope:

- Provider SDK implementation code.
- Report rendering.
- LAN transport internals.
- Ocentra-hosted default storage for child activity/evidence.

Decision tree:

| If the assignment touches...                | Route                                                              |
| ------------------------------------------- | ------------------------------------------------------------------ |
| Provider choice, scopes, auth expiry, quota | this workpack and account plan only for identity/session authority |
| Bundle encryption/key custody               | WP02 encryption/key custody before provider sync                   |
| Delete propagation                          | WP04 retention/delete/tombstone before sync success claims         |
| Export/import portability                   | WP05 export/import before backup/restore claims                    |
| Remote relay/live access                    | remote-access-plan; do not treat sync as live remote control       |

Expected sync states:

- `notConfigured`: parent has not chosen a provider.
- `configuredNoPermission`: provider linked but required permission missing.
- `syncPending`: local encrypted bundle/manifest is queued.
- `syncing`: upload/download in progress with cursor and retry budget.
- `synced`: manifest, encrypted payload, tombstones, and cursor agree.
- `conflict`: concurrent local/provider state needs deterministic resolution.
- `quotaExceeded`: provider rejects write; local data remains authoritative.
- `revoked`: provider permission was removed; no silent fallback to Ocentra storage.
- `corruptRemote`: manifest/checksum/decryption failed; do not import.
- `partialOutage`: provider unavailable; local writes continue with queued sync.

Decisions required:

- Minimum portable bundle format.
- Provider-neutral sync contract.
- Conflict resolution strategy.
- What happens when cloud provider auth expires or quota is exceeded.
- Whether Ocentra stores only relay/account metadata or any encrypted sync broker metadata.

Expected artifacts:

- Provider capability matrix.
- Sync state machine.
- Conflict and tombstone rules.
- Provider permission and revocation matrix.
- Encrypted manifest/bundle contract and provider-scope table.

Expected proof:

- Offline/partial outage/retry proof.
- Revoked provider permission proof.
- Quota/corruption/conflict proof.
- No provider receives unencrypted sensitive payload unless explicitly allowed.
- Cross-device tombstone replay proof.
- Provider disconnect proof.
- Wrong-household/wrong-key import rejection proof.

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
- Provider sync marks success before encrypted payload, manifest, and tombstone state are consistent.
- Sync resurrects deleted evidence.
- Provider receives readable child activity/evidence without explicit parent-approved custody proof.
