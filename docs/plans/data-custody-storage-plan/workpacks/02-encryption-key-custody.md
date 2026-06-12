# Workpack 02: Encryption Key Custody

Goal: define encryption and key custody so exported/synced data is unreadable without parent authority.

Context to read:

- `docs/expectations/data-custody.md`
- `docs/expectations/sync-export.md`
- `docs/expectations/static-analysis-security.md`
- `docs/plans/account-identity-family-plan/AGENTS.md` only if identity recovery is in scope.

In scope:

- Encryption-at-rest expectations for local store, export bundle, cloud sync bundle, backup, diagnostics, and temporary queues.
- Key owner, key recovery, key rotation, lost-key state, household transfer, and revoked-parent behavior.
- Metadata that must remain unencrypted for routing versus payload that must be encrypted.
- Algorithm/implementation decision criteria, not code.

Out of scope:

- Writing crypto implementation code.
- Choosing an auth provider unless it affects key recovery.

Decisions required:

- Parent passphrase/device-bound key/cloud-key split.
- Recovery tradeoff: zero-knowledge versus recoverable family support.
- Key rotation and migration states.
- How child devices receive only the minimum keys they need.

Expected artifacts:

- Key custody model.
- Encrypted bundle shape.
- Recovery and lost-key decision record.
- Redaction and secret-storage requirements.

Expected proof:

- Wrong-parent/wrong-household decrypt rejection.
- Revoked parent/device key invalidation.
- Corrupt bundle detection.
- No plaintext sensitive payload in logs/export/cloud sync proof.

Failure conditions:

- Raw readable JSON export for sensitive data.
- Ocentra-held universal key that can read parent data without an explicit product decision and disclosure.
