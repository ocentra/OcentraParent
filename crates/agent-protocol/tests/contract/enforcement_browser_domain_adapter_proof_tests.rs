use ocentra_eventing::expect_value::ExpectValue;
use std::collections::BTreeMap;

use crate::{
    constants::{self, v08_browser_domain_adapter_proof as proof},
    ParentPlatform, V08BrowserDomainAdapterExecutionState,
    V08BrowserDomainAdapterProofCapabilityName, V08BrowserDomainAdapterProofCapabilityStatus,
    V08BrowserDomainAdapterProofClaimState, V08BrowserDomainAdapterProofEntry,
    V08BrowserDomainAdapterProofEvidenceKind, V08BrowserDomainAdapterProofReadModel,
    V08BrowserDomainAdapterProofSurface, V08WindowsAppControlAdminRequirement,
    V08WindowsAppControlEventState, V08WindowsAppControlPolicyMutationState,
    V08WindowsAppControlProofState, V08WindowsAppControlReadinessState,
    V08WindowsAppControlRuleIdentityKind,
};

#[test]
fn browser_domain_surfaces_have_stable_protocol_strings() {
    let surfaces = [
        V08BrowserDomainAdapterProofSurface::WindowsManagedBrowserInterventionState,
        V08BrowserDomainAdapterProofSurface::WindowsManagedBrowserExactUrlManual,
        V08BrowserDomainAdapterProofSurface::WindowsUnmanagedBrowserTerminateBoundary,
        V08BrowserDomainAdapterProofSurface::WindowsUnmanagedBrowserWarnNoop,
        V08BrowserDomainAdapterProofSurface::WindowsUnmanagedBrowserExactEvidenceNotClaimed,
        V08BrowserDomainAdapterProofSurface::WindowsNetworkDomainFilterManual,
        V08BrowserDomainAdapterProofSurface::WindowsNetworkDomainAdapterUnavailable,
        V08BrowserDomainAdapterProofSurface::WindowsAuditVisibilityBoundary,
        V08BrowserDomainAdapterProofSurface::WindowsRestartRecoveryVisibilityBoundary,
        V08BrowserDomainAdapterProofSurface::WindowsBrowserPolicyRollbackVisibility,
        V08BrowserDomainAdapterProofSurface::LinuxBrowserDomainAdapterUnavailable,
        V08BrowserDomainAdapterProofSurface::MacosBrowserDomainAdapterUnavailable,
        V08BrowserDomainAdapterProofSurface::AndroidBrowserDomainAdapterManual,
        V08BrowserDomainAdapterProofSurface::IosBrowserDomainAdapterManual,
    ];
    let serialized =
        serde_json::to_value(surfaces).expect_value(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(
        serialized
            .as_array()
            .expect_value(constants::error::AGENT_EVENT_SERIALIZES)
            .len(),
        14
    );
    assert_eq!(
        surfaces[0].as_protocol_str(),
        proof::SURFACE_MANAGED_INTERVENTION
    );
    assert_eq!(surfaces[13].as_protocol_str(), proof::SURFACE_IOS_ADAPTER);
}

#[test]
fn browser_domain_states_serialize_as_contract_values() {
    assert_eq!(
        V08BrowserDomainAdapterProofCapabilityName::ManagedBrowserControl.as_protocol_str(),
        proof::CAPABILITY_MANAGED_BROWSER_CONTROL
    );
    assert_eq!(
        V08BrowserDomainAdapterProofCapabilityStatus::ManualRequired.as_protocol_str(),
        proof::STATUS_MANUAL_REQUIRED
    );
    assert_eq!(
        V08BrowserDomainAdapterProofEvidenceKind::NetworkDomain.as_protocol_str(),
        proof::EVIDENCE_NETWORK_DOMAIN
    );
    assert_eq!(
        V08BrowserDomainAdapterProofClaimState::DegradedBoundary.as_protocol_str(),
        proof::CLAIM_DEGRADED_BOUNDARY
    );
    assert_eq!(
        V08BrowserDomainAdapterExecutionState::ReturnsDegradedNoop.as_protocol_str(),
        proof::RETURNS_DEGRADED_NOOP
    );
}

#[test]
fn windows_app_control_states_serialize_as_contract_values() {
    assert_eq!(
        V08WindowsAppControlReadinessState::AuditOnly.as_protocol_str(),
        proof::APP_CONTROL_AUDIT_ONLY
    );
    assert_eq!(
        V08WindowsAppControlPolicyMutationState::CreateUpdateManualRequired.as_protocol_str(),
        proof::APP_CONTROL_POLICY_CREATE_UPDATE_MANUAL_REQUIRED
    );
    assert_eq!(
        V08WindowsAppControlRuleIdentityKind::Publisher.as_protocol_str(),
        proof::APP_CONTROL_IDENTITY_PUBLISHER
    );
    assert_eq!(
        V08WindowsAppControlAdminRequirement::AdministratorRequired.as_protocol_str(),
        proof::APP_CONTROL_ADMINISTRATOR_REQUIRED
    );
    assert_eq!(
        V08WindowsAppControlEventState::FailureVisible.as_protocol_str(),
        proof::APP_CONTROL_EVENT_FAILURE_VISIBLE
    );
}

#[test]
fn browser_domain_read_model_serializes_claim_boundaries_for_service_preview() {
    let entry = |proof_entry_id: &'static str,
                 surface,
                 platform,
                 capability,
                 evidence_kind,
                 product_claim_state,
                 adapter_execution_state| V08BrowserDomainAdapterProofEntry {
        schema_version: crate::policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        proof_entry_id: proof_entry_id.to_string(),
        surface,
        platform,
        capability,
        capability_status: V08BrowserDomainAdapterProofCapabilityStatus::ManualRequired,
        evidence_kind,
        product_claim_state,
        adapter_execution_state,
        linked_proof_commands: Vec::new(),
        linked_proof_artifacts: Vec::new(),
        manual_proof_requirements: vec![proof::REQUIREMENT_ROLLBACK.to_string()],
        claim_boundary: proof::CLAIM_NETWORK_FILTER_MANUAL.to_string(),
        fallback_behavior: proof::FALLBACK_NETWORK_FILTER_MANUAL.to_string(),
        managed_exact_url_claimed: false,
        unmanaged_exact_url_claimed: false,
        network_domain_blocking_claimed: false,
        broad_browser_control_claimed: false,
        unsupported_os_claimed: false,
        last_checked_at: crate::policy_constants::TEST_EVALUATED_AT.to_string(),
    };
    let read_model = V08BrowserDomainAdapterProofReadModel {
        schema_version: crate::policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        read_model_id: proof::READ_MODEL_ID.to_string(),
        generated_at: crate::policy_constants::TEST_EVALUATED_AT.to_string(),
        source_read_model_ids: vec![proof::SOURCE_BROAD_OS_PROOF.to_string()],
        windows_app_control_states: vec![app_control_state()],
        entries: vec![
            entry(
                proof::ENTRY_ID_MANAGED_INTERVENTION,
                V08BrowserDomainAdapterProofSurface::WindowsManagedBrowserInterventionState,
                ParentPlatform::Windows,
                V08BrowserDomainAdapterProofCapabilityName::ManagedBrowserControl,
                V08BrowserDomainAdapterProofEvidenceKind::ManagedBrowser,
                V08BrowserDomainAdapterProofClaimState::ImplementedBoundary,
                V08BrowserDomainAdapterExecutionState::ExecutesRealService,
            ),
            entry(
                proof::ENTRY_ID_UNMANAGED_WARN,
                V08BrowserDomainAdapterProofSurface::WindowsUnmanagedBrowserWarnNoop,
                ParentPlatform::Windows,
                V08BrowserDomainAdapterProofCapabilityName::UnmanagedBrowserDetection,
                V08BrowserDomainAdapterProofEvidenceKind::UnmanagedBrowser,
                V08BrowserDomainAdapterProofClaimState::DegradedBoundary,
                V08BrowserDomainAdapterExecutionState::ReturnsDegradedNoop,
            ),
            entry(
                proof::ENTRY_ID_NETWORK_FILTER_MANUAL,
                V08BrowserDomainAdapterProofSurface::WindowsNetworkDomainFilterManual,
                ParentPlatform::Windows,
                V08BrowserDomainAdapterProofCapabilityName::NetworkDomainBlocking,
                V08BrowserDomainAdapterProofEvidenceKind::NetworkDomain,
                V08BrowserDomainAdapterProofClaimState::ManualRequired,
                V08BrowserDomainAdapterExecutionState::ReturnsManualRequired,
            ),
        ],
    };
    let serialized =
        serde_json::to_value(read_model).expect_value(constants::error::AGENT_EVENT_SERIALIZES);
    let reparsed = serde_json::from_value::<V08BrowserDomainAdapterProofReadModel>(serialized)
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
    assert_eq!(claim_counts[proof::CLAIM_DEGRADED_BOUNDARY], 1);
    assert_eq!(claim_counts[proof::CLAIM_MANUAL_REQUIRED], 1);
    assert_eq!(reparsed.windows_app_control_states.len(), 1);
    assert_eq!(
        reparsed.windows_app_control_states[0].readiness_state,
        V08WindowsAppControlReadinessState::AuditOnly
    );
    assert!(!reparsed.windows_app_control_states[0].app_control_prevention_claimed);
    assert!(reparsed
        .entries
        .iter()
        .all(|entry| !entry.managed_exact_url_claimed));
    assert!(reparsed
        .entries
        .iter()
        .all(|entry| !entry.network_domain_blocking_claimed));
}

fn app_control_state() -> V08WindowsAppControlProofState {
    V08WindowsAppControlProofState {
        proof_state_id: proof::STATE_ID_APP_CONTROL_AUDIT_ONLY.to_string(),
        readiness_state: V08WindowsAppControlReadinessState::AuditOnly,
        policy_mutation_state: V08WindowsAppControlPolicyMutationState::AuditOnlyVisible,
        rule_identity_kinds: vec![
            V08WindowsAppControlRuleIdentityKind::Publisher,
            V08WindowsAppControlRuleIdentityKind::Path,
            V08WindowsAppControlRuleIdentityKind::Hash,
            V08WindowsAppControlRuleIdentityKind::Package,
        ],
        admin_requirement: V08WindowsAppControlAdminRequirement::AdministratorRequired,
        event_states: vec![V08WindowsAppControlEventState::AuditVisible],
        manual_proof_requirements: vec![
            proof::REQUIREMENT_WINDOWS_APP_CONTROL_AUDIT_POLICY.to_string(),
            proof::REQUIREMENT_WINDOWS_APP_CONTROL_AUDIT_QUERY.to_string(),
        ],
        claim_boundary: proof::CLAIM_WINDOWS_APP_CONTROL_AUDIT_ONLY.to_string(),
        fallback_behavior: proof::FALLBACK_WINDOWS_APP_CONTROL_AUDIT_ONLY.to_string(),
        app_control_prevention_claimed: false,
        policy_creation_claimed: false,
        policy_update_claimed: false,
        rollback_claimed: false,
        last_checked_at: crate::policy_constants::TEST_EVALUATED_AT.to_string(),
    }
}
