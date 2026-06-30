use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

macro_rules! matrix_text_identifier {
    ($name:ident) => {
        #[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(String);

        impl $name {
            pub fn parse(value: impl Into<String>) -> Option<Self> {
                let value = value.into();
                if value.trim().is_empty() {
                    None
                } else {
                    Some(Self(value))
                }
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl Display for $name {
            fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
                formatter.write_str(self.as_str())
            }
        }
    };
}

macro_rules! matrix_string_enum {
    ($name:ident { $($variant:ident => $value:ident),+ $(,)? }) => {
        #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
        #[serde(rename_all = "kebab-case")]
        pub enum $name {
            $( $variant, )+
        }

        impl $name {
            pub const fn as_str(&self) -> &'static str {
                match self {
                    $(Self::$variant => $value,)+
                }
            }
        }
    };
}

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
    "target/release-packages/ios/ocentra-parent-agent-ios-simulator-latest.zip";
pub const CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_IOS_PROOF_REF_1: &str =
    "packages/schema-domain/src/child-ios-entitlement-capability-proof.ts";
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

matrix_string_enum!(ChildArtifactMatrixPlatform {
    Windows => CHILD_ARTIFACT_MATRIX_PLATFORM_WINDOWS,
    Macos => CHILD_ARTIFACT_MATRIX_PLATFORM_MACOS,
    Linux => CHILD_ARTIFACT_MATRIX_PLATFORM_LINUX,
    Android => CHILD_ARTIFACT_MATRIX_PLATFORM_ANDROID,
    Ios => CHILD_ARTIFACT_MATRIX_PLATFORM_IOS,
});

matrix_string_enum!(ChildArtifactMatrixArtifactKind {
    WindowsMsiServicePackage => CHILD_ARTIFACT_MATRIX_ARTIFACT_KIND_WINDOWS_MSI_SERVICE_PACKAGE,
    MacosLaunchdPkg => CHILD_ARTIFACT_MATRIX_ARTIFACT_KIND_MACOS_LAUNCHD_PKG,
    LinuxSystemdDeb => CHILD_ARTIFACT_MATRIX_ARTIFACT_KIND_LINUX_SYSTEMD_DEB,
    AndroidDebugApk => CHILD_ARTIFACT_MATRIX_ARTIFACT_KIND_ANDROID_DEBUG_APK,
    IosSimulatorAppZip => CHILD_ARTIFACT_MATRIX_ARTIFACT_KIND_IOS_SIMULATOR_APP_ZIP,
});

matrix_string_enum!(ChildArtifactMatrixDistributionMode {
    DirectMsiDownload => CHILD_ARTIFACT_MATRIX_DISTRIBUTION_MODE_DIRECT_MSI_DOWNLOAD,
    DirectPkgDownload => CHILD_ARTIFACT_MATRIX_DISTRIBUTION_MODE_DIRECT_PKG_DOWNLOAD,
    DirectDebDownload => CHILD_ARTIFACT_MATRIX_DISTRIBUTION_MODE_DIRECT_DEB_DOWNLOAD,
    DebugApkSideload => CHILD_ARTIFACT_MATRIX_DISTRIBUTION_MODE_DEBUG_APK_SIDELOAD,
    UnsignedSimulatorZip => CHILD_ARTIFACT_MATRIX_DISTRIBUTION_MODE_UNSIGNED_SIMULATOR_ZIP,
});

matrix_string_enum!(ChildArtifactMatrixArtifactProofState {
    CiMechanicalProof => CHILD_ARTIFACT_MATRIX_ARTIFACT_PROOF_STATE_CI_MECHANICAL_PROOF,
    CiPackageOnly => CHILD_ARTIFACT_MATRIX_ARTIFACT_PROOF_STATE_CI_PACKAGE_ONLY,
    SimulatorScaffold => CHILD_ARTIFACT_MATRIX_ARTIFACT_PROOF_STATE_SIMULATOR_SCAFFOLD,
});

matrix_string_enum!(ChildArtifactMatrixProofSource {
    WindowsReleaseScript => CHILD_ARTIFACT_MATRIX_PROOF_SOURCE_WINDOWS_RELEASE_SCRIPT,
    MacosServicePackageProof => CHILD_ARTIFACT_MATRIX_PROOF_SOURCE_MACOS_SERVICE_PACKAGE_PROOF,
    LinuxServicePackageProof => CHILD_ARTIFACT_MATRIX_PROOF_SOURCE_LINUX_SERVICE_PACKAGE_PROOF,
    AndroidDeviceProofGate => CHILD_ARTIFACT_MATRIX_PROOF_SOURCE_ANDROID_DEVICE_PROOF_GATE,
    IosEntitlementProof => CHILD_ARTIFACT_MATRIX_PROOF_SOURCE_IOS_ENTITLEMENT_PROOF,
});

matrix_string_enum!(ChildArtifactMatrixSigningState {
    Unsigned => CHILD_ARTIFACT_MATRIX_SIGNING_STATE_UNSIGNED,
    DebugSigned => CHILD_ARTIFACT_MATRIX_SIGNING_STATE_DEBUG_SIGNED,
    SigningDisabled => CHILD_ARTIFACT_MATRIX_SIGNING_STATE_SIGNING_DISABLED,
});

matrix_string_enum!(ChildArtifactMatrixStoreDistributionState {
    NotApplicable => CHILD_ARTIFACT_MATRIX_STORE_DISTRIBUTION_STATE_NOT_APPLICABLE,
    Planned => CHILD_ARTIFACT_MATRIX_STORE_DISTRIBUTION_STATE_PLANNED,
});

matrix_string_enum!(ChildArtifactMatrixManagementState {
    NotApplicable => CHILD_ARTIFACT_MATRIX_MANAGEMENT_STATE_NOT_APPLICABLE,
    ManualRequired => CHILD_ARTIFACT_MATRIX_MANAGEMENT_STATE_MANUAL_REQUIRED,
    DeviceProofRequired => CHILD_ARTIFACT_MATRIX_MANAGEMENT_STATE_DEVICE_PROOF_REQUIRED,
});

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

fn path(value: &str) -> ChildArtifactMatrixPath {
    crate::schema_option_or_unreachable(ChildArtifactMatrixPath::parse(value), "matrix path")
}

fn boundary(value: &str) -> ChildArtifactMatrixBoundary {
    crate::schema_option_or_unreachable(
        ChildArtifactMatrixBoundary::parse(value),
        "matrix boundary",
    )
}

fn timestamp(value: &str) -> ChildArtifactMatrixTimestamp {
    crate::schema_option_or_unreachable(
        ChildArtifactMatrixTimestamp::parse(value),
        "matrix timestamp",
    )
}

struct ChildArtifactMatrixRowInput<'a> {
    platform: ChildArtifactMatrixPlatform,
    artifact_kind: ChildArtifactMatrixArtifactKind,
    distribution_mode: ChildArtifactMatrixDistributionMode,
    artifact_proof_state: ChildArtifactMatrixArtifactProofState,
    artifact_package_ref: &'a str,
    proof_source: ChildArtifactMatrixProofSource,
    proof_refs: &'a [&'a str],
    signing_state: ChildArtifactMatrixSigningState,
    store_distribution_state: ChildArtifactMatrixStoreDistributionState,
    device_owner_state: ChildArtifactMatrixManagementState,
    managed_profile_state: ChildArtifactMatrixManagementState,
    supervision_state: ChildArtifactMatrixManagementState,
    signing_boundary: &'a str,
    store_boundary: &'a str,
    management_boundary: &'a str,
    claim_boundary: &'a str,
}

fn row(input: ChildArtifactMatrixRowInput<'_>) -> ChildArtifactMatrixRow {
    let ChildArtifactMatrixRowInput {
        platform,
        artifact_kind,
        distribution_mode,
        artifact_proof_state,
        artifact_package_ref,
        proof_source,
        proof_refs,
        signing_state,
        store_distribution_state,
        device_owner_state,
        managed_profile_state,
        supervision_state,
        signing_boundary,
        store_boundary,
        management_boundary,
        claim_boundary,
    } = input;

    ChildArtifactMatrixRow {
        platform,
        artifact_kind,
        distribution_mode,
        artifact_proof_state,
        artifact_package_ref: path(artifact_package_ref),
        proof_source,
        proof_refs: proof_refs.iter().map(|value| path(value)).collect(),
        signing_state,
        store_distribution_state,
        device_owner_state,
        managed_profile_state,
        supervision_state,
        signing_boundary: boundary(signing_boundary),
        store_boundary: boundary(store_boundary),
        management_boundary: boundary(management_boundary),
        claim_boundary: boundary(claim_boundary),
    }
}

fn windows_row() -> ChildArtifactMatrixRow {
    row(ChildArtifactMatrixRowInput {
        platform: ChildArtifactMatrixPlatform::Windows,
        artifact_kind: ChildArtifactMatrixArtifactKind::WindowsMsiServicePackage,
        distribution_mode: ChildArtifactMatrixDistributionMode::DirectMsiDownload,
        artifact_proof_state: ChildArtifactMatrixArtifactProofState::CiMechanicalProof,
        artifact_package_ref: CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_WINDOWS_ARTIFACT_PACKAGE_REF,
        proof_source: ChildArtifactMatrixProofSource::WindowsReleaseScript,
        proof_refs: &[CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_WINDOWS_PROOF_REF],
        signing_state: ChildArtifactMatrixSigningState::Unsigned,
        store_distribution_state: ChildArtifactMatrixStoreDistributionState::NotApplicable,
        device_owner_state: ChildArtifactMatrixManagementState::NotApplicable,
        managed_profile_state: ChildArtifactMatrixManagementState::NotApplicable,
        supervision_state: ChildArtifactMatrixManagementState::NotApplicable,
        signing_boundary: CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_WINDOWS_SIGNING_BOUNDARY,
        store_boundary: CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_WINDOWS_STORE_BOUNDARY,
        management_boundary: CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_WINDOWS_MANAGEMENT_BOUNDARY,
        claim_boundary: CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_WINDOWS_CLAIM_BOUNDARY,
    })
}

fn macos_row() -> ChildArtifactMatrixRow {
    row(ChildArtifactMatrixRowInput {
        platform: ChildArtifactMatrixPlatform::Macos,
        artifact_kind: ChildArtifactMatrixArtifactKind::MacosLaunchdPkg,
        distribution_mode: ChildArtifactMatrixDistributionMode::DirectPkgDownload,
        artifact_proof_state: ChildArtifactMatrixArtifactProofState::CiMechanicalProof,
        artifact_package_ref: CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_MACOS_ARTIFACT_PACKAGE_REF,
        proof_source: ChildArtifactMatrixProofSource::MacosServicePackageProof,
        proof_refs: &[
            CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_MACOS_PROOF_REF_1,
            CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_MACOS_PROOF_REF_2,
            CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_MACOS_PROOF_REF_3,
        ],
        signing_state: ChildArtifactMatrixSigningState::Unsigned,
        store_distribution_state: ChildArtifactMatrixStoreDistributionState::NotApplicable,
        device_owner_state: ChildArtifactMatrixManagementState::NotApplicable,
        managed_profile_state: ChildArtifactMatrixManagementState::NotApplicable,
        supervision_state: ChildArtifactMatrixManagementState::NotApplicable,
        signing_boundary: CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_MACOS_SIGNING_BOUNDARY,
        store_boundary: CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_MACOS_STORE_BOUNDARY,
        management_boundary: CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_MACOS_MANAGEMENT_BOUNDARY,
        claim_boundary: CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_MACOS_CLAIM_BOUNDARY,
    })
}

fn linux_row() -> ChildArtifactMatrixRow {
    row(ChildArtifactMatrixRowInput {
        platform: ChildArtifactMatrixPlatform::Linux,
        artifact_kind: ChildArtifactMatrixArtifactKind::LinuxSystemdDeb,
        distribution_mode: ChildArtifactMatrixDistributionMode::DirectDebDownload,
        artifact_proof_state: ChildArtifactMatrixArtifactProofState::CiMechanicalProof,
        artifact_package_ref: CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_LINUX_ARTIFACT_PACKAGE_REF,
        proof_source: ChildArtifactMatrixProofSource::LinuxServicePackageProof,
        proof_refs: &[
            CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_LINUX_PROOF_REF_1,
            CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_LINUX_PROOF_REF_2,
            CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_LINUX_PROOF_REF_3,
        ],
        signing_state: ChildArtifactMatrixSigningState::Unsigned,
        store_distribution_state: ChildArtifactMatrixStoreDistributionState::NotApplicable,
        device_owner_state: ChildArtifactMatrixManagementState::NotApplicable,
        managed_profile_state: ChildArtifactMatrixManagementState::NotApplicable,
        supervision_state: ChildArtifactMatrixManagementState::NotApplicable,
        signing_boundary: CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_LINUX_SIGNING_BOUNDARY,
        store_boundary: CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_LINUX_STORE_BOUNDARY,
        management_boundary: CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_LINUX_MANAGEMENT_BOUNDARY,
        claim_boundary: CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_LINUX_CLAIM_BOUNDARY,
    })
}

fn android_row() -> ChildArtifactMatrixRow {
    row(ChildArtifactMatrixRowInput {
        platform: ChildArtifactMatrixPlatform::Android,
        artifact_kind: ChildArtifactMatrixArtifactKind::AndroidDebugApk,
        distribution_mode: ChildArtifactMatrixDistributionMode::DebugApkSideload,
        artifact_proof_state: ChildArtifactMatrixArtifactProofState::CiPackageOnly,
        artifact_package_ref: CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_ANDROID_ARTIFACT_PACKAGE_REF,
        proof_source: ChildArtifactMatrixProofSource::AndroidDeviceProofGate,
        proof_refs: &[
            CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_ANDROID_PROOF_REF_1,
            CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_ANDROID_PROOF_REF_2,
            CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_ANDROID_PROOF_REF_3,
        ],
        signing_state: ChildArtifactMatrixSigningState::DebugSigned,
        store_distribution_state: ChildArtifactMatrixStoreDistributionState::Planned,
        device_owner_state: ChildArtifactMatrixManagementState::ManualRequired,
        managed_profile_state: ChildArtifactMatrixManagementState::ManualRequired,
        supervision_state: ChildArtifactMatrixManagementState::NotApplicable,
        signing_boundary: CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_ANDROID_SIGNING_BOUNDARY,
        store_boundary: CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_ANDROID_STORE_BOUNDARY,
        management_boundary: CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_ANDROID_MANAGEMENT_BOUNDARY,
        claim_boundary: CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_ANDROID_CLAIM_BOUNDARY,
    })
}

fn ios_row() -> ChildArtifactMatrixRow {
    row(ChildArtifactMatrixRowInput {
        platform: ChildArtifactMatrixPlatform::Ios,
        artifact_kind: ChildArtifactMatrixArtifactKind::IosSimulatorAppZip,
        distribution_mode: ChildArtifactMatrixDistributionMode::UnsignedSimulatorZip,
        artifact_proof_state: ChildArtifactMatrixArtifactProofState::SimulatorScaffold,
        artifact_package_ref: CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_IOS_ARTIFACT_PACKAGE_REF,
        proof_source: ChildArtifactMatrixProofSource::IosEntitlementProof,
        proof_refs: &[
            CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_IOS_PROOF_REF_1,
            CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_IOS_PROOF_REF_2,
            CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_IOS_PROOF_REF_3,
        ],
        signing_state: ChildArtifactMatrixSigningState::SigningDisabled,
        store_distribution_state: ChildArtifactMatrixStoreDistributionState::Planned,
        device_owner_state: ChildArtifactMatrixManagementState::NotApplicable,
        managed_profile_state: ChildArtifactMatrixManagementState::NotApplicable,
        supervision_state: ChildArtifactMatrixManagementState::DeviceProofRequired,
        signing_boundary: CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_IOS_SIGNING_BOUNDARY,
        store_boundary: CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_IOS_STORE_BOUNDARY,
        management_boundary: CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_IOS_MANAGEMENT_BOUNDARY,
        claim_boundary: CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_IOS_CLAIM_BOUNDARY,
    })
}

fn sample_child_signing_store_device_owner_rows() -> Vec<ChildArtifactMatrixRow> {
    vec![
        windows_row(),
        macos_row(),
        linux_row(),
        android_row(),
        ios_row(),
    ]
}

pub fn sample_child_signing_store_device_owner_matrix_proof(
) -> ChildSigningStoreDeviceOwnerMatrixProof {
    ChildSigningStoreDeviceOwnerMatrixProof {
        schema_version: CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_SCHEMA_VERSION.to_string(),
        checked_at: timestamp(CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_CHECKED_AT),
        rows: sample_child_signing_store_device_owner_rows(),
        claim_boundaries: ChildArtifactMatrixClaimBoundaries {
            generic_matrix: boundary(
                CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_GENERIC_MATRIX_CLAIM_BOUNDARY,
            ),
            signing_parity: boundary(
                CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_SIGNING_PARITY_CLAIM_BOUNDARY,
            ),
            store_parity: boundary(
                CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_STORE_PARITY_CLAIM_BOUNDARY,
            ),
            management_parity: boundary(
                CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_MANAGEMENT_PARITY_CLAIM_BOUNDARY,
            ),
            parent_parity: boundary(
                CHILD_SIGNING_STORE_DEVICE_OWNER_MATRIX_PARENT_PARITY_CLAIM_BOUNDARY,
            ),
        },
    }
}
