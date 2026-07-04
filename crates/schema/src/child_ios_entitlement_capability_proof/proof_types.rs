use serde::{Deserialize, Serialize};

use super::{
    ChildIosEntitlementBoundary, ChildIosEntitlementBridgeState, ChildIosEntitlementBundleId,
    ChildIosEntitlementClassName, ChildIosEntitlementDeclarationState,
    ChildIosEntitlementPackagePhase, ChildIosEntitlementParentCapability,
    ChildIosEntitlementParentCapabilityStatus, ChildIosEntitlementProofState,
    ChildIosEntitlementProtocolCommand, ChildIosEntitlementProtocolEvent,
    ChildIosEntitlementRequirement, ChildIosEntitlementRuntimeOwner,
    ChildIosEntitlementSurfaceName, ChildIosEntitlementTimestamp,
};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChildIosEntitlementSurfaceProof {
    pub surface: ChildIosEntitlementSurfaceName,
    pub parent_capability: ChildIosEntitlementParentCapability,
    pub parent_capability_status: ChildIosEntitlementParentCapabilityStatus,
    pub declaration_state: ChildIosEntitlementDeclarationState,
    pub proof_state: ChildIosEntitlementProofState,
    pub runtime_owner: ChildIosEntitlementRuntimeOwner,
    pub proof_requirement: ChildIosEntitlementRequirement,
    pub claim_boundary: ChildIosEntitlementBoundary,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChildIosEntitlementPackageLifecycleProof {
    pub phase: ChildIosEntitlementPackagePhase,
    pub proof_state: ChildIosEntitlementProofState,
    pub runtime_owner: ChildIosEntitlementRuntimeOwner,
    pub proof_requirement: ChildIosEntitlementRequirement,
    pub claim_boundary: ChildIosEntitlementBoundary,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChildIosEntitlementProtocolBridgeProof {
    pub bundle_id: ChildIosEntitlementBundleId,
    pub status_surface_class: ChildIosEntitlementClassName,
    pub bridge_state: ChildIosEntitlementBridgeState,
    pub external_transport_state: ChildIosEntitlementBridgeState,
    pub commands: Vec<ChildIosEntitlementProtocolCommand>,
    pub events: Vec<ChildIosEntitlementProtocolEvent>,
    pub runtime_owner: ChildIosEntitlementRuntimeOwner,
    pub proof_requirement: ChildIosEntitlementRequirement,
    pub claim_boundary: ChildIosEntitlementBoundary,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChildIosEntitlementClaimBoundaries {
    pub simulator_package: ChildIosEntitlementBoundary,
    pub launch_availability: ChildIosEntitlementBoundary,
    pub family_controls: ChildIosEntitlementBoundary,
    pub device_activity: ChildIosEntitlementBoundary,
    pub screen_time: ChildIosEntitlementBoundary,
    pub network_extension: ChildIosEntitlementBoundary,
    pub notifications: ChildIosEntitlementBoundary,
    pub background_execution: ChildIosEntitlementBoundary,
    pub recovery_behavior: ChildIosEntitlementBoundary,
    pub provisioning_profile: ChildIosEntitlementBoundary,
    pub supervision: ChildIosEntitlementBoundary,
    pub signing_entitlements: ChildIosEntitlementBoundary,
    pub testflight: ChildIosEntitlementBoundary,
    pub device_proof: ChildIosEntitlementBoundary,
    pub capability_only_state: ChildIosEntitlementBoundary,
    pub external_transport: ChildIosEntitlementBoundary,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChildIosEntitlementCapabilityReadModel {
    pub schema_version: String,
    pub bundle_id: ChildIosEntitlementBundleId,
    pub status_surface_class: ChildIosEntitlementClassName,
    pub protocol_bridge_proof: ChildIosEntitlementProtocolBridgeProof,
    pub surface_proofs: Vec<ChildIosEntitlementSurfaceProof>,
    pub package_lifecycle_proofs: Vec<ChildIosEntitlementPackageLifecycleProof>,
    pub claim_boundaries: ChildIosEntitlementClaimBoundaries,
    pub updated_at: ChildIosEntitlementTimestamp,
}
