use std::collections::BTreeMap;

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::constants::v08_supported_adapter_runtime_proof as proof;
use ocentra_parent_agent_protocol::enforcement::ParentPlatform;
use ocentra_parent_agent_protocol::enforcement_supported_adapter_runtime_proof::V08SupportedAdapterRuntimeBoundary;
use ocentra_parent_agent_protocol::enforcement_supported_adapter_runtime_proof::V08SupportedAdapterRuntimeProofEntry;
use ocentra_parent_agent_protocol::enforcement_supported_adapter_runtime_proof::V08SupportedAdapterRuntimeProofReadModel;
use ocentra_parent_agent_protocol::enforcement_supported_adapter_runtime_proof::V08SupportedAdapterRuntimeState;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::logging::LogFields;
use ocentra_parent_agent_protocol::policy_constants;
use ocentra_parent_agent_protocol::transport::AgentCommandEnvelope;
use ocentra_parent_agent_protocol::transport::AgentCommandName;
use ocentra_parent_agent_protocol::transport::AgentEventEnvelope;
use ocentra_parent_agent_protocol::transport::AgentEventName;
use ocentra_parent_agent_protocol::transport::AgentMessageTarget;
use ocentra_parent_agent_protocol::transport::AgentPeer;
use ocentra_parent_agent_protocol::transport::AgentPeerRole;
use ocentra_parent_agent_protocol::transport::AgentRoute;
use ocentra_parent_agent_protocol::AGENT_PROTOCOL_SCHEMA_VERSION;

use super::enforcement_supported_adapter_runtime_proof_read_model::v08_supported_adapter_runtime_proof_read_model;
use crate::{lan_pairing::LanPairingRuntime, websocket::handle_command_text_for_test};

type TestResult = Result<(), String>;

#[test]
fn supported_adapter_runtime_proof_read_model_preserves_honest_states() {
    let read_model =
        v08_supported_adapter_runtime_proof_read_model(policy_constants::TEST_EVALUATED_AT);
    let state_counts = count_states(&read_model.entries);
    let platform_counts = count_platforms(&read_model.entries);

    assert_eq!(read_model.read_model_id, proof::READ_MODEL_ID);
    assert_eq!(read_model.entries.len(), 13);
    assert_eq!(
        state_count(&state_counts, proof::STATE_IMPLEMENTED_BOUNDARY),
        2
    );
    assert_eq!(state_count(&state_counts, proof::STATE_MANUAL_REQUIRED), 7);
    assert_eq!(state_count(&state_counts, proof::STATE_NOT_CLAIMED), 1);
    assert_eq!(state_count(&state_counts, proof::STATE_DEGRADED), 1);
    assert_eq!(state_count(&state_counts, proof::STATE_UNAVAILABLE), 1);
    assert_eq!(state_count(&state_counts, proof::STATE_UNSUPPORTED), 1);
    assert_eq!(
        platform_count(
            &platform_counts,
            policy_constants::TEST_PARENT_DEVICE_PLATFORM_WINDOWS
        ),
        9
    );
    assert_eq!(
        platform_count(&platform_counts, ParentPlatform::Linux.as_protocol_str()),
        1
    );
    assert_eq!(
        platform_count(&platform_counts, ParentPlatform::Macos.as_protocol_str()),
        1
    );
    assert_eq!(
        platform_count(&platform_counts, ParentPlatform::Android.as_protocol_str()),
        1
    );
    assert_eq!(
        platform_count(&platform_counts, ParentPlatform::Ios.as_protocol_str()),
        1
    );
    assert!(read_model
        .source_read_model_ids
        .contains(&proof::SOURCE_POLICY_DISPATCH_PROOF.to_string()));
    assert!(read_model
        .source_read_model_ids
        .contains(&proof::SOURCE_NETWORK_FLOW_EVIDENCE.to_string()));
    assert!(read_model
        .source_read_model_ids
        .contains(&proof::SOURCE_WINDOWS_ADAPTER_ARTIFACT_INGESTION_PROOF.to_string()));
}

#[test]
fn supported_adapter_runtime_proof_keeps_exact_boundaries() {
    let read_model =
        v08_supported_adapter_runtime_proof_read_model(policy_constants::TEST_EVALUATED_AT);

    assert_boundary_state(
        &read_model.entries,
        V08SupportedAdapterRuntimeBoundary::WindowsAppGameOwnedProcessTimeLimit,
        V08SupportedAdapterRuntimeState::ImplementedBoundary,
    );
    assert_boundary_state(
        &read_model.entries,
        V08SupportedAdapterRuntimeBoundary::WindowsNetworkFlowObservePolicyHandoff,
        V08SupportedAdapterRuntimeState::ImplementedBoundary,
    );
    assert_boundary_state(
        &read_model.entries,
        V08SupportedAdapterRuntimeBoundary::WindowsBroadInstalledAppBlockingManualGate,
        V08SupportedAdapterRuntimeState::ManualRequired,
    );
    assert_boundary_state(
        &read_model.entries,
        V08SupportedAdapterRuntimeBoundary::WindowsManagedExactActiveTabNotClaimed,
        V08SupportedAdapterRuntimeState::NotClaimed,
    );
    assert_boundary_state(
        &read_model.entries,
        V08SupportedAdapterRuntimeBoundary::WindowsBroadInstalledAppArtifactStatus,
        V08SupportedAdapterRuntimeState::ManualRequired,
    );
    assert_boundary_state(
        &read_model.entries,
        V08SupportedAdapterRuntimeBoundary::WindowsHostNetworkDomainArtifactStatus,
        V08SupportedAdapterRuntimeState::ManualRequired,
    );
    assert_boundary_state(
        &read_model.entries,
        V08SupportedAdapterRuntimeBoundary::WindowsManagedBrowserArtifactStatus,
        V08SupportedAdapterRuntimeState::ManualRequired,
    );
    assert_boundary_state(
        &read_model.entries,
        V08SupportedAdapterRuntimeBoundary::LinuxHostAdapterUnavailable,
        V08SupportedAdapterRuntimeState::Unavailable,
    );
    assert_boundary_state(
        &read_model.entries,
        V08SupportedAdapterRuntimeBoundary::MacosHostAdapterUnsupported,
        V08SupportedAdapterRuntimeState::Unsupported,
    );
    assert_boundary_state(
        &read_model.entries,
        V08SupportedAdapterRuntimeBoundary::WindowsAdapterPermissionDependencyDegraded,
        V08SupportedAdapterRuntimeState::Degraded,
    );
}

#[test]
fn supported_adapter_runtime_proof_does_not_upgrade_claim_flags() {
    let read_model =
        v08_supported_adapter_runtime_proof_read_model(policy_constants::TEST_EVALUATED_AT);

    assert!(read_model
        .entries
        .iter()
        .all(|entry| !entry.broad_installed_app_blocking_claimed));
    assert!(read_model
        .entries
        .iter()
        .all(|entry| !entry.network_domain_blocking_claimed));
    assert!(read_model
        .entries
        .iter()
        .all(|entry| !entry.exact_active_tab_enforcement_claimed));
    assert!(read_model
        .entries
        .iter()
        .all(|entry| !entry.notification_delivery_claimed));
    assert!(read_model
        .entries
        .iter()
        .all(|entry| !entry.tamper_hardening_claimed));
    assert!(read_model
        .entries
        .iter()
        .all(|entry| !entry.mobile_control_claimed));
    assert!(read_model
        .entries
        .iter()
        .all(|entry| !entry.unsupported_platform_behavior_claimed));
}

#[tokio::test]
async fn supported_adapter_runtime_proof_websocket_command_returns_service_read_model() -> TestResult
{
    let event = send_supported_adapter_runtime_proof_command().await?;

    assert_eq!(
        event.event,
        AgentEventName::AgentEnforcementSupportedAdapterRuntimeProofReported
    );
    assert_eq!(
        string_payload_field(&event, constants::field::READ_MODEL_ID)?,
        proof::READ_MODEL_ID
    );
    assert_eq!(
        number_payload_field(&event, constants::field::RETURNED)?,
        13.0
    );

    let read_model: V08SupportedAdapterRuntimeProofReadModel = ok(
        serde_json::from_str(string_payload_field(
            &event,
            constants::field::ENFORCEMENT_SUPPORTED_ADAPTER_RUNTIME_PROOF_READ_MODEL,
        )?),
        constants::error::AGENT_EVENT_SERIALIZES,
    )?;
    let host_network = entry_for(
        &read_model.entries,
        V08SupportedAdapterRuntimeBoundary::WindowsHostNetworkDomainBlockingManualGate,
    );

    assert_eq!(read_model.read_model_id, proof::READ_MODEL_ID);
    assert_eq!(read_model.entries.len(), 13);
    assert_eq!(
        host_network.runtime_state,
        V08SupportedAdapterRuntimeState::ManualRequired
    );
    assert!(host_network
        .manual_proof_requirements
        .contains(&proof::REQUIREMENT_HOST_DNS_OR_FILTER_APPLY.to_string()));
    let managed_browser_artifacts = entry_for(
        &read_model.entries,
        V08SupportedAdapterRuntimeBoundary::WindowsManagedBrowserArtifactStatus,
    );
    assert!(managed_browser_artifacts
        .linked_proof_artifacts
        .contains(&proof::ARTIFACT_WINDOWS_ADAPTER_ARTIFACT_INGESTION_PROOF.to_string()));
    assert!(managed_browser_artifacts
        .manual_proof_requirements
        .contains(&proof::REQUIREMENT_MANAGED_BROWSER_EXACT_URL_EVIDENCE.to_string()));

    Ok(())
}

async fn send_supported_adapter_runtime_proof_command() -> Result<AgentEventEnvelope, String> {
    let body = ok(
        serde_json::to_string(&command_envelope()),
        constants::error::AGENT_EVENT_SERIALIZES,
    )?;
    Ok(handle_command_text_for_test(&body, LanPairingRuntime::empty(), None).await)
}

fn command_envelope() -> AgentCommandEnvelope {
    AgentCommandEnvelope {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        message_id: proof::READ_MODEL_ID.to_string(),
        sent_at: policy_constants::TEST_EVALUATED_AT.to_string(),
        source: AgentPeer {
            peer_id: constants::peer::PORTAL_DEV.to_string(),
            role: AgentPeerRole::Portal,
        },
        target: AgentMessageTarget {
            device_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
            platform: policy_constants::TEST_PARENT_DEVICE_PLATFORM_WINDOWS.to_string(),
            route: AgentRoute::Localhost,
        },
        command: AgentCommandName::AgentEnforcementSupportedAdapterRuntimeProofGet,
        payload: LogFields::new(),
    }
}

fn entry_for(
    entries: &[V08SupportedAdapterRuntimeProofEntry],
    boundary: V08SupportedAdapterRuntimeBoundary,
) -> &V08SupportedAdapterRuntimeProofEntry {
    entries
        .iter()
        .find(|entry| entry.runtime_boundary == boundary)
        .unwrap_or_else(|| panic!("{}", proof::READ_MODEL_ID))
}

fn assert_boundary_state(
    entries: &[V08SupportedAdapterRuntimeProofEntry],
    boundary: V08SupportedAdapterRuntimeBoundary,
    runtime_state: V08SupportedAdapterRuntimeState,
) {
    let entry = entry_for(entries, boundary);

    assert_eq!(entry.runtime_state, runtime_state);
}

fn count_states(entries: &[V08SupportedAdapterRuntimeProofEntry]) -> BTreeMap<&'static str, usize> {
    entries.iter().fold(BTreeMap::new(), |mut counts, entry| {
        *counts
            .entry(entry.runtime_state.as_protocol_str())
            .or_default() += 1;
        counts
    })
}

fn state_count(counts: &BTreeMap<&'static str, usize>, state: &'static str) -> usize {
    *counts.get(state).unwrap_or(&0)
}

fn count_platforms(
    entries: &[V08SupportedAdapterRuntimeProofEntry],
) -> BTreeMap<&'static str, usize> {
    entries.iter().fold(BTreeMap::new(), |mut counts, entry| {
        *counts.entry(entry.platform.as_protocol_str()).or_default() += 1;
        counts
    })
}

fn platform_count(counts: &BTreeMap<&'static str, usize>, platform: &str) -> usize {
    *counts.get(platform).unwrap_or(&0)
}

fn string_payload_field<'a>(event: &'a AgentEventEnvelope, field: &str) -> Result<&'a str, String> {
    match event.payload.get(field) {
        Some(LogFieldValue::String(value)) => Ok(value.as_str()),
        _ => Err(constants::error::AGENT_EVENT_SERIALIZES.to_string()),
    }
}

fn number_payload_field(event: &AgentEventEnvelope, field: &str) -> Result<f64, String> {
    match event.payload.get(field) {
        Some(LogFieldValue::Number(value)) => Ok(*value),
        _ => Err(constants::error::AGENT_EVENT_SERIALIZES.to_string()),
    }
}

fn ok<T, E: std::fmt::Debug>(result: Result<T, E>, context: &str) -> Result<T, String> {
    result.map_err(|error| format!("{context}: {error:?}"))
}
