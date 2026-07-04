use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::DeviceRoleRuntimeReadModel;
use ocentra_parent_agent_protocol::lan_pairing::DeviceRuntimeAiProviderState;
use ocentra_parent_agent_protocol::lan_pairing::DeviceRuntimeLocalAiClaim;
use ocentra_parent_agent_protocol::lan_pairing::DeviceRuntimeRole;
use ocentra_parent_agent_protocol::lan_pairing::DeviceRuntimeRoleEntry;
use ocentra_parent_agent_protocol::lan_pairing::DeviceRuntimeRoleState;
use ocentra_parent_agent_protocol::lan_pairing::DeviceRuntimeRouteState;
use ocentra_parent_agent_protocol::lan_pairing::DeviceRuntimeSurface;
use ocentra_parent_agent_protocol::lan_pairing::LanAiProviderRoutingState;
use ocentra_parent_agent_protocol::lan_pairing_authority::LanPairingParentAuthority;
use ocentra_parent_agent_protocol::lan_pairing_provider_selection::LanProviderSelectionCloudRelayDecisionState;
use ocentra_parent_agent_protocol::lan_pairing_provider_selection::LanProviderSelectionCloudRelayImplementationState;
use ocentra_parent_agent_protocol::lan_pairing_provider_selection::LanProviderSelectionLifecycleState;
use ocentra_parent_agent_protocol::lan_pairing_provider_selection::LanProviderSelectionManualRequirement;
use ocentra_parent_agent_protocol::lan_pairing_provider_selection::LanProviderSelectionPolicyDecision;
use ocentra_parent_agent_protocol::lan_pairing_provider_selection::LanProviderSelectionProofState;

use crate::{
    app::lan_pairing::LanPairingRuntime,
    lan_pairing_provider_selection_read_model::provider_selection_read_model,
    lan_pairing_test_commands::paired_runtime,
};

#[tokio::test]
async fn provider_selection_read_model_selects_authorized_local_provider_without_product_claim() {
    let runtime = lan_ai_provider_runtime().await;
    let read_model = provider_selection_read_model(&runtime);

    assert_eq!(
        read_model.selected_provider_route_id.as_deref(),
        Some(constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK)
    );
    assert_eq!(
        read_model.authorized_provider_selection_state,
        LanProviderSelectionProofState::CiMechanicalProof
    );
    assert_eq!(
        read_model.physical_household_provider_proof_state,
        LanProviderSelectionProofState::ManualRequired
    );
    assert_eq!(
        read_model.cloud_relay_implementation_state,
        LanProviderSelectionCloudRelayImplementationState::NotImplemented
    );
    assert_eq!(
        read_model.cloud_relay_decision_state,
        LanProviderSelectionCloudRelayDecisionState::ManualDecisionRequired
    );
    assert!(read_model
        .candidates
        .iter()
        .any(|candidate| candidate.lifecycle_state
            == LanProviderSelectionLifecycleState::CandidateSelected
            && candidate.routing_state == LanAiProviderRoutingState::AuthorizedResult
            && candidate.policy_decision
                == LanProviderSelectionPolicyDecision::SelectAuthorizedProvider));
    assert!(read_model
        .candidates
        .iter()
        .any(|candidate| candidate.routing_state
            == LanAiProviderRoutingState::UnsupportedCapability
            && candidate.policy_decision
                == LanProviderSelectionPolicyDecision::RefuseUnsupportedCapability));
    assert!(read_model
        .manual_requirements
        .iter()
        .any(|requirement| requirement.requirement
            == LanProviderSelectionManualRequirement::PhysicalHouseholdProviderHost));
}

#[test]
fn provider_selection_read_model_refuses_unpaired_or_missing_provider_routes() {
    let runtime = LanPairingRuntime::empty();
    let read_model = provider_selection_read_model(&runtime);

    assert_eq!(read_model.selected_provider_route_id, None);
    assert!(read_model
        .candidates
        .iter()
        .any(|candidate| candidate.lifecycle_state
            == LanProviderSelectionLifecycleState::CandidateUnavailable
            && candidate.policy_decision
                == LanProviderSelectionPolicyDecision::RefuseUnpairedProvider));
    assert!(read_model
        .candidates
        .iter()
        .any(|candidate| candidate.lifecycle_state
            == LanProviderSelectionLifecycleState::NotImplemented
            && candidate.policy_decision
                == LanProviderSelectionPolicyDecision::RequireCloudRelayDecision));
}

#[tokio::test]
async fn provider_selection_read_model_marks_stale_selected_provider_route_blocked() {
    let runtime = lan_ai_provider_runtime().await;
    assert!(runtime.mark_selected_stale_for_test());
    let read_model = provider_selection_read_model(&runtime);

    assert_eq!(read_model.selected_provider_route_id, None);
    assert!(read_model
        .candidates
        .iter()
        .any(|candidate| candidate.policy_decision
            == LanProviderSelectionPolicyDecision::RefuseRouteBlockedProvider
            && candidate.rejection_reason.is_some()));
}

#[tokio::test]
async fn provider_selection_read_model_degrades_stale_provider_heartbeat() {
    let runtime = lan_ai_provider_runtime().await;
    runtime.mark_lan_ai_provider_heartbeat_stale_for_test();
    let read_model = provider_selection_read_model(&runtime);

    assert_eq!(read_model.selected_provider_route_id, None);
    assert!(read_model.candidates.iter().any(|candidate| {
        candidate.lifecycle_state == LanProviderSelectionLifecycleState::CandidateDegraded
            && candidate.routing_state == LanAiProviderRoutingState::Degraded
            && candidate.policy_decision
                == LanProviderSelectionPolicyDecision::DegradeProviderUnavailable
    }));
}

async fn lan_ai_provider_runtime() -> LanPairingRuntime {
    let mut runtime = paired_runtime().await;
    runtime.device_roles = DeviceRoleRuntimeReadModel {
        schema_version: constants::lan_pairing::SCHEMA_VERSION_TEXT
            .to_string()
            .into(),
        physical_device_id: constants::local_ai_runtime::PHYSICAL_DEVICE_LOCAL.to_string(),
        surface: DeviceRuntimeSurface::ParentDesktop,
        platform: constants::local_ai_runtime::PLATFORM_OS_WINDOWS.to_string(),
        roles: vec![
            role_entry(DeviceRuntimeRole::ParentController),
            role_entry(DeviceRuntimeRole::ChildAgent),
            role_entry(DeviceRuntimeRole::AiProvider),
        ],
        primary_role: DeviceRuntimeRole::ParentController,
        controller_lease_id: Some(constants::lan_pairing::CONTROLLER_LEASE_ID.to_string()),
        parent_authority: Some(LanPairingParentAuthority::ActiveController),
        selected_route_id: Some(constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string()),
        route_state: DeviceRuntimeRouteState::LocalNetwork,
        lan_ai_provider_state: DeviceRuntimeAiProviderState::Available,
        local_ai_runtime_claim: DeviceRuntimeLocalAiClaim::SharedPhysicalDeviceSingleton,
        updated_at: constants::local_ai_runtime::TEST_CHECKED_AT.to_string(),
    };
    runtime.lan_ai_provider_capabilities =
        vec![constants::local_ai_runtime::CAPABILITY_CHAT_COMPLETION.to_string()];
    runtime
}

fn role_entry(role: DeviceRuntimeRole) -> DeviceRuntimeRoleEntry {
    DeviceRuntimeRoleEntry {
        role,
        state: DeviceRuntimeRoleState::Implemented,
    }
}
