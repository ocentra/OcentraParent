use super::identifiers::{boundary, requirement};
use super::*;

fn lifecycle_proof(
    phase: ChildIosEntitlementPackagePhase,
    proof_state: ChildIosEntitlementProofState,
    runtime_owner: ChildIosEntitlementRuntimeOwner,
) -> ChildIosEntitlementPackageLifecycleProof {
    let proof_requirement = requirement(&format!(
        "{} proof state is {}",
        phase.as_str(),
        proof_state.as_str()
    ));
    let claim_boundary = boundary(&format!(
        "{} does not upgrade iOS child capability without entitlement signing or device evidence",
        phase.as_str()
    ));

    ChildIosEntitlementPackageLifecycleProof {
        phase,
        proof_state,
        runtime_owner,
        proof_requirement,
        claim_boundary,
    }
}

pub(super) fn sample_package_lifecycle_proofs() -> Vec<ChildIosEntitlementPackageLifecycleProof> {
    let mut proofs = sample_package_lifecycle_proofs_project_and_build();
    proofs.extend(sample_package_lifecycle_proofs_runtime_and_install());
    proofs.extend(sample_package_lifecycle_proofs_signing_and_recovery());
    proofs
}

fn sample_package_lifecycle_proofs_project_and_build(
) -> Vec<ChildIosEntitlementPackageLifecycleProof> {
    vec![
        lifecycle_proof(
            ChildIosEntitlementPackagePhase::XcodeProjectTarget,
            ChildIosEntitlementProofState::CiMechanicalProof,
            ChildIosEntitlementRuntimeOwner::IosXcodeProject,
        ),
        lifecycle_proof(
            ChildIosEntitlementPackagePhase::BundleIdentifier,
            ChildIosEntitlementProofState::CiMechanicalProof,
            ChildIosEntitlementRuntimeOwner::IosXcodeProject,
        ),
        lifecycle_proof(
            ChildIosEntitlementPackagePhase::SimulatorBuildScript,
            ChildIosEntitlementProofState::CiMechanicalProof,
            ChildIosEntitlementRuntimeOwner::IosSimulatorBuildScript,
        ),
        lifecycle_proof(
            ChildIosEntitlementPackagePhase::StatusView,
            ChildIosEntitlementProofState::SimulatorScaffold,
            ChildIosEntitlementRuntimeOwner::IosSwiftScaffold,
        ),
    ]
}

fn sample_package_lifecycle_proofs_runtime_and_install(
) -> Vec<ChildIosEntitlementPackageLifecycleProof> {
    vec![
        lifecycle_proof(
            ChildIosEntitlementPackagePhase::InfoPlist,
            ChildIosEntitlementProofState::CiMechanicalProof,
            ChildIosEntitlementRuntimeOwner::IosInfoPlist,
        ),
        lifecycle_proof(
            ChildIosEntitlementPackagePhase::SimulatorBuild,
            ChildIosEntitlementProofState::ManualRequired,
            ChildIosEntitlementRuntimeOwner::IosSimulatorBuildScript,
        ),
        lifecycle_proof(
            ChildIosEntitlementPackagePhase::SimulatorLaunch,
            ChildIosEntitlementProofState::ManualRequired,
            ChildIosEntitlementRuntimeOwner::AppleSimulatorHost,
        ),
        lifecycle_proof(
            ChildIosEntitlementPackagePhase::DeviceInstall,
            ChildIosEntitlementProofState::DeviceProofRequired,
            ChildIosEntitlementRuntimeOwner::AppleDeviceProof,
        ),
        lifecycle_proof(
            ChildIosEntitlementPackagePhase::DeviceLaunch,
            ChildIosEntitlementProofState::DeviceProofRequired,
            ChildIosEntitlementRuntimeOwner::AppleDeviceProof,
        ),
    ]
}

fn sample_package_lifecycle_proofs_signing_and_recovery(
) -> Vec<ChildIosEntitlementPackageLifecycleProof> {
    vec![
        lifecycle_proof(
            ChildIosEntitlementPackagePhase::TestflightInstall,
            ChildIosEntitlementProofState::DeviceProofRequired,
            ChildIosEntitlementRuntimeOwner::AppleTestflight,
        ),
        lifecycle_proof(
            ChildIosEntitlementPackagePhase::SigningProfile,
            ChildIosEntitlementProofState::SigningRequired,
            ChildIosEntitlementRuntimeOwner::AppleSigning,
        ),
        lifecycle_proof(
            ChildIosEntitlementPackagePhase::EntitlementReview,
            ChildIosEntitlementProofState::EntitlementRequired,
            ChildIosEntitlementRuntimeOwner::AppleEntitlement,
        ),
        lifecycle_proof(
            ChildIosEntitlementPackagePhase::RecoveryBehavior,
            ChildIosEntitlementProofState::NotImplemented,
            ChildIosEntitlementRuntimeOwner::AppleBackgroundMode,
        ),
    ]
}
