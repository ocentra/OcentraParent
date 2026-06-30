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

Status: complete.

Owner surface:

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

## Proved matrix states

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

## Validations

- `cargo test -p ocentra-schema --test contract child_signing_store_device_owner_matrix`
- `cmd /c npm exec --workspace @ocentra-parent/schema-domain -- vitest run tests/proof/child-signing-store-device-owner-matrix.test.ts`
- `cmd /c node scripts/test/child-signing-store-device-owner-matrix.mjs`
- `cmd /c npm run lint:architecture -- --files packages/schema-domain/src/child-signing-store-device-owner-matrix.ts packages/schema-domain/src/generated/child-signing-store-device-owner-matrix-contracts.ts packages/schema-domain/tests/proof/child-signing-store-device-owner-matrix.test.ts scripts/test/child-signing-store-device-owner-matrix.mjs`
- `cargo lint-architecture crates/schema/src/child_signing_store_device_owner_matrix.rs crates/schema/src/child_signing_store_device_owner_matrix_ts.rs crates/schema/src/bin/export_child_signing_store_device_owner_matrix_contract_types.rs crates/schema/tests/contract/child_signing_store_device_owner_matrix.rs crates/schema/src/lib.rs crates/schema/tests/contract.rs`

## External non-gating blocker

- `cmd /c npm run build --workspace @ocentra-parent/schema-domain` is currently red outside the WP09 owner surface.
- Exact failing files are `packages/schema-domain/src/generated/parent-owned-sync-export-contracts.ts`, `packages/schema-domain/src/parent-owned-sync-export-validation.ts`, and `packages/schema-domain/src/parent-owned-sync-export.ts`.
- WP09 completion is based on the focused Rust contract, thin adapter, proof runner, and focused architecture gates above; it does not claim the unrelated package-wide build is green.

## Completion checklist

- [x] Windows row is explicit about artifact kind, signing state, store state, and non-mobile management state.
- [x] macOS row is explicit about artifact kind, signing state, store state, and non-mobile management state.
- [x] Linux row is explicit about artifact kind, signing state, store state, and non-mobile management state.
- [x] Android row is explicit about debug signing, Play Store planning, and device-owner/managed-profile manual-required truth.
- [x] iOS row is explicit about signing-disabled simulator packaging, planned store distribution, and supervision device-proof-required truth.
- [x] Per-platform rows cite platform-specific proof references instead of a generic matrix claim.
- [x] Manual-required rows remain visible where platform proof is absent.
- [x] Device-owner, managed-profile, and supervision claims stay platform-specific.
- [x] Parent-client parity is explicitly excluded from the matrix.
- [x] Canonical shared matrix truth is Rust-owned; `schema-domain` stays generated/thin only.
- [x] Real tests live under `packages/schema-domain/tests/proof/`.
- [x] Real Rust contract tests live under `crates/schema/tests/contract/`.
- [x] Real proof runner lives under `scripts/test/`.
- [x] Proof root and focused validations are recorded under the WP09 output path.
