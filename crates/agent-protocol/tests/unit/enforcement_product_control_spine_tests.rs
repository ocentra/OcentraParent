use ocentra_eventing::expect_value::ExpectValue;
use std::collections::BTreeMap;

use crate::{
    constants::{self, v08_enforcement_product_control_spine as spine},
    ParentPlatform, V08EnforcementProductControlCapabilityName,
    V08EnforcementProductControlCapabilityStatus, V08EnforcementProductControlClaimState,
    V08EnforcementProductControlDevicePolicyState, V08EnforcementProductControlExecutionState,
    V08EnforcementProductControlParentAction, V08EnforcementProductControlSpineEntry,
    V08EnforcementProductControlSpineReadModel, V08EnforcementProductControlSurface,
    V08EnforcementProductControlSurfaceKind,
};

#[test]
fn product_control_surfaces_have_stable_protocol_strings() {
    let surfaces = [
        V08EnforcementProductControlSurface::WindowsOwnedProcessTimeLimit,
        V08EnforcementProductControlSurface::WindowsAppTimeLimitLifecycle,
        V08EnforcementProductControlSurface::WindowsManagedBrowserSessionIntervention,
        V08EnforcementProductControlSurface::WindowsUnmanagedBrowserProcessFallback,
        V08EnforcementProductControlSurface::WindowsPolicyDryRunPreview,
        V08EnforcementProductControlSurface::WindowsApprovalOverrideAudit,
        V08EnforcementProductControlSurface::WindowsRestartRecoveryTimer,
        V08EnforcementProductControlSurface::WindowsRollbackAuditBoundary,
        V08EnforcementProductControlSurface::WindowsChildFacingExplanation,
        V08EnforcementProductControlSurface::WindowsBroadAppBlocking,
        V08EnforcementProductControlSurface::WindowsNetworkDomainBlocking,
        V08EnforcementProductControlSurface::WindowsManagedExactUrlControl,
        V08EnforcementProductControlSurface::WindowsUnmanagedExactUrlNotClaimed,
        V08EnforcementProductControlSurface::WindowsPermissionLossAlerts,
        V08EnforcementProductControlSurface::WindowsTamperUninstallAlerts,
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
    assert_eq!(surfaces[0].as_protocol_str(), spine::SURFACE_OWNED_PROCESS);
    assert_eq!(
        surfaces[14].as_protocol_str(),
        spine::SURFACE_TAMPER_UNINSTALL
    );
}

#[test]
fn product_control_states_serialize_as_contract_values() {
    assert_eq!(
        V08EnforcementProductControlSurfaceKind::ManagedBrowser.as_protocol_str(),
        spine::KIND_MANAGED_BROWSER
    );
    assert_eq!(
        V08EnforcementProductControlCapabilityName::TypedProtocolBridge.as_protocol_str(),
        spine::CAPABILITY_TYPED_PROTOCOL_BRIDGE
    );
    assert_eq!(
        V08EnforcementProductControlCapabilityStatus::ManualRequired.as_protocol_str(),
        spine::STATUS_MANUAL_REQUIRED
    );
    assert_eq!(
        V08EnforcementProductControlClaimState::DryRunOnly.as_protocol_str(),
        spine::CLAIM_DRY_RUN_ONLY
    );
    assert_eq!(
        V08EnforcementProductControlExecutionState::ReturnsDegradedNoop.as_protocol_str(),
        spine::RETURNS_DEGRADED_NOOP
    );
    assert_eq!(
        V08EnforcementProductControlDevicePolicyState::PreviewOnly.as_protocol_str(),
        spine::DEVICE_POLICY_PREVIEW_ONLY
    );
    assert_eq!(
        V08EnforcementProductControlParentAction::BlockScopedProcess.as_protocol_str(),
        spine::ACTION_BLOCK_SCOPED_PROCESS
    );
}

#[test]
fn product_control_read_model_serializes_parent_visible_action_states() {
    let read_model = V08EnforcementProductControlSpineReadModel {
        schema_version: crate::policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        read_model_id: spine::READ_MODEL_ID.to_string(),
        generated_at: crate::policy_constants::TEST_EVALUATED_AT.to_string(),
        source_read_model_ids: vec![spine::SOURCE_CROSS_PLATFORM_CAPABILITY.to_string()],
        entries: vec![
            entry(EntryFixture {
                entry_id: spine::ENTRY_ID_OWNED_PROCESS,
                surface: V08EnforcementProductControlSurface::WindowsOwnedProcessTimeLimit,
                surface_kind: V08EnforcementProductControlSurfaceKind::Process,
                capability: V08EnforcementProductControlCapabilityName::OwnedProcessTerminate,
                product_claim_state: V08EnforcementProductControlClaimState::ImplementedBoundary,
                adapter_execution_state:
                    V08EnforcementProductControlExecutionState::ExecutesRealService,
                device_policy_state: V08EnforcementProductControlDevicePolicyState::ControlCapable,
                parent_visible_actions: vec![
                    V08EnforcementProductControlParentAction::Observe,
                    V08EnforcementProductControlParentAction::TimeLimit,
                    V08EnforcementProductControlParentAction::BlockScopedProcess,
                ],
            }),
            entry(EntryFixture {
                entry_id: spine::ENTRY_ID_POLICY_DRY_RUN,
                surface: V08EnforcementProductControlSurface::WindowsPolicyDryRunPreview,
                surface_kind: V08EnforcementProductControlSurfaceKind::Policy,
                capability: V08EnforcementProductControlCapabilityName::TypedProtocolBridge,
                product_claim_state: V08EnforcementProductControlClaimState::DryRunOnly,
                adapter_execution_state:
                    V08EnforcementProductControlExecutionState::ReturnsDryRunPreview,
                device_policy_state: V08EnforcementProductControlDevicePolicyState::PreviewOnly,
                parent_visible_actions: vec![
                    V08EnforcementProductControlParentAction::DryRunPreview,
                    V08EnforcementProductControlParentAction::AskParent,
                ],
            }),
            entry(EntryFixture {
                entry_id: spine::ENTRY_ID_NETWORK_DOMAIN,
                surface: V08EnforcementProductControlSurface::WindowsNetworkDomainBlocking,
                surface_kind: V08EnforcementProductControlSurfaceKind::NetworkDomain,
                capability: V08EnforcementProductControlCapabilityName::NetworkDomainBlocking,
                product_claim_state: V08EnforcementProductControlClaimState::ManualRequired,
                adapter_execution_state:
                    V08EnforcementProductControlExecutionState::ReturnsManualRequired,
                device_policy_state: V08EnforcementProductControlDevicePolicyState::ManualRequired,
                parent_visible_actions: vec![V08EnforcementProductControlParentAction::ReportOnly],
            }),
        ],
    };
    let serialized =
        serde_json::to_value(read_model).expect_value(constants::error::AGENT_EVENT_SERIALIZES);
    let reparsed = serde_json::from_value::<V08EnforcementProductControlSpineReadModel>(serialized)
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

    assert_eq!(reparsed.read_model_id, spine::READ_MODEL_ID);
    assert_eq!(claim_counts[spine::CLAIM_IMPLEMENTED_BOUNDARY], 1);
    assert_eq!(claim_counts[spine::CLAIM_DRY_RUN_ONLY], 1);
    assert_eq!(claim_counts[spine::CLAIM_MANUAL_REQUIRED], 1);
    assert_eq!(
        reparsed.entries[0].parent_visible_actions[2].as_protocol_str(),
        spine::ACTION_BLOCK_SCOPED_PROCESS
    );
    assert!(reparsed
        .entries
        .iter()
        .all(|entry| !entry.network_domain_blocking_claimed));
    assert!(reparsed
        .entries
        .iter()
        .all(|entry| !entry.tamper_resistance_claimed));
}

struct EntryFixture {
    entry_id: &'static str,
    surface: V08EnforcementProductControlSurface,
    surface_kind: V08EnforcementProductControlSurfaceKind,
    capability: V08EnforcementProductControlCapabilityName,
    product_claim_state: V08EnforcementProductControlClaimState,
    adapter_execution_state: V08EnforcementProductControlExecutionState,
    device_policy_state: V08EnforcementProductControlDevicePolicyState,
    parent_visible_actions: Vec<V08EnforcementProductControlParentAction>,
}

fn entry(fixture: EntryFixture) -> V08EnforcementProductControlSpineEntry {
    V08EnforcementProductControlSpineEntry {
        schema_version: crate::policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        entry_id: fixture.entry_id.to_string(),
        surface: fixture.surface,
        surface_kind: fixture.surface_kind,
        platform: ParentPlatform::Windows,
        capability: fixture.capability,
        capability_status: V08EnforcementProductControlCapabilityStatus::ManualRequired,
        product_claim_state: fixture.product_claim_state,
        adapter_execution_state: fixture.adapter_execution_state,
        device_policy_state: fixture.device_policy_state,
        parent_visible_actions: fixture.parent_visible_actions,
        linked_proof_commands: Vec::new(),
        linked_proof_artifacts: Vec::new(),
        manual_proof_requirements: vec![spine::REQUIREMENT_ROLLBACK.to_string()],
        claim_boundary: spine::REQUIREMENT_AUDIT_CUSTODY.to_string(),
        fallback_behavior: spine::REQUIREMENT_ROLLBACK.to_string(),
        broad_app_blocking_claimed: false,
        network_domain_blocking_claimed: false,
        managed_exact_url_blocking_claimed: false,
        unmanaged_exact_url_claimed: false,
        tamper_resistance_claimed: false,
        notification_delivery_claimed: false,
        last_checked_at: crate::policy_constants::TEST_EVALUATED_AT.to_string(),
    }
}
