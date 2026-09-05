use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

#[macro_use]
mod macros;
mod sample;

// Rust-owned schema and value constants. Keep each string on its own const line so the
// string-boundary guard recognizes the ownership boundary.
pub const CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_SCHEMA_VERSION: &str =
    "child-signing-store-device-owner-matrix-proof";

pub const CHILD_ARTIFACT_MATRIX_PLATFORM_WINDOWS: &str = "windows";
pub const CHILD_ARTIFACT_MATRIX_PLATFORM_MACOS: &str = "macos";
pub const CHILD_ARTIFACT_MATRIX_PLATFORM_LINUX: &str = "linux";
pub const CHILD_ARTIFACT_MATRIX_PLATFORM_ANDROID: &str = "android";
pub const CHILD_ARTIFACT_MATRIX_PLATFORM_IOS: &str = "ios";

pub const CHILD_ARTIFACT_MATRIX_ARTIFACT_KIND_WINDOWS_MSI_SERVICE_PACKAGE: &str =
    "windows-msi-service-package";
pub const CHILD_ARTIFACT_MATRIX_ARTIFACT_KIND_MACOS_LAUNCHD_PKG: &str = "macos-launchd-pkg";
pub const CHILD_ARTIFACT_MATRIX_ARTIFACT_KIND_LINUX_SYSTEMD_DEB: &str = "linux-systemd-deb";
pub const CHILD_ARTIFACT_MATRIX_ARTIFACT_KIND_ANDROID_DEBUG_APK: &str = "android-debug-apk";
pub const CHILD_ARTIFACT_MATRIX_ARTIFACT_KIND_IOS_SIMULATOR_APP_ZIP: &str = "ios-simulator-app-zip";

pub const CHILD_ARTIFACT_MATRIX_DISTRIBUTION_MODE_DIRECT_MSI_DOWNLOAD: &str = "direct-msi-download";
pub const CHILD_ARTIFACT_MATRIX_DISTRIBUTION_MODE_DIRECT_PKG_DOWNLOAD: &str = "direct-pkg-download";
pub const CHILD_ARTIFACT_MATRIX_DISTRIBUTION_MODE_DIRECT_DEB_DOWNLOAD: &str = "direct-deb-download";
pub const CHILD_ARTIFACT_MATRIX_DISTRIBUTION_MODE_DEBUG_APK_SIDELOAD: &str = "debug-apk-sideload";
pub const CHILD_ARTIFACT_MATRIX_DISTRIBUTION_MODE_UNSIGNED_SIMULATOR_ZIP: &str =
    "unsigned-simulator-zip";

pub const CHILD_ARTIFACT_MATRIX_ARTIFACT_PROOF_STATE_CI_MECHANICAL_PROOF: &str =
    "ci-mechanical-proof";
pub const CHILD_ARTIFACT_MATRIX_ARTIFACT_PROOF_STATE_CI_PACKAGE_ONLY: &str = "ci-package-only";
pub const CHILD_ARTIFACT_MATRIX_ARTIFACT_PROOF_STATE_SIMULATOR_SCAFFOLD: &str =
    "simulator-scaffold";

pub const CHILD_ARTIFACT_MATRIX_PROOF_SOURCE_WINDOWS_RELEASE_SCRIPT: &str =
    "windows-release-script";
pub const CHILD_ARTIFACT_MATRIX_PROOF_SOURCE_MACOS_SERVICE_PACKAGE_PROOF: &str =
    "macos-service-package-proof";
pub const CHILD_ARTIFACT_MATRIX_PROOF_SOURCE_LINUX_SERVICE_PACKAGE_PROOF: &str =
    "linux-service-package-proof";
pub const CHILD_ARTIFACT_MATRIX_PROOF_SOURCE_ANDROID_DEVICE_PROOF_GATE: &str =
    "android-device-proof-gate";
pub const CHILD_ARTIFACT_MATRIX_PROOF_SOURCE_IOS_ENTITLEMENT_PROOF: &str = "ios-entitlement-proof";

pub const CHILD_ARTIFACT_MATRIX_SIGNING_STATE_UNSIGNED: &str = "unsigned";
pub const CHILD_ARTIFACT_MATRIX_SIGNING_STATE_DEBUG_SIGNED: &str = "debug-signed";
pub const CHILD_ARTIFACT_MATRIX_SIGNING_STATE_SIGNING_DISABLED: &str = "signing-disabled";

pub const CHILD_ARTIFACT_MATRIX_STORE_DISTRIBUTION_STATE_NOT_APPLICABLE: &str = "not-applicable";
pub const CHILD_ARTIFACT_MATRIX_STORE_DISTRIBUTION_STATE_PLANNED: &str = "planned";

pub const CHILD_ARTIFACT_MATRIX_MANAGEMENT_STATE_NOT_APPLICABLE: &str = "not-applicable";
pub const CHILD_ARTIFACT_MATRIX_MANAGEMENT_STATE_MANUAL_REQUIRED: &str = "manual-required";
pub const CHILD_ARTIFACT_MATRIX_MANAGEMENT_STATE_DEVICE_PROOF_REQUIRED: &str =
    "device-proof-required";

pub const CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_CHECKED_AT: &str = "2026-06-28T19:45:00.000Z";

pub const CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_WINDOWS_ARTIFACT_PACKAGE_REF: &str =
    "target/release-packages/ocentra-parent-agent-windows-x64-latest.msi";
pub const CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_WINDOWS_PROOF_REF: &str =
    "scripts/release/windows/build-agent-package.ps1";
pub const CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_WINDOWS_SIGNING_BOUNDARY: &str = "Windows MSI packaging script signs the updater manifest but does not Authenticode-sign the child MSI or service binaries in this proof surface";
pub const CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_WINDOWS_STORE_BOUNDARY: &str = "Windows child artifact is a direct MSI download; no Microsoft Store or other store publication is claimed";
pub const CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_WINDOWS_MANAGEMENT_BOUNDARY: &str = "Windows child artifact has no device-owner, managed-profile, or supervision claim in this matrix";
pub const CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_WINDOWS_CLAIM_BOUNDARY: &str = "Windows row proves MSI/service packaging and signed update-manifest wiring only; it does not prove signed child artifacts, store publication, or parent-client parity";

pub const CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_MACOS_ARTIFACT_PACKAGE_REF: &str =
    "target/release-packages/macos/ocentra-parent-agent-macos-latest.pkg";
pub const CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_MACOS_PROOF_REF_1: &str =
    "packages/schema-domain/src/child-macos-service-package-proof.ts";
pub const CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_MACOS_PROOF_REF_2: &str =
    "scripts/test/child-macos-service-package-proof.mjs";
pub const CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_MACOS_PROOF_REF_3: &str =
    "scripts/release/macos/build-agent-package.sh";
pub const CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_MACOS_SIGNING_BOUNDARY: &str = "macOS child package stays unsigned in this proof surface because no codesign or productsign artifact is attached";
pub const CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_MACOS_STORE_BOUNDARY: &str = "macOS child artifact is a direct pkg download; no Mac App Store or other store publication is claimed";
pub const CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_MACOS_MANAGEMENT_BOUNDARY: &str = "macOS child artifact has no device-owner, managed-profile, or supervision claim in this matrix";
pub const CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_MACOS_CLAIM_BOUNDARY: &str = "macOS row proves launchd pkg packaging only; it does not prove notarization, store publication, uninstall cleanup, or parent-client parity";

pub const CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_LINUX_ARTIFACT_PACKAGE_REF: &str =
    "target/release-packages/linux/ocentra-parent-agent-linux-amd64-latest.deb";
pub const CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_LINUX_PROOF_REF_1: &str =
    "packages/schema-domain/src/child-linux-service-package-proof.ts";
pub const CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_LINUX_PROOF_REF_2: &str =
    "scripts/test/child-linux-service-package-proof.mjs";
pub const CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_LINUX_PROOF_REF_3: &str =
    "scripts/release/linux/build-agent-package.sh";
pub const CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_LINUX_SIGNING_BOUNDARY: &str = "the child Linux package is unsigned in this proof surface because no debsig, dpkg-sig, GPG, or repository signature artifact is attached";
pub const CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_LINUX_STORE_BOUNDARY: &str = "Linux child artifact is a direct .deb download; no apt repository, Snap, or other store publication is claimed";
pub const CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_LINUX_MANAGEMENT_BOUNDARY: &str = "Linux child artifact has no device-owner, managed-profile, or supervision claim in this matrix";
pub const CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_LINUX_CLAIM_BOUNDARY: &str = "Linux row proves systemd .deb packaging and baseline metadata only; it does not prove signed repositories, store publication, or parent-client parity";

pub const CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_ANDROID_ARTIFACT_PACKAGE_REF: &str =
    "target/release-packages/android/ocentra-parent-agent-android-debug-latest.apk";
pub const CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_ANDROID_PROOF_REF_1: &str =
    "packages/schema-domain/src/child-android-device-proof-artifact-gate.ts";
pub const CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_ANDROID_PROOF_REF_2: &str =
    "scripts/test/child-android-device-proof-artifact-gate.mjs";
pub const CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_ANDROID_PROOF_REF_3: &str =
    "scripts/release/android/build-agent-package.mjs";
pub const CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_ANDROID_SIGNING_BOUNDARY: &str = "Android child artifact is a debug APK build; Play Store signing remains planned and not collected";
pub const CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_ANDROID_STORE_BOUNDARY: &str = "Android Play Store distribution remains planned and not collected; debug APK proof does not claim release-track publication";
pub const CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_ANDROID_MANAGEMENT_BOUNDARY: &str = "Android device-owner and managed-profile states remain manual-required without enrollment evidence";
pub const CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_ANDROID_CLAIM_BOUNDARY: &str = "Android row proves debug APK package output only; it does not prove device-owner, managed-profile, Play Store distribution, or parent-client parity";

pub const CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_IOS_ARTIFACT_PACKAGE_REF: &str =
    "target/release-packages/ios/ocentra-child-agent-ios-simulator-latest.zip";
pub const CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_IOS_PROOF_REF_1: &str =
    "packages/schema-domain/src/generated-child-ios-entitlement-capability-proof-contracts.ts";
pub const CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_IOS_PROOF_REF_2: &str =
    "scripts/test/child-ios-entitlement-capability-proof.mjs";
pub const CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_IOS_PROOF_REF_3: &str =
    "scripts/release/ios/build-simulator-app.sh";
pub const CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_IOS_SIGNING_BOUNDARY: &str = "iOS simulator package is built with code signing disabled; Apple signing, provisioning, and entitlements remain manual-required";
pub const CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_IOS_STORE_BOUNDARY: &str = "iOS TestFlight and App Store distribution remain device-proof-required or planned; simulator ZIP proof does not claim store publication";
pub const CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_IOS_MANAGEMENT_BOUNDARY: &str = "iOS supervision remains device-proof-required; no device-owner or managed-profile claim exists for the child iOS slice";
pub const CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_IOS_CLAIM_BOUNDARY: &str = "iOS row proves simulator scaffold packaging only; it does not prove Apple provisioning, supervision parity, hidden daemon authority, or parent-client parity";

pub const CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_GENERIC_MATRIX_CLAIM_BOUNDARY: &str = "matrix rows summarize platform-specific package or proof artifacts only; they do not replace platform-specific package, device, or store proof";
pub const CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_SIGNING_PARITY_CLAIM_BOUNDARY: &str = "artifact signing states stay row-specific; signed update manifests, debug APK signatures, or unsigned simulator builds do not imply cross-platform signing parity";
pub const CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_STORE_PARITY_CLAIM_BOUNDARY: &str = "store states stay row-specific; direct-download rows do not claim Microsoft Store, Mac App Store, Linux repository publication, Play Store, TestFlight, or App Store publication";
pub const CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_MANAGEMENT_PARITY_CLAIM_BOUNDARY: &str = "device-owner, managed-profile, and supervision states stay platform-specific and manual-required, device-proof-required, or not-applicable unless a row proves otherwise";
pub const CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_PARENT_PARITY_CLAIM_BOUNDARY: &str = "child artifact matrix does not imply parent-client parity, hidden daemons, or broader child runtime readiness";

matrix_string_enums!(
    ChildArtifactMatrixPlatform {
        variants: [Windows, Macos, Linux, Android, Ios],
        values: [
            CHILD_ARTIFACT_MATRIX_PLATFORM_WINDOWS,
            CHILD_ARTIFACT_MATRIX_PLATFORM_MACOS,
            CHILD_ARTIFACT_MATRIX_PLATFORM_LINUX,
            CHILD_ARTIFACT_MATRIX_PLATFORM_ANDROID,
            CHILD_ARTIFACT_MATRIX_PLATFORM_IOS,
        ],
    },
    ChildArtifactMatrixArtifactKind {
        variants: [
            WindowsMsiServicePackage,
            MacosLaunchdPkg,
            LinuxSystemdDeb,
            AndroidDebugApk,
            IosSimulatorAppZip,
        ],
        values: [
            CHILD_ARTIFACT_MATRIX_ARTIFACT_KIND_WINDOWS_MSI_SERVICE_PACKAGE,
            CHILD_ARTIFACT_MATRIX_ARTIFACT_KIND_MACOS_LAUNCHD_PKG,
            CHILD_ARTIFACT_MATRIX_ARTIFACT_KIND_LINUX_SYSTEMD_DEB,
            CHILD_ARTIFACT_MATRIX_ARTIFACT_KIND_ANDROID_DEBUG_APK,
            CHILD_ARTIFACT_MATRIX_ARTIFACT_KIND_IOS_SIMULATOR_APP_ZIP,
        ],
    },
    ChildArtifactMatrixDistributionMode {
        variants: [
            DirectMsiDownload,
            DirectPkgDownload,
            DirectDebDownload,
            DebugApkSideload,
            UnsignedSimulatorZip,
        ],
        values: [
            CHILD_ARTIFACT_MATRIX_DISTRIBUTION_MODE_DIRECT_MSI_DOWNLOAD,
            CHILD_ARTIFACT_MATRIX_DISTRIBUTION_MODE_DIRECT_PKG_DOWNLOAD,
            CHILD_ARTIFACT_MATRIX_DISTRIBUTION_MODE_DIRECT_DEB_DOWNLOAD,
            CHILD_ARTIFACT_MATRIX_DISTRIBUTION_MODE_DEBUG_APK_SIDELOAD,
            CHILD_ARTIFACT_MATRIX_DISTRIBUTION_MODE_UNSIGNED_SIMULATOR_ZIP,
        ],
    },
    ChildArtifactMatrixArtifactProofState {
        variants: [CiMechanicalProof, CiPackageOnly, SimulatorScaffold],
        values: [
            CHILD_ARTIFACT_MATRIX_ARTIFACT_PROOF_STATE_CI_MECHANICAL_PROOF,
            CHILD_ARTIFACT_MATRIX_ARTIFACT_PROOF_STATE_CI_PACKAGE_ONLY,
            CHILD_ARTIFACT_MATRIX_ARTIFACT_PROOF_STATE_SIMULATOR_SCAFFOLD,
        ],
    },
    ChildArtifactMatrixProofSource {
        variants: [
            WindowsReleaseScript,
            MacosServicePackageProof,
            LinuxServicePackageProof,
            AndroidDeviceProofGate,
            IosEntitlementProof,
        ],
        values: [
            CHILD_ARTIFACT_MATRIX_PROOF_SOURCE_WINDOWS_RELEASE_SCRIPT,
            CHILD_ARTIFACT_MATRIX_PROOF_SOURCE_MACOS_SERVICE_PACKAGE_PROOF,
            CHILD_ARTIFACT_MATRIX_PROOF_SOURCE_LINUX_SERVICE_PACKAGE_PROOF,
            CHILD_ARTIFACT_MATRIX_PROOF_SOURCE_ANDROID_DEVICE_PROOF_GATE,
            CHILD_ARTIFACT_MATRIX_PROOF_SOURCE_IOS_ENTITLEMENT_PROOF,
        ],
    },
    ChildArtifactMatrixSigningState {
        variants: [Unsigned, DebugSigned, SigningDisabled],
        values: [
            CHILD_ARTIFACT_MATRIX_SIGNING_STATE_UNSIGNED,
            CHILD_ARTIFACT_MATRIX_SIGNING_STATE_DEBUG_SIGNED,
            CHILD_ARTIFACT_MATRIX_SIGNING_STATE_SIGNING_DISABLED,
        ],
    },
    ChildArtifactMatrixStoreDistributionState {
        variants: [NotApplicable, Planned],
        values: [
            CHILD_ARTIFACT_MATRIX_STORE_DISTRIBUTION_STATE_NOT_APPLICABLE,
            CHILD_ARTIFACT_MATRIX_STORE_DISTRIBUTION_STATE_PLANNED,
        ],
    },
    ChildArtifactMatrixManagementState {
        variants: [NotApplicable, ManualRequired, DeviceProofRequired],
        values: [
            CHILD_ARTIFACT_MATRIX_MANAGEMENT_STATE_NOT_APPLICABLE,
            CHILD_ARTIFACT_MATRIX_MANAGEMENT_STATE_MANUAL_REQUIRED,
            CHILD_ARTIFACT_MATRIX_MANAGEMENT_STATE_DEVICE_PROOF_REQUIRED,
        ],
    },
);

matrix_text_identifier!(ChildArtifactMatrixPath);
matrix_text_identifier!(ChildArtifactMatrixBoundary);
matrix_text_identifier!(ChildArtifactMatrixTimestamp);

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChildArtifactMatrixRow {
    pub platform: ChildArtifactMatrixPlatform,
    pub artifact_kind: ChildArtifactMatrixArtifactKind,
    pub distribution_mode: ChildArtifactMatrixDistributionMode,
    pub artifact_proof_state: ChildArtifactMatrixArtifactProofState,
    pub artifact_package_ref: ChildArtifactMatrixPath,
    pub proof_source: ChildArtifactMatrixProofSource,
    pub proof_refs: Vec<ChildArtifactMatrixPath>,
    pub signing_state: ChildArtifactMatrixSigningState,
    pub store_distribution_state: ChildArtifactMatrixStoreDistributionState,
    pub device_owner_state: ChildArtifactMatrixManagementState,
    pub managed_profile_state: ChildArtifactMatrixManagementState,
    pub supervision_state: ChildArtifactMatrixManagementState,
    pub signing_boundary: ChildArtifactMatrixBoundary,
    pub store_boundary: ChildArtifactMatrixBoundary,
    pub management_boundary: ChildArtifactMatrixBoundary,
    pub claim_boundary: ChildArtifactMatrixBoundary,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChildArtifactMatrixClaimBoundaries {
    pub generic_matrix: ChildArtifactMatrixBoundary,
    pub signing_parity: ChildArtifactMatrixBoundary,
    pub store_parity: ChildArtifactMatrixBoundary,
    pub management_parity: ChildArtifactMatrixBoundary,
    pub parent_parity: ChildArtifactMatrixBoundary,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChildSigningStoreDeviceOwnerMatrixProof {
    pub schema_version: String,
    pub checked_at: ChildArtifactMatrixTimestamp,
    pub rows: Vec<ChildArtifactMatrixRow>,
    pub claim_boundaries: ChildArtifactMatrixClaimBoundaries,
}

pub fn sample_child_signing_store_device_owner_matrix_proof(
) -> ChildSigningStoreDeviceOwnerMatrixProof {
    sample::sample_child_signing_store_device_owner_matrix_proof()
}
