use super::*;

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

#[derive(Clone, Copy)]
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

fn row(input: &ChildArtifactMatrixRowInput<'_>) -> ChildArtifactMatrixRow {
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
    } = *input;

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
    row(&ChildArtifactMatrixRowInput {
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
    row(&ChildArtifactMatrixRowInput {
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
    row(&ChildArtifactMatrixRowInput {
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
    row(&ChildArtifactMatrixRowInput {
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
    row(&ChildArtifactMatrixRowInput {
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

pub(super) fn sample_child_signing_store_device_owner_matrix_proof(
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
