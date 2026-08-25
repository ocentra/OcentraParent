# Workpack 09 - Child Signing Store Device Owner Matrix

<!-- agent-capsule -->

> Agent Capsule
> Plan: `child-agent-runtime-distribution-plan`
> Workpack: `09-child-signing-store-device-owner-matrix`
> Kind: workpack route and proof contract.

<!-- /agent-capsule -->

Purpose: define the signing, store, and device-owner matrix for child artifacts by platform.

## Owns

- signing state by artifact
- store distribution state by platform
- device-owner or managed-profile truth where applicable
- manual-required state when a platform cannot prove the claim

## Must prove

- each child artifact names its signing and store assumptions explicitly
- device-owner claims are platform-specific and evidence-backed
- unsupported store or signing states are manual-required, not hidden
- the matrix covers Windows, macOS, Linux, Android, and iOS rows honestly

## Failure conditions

- a generic signing/store claim replaces the platform matrix
- device-owner support is implied without platform proof
- manual-required rows are omitted
- child and parent artifact claims are conflated

## Execution truth

Status: source partial / integration missing. Windows updater enforcement and a typed platform matrix exist; live WP10 handoff consumption, scheduling/restart ownership, non-Windows update/signing/store implementation, tests, validation, and proof remain open.

## Production code boundary

- `crates/agent-updater/src/manifest.rs` owns Windows child manifest policy: canonical serde payload bytes, child product/package/service identity, safe `.msi` artifact names, strict SHA-256, and external Ed25519 verification.
- `crates/agent-updater/src/hash.rs` rejects malformed checksums before artifact install; `update.rs` rejects unsafe names, uses an updater-owned random temporary directory/file, waits for MSI completion, and cleans the directory afterward.
- `scripts/release/windows/build-agent-package.ps1` requires an externally supplied child signing key for normal releases; ephemeral keys require an explicit preview switch.
- `scripts/release/windows/install-latest-windows.ps1` requires an externally supplied verifier executable and `OCENTRA_CHILD_UPDATE_PUBLIC_KEY_BASE64` before consuming a release manifest.
- Windows MSI is the only platform-owned update consumer in this slice. macOS/Linux/Android/iOS signing, store, and device-owner states remain manual-required or unsigned as documented below.
- Existing deferred updater contract fixtures still use legacy `OcentraParent*` identities and must be migrated before validation; no test result is inferred from this code pass.

The updater binary/CLI invokes the Windows update path directly and never calls `consume_setup_device_trust_handoff`. No production scheduler, durable retry/restart state, or setup/device-trust handoff caller connects the typed projection to update execution. macOS, Linux, Android, and iOS remain matrix statements/manual-required states rather than platform update/store implementations.

## Required production source outcome

- consume WP10's live durable handoff before update/install execution and retain its no-claim/manual-required state;
- own scheduler/retry/restart and installer outcome handoff without treating process completion as installed service health;
- finish platform-specific signing/store/update ownership or retain explicit unsupported/manual-required results;
- use canonical package identities from WP02-WP06 and never infer device authority from the matrix.

Implementation dependencies: Child WP02-WP06 and WP10 reviewed implementation. Normal READY/DONE remains strict.

## Expected test-source gap

- live handoff acceptance/rejection, replay/expiry/manual-required behavior, scheduler retry, restart, and installer result projection;
- signature, key, manifest, checksum, artifact-name, and service-identity negative cases through the production update path;
- crash/restart and cleanup of updater-owned temporary custody;
- platform-specific signing/store/device-owner behavior and explicit unsupported/manual-required outcomes.

Historical contract/proof owner surface (not runtime completion):

- `crates/schema/src/child_signing_store_device_owner_matrix.rs`
- `crates/schema/src/child_signing_store_device_owner_matrix_ts.rs`
- `crates/schema/src/bin/export_child_signing_store_device_owner_matrix_contract_types.rs`
- `crates/schema/tests/contract/child_signing_store_device_owner_matrix.rs`
- `packages/schema-domain/src/generated/child-signing-store-device-owner-matrix-contracts.ts`
- `packages/schema-domain/src/child-signing-store-device-owner-matrix.ts`
- `packages/schema-domain/tests/proof/child-signing-store-device-owner-matrix.test.ts`
- `scripts/test/child-signing-store-device-owner-matrix.mjs`
- `output/child-agent-runtime-distribution-plan-proof/09-child-signing-store-device-owner-matrix/`

Proof root:

- `output/child-agent-runtime-distribution-plan-proof/09-child-signing-store-device-owner-matrix/`
- runtime proof JSON: `test-results/child-signing-store-device-owner-matrix/proof.json`

## Contract matrix states (not runtime proof)

- Windows: `windows-msi-service-package`, direct MSI download, `unsigned` child artifact, signed updater manifest only, store `not-applicable`, device-owner/managed-profile/supervision `not-applicable`.
- macOS: `macos-launchd-pkg`, direct pkg download, `unsigned` child artifact, store `not-applicable`, device-owner/managed-profile/supervision `not-applicable`.
- Linux: `linux-systemd-deb`, direct `.deb` download, `unsigned` child artifact, store `not-applicable`, device-owner/managed-profile/supervision `not-applicable`.
- Android: `android-debug-apk`, debug APK sideload, `debug-signed` artifact, Play Store `planned`, device-owner `manual-required`, managed-profile `manual-required`, supervision `not-applicable`.
- iOS: `ios-simulator-app-zip`, unsigned simulator ZIP, `signing-disabled` artifact, TestFlight/App Store `planned`, supervision `device-proof-required`, device-owner/managed-profile `not-applicable`.

## Manual-required and no-claim boundary

- Windows updater-manifest signing does not upgrade the child MSI or service binaries to Authenticode-signed proof.
- macOS pkg proof does not claim codesign, notarization, store publication, uninstall cleanup, or parent-client parity.
- Linux `.deb` proof does not claim package signing, repository publication, distro-wide parity, or parent-client parity.
- Android debug APK proof does not claim Play Store release-track publication, device-owner parity, managed-profile parity, or external child-agent parity.
- iOS simulator proof does not claim Apple signing, provisioning, supervision parity, hidden daemon authority, or parent-client parity.
- The generic matrix does not replace platform-specific package, device, or store proofs.

## Canonical ownership boundary

- Rust/shared contract truth now lives in `crates/schema/src/child_signing_store_device_owner_matrix.rs`.
- `packages/schema-domain/src/generated/child-signing-store-device-owner-matrix-contracts.ts` is checked-in output from the Rust exporter only.
- `packages/schema-domain/src/child-signing-store-device-owner-matrix.ts` is a thin parse/coverage adapter and is no longer the canonical matrix owner.
- Parent-client distribution remains a sibling owner and is not implied by this child matrix.

## Deferred validations (not run in this production pass)

- `cargo test -p ocentra-schema --test contract child_signing_store_device_owner_matrix`
- `cmd /c npm exec --workspace @ocentra-parent/schema-domain -- vitest run tests/proof/child-signing-store-device-owner-matrix.test.ts`
- `cmd /c node scripts/test/child-signing-store-device-owner-matrix.mjs`
- `cmd /c npm run lint:architecture -- --files packages/schema-domain/src/child-signing-store-device-owner-matrix.ts packages/schema-domain/src/generated/child-signing-store-device-owner-matrix-contracts.ts packages/schema-domain/tests/proof/child-signing-store-device-owner-matrix.test.ts scripts/test/child-signing-store-device-owner-matrix.mjs`
- `cargo lint-architecture crates/schema/src/child_signing_store_device_owner_matrix.rs crates/schema/src/child_signing_store_device_owner_matrix_ts.rs crates/schema/src/bin/export_child_signing_store_device_owner_matrix_contract_types.rs crates/schema/tests/contract/child_signing_store_device_owner_matrix.rs crates/schema/src/lib.rs crates/schema/tests/contract.rs`

## Historical validation note

- `cmd /c npm run build --workspace @ocentra-parent/schema-domain` is currently red outside the WP09 owner surface.
- Exact failing files are `packages/schema-domain/src/generated/parent-owned-sync-export-contracts.ts`, `packages/schema-domain/src/parent-owned-sync-export-validation.ts`, and `packages/schema-domain/src/parent-owned-sync-export.ts`.
- The earlier focused contract packet did not establish the missing updater/handoff/platform runtime source and does not close WP09.

## Production-pass checklist

- [x] Windows updater enforces signed child manifest, strict checksum, safe artifact identity, and child service identity.
- [x] Windows release packaging requires external signing authority unless an explicit preview-only ephemeral switch is supplied.
- [x] Windows bootstrap fails closed without an external verifier and trusted public key.
- [x] macOS/Linux/Android/iOS signing/store/device-owner states remain explicit and non-upgraded.
- [ ] Platform matrix contract tests and focused validation are run.
- [ ] Release proof artifacts are refreshed under the declared output root.
- [ ] Store/device-owner artifacts are collected where platform authority is external.
