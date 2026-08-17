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

## Completion

- Status: production source accepted for the shared WP02 boundary; current expected tests, focused execution, proof refresh, and broader readiness remain open.
- Proof root: `output/data-custody-storage-plan-proof/02-encryption-key-custody/`
- Canonical owners: `crates/schema` for the shared key-custody contract and `crates/storage-custody-core` for platform-row and decrypt-decision state derivation.
- TS/shared edge note: no new `packages/schema-domain` surface was needed for WP02. TS ownership was not widened.

## Source-wave checkpoint (2026-08-17)

- `crates/storage-custody-core/src/encryption_key_custody_scope.rs` now binds the requested decrypt scope to the selected platform row's decrypt authority instead of trusting caller-supplied match flags alone.
- The implementation remains a shared custody decision boundary. Platform key wrappers, real provider consumers, and Device Trust authority stay with their owning plans.
- No tests were written or run in this source wave. The cross-scope negative matrix belongs to the expected-test wave; the older validation/proof record is not current acceptance for this changed source.

## Required acceptance proved

- Who can decrypt what is explicit by role, holder, unlock scope, and platform surface in the Rust-owned contract.
- Wrong-household and wrong-device decrypt attempts fail closed in the Rust runtime derivation.
- Revoked-key and lost-key states stay explicit in the Rust runtime derivation.
- Linux remains manual-required until a real secret-store decision exists.
- Android and iOS remain limited until device proof exists, and hosted portal cannot become a decrypt root.
- No universal Ocentra decrypt key is allowed by the Rust-owned contract/runtime path.

## Proof artifacts

- `00-key-custody-model-proof.md`
- `01-platform-key-wrapper-matrix-proof.md`
- `02-wrong-key-negative-proof.md`
- `03-revoked-device-negative-proof.md`
- `04-no-universal-ocentra-key-proof.md`
- `05-recovery-mode-proof.md`
- `16-validation-commands.log`

## Focused validations

- `cargo test -p ocentra-schema --test contract encryption_key_custody`
- `cargo test -p ocentra-storage-custody-core encryption_key_custody`
- `cargo lint-architecture crates/schema/src/lib.rs crates/schema/src/encryption_key_custody.rs crates/schema/src/data_custody_source_of_truth_ts.rs crates/schema/tests/contract.rs crates/schema/tests/contract/encryption_key_custody.rs crates/storage-custody-core/src/lib.rs crates/storage-custody-core/src/encryption_key_custody.rs crates/storage-custody-core/tests/unit.rs crates/storage-custody-core/tests/unit/encryption_key_custody.rs`

## Adjacent handoffs

- Device-trust remains the adjacent owner for proving mobile/device authority; WP02 consumes that proof state and does not re-own device-trust implementation.
- Provider sync remains a sibling owner for upload/download runtime and uses, but does not re-own, the WP02 key-custody boundary.
- Portal/web surfaces remain sibling owners for UI only and do not become decrypt roots in WP02.

## No-claim boundary

- No cryptographic primitive implementation claim is made.
- No provider sync runtime claim is made.
- No hosted portal decrypt-root claim is made.
- No LAN claim is made.
- No mobile decrypt-readiness claim is made beyond explicit proof-gated state.
