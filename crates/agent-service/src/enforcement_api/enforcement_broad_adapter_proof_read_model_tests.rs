use std::collections::BTreeMap;

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::constants::enforcement_broad_adapter_proof as proof;
use ocentra_parent_agent_protocol::enforcement::ParentPlatform;
use ocentra_parent_agent_protocol::enforcement_broad_adapter_proof::V08BroadAdapterRuntimeClaimState;
use ocentra_parent_agent_protocol::enforcement_broad_adapter_proof::V08BroadAdapterRuntimeProofEntry;
use ocentra_parent_agent_protocol::enforcement_broad_adapter_proof::V08BroadAdapterRuntimeProofReadModel;
use ocentra_parent_agent_protocol::enforcement_broad_adapter_proof::V08BroadAdapterRuntimeSurface;
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

use super::enforcement_broad_adapter_proof_read_model::v08_broad_adapter_proof_read_model;
use crate::{lan_pairing::LanPairingRuntime, websocket::handle_command_text_for_test};

type TestResult = Result<(), String>;

#[test]
fn broad_adapter_proof_read_model_preserves_honest_runtime_states() {
    let read_model = v08_broad_adapter_proof_read_model(policy_constants::TEST_EVALUATED_AT);
    let claim_counts = count_claims(&read_model.entries);
    let platform_counts = count_platforms(&read_model.entries);

    assert_eq!(read_model.read_model_id, proof::READ_MODEL_ID);
    assert_eq!(read_model.entries.len(), 10);
    assert_eq!(
        claim_count(&claim_counts, proof::CLAIM_IMPLEMENTED_BOUNDARY),
        2
    );
    assert_eq!(claim_count(&claim_counts, proof::CLAIM_MANUAL_REQUIRED), 6);
    assert_eq!(claim_count(&claim_counts, proof::CLAIM_UNAVAILABLE), 1);
    assert_eq!(claim_count(&claim_counts, proof::CLAIM_NOT_CLAIMED), 1);
    assert_eq!(
        platform_count(
            &platform_counts,
            policy_constants::TEST_PARENT_DEVICE_PLATFORM_WINDOWS
        ),
        6
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
        .contains(&proof::SOURCE_BROWSER_DOMAIN_ADAPTER_PROOF.to_string()));
    assert!(read_model
        .source_read_model_ids
        .contains(&proof::SOURCE_OS_ADAPTER_MANUAL_ARTIFACT_GATES.to_string()));
}

#[test]
fn broad_adapter_proof_read_model_keeps_surface_outcomes_exact() {
    let read_model = v08_broad_adapter_proof_read_model(policy_constants::TEST_EVALUATED_AT);

    assert_surface_state(
        &read_model.entries,
        V08BroadAdapterRuntimeSurface::WindowsOwnedProcessAndTimerRuntimeBoundary,
        V08BroadAdapterRuntimeClaimState::ImplementedBoundary,
    );
    assert_surface_state(
        &read_model.entries,
        V08BroadAdapterRuntimeSurface::WindowsManagedBrowserSessionRuntimeBoundary,
        V08BroadAdapterRuntimeClaimState::ImplementedBoundary,
    );
    assert_surface_state(
        &read_model.entries,
        V08BroadAdapterRuntimeSurface::WindowsNetworkDomainRuntimeGate,
        V08BroadAdapterRuntimeClaimState::ManualRequired,
    );
    assert_surface_state(
        &read_model.entries,
        V08BroadAdapterRuntimeSurface::WindowsUnmanagedBrowserExactEvidenceRuntimeGap,
        V08BroadAdapterRuntimeClaimState::NotClaimed,
    );
    assert_surface_state(
        &read_model.entries,
        V08BroadAdapterRuntimeSurface::LinuxHostRuntimeUnavailable,
        V08BroadAdapterRuntimeClaimState::Unavailable,
    );
}

#[test]
fn broad_adapter_proof_read_model_does_not_upgrade_claim_flags() {
    let read_model = v08_broad_adapter_proof_read_model(policy_constants::TEST_EVALUATED_AT);

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
        .all(|entry| !entry.managed_browser_exact_url_claimed));
    assert!(read_model
        .entries
        .iter()
        .all(|entry| !entry.unmanaged_browser_exact_evidence_claimed));
    assert!(read_model
        .entries
        .iter()
        .all(|entry| !entry.unsupported_platform_claimed));
    assert!(read_model
        .entries
        .iter()
        .all(|entry| !entry.mobile_privilege_claimed));
}

#[tokio::test]
async fn broad_adapter_proof_websocket_command_returns_service_read_model() -> TestResult {
    let event = send_broad_adapter_proof_command().await?;

    assert_eq!(
        event.event,
        AgentEventName::AgentEnforcementBroadAdapterProofReported
    );
    assert_eq!(
        string_payload_field(&event, constants::field::READ_MODEL_ID)?,
        proof::READ_MODEL_ID
    );
    assert_eq!(
        number_payload_field(&event, constants::field::RETURNED)?,
        10.0
    );

    let read_model: V08BroadAdapterRuntimeProofReadModel = ok(
        serde_json::from_str(string_payload_field(
            &event,
            constants::field::ENFORCEMENT_BROAD_ADAPTER_PROOF_READ_MODEL,
        )?),
        constants::error::AGENT_EVENT_SERIALIZES,
    )?;
    let managed_exact = entry_for(
        &read_model.entries,
        V08BroadAdapterRuntimeSurface::WindowsManagedBrowserExactUrlRuntimeGate,
    );

    assert_eq!(read_model.read_model_id, proof::READ_MODEL_ID);
    assert_eq!(read_model.entries.len(), 10);
    assert_eq!(
        managed_exact.product_claim_state,
        V08BroadAdapterRuntimeClaimState::ManualRequired
    );
    assert!(managed_exact
        .manual_proof_requirements
        .contains(&proof::REQUIREMENT_EXACT_URL_APPLY.to_string()));

    Ok(())
}

async fn send_broad_adapter_proof_command() -> Result<AgentEventEnvelope, String> {
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
        command: AgentCommandName::AgentEnforcementBroadAdapterProofGet,
        payload: LogFields::new(),
    }
}

fn entry_for(
    entries: &[V08BroadAdapterRuntimeProofEntry],
    surface: V08BroadAdapterRuntimeSurface,
) -> &V08BroadAdapterRuntimeProofEntry {
    entries
        .iter()
        .find(|entry| entry.runtime_surface == surface)
        .unwrap_or_else(|| panic!("{}", proof::READ_MODEL_ID))
}

fn assert_surface_state(
    entries: &[V08BroadAdapterRuntimeProofEntry],
    surface: V08BroadAdapterRuntimeSurface,
    product_claim_state: V08BroadAdapterRuntimeClaimState,
) {
    let entry = entry_for(entries, surface);

    assert_eq!(entry.product_claim_state, product_claim_state);
}

fn count_claims(entries: &[V08BroadAdapterRuntimeProofEntry]) -> BTreeMap<&'static str, usize> {
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

fn count_platforms(entries: &[V08BroadAdapterRuntimeProofEntry]) -> BTreeMap<&'static str, usize> {
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
