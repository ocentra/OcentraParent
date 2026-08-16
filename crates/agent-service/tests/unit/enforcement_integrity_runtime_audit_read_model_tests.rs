use std::collections::BTreeMap;
use std::primitive::str as TestStr;
use std::string::String as TestString;

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
use ocentra_parent_agent_service::test_support::handle_local_command_text_for_test;

use super::enforcement_api::enforcement_integrity_runtime_audit_read_model::{
    v08_enforcement_integrity_runtime_audit_read_model, GeneratedAtTextRef,
};
use crate::test_invariants::require_some;

type TestResult = Result<(), TestString>;

#[test]
fn enforcement_integrity_runtime_audit_read_model_covers_required_states() {
    let read_model = v08_enforcement_integrity_runtime_audit_read_model(GeneratedAtTextRef(
        policy_constants::TEST_EVALUATED_AT,
    ));
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
    for (result, expected_count) in [
        (V08EnforcementIntegrityRuntimeAuditResult::Succeeded, 1),
        (V08EnforcementIntegrityRuntimeAuditResult::Failed, 2),
        (V08EnforcementIntegrityRuntimeAuditResult::Unavailable, 3),
        (V08EnforcementIntegrityRuntimeAuditResult::Expired, 1),
        (V08EnforcementIntegrityRuntimeAuditResult::RolledBack, 1),
        (V08EnforcementIntegrityRuntimeAuditResult::Superseded, 1),
        (V08EnforcementIntegrityRuntimeAuditResult::NoOp, 1),
        (V08EnforcementIntegrityRuntimeAuditResult::ManualRequired, 2),
        (V08EnforcementIntegrityRuntimeAuditResult::Unsupported, 1),
        (V08EnforcementIntegrityRuntimeAuditResult::ObserveOnly, 1),
    ] {
        assert_eq!(result_count(&result_counts, result), expected_count);
    }
    for (state, expected_count) in [
        (
            V08EnforcementIntegrityRuntimeAuditIntegrityState::Running,
            8,
        ),
        (
            V08EnforcementIntegrityRuntimeAuditIntegrityState::PermissionMissing,
            1,
        ),
        (
            V08EnforcementIntegrityRuntimeAuditIntegrityState::AdapterUnavailable,
            1,
        ),
        (
            V08EnforcementIntegrityRuntimeAuditIntegrityState::StaleHeartbeat,
            1,
        ),
        (
            V08EnforcementIntegrityRuntimeAuditIntegrityState::TamperSignalManualRequired,
            1,
        ),
    ] {
        assert_eq!(integrity_count(&integrity_counts, state), expected_count);
    }
}

#[test]
fn enforcement_integrity_runtime_audit_preserves_no_claim_flags() {
    let read_model = v08_enforcement_integrity_runtime_audit_read_model(GeneratedAtTextRef(
        policy_constants::TEST_EVALUATED_AT,
    ));

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

async fn send_supported_adapter_runtime_proof_command() -> Result<AgentEventEnvelope, TestString> {
    let body = ok(
        serde_json::to_string(&command_envelope()),
        constants::error::AGENT_EVENT_SERIALIZES,
    )?;
    Ok(handle_local_command_text_for_test(crate::test_text::TestText::from_display(body)).await)
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
    audit_entry_id: &TestStr,
    integrity_state: V08EnforcementIntegrityRuntimeAuditIntegrityState,
) {
    let entry = require_some(
        entries
            .iter()
            .find(|candidate| candidate.audit_entry_id == audit_entry_id),
        proof::READ_MODEL_ID,
    );

    assert_eq!(entry.integrity_state, integrity_state);
}

fn count_results(
    entries: &[V08EnforcementIntegrityRuntimeAuditEntry],
) -> BTreeMap<V08EnforcementIntegrityRuntimeAuditResult, usize> {
    count_by(entries, |entry| entry.result)
}

fn count_integrity_states(
    entries: &[V08EnforcementIntegrityRuntimeAuditEntry],
) -> BTreeMap<V08EnforcementIntegrityRuntimeAuditIntegrityState, usize> {
    count_by(entries, |entry| entry.integrity_state)
}

fn count_by<TEntry, TKey>(
    entries: &[TEntry],
    key_for: impl Fn(&TEntry) -> TKey,
) -> BTreeMap<TKey, usize>
where
    TKey: Copy + Ord,
{
    entries.iter().fold(BTreeMap::new(), |mut counts, entry| {
        *counts.entry(key_for(entry)).or_default() += 1;
        counts
    })
}

fn result_count(
    counts: &BTreeMap<V08EnforcementIntegrityRuntimeAuditResult, usize>,
    result: V08EnforcementIntegrityRuntimeAuditResult,
) -> usize {
    *counts.get(&result).unwrap_or(&0)
}

fn integrity_count(
    counts: &BTreeMap<V08EnforcementIntegrityRuntimeAuditIntegrityState, usize>,
    state: V08EnforcementIntegrityRuntimeAuditIntegrityState,
) -> usize {
    *counts.get(&state).unwrap_or(&0)
}

fn string_payload_field<'a>(
    event: &'a AgentEventEnvelope,
    field: &TestStr,
) -> Result<&'a TestStr, TestString> {
    match event.payload.get(field) {
        Some(LogFieldValue::String(value)) => Ok(value.as_str()),
        _ => Err(constants::error::AGENT_EVENT_SERIALIZES.to_string()),
    }
}

fn ok<T, E: std::fmt::Debug>(result: Result<T, E>, context: &TestStr) -> Result<T, TestString> {
    result.map_err(|error| format!("{context}: {error:?}"))
}
