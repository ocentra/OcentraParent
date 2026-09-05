use crate::test_text::{test_some as some, TestResult, TestText};
use ocentra_parent_agent_core::enforcement_readiness::broad_os_adapter_readiness;
use ocentra_parent_agent_protocol::constants::enforcement;
use ocentra_parent_agent_protocol::enforcement::EnforcementCapabilityState;
use ocentra_parent_agent_protocol::enforcement_readiness::{
    EnforcementBroadAdapterCapability, EnforcementBroadAdapterReadinessEntry,
    EnforcementBroadOsAdapterReadinessMatrix, EnforcementReadinessProofLevel,
    EnforcementReadinessRuntimeOwner, EnforcementReadinessState,
};
use ocentra_parent_agent_protocol::policy_constants as policy;

#[test]
fn broad_os_adapter_readiness_keeps_unproved_claims_manual_or_not_claimed() -> TestResult {
    let matrix = broad_os_adapter_readiness(policy::TEST_EVALUATED_AT);

    assert_eq!(
        matrix.matrix_id,
        enforcement::READINESS_MATRIX_ID_V0_8_BROAD_OS_ADAPTER
    );
    assert_eq!(matrix.entries.len(), 9);
    assert_eq!(
        entry(&matrix, EnforcementBroadAdapterCapability::BroadAppBlocking)?.readiness_state,
        expected_manual_or_unavailable()
    );
    assert_eq!(
        entry(
            &matrix,
            EnforcementBroadAdapterCapability::NetworkDomainBlocking
        )?
        .runtime_owner,
        EnforcementReadinessRuntimeOwner::ManualProof
    );
    assert_eq!(
        entry(
            &matrix,
            EnforcementBroadAdapterCapability::ManagedBrowserServiceCommand
        )?
        .claim_boundary,
        enforcement::CLAIM_BOUNDARY_MANAGED_BROWSER_SERVICE_COMMAND
    );
    assert_eq!(
        entry(
            &matrix,
            EnforcementBroadAdapterCapability::UnmanagedBrowserExactEvidence
        )?
        .readiness_state,
        EnforcementReadinessState::NotClaimed
    );
    assert_eq!(
        entry(
            &matrix,
            EnforcementBroadAdapterCapability::UnmanagedBrowserExactEvidence
        )?
        .proof_level,
        EnforcementReadinessProofLevel::NotProved
    );

    Ok(())
}

#[test]
fn readiness_entries_stay_limited_to_supported_or_manual_capabilities() -> TestResult {
    let matrix = broad_os_adapter_readiness(policy::TEST_EVALUATED_AT);

    for capability in [
        EnforcementBroadAdapterCapability::OwnedProcessTerminate,
        EnforcementBroadAdapterCapability::AppTimeLimit,
        EnforcementBroadAdapterCapability::UnmanagedBrowserProcessOnly,
    ] {
        let entry = entry(&matrix, capability)?;
        if capability == EnforcementBroadAdapterCapability::AppTimeLimit {
            assert_eq!(
                entry.capability_state,
                EnforcementCapabilityState::ManualRequired
            );
            assert_eq!(
                entry.readiness_state,
                EnforcementReadinessState::ManualRequired
            );
            assert_eq!(
                entry.proof_level,
                EnforcementReadinessProofLevel::ManualProofRequired
            );
            assert_eq!(
                entry.required_artifacts,
                vec![enforcement::ARTIFACT_APP_TIME_LIMIT_EXECUTOR.to_string()]
            );
            continue;
        }
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
            assert_eq!(
                entry.required_artifacts,
                vec![enforcement::UNAVAILABLE_UNSUPPORTED_PLATFORM.to_string()]
            );
        }
    }

    Ok(())
}

fn entry(
    matrix: &EnforcementBroadOsAdapterReadinessMatrix,
    capability: EnforcementBroadAdapterCapability,
) -> Result<&EnforcementBroadAdapterReadinessEntry, TestText> {
    some(
        matrix
            .entries
            .iter()
            .find(|entry| entry.capability == capability),
        enforcement::READINESS_MATRIX_ID_V0_8_BROAD_OS_ADAPTER,
    )
}

#[cfg(windows)]
fn expected_manual_or_unavailable() -> EnforcementReadinessState {
    EnforcementReadinessState::ManualRequired
}

#[cfg(not(windows))]
fn expected_manual_or_unavailable() -> EnforcementReadinessState {
    EnforcementReadinessState::Unavailable
}
