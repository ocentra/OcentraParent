use std::collections::BTreeMap;

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::constants::v08_browser_domain_adapter_proof as proof;
use ocentra_parent_agent_protocol::enforcement::ParentPlatform;
use ocentra_parent_agent_protocol::enforcement_browser_domain_adapter_proof::V08BrowserDomainAdapterExecutionState;
use ocentra_parent_agent_protocol::enforcement_browser_domain_adapter_proof::V08BrowserDomainAdapterProofCapabilityName;
use ocentra_parent_agent_protocol::enforcement_browser_domain_adapter_proof::V08BrowserDomainAdapterProofCapabilityStatus;
use ocentra_parent_agent_protocol::enforcement_browser_domain_adapter_proof::V08BrowserDomainAdapterProofClaimState;
use ocentra_parent_agent_protocol::enforcement_browser_domain_adapter_proof::V08BrowserDomainAdapterProofEntry;
use ocentra_parent_agent_protocol::enforcement_browser_domain_adapter_proof::V08BrowserDomainAdapterProofEvidenceKind;
use ocentra_parent_agent_protocol::enforcement_browser_domain_adapter_proof::V08BrowserDomainAdapterProofReadModel;
use ocentra_parent_agent_protocol::enforcement_browser_domain_adapter_proof::V08BrowserDomainAdapterProofSurface;
use ocentra_parent_agent_protocol::enforcement_browser_domain_adapter_proof::V08WindowsAppControlProofState;
use ocentra_parent_agent_protocol::enforcement_browser_domain_adapter_proof::V08WindowsAppControlReadinessState;
use ocentra_parent_agent_protocol::enforcement_browser_domain_adapter_proof::V08WindowsAppControlRuleIdentityKind;
use ocentra_parent_agent_protocol::policy_constants;

use super::test_text::{count_for_display, test_ok, TestResult, TestText};
use crate::{
    enforcement_browser_domain_adapter_proof_read_model::v08_browser_domain_adapter_proof_read_model,
    test_require_some::require_some,
};

#[test]
fn browser_domain_read_model_preserves_honest_adapter_states() {
    let read_model =
        v08_browser_domain_adapter_proof_read_model(policy_constants::TEST_EVALUATED_AT);
    let claim_counts = count_claims(&read_model.entries);
    let platform_counts = count_platforms(&read_model.entries);

    assert_eq!(read_model.read_model_id, proof::READ_MODEL_ID);
    assert_eq!(read_model.entries.len(), 14);
    assert_eq!(read_model.windows_app_control_states.len(), 6);
    assert_eq!(
        claim_count(&claim_counts, proof::CLAIM_IMPLEMENTED_BOUNDARY),
        4
    );
    assert_eq!(
        claim_count(&claim_counts, proof::CLAIM_DEGRADED_BOUNDARY),
        1
    );
    assert_eq!(claim_count(&claim_counts, proof::CLAIM_MANUAL_REQUIRED), 5);
    assert_eq!(claim_count(&claim_counts, proof::CLAIM_UNAVAILABLE), 3);
    assert_eq!(claim_count(&claim_counts, proof::CLAIM_NOT_CLAIMED), 1);
    assert_eq!(
        platform_count(&platform_counts, ParentPlatform::Windows),
        10
    );
    assert_eq!(platform_count(&platform_counts, ParentPlatform::Linux), 1);
    assert_eq!(platform_count(&platform_counts, ParentPlatform::Macos), 1);
    assert_eq!(platform_count(&platform_counts, ParentPlatform::Android), 1);
    assert_eq!(platform_count(&platform_counts, ParentPlatform::Ios), 1);
    assert!(read_model
        .source_read_model_ids
        .contains(&proof::SOURCE_BROAD_OS_PROOF.to_string()));
    assert!(read_model
        .source_read_model_ids
        .contains(&proof::SOURCE_CROSS_PLATFORM_PROOF.to_string()));
    assert_app_control_readiness_states(&read_model.windows_app_control_states);
}

#[test]
fn browser_domain_read_model_keeps_surface_states_exact() {
    let read_model =
        v08_browser_domain_adapter_proof_read_model(policy_constants::TEST_EVALUATED_AT);
    assert_surface_state(
        &read_model.entries,
        V08BrowserDomainAdapterProofSurface::WindowsManagedBrowserInterventionState,
        V08BrowserDomainAdapterProofCapabilityName::ManagedBrowserControl,
        V08BrowserDomainAdapterProofCapabilityStatus::ManualRequired,
        V08BrowserDomainAdapterProofEvidenceKind::ManagedBrowser,
        V08BrowserDomainAdapterProofClaimState::ManualRequired,
        V08BrowserDomainAdapterExecutionState::ReturnsManualRequired,
    );
    assert_surface_state(
        &read_model.entries,
        V08BrowserDomainAdapterProofSurface::WindowsManagedBrowserExactUrlManual,
        V08BrowserDomainAdapterProofCapabilityName::ManagedBrowserControl,
        V08BrowserDomainAdapterProofCapabilityStatus::ManualRequired,
        V08BrowserDomainAdapterProofEvidenceKind::ManagedBrowser,
        V08BrowserDomainAdapterProofClaimState::ManualRequired,
        V08BrowserDomainAdapterExecutionState::ReturnsManualRequired,
    );
    assert_surface_state(
        &read_model.entries,
        V08BrowserDomainAdapterProofSurface::WindowsUnmanagedBrowserExactEvidenceNotClaimed,
        V08BrowserDomainAdapterProofCapabilityName::UnmanagedBrowserDetection,
        V08BrowserDomainAdapterProofCapabilityStatus::NotImplemented,
        V08BrowserDomainAdapterProofEvidenceKind::UnmanagedBrowser,
        V08BrowserDomainAdapterProofClaimState::NotClaimed,
        V08BrowserDomainAdapterExecutionState::NotInvoked,
    );
    assert_surface_state(
        &read_model.entries,
        V08BrowserDomainAdapterProofSurface::WindowsNetworkDomainFilterManual,
        V08BrowserDomainAdapterProofCapabilityName::NetworkDomainBlocking,
        V08BrowserDomainAdapterProofCapabilityStatus::ManualRequired,
        V08BrowserDomainAdapterProofEvidenceKind::NetworkDomain,
        V08BrowserDomainAdapterProofClaimState::ManualRequired,
        V08BrowserDomainAdapterExecutionState::ReturnsManualRequired,
    );
}

#[test]
fn browser_domain_read_model_does_not_upgrade_exact_or_domain_claims() {
    let read_model =
        v08_browser_domain_adapter_proof_read_model(policy_constants::TEST_EVALUATED_AT);
    let managed_exact_url = entry_for(
        &read_model.entries,
        V08BrowserDomainAdapterProofSurface::WindowsManagedBrowserExactUrlManual,
    );
    let unmanaged_exact = entry_for(
        &read_model.entries,
        V08BrowserDomainAdapterProofSurface::WindowsUnmanagedBrowserExactEvidenceNotClaimed,
    );
    let network_manual = entry_for(
        &read_model.entries,
        V08BrowserDomainAdapterProofSurface::WindowsNetworkDomainFilterManual,
    );
    let network_unavailable = entry_for(
        &read_model.entries,
        V08BrowserDomainAdapterProofSurface::WindowsNetworkDomainAdapterUnavailable,
    );

    assert_eq!(
        managed_exact_url.product_claim_state,
        V08BrowserDomainAdapterProofClaimState::ManualRequired
    );
    assert_eq!(
        unmanaged_exact.product_claim_state,
        V08BrowserDomainAdapterProofClaimState::NotClaimed
    );
    assert_eq!(
        network_manual.product_claim_state,
        V08BrowserDomainAdapterProofClaimState::ManualRequired
    );
    assert_eq!(
        network_unavailable.product_claim_state,
        V08BrowserDomainAdapterProofClaimState::Unavailable
    );
    assert!(read_model
        .entries
        .iter()
        .all(|entry| !entry.managed_exact_url_claimed));
    assert!(read_model
        .entries
        .iter()
        .all(|entry| !entry.unmanaged_exact_url_claimed));
    assert!(read_model
        .entries
        .iter()
        .all(|entry| !entry.network_domain_blocking_claimed));
    assert!(read_model
        .entries
        .iter()
        .all(|entry| !entry.broad_browser_control_claimed));
    assert!(read_model
        .entries
        .iter()
        .all(|entry| !entry.unsupported_os_claimed));
}

#[test]
fn browser_domain_read_model_serializes_for_service_preview() -> TestResult {
    let read_model =
        v08_browser_domain_adapter_proof_read_model(policy_constants::TEST_EVALUATED_AT);
    let serialized = test_ok(
        serde_json::to_value(read_model),
        constants::error::AGENT_EVENT_SERIALIZES,
    )?;
    let reparsed = test_ok(
        serde_json::from_value::<V08BrowserDomainAdapterProofReadModel>(serialized),
        constants::error::AGENT_EVENT_SERIALIZES,
    )?;
    let linux = entry_for(
        &reparsed.entries,
        V08BrowserDomainAdapterProofSurface::LinuxBrowserDomainAdapterUnavailable,
    );
    let rollback = entry_for(
        &reparsed.entries,
        V08BrowserDomainAdapterProofSurface::WindowsBrowserPolicyRollbackVisibility,
    );
    let app_control_audit = app_control_state_for(
        &reparsed.windows_app_control_states,
        V08WindowsAppControlReadinessState::AuditOnly,
    );

    assert_eq!(reparsed.read_model_id, proof::READ_MODEL_ID);
    assert_eq!(linux.platform, ParentPlatform::Linux);
    assert_eq!(
        linux.product_claim_state,
        V08BrowserDomainAdapterProofClaimState::Unavailable
    );
    assert!(linux
        .manual_proof_requirements
        .contains(&proof::REQUIREMENT_LINUX_ADAPTER.to_string()));
    assert!(rollback
        .linked_proof_commands
        .contains(&proof::COMMAND_BROWSER_POLICY_ROLLBACK_TEST.to_string()));
    assert!(app_control_audit
        .event_states
        .iter()
        .any(|state| state.as_protocol_str() == proof::APP_CONTROL_EVENT_AUDIT_VISIBLE));
    assert!(!app_control_audit.app_control_prevention_claimed);

    Ok(())
}

fn entry_for(
    entries: &[V08BrowserDomainAdapterProofEntry],
    surface: V08BrowserDomainAdapterProofSurface,
) -> &V08BrowserDomainAdapterProofEntry {
    require_some(
        entries.iter().find(|entry| entry.surface == surface),
        proof::READ_MODEL_ID,
    )
}

fn assert_surface_state(
    entries: &[V08BrowserDomainAdapterProofEntry],
    surface: V08BrowserDomainAdapterProofSurface,
    capability: V08BrowserDomainAdapterProofCapabilityName,
    capability_status: V08BrowserDomainAdapterProofCapabilityStatus,
    evidence_kind: V08BrowserDomainAdapterProofEvidenceKind,
    product_claim_state: V08BrowserDomainAdapterProofClaimState,
    adapter_execution_state: V08BrowserDomainAdapterExecutionState,
) {
    let entry = entry_for(entries, surface);

    assert_eq!(entry.capability, capability);
    assert_eq!(entry.capability_status, capability_status);
    assert_eq!(entry.evidence_kind, evidence_kind);
    assert_eq!(entry.product_claim_state, product_claim_state);
    assert_eq!(entry.adapter_execution_state, adapter_execution_state);
}

fn assert_app_control_readiness_states(states: &[V08WindowsAppControlProofState]) {
    let readiness_counts = states.iter().fold(BTreeMap::new(), |mut counts, state| {
        *counts
            .entry(TestText::from_display(
                state.readiness_state.as_protocol_str(),
            ))
            .or_default() += 1;
        counts
    });
    let readiness =
        app_control_state_for(states, V08WindowsAppControlReadinessState::ReadinessCheck);
    let enforced = app_control_state_for(states, V08WindowsAppControlReadinessState::Enforced);
    let unavailable =
        app_control_state_for(states, V08WindowsAppControlReadinessState::Unavailable);
    let failed = app_control_state_for(states, V08WindowsAppControlReadinessState::Failed);

    assert_eq!(
        claim_count(&readiness_counts, proof::APP_CONTROL_READINESS_CHECK),
        1
    );
    assert_eq!(
        claim_count(&readiness_counts, proof::APP_CONTROL_AUDIT_ONLY),
        1
    );
    assert_eq!(
        claim_count(&readiness_counts, proof::APP_CONTROL_ENFORCED),
        1
    );
    assert_eq!(
        claim_count(&readiness_counts, proof::APP_CONTROL_MANUAL_REQUIRED),
        1
    );
    assert_eq!(
        claim_count(&readiness_counts, proof::APP_CONTROL_UNAVAILABLE),
        1
    );
    assert_eq!(claim_count(&readiness_counts, proof::APP_CONTROL_FAILED), 1);
    assert_eq!(readiness.rule_identity_kinds.len(), 4);
    assert!(readiness
        .rule_identity_kinds
        .contains(&V08WindowsAppControlRuleIdentityKind::Publisher));
    assert_eq!(
        enforced.policy_mutation_state.as_protocol_str(),
        proof::APP_CONTROL_POLICY_CREATE_UPDATE_MANUAL_REQUIRED
    );
    assert_eq!(unavailable.rule_identity_kinds.len(), 0);
    assert!(failed
        .event_states
        .iter()
        .any(|state| state.as_protocol_str() == proof::APP_CONTROL_EVENT_FAILURE_VISIBLE));
    assert!(states
        .iter()
        .all(|state| !state.app_control_prevention_claimed
            && !state.policy_creation_claimed
            && !state.policy_update_claimed
            && !state.rollback_claimed));
}

fn app_control_state_for(
    states: &[V08WindowsAppControlProofState],
    readiness_state: V08WindowsAppControlReadinessState,
) -> &V08WindowsAppControlProofState {
    require_some(
        states
            .iter()
            .find(|state| state.readiness_state == readiness_state),
        proof::READ_MODEL_ID,
    )
}

fn count_claims(entries: &[V08BrowserDomainAdapterProofEntry]) -> BTreeMap<TestText, usize> {
    entries.iter().fold(BTreeMap::new(), |mut counts, entry| {
        *counts
            .entry(TestText::from_display(
                entry.product_claim_state.as_protocol_str(),
            ))
            .or_default() += 1;
        counts
    })
}

fn claim_count(counts: &BTreeMap<TestText, usize>, claim: impl std::fmt::Display) -> usize {
    count_for_display(counts, claim)
}

fn count_platforms(entries: &[V08BrowserDomainAdapterProofEntry]) -> BTreeMap<TestText, usize> {
    entries.iter().fold(BTreeMap::new(), |mut counts, entry| {
        *counts
            .entry(TestText::from_display(entry.platform.as_protocol_str()))
            .or_default() += 1;
        counts
    })
}

fn platform_count(counts: &BTreeMap<TestText, usize>, platform: ParentPlatform) -> usize {
    count_for_display(counts, platform.as_protocol_str())
}
