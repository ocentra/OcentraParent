use ocentra_eventing::expect_value::ExpectValue;
use std::collections::BTreeMap;

use crate::{
    constants::{self, v08_cross_platform_enforcement_capability_proof as proof},
    ParentPlatform, V08CrossPlatformAdapterExecutionState, V08CrossPlatformCapabilityStatus,
    V08CrossPlatformEnforcementCapabilityClaimState, V08CrossPlatformEnforcementCapabilityName,
    V08CrossPlatformEnforcementCapabilityProofEntry,
    V08CrossPlatformEnforcementCapabilityProofReadModel,
    V08CrossPlatformEnforcementCapabilitySurface,
};

#[test]
fn cross_platform_capability_surfaces_have_stable_protocol_strings() {
    let surfaces = [
        V08CrossPlatformEnforcementCapabilitySurface::WindowsOwnedProcessTerminate,
        V08CrossPlatformEnforcementCapabilitySurface::WindowsAppTimeLimitLifecycle,
        V08CrossPlatformEnforcementCapabilitySurface::WindowsManagedBrowserBoundary,
        V08CrossPlatformEnforcementCapabilitySurface::WindowsUnmanagedBrowserProcessBoundary,
        V08CrossPlatformEnforcementCapabilitySurface::WindowsBroadInstalledAppBlocking,
        V08CrossPlatformEnforcementCapabilitySurface::WindowsNetworkDomainBlocking,
        V08CrossPlatformEnforcementCapabilitySurface::LinuxEnforcementAdapterScaffold,
        V08CrossPlatformEnforcementCapabilitySurface::MacosEnforcementAdapterScaffold,
        V08CrossPlatformEnforcementCapabilitySurface::AndroidDeviceOwnerPolicy,
        V08CrossPlatformEnforcementCapabilitySurface::AndroidPackageLifecycle,
        V08CrossPlatformEnforcementCapabilitySurface::AndroidStoreDistribution,
        V08CrossPlatformEnforcementCapabilitySurface::IosFamilyControls,
        V08CrossPlatformEnforcementCapabilitySurface::IosSigningEntitlements,
        V08CrossPlatformEnforcementCapabilitySurface::IosTestflightDistribution,
        V08CrossPlatformEnforcementCapabilitySurface::IosStoreDistribution,
    ];
    let serialized =
        serde_json::to_value(surfaces).expect_value(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(
        serialized
            .as_array()
            .expect_value(constants::error::AGENT_EVENT_SERIALIZES)
            .len(),
        15
    );
    assert_eq!(
        surfaces[0].as_protocol_str(),
        proof::SURFACE_WINDOWS_OWNED_PROCESS
    );
    assert_eq!(surfaces[14].as_protocol_str(), proof::SURFACE_IOS_STORE);
}

#[test]
fn cross_platform_capability_states_serialize_as_contract_values() {
    assert_eq!(
        V08CrossPlatformEnforcementCapabilityName::DeviceOwnerPolicy.as_protocol_str(),
        proof::CAPABILITY_DEVICE_OWNER_POLICY
    );
    assert_eq!(
        V08CrossPlatformCapabilityStatus::ManualRequired.as_protocol_str(),
        proof::STATUS_MANUAL_REQUIRED
    );
    assert_eq!(
        V08CrossPlatformCapabilityStatus::Unavailable.as_protocol_str(),
        proof::STATUS_UNAVAILABLE
    );
    assert_eq!(
        V08CrossPlatformEnforcementCapabilityClaimState::ImplementedBoundary.as_protocol_str(),
        proof::CLAIM_IMPLEMENTED_BOUNDARY
    );
    assert_eq!(
        V08CrossPlatformEnforcementCapabilityClaimState::NotClaimed.as_protocol_str(),
        proof::CLAIM_NOT_CLAIMED
    );
    assert_eq!(
        V08CrossPlatformAdapterExecutionState::ScaffoldOnly.as_protocol_str(),
        proof::SCAFFOLD_ONLY
    );
    assert_eq!(
        V08CrossPlatformAdapterExecutionState::ReturnsUnavailable.as_protocol_str(),
        proof::RETURNS_UNAVAILABLE
    );
}

#[test]
fn cross_platform_read_model_serializes_claim_boundaries_for_service_preview() {
    let entry =
        |proof_entry_id: &'static str,
         surface,
         platform,
         capability,
         product_claim_state,
         adapter_execution_state| V08CrossPlatformEnforcementCapabilityProofEntry {
            schema_version: crate::policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
            proof_entry_id: proof_entry_id.to_string(),
            surface,
            platform,
            capability,
            capability_status: V08CrossPlatformCapabilityStatus::ManualRequired,
            product_claim_state,
            adapter_execution_state,
            linked_proof_commands: Vec::new(),
            linked_proof_artifacts: Vec::new(),
            manual_proof_requirements: vec![proof::REQUIREMENT_ROLLBACK.to_string()],
            claim_boundary: proof::CLAIM_WINDOWS_BROAD_APP.to_string(),
            fallback_behavior: proof::FALLBACK_WINDOWS_BROAD_APP.to_string(),
            broad_blocking_claimed: false,
            exact_url_claimed: false,
            privileged_mobile_claimed: false,
            production_distribution_claimed: false,
            last_checked_at: crate::policy_constants::TEST_EVALUATED_AT.to_string(),
        };
    let read_model = V08CrossPlatformEnforcementCapabilityProofReadModel {
        schema_version: crate::policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        read_model_id: proof::READ_MODEL_ID.to_string(),
        generated_at: crate::policy_constants::TEST_EVALUATED_AT.to_string(),
        source_read_model_ids: vec![proof::SOURCE_BROAD_PROOF.to_string()],
        entries: vec![
            entry(
                proof::ENTRY_ID_WINDOWS_OWNED_PROCESS,
                V08CrossPlatformEnforcementCapabilitySurface::WindowsOwnedProcessTerminate,
                ParentPlatform::Windows,
                V08CrossPlatformEnforcementCapabilityName::OwnedProcessTerminate,
                V08CrossPlatformEnforcementCapabilityClaimState::ImplementedBoundary,
                V08CrossPlatformAdapterExecutionState::ExecutesRealService,
            ),
            entry(
                proof::ENTRY_ID_ANDROID_DEVICE_OWNER,
                V08CrossPlatformEnforcementCapabilitySurface::AndroidDeviceOwnerPolicy,
                ParentPlatform::Android,
                V08CrossPlatformEnforcementCapabilityName::DeviceOwnerPolicy,
                V08CrossPlatformEnforcementCapabilityClaimState::ManualRequired,
                V08CrossPlatformAdapterExecutionState::ReturnsManualRequired,
            ),
        ],
    };
    let serialized =
        serde_json::to_value(read_model).expect_value(constants::error::AGENT_EVENT_SERIALIZES);
    let reparsed =
        serde_json::from_value::<V08CrossPlatformEnforcementCapabilityProofReadModel>(serialized)
            .expect_value(constants::error::AGENT_EVENT_SERIALIZES);
    let claim_counts: BTreeMap<&'static str, usize> =
        reparsed
            .entries
            .iter()
            .fold(BTreeMap::new(), |mut counts, entry| {
                *counts
                    .entry(entry.product_claim_state.as_protocol_str())
                    .or_default() += 1;
                counts
            });

    assert_eq!(reparsed.read_model_id, proof::READ_MODEL_ID);
    assert_eq!(claim_counts[proof::CLAIM_IMPLEMENTED_BOUNDARY], 1);
    assert_eq!(claim_counts[proof::CLAIM_MANUAL_REQUIRED], 1);
    assert!(reparsed
        .entries
        .iter()
        .all(|entry| !entry.broad_blocking_claimed));
    assert!(reparsed
        .entries
        .iter()
        .all(|entry| !entry.privileged_mobile_claimed));
}
