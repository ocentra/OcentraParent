use std::collections::BTreeMap;

use ocentra_parent_agent_protocol::{
    constants::{self, v08_browser_domain_adapter_proof as proof},
    policy_constants, ParentPlatform, V08BrowserDomainAdapterProofClaimState,
    V08BrowserDomainAdapterProofEntry, V08BrowserDomainAdapterProofReadModel,
    V08BrowserDomainAdapterProofSurface,
};

use crate::enforcement_browser_domain_adapter_proof_read_model::v08_browser_domain_adapter_proof_read_model;

#[test]
fn browser_domain_read_model_preserves_honest_adapter_states() {
    let read_model =
        v08_browser_domain_adapter_proof_read_model(policy_constants::TEST_EVALUATED_AT);
    let claim_counts = count_claims(&read_model.entries);
    let platform_counts = count_platforms(&read_model.entries);

    assert_eq!(read_model.read_model_id, proof::READ_MODEL_ID);
    assert_eq!(read_model.entries.len(), 14);
    assert_eq!(
        claim_count(&claim_counts, proof::CLAIM_IMPLEMENTED_BOUNDARY),
        5
    );
    assert_eq!(
        claim_count(&claim_counts, proof::CLAIM_DEGRADED_BOUNDARY),
        1
    );
    assert_eq!(claim_count(&claim_counts, proof::CLAIM_MANUAL_REQUIRED), 4);
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
fn browser_domain_read_model_serializes_for_service_preview() {
    let read_model =
        v08_browser_domain_adapter_proof_read_model(policy_constants::TEST_EVALUATED_AT);
    let serialized =
        serde_json::to_value(read_model).expect(constants::error::AGENT_EVENT_SERIALIZES);
    let reparsed = serde_json::from_value::<V08BrowserDomainAdapterProofReadModel>(serialized)
        .expect(constants::error::AGENT_EVENT_SERIALIZES);
    let linux = entry_for(
        &reparsed.entries,
        V08BrowserDomainAdapterProofSurface::LinuxBrowserDomainAdapterUnavailable,
    );
    let rollback = entry_for(
        &reparsed.entries,
        V08BrowserDomainAdapterProofSurface::WindowsBrowserPolicyRollbackVisibility,
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
}

fn entry_for(
    entries: &[V08BrowserDomainAdapterProofEntry],
    surface: V08BrowserDomainAdapterProofSurface,
) -> &V08BrowserDomainAdapterProofEntry {
    entries
        .iter()
        .find(|entry| entry.surface == surface)
        .expect(proof::READ_MODEL_ID)
}

fn count_claims(entries: &[V08BrowserDomainAdapterProofEntry]) -> BTreeMap<&'static str, usize> {
    entries.iter().fold(BTreeMap::new(), |mut counts, entry| {
        *counts
            .entry(entry.product_claim_state.as_protocol_str())
            .or_default() += 1;
        counts
    })
}

fn claim_count(counts: &BTreeMap<&'static str, usize>, claim: &'static str) -> usize {
    *counts.get(claim).unwrap_or(&0)
}

fn count_platforms(entries: &[V08BrowserDomainAdapterProofEntry]) -> BTreeMap<&'static str, usize> {
    entries.iter().fold(BTreeMap::new(), |mut counts, entry| {
        *counts.entry(entry.platform.as_protocol_str()).or_default() += 1;
        counts
    })
}

fn platform_count(counts: &BTreeMap<&'static str, usize>, platform: ParentPlatform) -> usize {
    *counts.get(platform.as_protocol_str()).unwrap_or(&0)
}
