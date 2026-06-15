# Workpack 02: Encryption Key Custody

Goal: define encryption and key custody so exported or synced data is unreadable without the right household authority.

Context to read:

- `docs/plans/data-custody-storage-plan/DECISIONS.md`
- `docs/plans/data-custody-storage-plan/KEY_CUSTODY_MODEL.md`
- `docs/plans/data-custody-storage-plan/PLATFORM_KEY_CUSTODY_MATRIX.md`
- `docs/expectations/data-custody.md`
- `docs/expectations/sync-export.md`
- `docs/expectations/static-analysis-security.md`

In scope:

- Encryption-at-rest expectations for local store, export bundle, cloud sync bundle, backup, diagnostics, and temporary queues.
- Key owner, key recovery, key rotation, lost-key state, household transfer, and revoked-parent behavior.
- Platform-specific custody states for Windows, macOS, Linux, Android, iOS, web, parent desktop, child service, parent mobile, and child mobile.
- Metadata that must remain unencrypted for routing versus payload that must be encrypted.

Out of scope:

- Crypto implementation code.
- Auth-provider selection unless it affects key recovery.
- Provider sync flow beyond custody and decrypt authority.

Acceptance:

- Who can decrypt what is explicit by role and platform.
- Wrong-household, wrong-device, revoked-key, and lost-key states fail closed.
- Linux remains manual-required until a real secret-store decision exists.
- Android and iOS remain limited until device proof exists.

Expected artifacts:

- Key custody model.
- Platform custody matrix.
- Recovery and lost-key decision record.
- Redaction and secret-storage requirements.
- Loss and revocation matrix.

Expected proof names:

- `data-custody.keys.hierarchy-contract`
- `data-custody.keys.platform-custody-matrix`
- `data-custody.keys.wrong-household-negative`
- `data-custody.keys.revocation-negative`
- `data-custody.keys.loss-manual-required`

Failure conditions:

- Raw readable JSON export for sensitive data.
- A universal decrypt key that reads parent data by default.
- Mobile or web custody claims without platform proof.

