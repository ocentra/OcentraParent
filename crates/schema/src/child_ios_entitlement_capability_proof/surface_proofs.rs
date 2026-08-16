use super::identifiers::{boundary, requirement};
use super::*;

fn surface_proof(
    surface: ChildIosEntitlementSurfaceName,
    parent_capability: ChildIosEntitlementParentCapability,
    parent_capability_status: ChildIosEntitlementParentCapabilityStatus,
    declaration_state: ChildIosEntitlementDeclarationState,
    proof_state: ChildIosEntitlementProofState,
    runtime_owner: ChildIosEntitlementRuntimeOwner,
) -> ChildIosEntitlementSurfaceProof {
    let proof_requirement = requirement(&format!(
        "{} remains {} until Apple artifacts change it",
        surface.as_str(),
        proof_state.as_str()
    ));
    let claim_boundary = boundary(proof_requirement.as_str());

    ChildIosEntitlementSurfaceProof {
        surface,
        parent_capability,
        parent_capability_status,
        declaration_state,
        proof_state,
        runtime_owner,
        proof_requirement,
        claim_boundary,
    }
}

pub(super) fn sample_surface_proofs() -> Vec<ChildIosEntitlementSurfaceProof> {
    let mut proofs = sample_surface_proofs_identity_and_declaration();
    proofs.extend(sample_surface_proofs_runtime_requirements());
    proofs.extend(sample_surface_proofs_distribution_and_proof());
    proofs
}

fn sample_surface_proofs_identity_and_declaration() -> Vec<ChildIosEntitlementSurfaceProof> {
    vec![
        surface_proof(
            ChildIosEntitlementSurfaceName::SimulatorAppTarget,
            ChildIosEntitlementParentCapability::PackageLifecycle,
            ChildIosEntitlementParentCapabilityStatus::ManualRequired,
            ChildIosEntitlementDeclarationState::DeclaredInProject,
            ChildIosEntitlementProofState::CiMechanicalProof,
            ChildIosEntitlementRuntimeOwner::IosXcodeProject,
        ),
        surface_proof(
            ChildIosEntitlementSurfaceName::BundleIdentifier,
            ChildIosEntitlementParentCapability::PackageLifecycle,
            ChildIosEntitlementParentCapabilityStatus::ManualRequired,
            ChildIosEntitlementDeclarationState::DeclaredInProject,
            ChildIosEntitlementProofState::CiMechanicalProof,
            ChildIosEntitlementRuntimeOwner::IosXcodeProject,
        ),
        surface_proof(
            ChildIosEntitlementSurfaceName::StatusSurface,
            ChildIosEntitlementParentCapability::TypedProtocolBridge,
            ChildIosEntitlementParentCapabilityStatus::Scaffold,
            ChildIosEntitlementDeclarationState::ScaffoldStatusLabel,
            ChildIosEntitlementProofState::SimulatorScaffold,
            ChildIosEntitlementRuntimeOwner::IosSwiftScaffold,
        ),
        surface_proof(
            ChildIosEntitlementSurfaceName::FamilyControlsEntitlement,
            ChildIosEntitlementParentCapability::FamilyControlsEntitlement,
            ChildIosEntitlementParentCapabilityStatus::ManualRequired,
            ChildIosEntitlementDeclarationState::NotDeclared,
            ChildIosEntitlementProofState::EntitlementRequired,
            ChildIosEntitlementRuntimeOwner::AppleEntitlement,
        ),
    ]
}

fn sample_surface_proofs_runtime_requirements() -> Vec<ChildIosEntitlementSurfaceProof> {
    vec![
        surface_proof(
            ChildIosEntitlementSurfaceName::DeviceActivityFramework,
            ChildIosEntitlementParentCapability::DeviceActivity,
            ChildIosEntitlementParentCapabilityStatus::ManualRequired,
            ChildIosEntitlementDeclarationState::NotDeclared,
            ChildIosEntitlementProofState::EntitlementRequired,
            ChildIosEntitlementRuntimeOwner::AppleDeviceFramework,
        ),
        surface_proof(
            ChildIosEntitlementSurfaceName::ScreenTimeApi,
            ChildIosEntitlementParentCapability::ScreenTimeApi,
            ChildIosEntitlementParentCapabilityStatus::ManualRequired,
            ChildIosEntitlementDeclarationState::NotDeclared,
            ChildIosEntitlementProofState::EntitlementRequired,
            ChildIosEntitlementRuntimeOwner::AppleDeviceFramework,
        ),
        surface_proof(
            ChildIosEntitlementSurfaceName::NetworkExtension,
            ChildIosEntitlementParentCapability::NetworkExtension,
            ChildIosEntitlementParentCapabilityStatus::ManualRequired,
            ChildIosEntitlementDeclarationState::NotDeclared,
            ChildIosEntitlementProofState::EntitlementRequired,
            ChildIosEntitlementRuntimeOwner::AppleNetworkExtension,
        ),
        surface_proof(
            ChildIosEntitlementSurfaceName::NotificationsPermission,
            ChildIosEntitlementParentCapability::Notifications,
            ChildIosEntitlementParentCapabilityStatus::ManualRequired,
            ChildIosEntitlementDeclarationState::NotDeclared,
            ChildIosEntitlementProofState::ManualRequired,
            ChildIosEntitlementRuntimeOwner::AppleNotificationPermission,
        ),
    ]
}

fn sample_surface_proofs_distribution_and_proof() -> Vec<ChildIosEntitlementSurfaceProof> {
    vec![
        surface_proof(
            ChildIosEntitlementSurfaceName::BackgroundExecution,
            ChildIosEntitlementParentCapability::BackgroundExecution,
            ChildIosEntitlementParentCapabilityStatus::ManualRequired,
            ChildIosEntitlementDeclarationState::NotDeclared,
            ChildIosEntitlementProofState::ManualRequired,
            ChildIosEntitlementRuntimeOwner::AppleBackgroundMode,
        ),
        surface_proof(
            ChildIosEntitlementSurfaceName::ProvisioningProfile,
            ChildIosEntitlementParentCapability::SigningEntitlements,
            ChildIosEntitlementParentCapabilityStatus::ManualRequired,
            ChildIosEntitlementDeclarationState::NotApplicable,
            ChildIosEntitlementProofState::SigningRequired,
            ChildIosEntitlementRuntimeOwner::AppleSigning,
        ),
        surface_proof(
            ChildIosEntitlementSurfaceName::SupervisionState,
            ChildIosEntitlementParentCapability::PackageLifecycle,
            ChildIosEntitlementParentCapabilityStatus::ManualRequired,
            ChildIosEntitlementDeclarationState::NotApplicable,
            ChildIosEntitlementProofState::DeviceProofRequired,
            ChildIosEntitlementRuntimeOwner::AppleDeviceProof,
        ),
        surface_proof(
            ChildIosEntitlementSurfaceName::SigningEntitlements,
            ChildIosEntitlementParentCapability::SigningEntitlements,
            ChildIosEntitlementParentCapabilityStatus::ManualRequired,
            ChildIosEntitlementDeclarationState::NotApplicable,
            ChildIosEntitlementProofState::SigningRequired,
            ChildIosEntitlementRuntimeOwner::AppleSigning,
        ),
        surface_proof(
            ChildIosEntitlementSurfaceName::TestflightDistribution,
            ChildIosEntitlementParentCapability::TestflightDistribution,
            ChildIosEntitlementParentCapabilityStatus::ManualRequired,
            ChildIosEntitlementDeclarationState::NotApplicable,
            ChildIosEntitlementProofState::DeviceProofRequired,
            ChildIosEntitlementRuntimeOwner::AppleTestflight,
        ),
        surface_proof(
            ChildIosEntitlementSurfaceName::PhysicalDeviceProof,
            ChildIosEntitlementParentCapability::PackageLifecycle,
            ChildIosEntitlementParentCapabilityStatus::ManualRequired,
            ChildIosEntitlementDeclarationState::NotApplicable,
            ChildIosEntitlementProofState::DeviceProofRequired,
            ChildIosEntitlementRuntimeOwner::AppleDeviceProof,
        ),
        surface_proof(
            ChildIosEntitlementSurfaceName::AppStoreDistribution,
            ChildIosEntitlementParentCapability::StoreDistribution,
            ChildIosEntitlementParentCapabilityStatus::Planned,
            ChildIosEntitlementDeclarationState::NotApplicable,
            ChildIosEntitlementProofState::Planned,
            ChildIosEntitlementRuntimeOwner::AppStoreConnect,
        ),
    ]
}
