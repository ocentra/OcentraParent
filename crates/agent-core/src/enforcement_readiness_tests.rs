use crate::enforcement_readiness::broad_os_adapter_readiness;
use ocentra_parent_agent_protocol::{
    constants::enforcement, policy_constants as policy, EnforcementBroadAdapterCapability,
    EnforcementCapabilityState, EnforcementReadinessProofLevel, EnforcementReadinessRuntimeOwner,
    EnforcementReadinessState,
};

#[test]
fn broad_os_adapter_readiness_keeps_unproved_claims_manual_or_not_claimed() {
    let matrix = broad_os_adapter_readiness(policy::TEST_EVALUATED_AT);

    assert_eq!(
        matrix.matrix_id,
        enforcement::READINESS_MATRIX_ID_V0_8_BROAD_OS_ADAPTER
    );
    assert_eq!(matrix.entries.len(), 9);
    assert_eq!(
        entry(&matrix, EnforcementBroadAdapterCapability::BroadAppBlocking).readiness_state,
        expected_manual_or_unavailable()
    );
    assert_eq!(
        entry(
            &matrix,
            EnforcementBroadAdapterCapability::NetworkDomainBlocking
        )
        .runtime_owner,
        EnforcementReadinessRuntimeOwner::ManualProof
    );
    assert_eq!(
        entry(
            &matrix,
            EnforcementBroadAdapterCapability::ManagedBrowserServiceCommand
        )
        .claim_boundary,
        enforcement::CLAIM_BOUNDARY_MANAGED_BROWSER_SERVICE_COMMAND
    );
    assert_eq!(
        entry(
            &matrix,
            EnforcementBroadAdapterCapability::UnmanagedBrowserExactEvidence
        )
        .readiness_state,
        EnforcementReadinessState::NotClaimed
    );
    assert_eq!(
        entry(
            &matrix,
            EnforcementBroadAdapterCapability::UnmanagedBrowserExactEvidence
        )
        .proof_level,
        EnforcementReadinessProofLevel::NotProved
    );
}

#[test]
fn implemented_readiness_entries_stay_limited_to_supported_capabilities() {
    let matrix = broad_os_adapter_readiness(policy::TEST_EVALUATED_AT);

    for capability in [
        EnforcementBroadAdapterCapability::OwnedProcessTerminate,
        EnforcementBroadAdapterCapability::AppTimeLimit,
        EnforcementBroadAdapterCapability::UnmanagedBrowserProcessOnly,
    ] {
        let entry = entry(&matrix, capability);
        #[cfg(windows)]
        {
            assert_eq!(
                entry.capability_state,
                EnforcementCapabilityState::Supported
            );
            assert_eq!(
                entry.readiness_state,
                EnforcementReadinessState::Implemented
            );
            assert_eq!(
                entry.proof_level,
                EnforcementReadinessProofLevel::RealServiceProof
            );
        }
        #[cfg(not(windows))]
        {
            assert_eq!(
                entry.capability_state,
                EnforcementCapabilityState::Unavailable
            );
            assert_eq!(
                entry.readiness_state,
                EnforcementReadinessState::Unavailable
            );
            assert!(!entry.required_artifacts.is_empty());
        }
    }
}

fn entry(
    matrix: &ocentra_parent_agent_protocol::EnforcementBroadOsAdapterReadinessMatrix,
    capability: EnforcementBroadAdapterCapability,
) -> &ocentra_parent_agent_protocol::EnforcementBroadAdapterReadinessEntry {
    matrix
        .entries
        .iter()
        .find(|entry| entry.capability == capability)
        .expect(enforcement::READINESS_MATRIX_ID_V0_8_BROAD_OS_ADAPTER)
}

#[cfg(windows)]
fn expected_manual_or_unavailable() -> EnforcementReadinessState {
    EnforcementReadinessState::ManualRequired
}

#[cfg(not(windows))]
fn expected_manual_or_unavailable() -> EnforcementReadinessState {
    EnforcementReadinessState::Unavailable
}
