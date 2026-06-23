use std::collections::BTreeMap;

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::constants::v08_enforcement_integrity_runtime_audit as proof;
use ocentra_parent_agent_protocol::constants::v08_integrity_alert_status_bridge as bridge;
use ocentra_parent_agent_protocol::constants::v08_notification_provider_status_boundary as boundary;
use ocentra_parent_agent_protocol::enforcement_integrity_runtime_audit::V08EnforcementIntegrityRuntimeAuditEntry;
use ocentra_parent_agent_protocol::enforcement_integrity_runtime_audit::V08EnforcementIntegrityRuntimeAuditIntegrityState;
use ocentra_parent_agent_protocol::enforcement_integrity_runtime_audit::V08EnforcementIntegrityRuntimeAuditReadModel;
use ocentra_parent_agent_protocol::enforcement_integrity_runtime_audit::V08EnforcementIntegrityRuntimeAuditResult;
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

use super::enforcement_integrity_runtime_audit_read_model::v08_enforcement_integrity_runtime_audit_read_model;
use crate::{lan_pairing::LanPairingRuntime, websocket::handle_command_text_for_test};

type TestResult = Result<(), String>;

#[test]
fn enforcement_integrity_runtime_audit_read_model_covers_required_states() {
    let read_model =
        v08_enforcement_integrity_runtime_audit_read_model(policy_constants::TEST_EVALUATED_AT);
    let result_counts = count_results(&read_model.entries);
    let integrity_counts = count_integrity_states(&read_model.entries);

    assert_eq!(read_model.read_model_id, proof::READ_MODEL_ID);
    assert_eq!(read_model.entries.len(), 14);
    assert_eq!(
        read_model.integrity_alert_status_bridge.read_model_id,
        bridge::READ_MODEL_ID
    );
    assert_eq!(read_model.integrity_alert_status_bridge.entries.len(), 4);
    assert_eq!(
        read_model
            .notification_provider_status_boundary
            .read_model_id,
        boundary::READ_MODEL_ID
    );
    assert_eq!(
        read_model
            .notification_provider_status_boundary
            .entries
            .len(),
        5
    );
    assert_eq!(result_count(&result_counts, proof::RESULT_SUCCEEDED), 1);
    assert_eq!(result_count(&result_counts, proof::RESULT_FAILED), 2);
    assert_eq!(result_count(&result_counts, proof::RESULT_UNAVAILABLE), 3);
    assert_eq!(result_count(&result_counts, proof::RESULT_EXPIRED), 1);
    assert_eq!(result_count(&result_counts, proof::RESULT_ROLLED_BACK), 1);
    assert_eq!(result_count(&result_counts, proof::RESULT_SUPERSEDED), 1);
    assert_eq!(result_count(&result_counts, proof::RESULT_NO_OP), 1);
    assert_eq!(
        result_count(&result_counts, proof::RESULT_MANUAL_REQUIRED),
        2
    );
    assert_eq!(result_count(&result_counts, proof::RESULT_UNSUPPORTED), 1);
    assert_eq!(result_count(&result_counts, proof::RESULT_OBSERVE_ONLY), 1);
    assert_eq!(
        integrity_count(&integrity_counts, proof::INTEGRITY_RUNNING),
        8
    );
    assert_eq!(
        integrity_count(&integrity_counts, proof::INTEGRITY_PERMISSION_MISSING),
        1
    );
    assert_eq!(
        integrity_count(&integrity_counts, proof::INTEGRITY_ADAPTER_UNAVAILABLE),
        1
    );
    assert_eq!(
        integrity_count(&integrity_counts, proof::INTEGRITY_STALE_HEARTBEAT),
        1
    );
    assert_eq!(
        integrity_count(
            &integrity_counts,
            proof::INTEGRITY_TAMPER_SIGNAL_MANUAL_REQUIRED
        ),
        1
    );
}

#[test]
fn enforcement_integrity_runtime_audit_preserves_no_claim_flags() {
    let read_model =
        v08_enforcement_integrity_runtime_audit_read_model(policy_constants::TEST_EVALUATED_AT);

    assert!(read_model
        .entries
        .iter()
        .all(|entry| !entry.broad_installed_app_blocking_claimed));
    assert!(read_model
        .entries
        .iter()
        .all(|entry| !entry.host_network_domain_blocking_claimed));
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
        .all(|entry| !entry.mobile_privilege_claimed));
    assert!(read_model
        .entries
        .iter()
        .all(|entry| !entry.stealth_persistence_claimed));
    assert!(read_model
        .entries
        .iter()
        .all(|entry| !entry.privilege_escalation_claimed));
    assert!(read_model
        .integrity_alert_status_bridge
        .entries
        .iter()
        .all(|entry| !entry.provider_delivery_claimed));
    assert!(read_model
        .integrity_alert_status_bridge
        .entries
        .iter()
        .all(|entry| !entry.tamper_resistance_claimed));
    assert!(read_model
        .notification_provider_status_boundary
        .entries
        .iter()
        .all(|entry| !entry.provider_delivery_observed));
    assert!(read_model
        .notification_provider_status_boundary
        .entries
        .iter()
        .all(|entry| !entry.delivered_notification_claimed));
    assert!(read_model
        .notification_provider_status_boundary
        .entries
        .iter()
        .any(|entry| entry.status_entry_id == boundary::ENTRY_DELIVERED));
}

#[tokio::test]
async fn supported_adapter_runtime_websocket_event_includes_integrity_audit_read_model(
) -> TestResult {
    let event = send_supported_adapter_runtime_proof_command().await?;

    assert_eq!(
        event.event,
        AgentEventName::AgentEnforcementSupportedAdapterRuntimeProofReported
    );
    let read_model: V08EnforcementIntegrityRuntimeAuditReadModel = ok(
        serde_json::from_str(string_payload_field(
            &event,
            constants::field::ENFORCEMENT_INTEGRITY_RUNTIME_AUDIT_READ_MODEL,
        )?),
        constants::error::AGENT_EVENT_SERIALIZES,
    )?;

    assert_eq!(read_model.read_model_id, proof::READ_MODEL_ID);
    assert_eq!(read_model.entries.len(), 14);
    assert_eq!(
        read_model.integrity_alert_status_bridge.read_model_id,
        bridge::READ_MODEL_ID
    );
    assert_eq!(read_model.integrity_alert_status_bridge.entries.len(), 4);
    assert_eq!(
        read_model
            .notification_provider_status_boundary
            .read_model_id,
        boundary::READ_MODEL_ID
    );
    assert_eq!(
        read_model
            .notification_provider_status_boundary
            .entries
            .len(),
        5
    );
    assert_entry_integrity(
        &read_model.entries,
        proof::ENTRY_PERMISSION_LOSS,
        V08EnforcementIntegrityRuntimeAuditIntegrityState::PermissionMissing,
    );
    assert_entry_integrity(
        &read_model.entries,
        proof::ENTRY_STALE_HEARTBEAT,
        V08EnforcementIntegrityRuntimeAuditIntegrityState::StaleHeartbeat,
    );
    assert!(read_model
        .source_read_model_ids
        .contains(&proof::SOURCE_TIMER_RECOVERY_STATE.to_string()));
    assert!(read_model
        .integrity_alert_status_bridge
        .entries
        .iter()
        .any(|entry| entry.bridge_entry_id == bridge::ENTRY_STOPPED_OR_REMOVED));
    assert!(read_model
        .notification_provider_status_boundary
        .entries
        .iter()
        .any(|entry| entry.status_entry_id == boundary::ENTRY_MANUAL_REQUIRED));

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

fn assert_entry_integrity(
    entries: &[V08EnforcementIntegrityRuntimeAuditEntry],
    audit_entry_id: &str,
    integrity_state: V08EnforcementIntegrityRuntimeAuditIntegrityState,
) {
    let entry = entries
        .iter()
        .find(|candidate| candidate.audit_entry_id == audit_entry_id)
        .unwrap_or_else(|| panic!("{}", proof::READ_MODEL_ID));

    assert_eq!(entry.integrity_state, integrity_state);
}

fn count_results(
    entries: &[V08EnforcementIntegrityRuntimeAuditEntry],
) -> BTreeMap<&'static str, usize> {
    entries.iter().fold(BTreeMap::new(), |mut counts, entry| {
        *counts.entry(result_name(entry.result)).or_default() += 1;
        counts
    })
}

fn count_integrity_states(
    entries: &[V08EnforcementIntegrityRuntimeAuditEntry],
) -> BTreeMap<&'static str, usize> {
    entries.iter().fold(BTreeMap::new(), |mut counts, entry| {
        *counts
            .entry(integrity_name(entry.integrity_state))
            .or_default() += 1;
        counts
    })
}

fn result_name(result: V08EnforcementIntegrityRuntimeAuditResult) -> &'static str {
    match result {
        V08EnforcementIntegrityRuntimeAuditResult::Succeeded => proof::RESULT_SUCCEEDED,
        V08EnforcementIntegrityRuntimeAuditResult::Failed => proof::RESULT_FAILED,
        V08EnforcementIntegrityRuntimeAuditResult::Unavailable => proof::RESULT_UNAVAILABLE,
        V08EnforcementIntegrityRuntimeAuditResult::Expired => proof::RESULT_EXPIRED,
        V08EnforcementIntegrityRuntimeAuditResult::RolledBack => proof::RESULT_ROLLED_BACK,
        V08EnforcementIntegrityRuntimeAuditResult::Superseded => proof::RESULT_SUPERSEDED,
        V08EnforcementIntegrityRuntimeAuditResult::NoOp => proof::RESULT_NO_OP,
        V08EnforcementIntegrityRuntimeAuditResult::ManualRequired => proof::RESULT_MANUAL_REQUIRED,
        V08EnforcementIntegrityRuntimeAuditResult::Unsupported => proof::RESULT_UNSUPPORTED,
        V08EnforcementIntegrityRuntimeAuditResult::ObserveOnly => proof::RESULT_OBSERVE_ONLY,
    }
}

fn integrity_name(state: V08EnforcementIntegrityRuntimeAuditIntegrityState) -> &'static str {
    match state {
        V08EnforcementIntegrityRuntimeAuditIntegrityState::Running => proof::INTEGRITY_RUNNING,
        V08EnforcementIntegrityRuntimeAuditIntegrityState::PermissionMissing => {
            proof::INTEGRITY_PERMISSION_MISSING
        }
        V08EnforcementIntegrityRuntimeAuditIntegrityState::AdapterUnavailable => {
            proof::INTEGRITY_ADAPTER_UNAVAILABLE
        }
        V08EnforcementIntegrityRuntimeAuditIntegrityState::StaleHeartbeat => {
            proof::INTEGRITY_STALE_HEARTBEAT
        }
        V08EnforcementIntegrityRuntimeAuditIntegrityState::ServiceStopped => {
            proof::INTEGRITY_SERVICE_STOPPED
        }
        V08EnforcementIntegrityRuntimeAuditIntegrityState::UninstallDetectionManualRequired => {
            proof::INTEGRITY_UNINSTALL_DETECTION_MANUAL_REQUIRED
        }
        V08EnforcementIntegrityRuntimeAuditIntegrityState::TamperSignalManualRequired => {
            proof::INTEGRITY_TAMPER_SIGNAL_MANUAL_REQUIRED
        }
        V08EnforcementIntegrityRuntimeAuditIntegrityState::AntiTamperNotClaimed => {
            proof::INTEGRITY_ANTI_TAMPER_NOT_CLAIMED
        }
        V08EnforcementIntegrityRuntimeAuditIntegrityState::NotApplicable => {
            proof::INTEGRITY_NOT_APPLICABLE
        }
    }
}

fn result_count(counts: &BTreeMap<&'static str, usize>, result: &'static str) -> usize {
    *counts.get(result).unwrap_or(&0)
}

fn integrity_count(counts: &BTreeMap<&'static str, usize>, state: &'static str) -> usize {
    *counts.get(state).unwrap_or(&0)
}

fn string_payload_field<'a>(event: &'a AgentEventEnvelope, field: &str) -> Result<&'a str, String> {
    match event.payload.get(field) {
        Some(LogFieldValue::String(value)) => Ok(value.as_str()),
        _ => Err(constants::error::AGENT_EVENT_SERIALIZES.to_string()),
    }
}

fn ok<T, E: std::fmt::Debug>(result: Result<T, E>, context: &str) -> Result<T, String> {
    result.map_err(|error| format!("{context}: {error:?}"))
}
