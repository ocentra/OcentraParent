use std::collections::BTreeMap;
use std::primitive::str as TestStr;
use std::string::String as TestString;

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::constants::v08_enforcement_product_control_spine as spine;
use ocentra_parent_agent_protocol::enforcement_product_control_spine::V08EnforcementProductControlClaimState;
use ocentra_parent_agent_protocol::enforcement_product_control_spine::V08EnforcementProductControlDevicePolicyState;
use ocentra_parent_agent_protocol::enforcement_product_control_spine::V08EnforcementProductControlExecutionState;
use ocentra_parent_agent_protocol::enforcement_product_control_spine::V08EnforcementProductControlParentAction;
use ocentra_parent_agent_protocol::enforcement_product_control_spine::V08EnforcementProductControlSpineEntry;
use ocentra_parent_agent_protocol::enforcement_product_control_spine::V08EnforcementProductControlSpineReadModel;
use ocentra_parent_agent_protocol::enforcement_product_control_spine::V08EnforcementProductControlSurface;
use ocentra_parent_agent_protocol::policy_constants;

use crate::{
    enforcement_os_adapter_product_proof_read_model::product_control_spine::v08_enforcement_product_control_spine_read_model,
    test_invariants::require_some,
};

type TestResult = Result<(), TestString>;

#[test]
fn product_control_service_read_model_wires_runtime_sources() {
    let read_model =
        v08_enforcement_product_control_spine_read_model(policy_constants::TEST_EVALUATED_AT);
    let claim_counts = count_claims(&read_model.entries);
    let policy_counts = count_policy_states(&read_model.entries);

    assert_eq!(read_model.read_model_id, spine::READ_MODEL_ID);
    assert_eq!(read_model.entries.len(), 15);
    assert_eq!(
        claim_count(&claim_counts, spine::CLAIM_IMPLEMENTED_BOUNDARY),
        6
    );
    assert_eq!(
        claim_count(&claim_counts, spine::CLAIM_DEGRADED_BOUNDARY),
        1
    );
    assert_eq!(claim_count(&claim_counts, spine::CLAIM_DRY_RUN_ONLY), 1);
    assert_eq!(claim_count(&claim_counts, spine::CLAIM_MANUAL_REQUIRED), 6);
    assert_eq!(claim_count(&claim_counts, spine::CLAIM_NOT_CLAIMED), 1);
    assert_eq!(
        policy_count(&policy_counts, spine::DEVICE_POLICY_CONTROL_CAPABLE),
        5
    );
    assert_eq!(
        policy_count(&policy_counts, spine::DEVICE_POLICY_MANUAL_REQUIRED),
        6
    );
    assert!(read_model
        .source_read_model_ids
        .contains(&spine::SOURCE_CROSS_PLATFORM_CAPABILITY.to_string()));
    assert!(read_model
        .source_read_model_ids
        .contains(&spine::SOURCE_BROWSER_DOMAIN.to_string()));
    assert!(read_model
        .source_read_model_ids
        .contains(&spine::SOURCE_OS_ADAPTER_PRODUCT.to_string()));
}

#[test]
fn product_control_service_read_model_keeps_action_boundaries_exact() {
    let read_model =
        v08_enforcement_product_control_spine_read_model(policy_constants::TEST_EVALUATED_AT);
    let owned_process = entry_for(
        &read_model,
        V08EnforcementProductControlSurface::WindowsOwnedProcessTimeLimit,
    );
    let unmanaged_browser = entry_for(
        &read_model,
        V08EnforcementProductControlSurface::WindowsUnmanagedBrowserProcessFallback,
    );
    let policy_preview = entry_for(
        &read_model,
        V08EnforcementProductControlSurface::WindowsPolicyDryRunPreview,
    );
    let network_domain = entry_for(
        &read_model,
        V08EnforcementProductControlSurface::WindowsNetworkDomainBlocking,
    );

    assert_eq!(
        owned_process.device_policy_state,
        V08EnforcementProductControlDevicePolicyState::ControlCapable
    );
    assert_eq!(
        owned_process.parent_visible_actions,
        vec![
            V08EnforcementProductControlParentAction::Observe,
            V08EnforcementProductControlParentAction::TimeLimit,
            V08EnforcementProductControlParentAction::BlockScopedProcess
        ]
    );
    assert_eq!(
        unmanaged_browser.product_claim_state,
        V08EnforcementProductControlClaimState::DegradedBoundary
    );
    assert_eq!(
        unmanaged_browser.adapter_execution_state,
        V08EnforcementProductControlExecutionState::ReturnsDegradedNoop
    );
    assert_eq!(
        policy_preview.adapter_execution_state,
        V08EnforcementProductControlExecutionState::ReturnsDryRunPreview
    );
    assert_eq!(
        network_domain.adapter_execution_state,
        V08EnforcementProductControlExecutionState::ReturnsManualRequired
    );
    assert_eq!(
        network_domain.parent_visible_actions,
        vec![V08EnforcementProductControlParentAction::ReportOnly]
    );
}

#[test]
fn product_control_service_read_model_refuses_unproved_claim_upgrades() {
    let read_model =
        v08_enforcement_product_control_spine_read_model(policy_constants::TEST_EVALUATED_AT);
    let managed_exact = entry_for(
        &read_model,
        V08EnforcementProductControlSurface::WindowsManagedExactUrlControl,
    );
    let unmanaged_exact = entry_for(
        &read_model,
        V08EnforcementProductControlSurface::WindowsUnmanagedExactUrlNotClaimed,
    );
    let tamper = entry_for(
        &read_model,
        V08EnforcementProductControlSurface::WindowsTamperUninstallAlerts,
    );

    assert_eq!(
        managed_exact.device_policy_state,
        V08EnforcementProductControlDevicePolicyState::ManualRequired
    );
    assert_eq!(
        unmanaged_exact.product_claim_state,
        V08EnforcementProductControlClaimState::NotClaimed
    );
    assert!(tamper
        .manual_proof_requirements
        .contains(&spine::REQUIREMENT_NON_STEALTH_ALERT.to_string()));
    assert!(read_model
        .entries
        .iter()
        .all(|entry| !entry.broad_app_blocking_claimed));
    assert!(read_model
        .entries
        .iter()
        .all(|entry| !entry.network_domain_blocking_claimed));
    assert!(read_model
        .entries
        .iter()
        .all(|entry| !entry.managed_exact_url_blocking_claimed));
    assert!(read_model
        .entries
        .iter()
        .all(|entry| !entry.unmanaged_exact_url_claimed));
    assert!(read_model
        .entries
        .iter()
        .all(|entry| !entry.tamper_resistance_claimed));
    assert!(read_model
        .entries
        .iter()
        .all(|entry| !entry.notification_delivery_claimed));
}

#[test]
fn product_control_service_read_model_serializes_for_runtime_consumers() -> TestResult {
    let read_model =
        v08_enforcement_product_control_spine_read_model(policy_constants::TEST_EVALUATED_AT);
    let serialized = ok(
        serde_json::to_value(read_model),
        constants::error::AGENT_EVENT_SERIALIZES,
    )?;
    let reparsed = ok(
        serde_json::from_value::<V08EnforcementProductControlSpineReadModel>(serialized),
        constants::error::AGENT_EVENT_SERIALIZES,
    )?;
    let permission = entry_for(
        &reparsed,
        V08EnforcementProductControlSurface::WindowsPermissionLossAlerts,
    );

    assert_eq!(reparsed.read_model_id, spine::READ_MODEL_ID);
    assert_eq!(
        permission.product_claim_state,
        V08EnforcementProductControlClaimState::ManualRequired
    );
    assert!(permission
        .manual_proof_requirements
        .contains(&spine::REQUIREMENT_DELIVERY_RECEIPT.to_string()));

    Ok(())
}

fn entry_for(
    read_model: &V08EnforcementProductControlSpineReadModel,
    surface: V08EnforcementProductControlSurface,
) -> &V08EnforcementProductControlSpineEntry {
    require_some(
        read_model
            .entries
            .iter()
            .find(|entry| entry.surface == surface),
        spine::READ_MODEL_ID,
    )
}

fn count_claims(
    entries: &[V08EnforcementProductControlSpineEntry],
) -> BTreeMap<&'static TestStr, usize> {
    entries.iter().fold(BTreeMap::new(), |mut counts, entry| {
        *counts
            .entry(entry.product_claim_state.as_protocol_str())
            .or_default() += 1;
        counts
    })
}

fn count_policy_states(
    entries: &[V08EnforcementProductControlSpineEntry],
) -> BTreeMap<&'static TestStr, usize> {
    entries.iter().fold(BTreeMap::new(), |mut counts, entry| {
        *counts
            .entry(entry.device_policy_state.as_protocol_str())
            .or_default() += 1;
        counts
    })
}

fn claim_count(counts: &BTreeMap<&'static TestStr, usize>, claim: &'static TestStr) -> usize {
    *counts.get(claim).unwrap_or(&0)
}

fn policy_count(counts: &BTreeMap<&'static TestStr, usize>, state: &'static TestStr) -> usize {
    *counts.get(state).unwrap_or(&0)
}

fn ok<T, E: std::fmt::Debug>(result: Result<T, E>, context: &TestStr) -> Result<T, TestString> {
    result.map_err(|error| format!("{context}: {error:?}"))
}
