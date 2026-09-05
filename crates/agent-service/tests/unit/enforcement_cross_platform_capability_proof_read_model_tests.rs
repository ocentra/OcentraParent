use std::collections::BTreeMap;

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::constants::enforcement as enforcement_constants;
use ocentra_parent_agent_protocol::constants::v08_browser_domain_adapter_proof as browser_proof;
use ocentra_parent_agent_protocol::constants::v08_cross_platform_enforcement_capability_proof as proof;
use ocentra_parent_agent_protocol::enforcement::ParentPlatform;
use ocentra_parent_agent_protocol::enforcement_cross_platform_capability_proof::V08CrossPlatformAdapterExecutionState;
use ocentra_parent_agent_protocol::enforcement_cross_platform_capability_proof::V08CrossPlatformEnforcementCapabilityClaimState;
use ocentra_parent_agent_protocol::enforcement_cross_platform_capability_proof::V08CrossPlatformEnforcementCapabilityProofEntry;
use ocentra_parent_agent_protocol::enforcement_cross_platform_capability_proof::V08CrossPlatformEnforcementCapabilityProofReadModel;
use ocentra_parent_agent_protocol::enforcement_cross_platform_capability_proof::V08CrossPlatformEnforcementCapabilitySurface;
use ocentra_parent_agent_protocol::policy_constants;

use super::test_text::TestText;
use crate::{
    enforcement_cross_platform_capability_proof_read_model::{
        v08_cross_platform_enforcement_capability_proof_read_model, GeneratedAtTextRef,
    },
    test_require_ok::require_ok,
    test_require_some::require_some,
};

#[test]
fn cross_platform_read_model_preserves_honest_capability_states() {
    let read_model = v08_cross_platform_enforcement_capability_proof_read_model(
        GeneratedAtTextRef(policy_constants::TEST_EVALUATED_AT),
    );
    let claim_counts = count_claims(&read_model.entries);
    let platform_counts = count_platforms(&read_model.entries);

    assert_eq!(read_model.read_model_id, proof::READ_MODEL_ID);
    assert_eq!(read_model.entries.len(), 15);
    assert_eq!(
        claim_count(&claim_counts, proof::CLAIM_IMPLEMENTED_BOUNDARY),
        2
    );
    assert_eq!(claim_count(&claim_counts, proof::CLAIM_MANUAL_REQUIRED), 9);
    assert_eq!(claim_count(&claim_counts, proof::CLAIM_SCAFFOLD), 2);
    assert_eq!(claim_count(&claim_counts, proof::CLAIM_PLANNED), 2);
    assert_eq!(platform_count(&platform_counts, ParentPlatform::Windows), 6);
    assert_eq!(platform_count(&platform_counts, ParentPlatform::Linux), 1);
    assert_eq!(platform_count(&platform_counts, ParentPlatform::Macos), 1);
    assert_eq!(platform_count(&platform_counts, ParentPlatform::Android), 3);
    assert_eq!(platform_count(&platform_counts, ParentPlatform::Ios), 4);
    assert!(read_model
        .source_read_model_ids
        .contains(&proof::SOURCE_BROAD_PROOF.to_string()));
    assert!(read_model
        .source_read_model_ids
        .contains(&proof::SOURCE_PRODUCT_PROOF.to_string()));
}

#[test]
fn cross_platform_read_model_does_not_upgrade_unproved_claims() {
    let read_model = v08_cross_platform_enforcement_capability_proof_read_model(
        GeneratedAtTextRef(policy_constants::TEST_EVALUATED_AT),
    );
    let windows_broad = entry_for(
        &read_model.entries,
        V08CrossPlatformEnforcementCapabilitySurface::WindowsBroadInstalledAppBlocking,
    );
    let windows_app_time = entry_for(
        &read_model.entries,
        V08CrossPlatformEnforcementCapabilitySurface::WindowsAppTimeLimitLifecycle,
    );
    let managed_browser = entry_for(
        &read_model.entries,
        V08CrossPlatformEnforcementCapabilitySurface::WindowsManagedBrowserBoundary,
    );
    let android_device_owner = entry_for(
        &read_model.entries,
        V08CrossPlatformEnforcementCapabilitySurface::AndroidDeviceOwnerPolicy,
    );
    let ios_store = entry_for(
        &read_model.entries,
        V08CrossPlatformEnforcementCapabilitySurface::IosStoreDistribution,
    );

    assert_eq!(
        windows_broad.product_claim_state,
        V08CrossPlatformEnforcementCapabilityClaimState::ManualRequired
    );
    assert_eq!(
        windows_app_time.product_claim_state,
        V08CrossPlatformEnforcementCapabilityClaimState::ManualRequired
    );
    assert_eq!(
        managed_browser.product_claim_state,
        V08CrossPlatformEnforcementCapabilityClaimState::ManualRequired
    );
    assert_eq!(
        windows_app_time.adapter_execution_state,
        V08CrossPlatformAdapterExecutionState::ReturnsManualRequired
    );
    assert_eq!(
        managed_browser.adapter_execution_state,
        V08CrossPlatformAdapterExecutionState::ReturnsManualRequired
    );
    assert_eq!(
        windows_app_time.manual_proof_requirements,
        vec![
            enforcement_constants::ARTIFACT_APP_TIME_LIMIT_EXECUTOR.to_string(),
            proof::REQUIREMENT_ROLLBACK.to_string(),
            proof::REQUIREMENT_AUDIT_CUSTODY.to_string(),
        ]
    );
    assert_eq!(
        managed_browser.manual_proof_requirements,
        vec![
            browser_proof::REQUIREMENT_MANAGED_PROFILE.to_string(),
            browser_proof::REQUIREMENT_ACTIVE_TAB.to_string(),
            browser_proof::REQUIREMENT_ROLLBACK.to_string(),
            browser_proof::REQUIREMENT_AUDIT_CUSTODY.to_string(),
        ]
    );
    assert_eq!(
        android_device_owner.product_claim_state,
        V08CrossPlatformEnforcementCapabilityClaimState::ManualRequired
    );
    assert_eq!(
        ios_store.product_claim_state,
        V08CrossPlatformEnforcementCapabilityClaimState::Planned
    );
    assert!(read_model
        .entries
        .iter()
        .all(|entry| !entry.broad_blocking_claimed));
    assert!(read_model
        .entries
        .iter()
        .all(|entry| !entry.exact_url_claimed));
    assert!(read_model
        .entries
        .iter()
        .all(|entry| !entry.privileged_mobile_claimed));
    assert!(read_model
        .entries
        .iter()
        .all(|entry| !entry.production_distribution_claimed));
}

#[test]
fn cross_platform_read_model_serializes_for_service_preview() {
    let read_model = v08_cross_platform_enforcement_capability_proof_read_model(
        GeneratedAtTextRef(policy_constants::TEST_EVALUATED_AT),
    );
    let serialized = require_ok(
        serde_json::to_value(read_model),
        constants::error::AGENT_EVENT_SERIALIZES,
    );
    let reparsed = require_ok(
        serde_json::from_value::<V08CrossPlatformEnforcementCapabilityProofReadModel>(serialized),
        constants::error::AGENT_EVENT_SERIALIZES,
    );
    let linux = entry_for(
        &reparsed.entries,
        V08CrossPlatformEnforcementCapabilitySurface::LinuxEnforcementAdapterScaffold,
    );

    assert_eq!(reparsed.read_model_id, proof::READ_MODEL_ID);
    assert_eq!(linux.platform, ParentPlatform::Linux);
    assert_eq!(
        linux.product_claim_state,
        V08CrossPlatformEnforcementCapabilityClaimState::Scaffold
    );
    assert!(linux
        .manual_proof_requirements
        .contains(&proof::REQUIREMENT_LINUX_ADAPTER.to_string()));
}

fn entry_for(
    entries: &[V08CrossPlatformEnforcementCapabilityProofEntry],
    surface: V08CrossPlatformEnforcementCapabilitySurface,
) -> &V08CrossPlatformEnforcementCapabilityProofEntry {
    require_some(
        entries.iter().find(|entry| entry.surface == surface),
        proof::READ_MODEL_ID,
    )
}

fn count_claims(
    entries: &[V08CrossPlatformEnforcementCapabilityProofEntry],
) -> BTreeMap<TestText, usize> {
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
    *counts.get(&TestText::from_display(claim)).unwrap_or(&0)
}

fn count_platforms(
    entries: &[V08CrossPlatformEnforcementCapabilityProofEntry],
) -> BTreeMap<TestText, usize> {
    entries.iter().fold(BTreeMap::new(), |mut counts, entry| {
        *counts
            .entry(TestText::from_display(entry.platform.as_protocol_str()))
            .or_default() += 1;
        counts
    })
}

fn platform_count(counts: &BTreeMap<TestText, usize>, platform: ParentPlatform) -> usize {
    *counts
        .get(&TestText::from_display(platform.as_protocol_str()))
        .unwrap_or(&0)
}
