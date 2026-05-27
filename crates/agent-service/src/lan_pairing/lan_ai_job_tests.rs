use ocentra_parent_agent_protocol::{
    constants, AgentCommandName, AgentEventName, LogFieldValue, LogFields,
};

use crate::{
    lan_pairing_test_assertions::assert_rejection_with_audit,
    lan_pairing_test_commands::{
        command_for_target, intent_payload_for_kind, local_network_target, paired_runtime,
        serialize_command,
    },
    websocket::handle_command_text_for_test,
};

#[tokio::test]
async fn lan_ai_provider_status_advertises_capability_to_observer_without_raw_activity() {
    let runtime = paired_runtime().await;
    let event = handle_command_text_for_test(
        &serialize_command(command_for_target(
            AgentCommandName::AgentLanAiProviderStatusGet,
            local_network_target(constants::lan_pairing::CHILD_DEVICE_ID),
            with_parent_authority(
                intent_payload_for_kind(
                    constants::lan_pairing::LAN_AI_PROVIDER_STATUS_INTENT_ID,
                    constants::lan_pairing::CHILD_DEVICE_ID,
                    constants::lan_pairing::PROOF_DIGEST,
                    constants::lan_pairing::EXPIRES_AT,
                    constants::value::LAN_INTENT_LAN_AI_PROVIDER_STATUS,
                ),
                constants::value::LAN_PARENT_AUTHORITY_OBSERVER,
            ),
        )),
        runtime,
        Some(constants::lan_pairing::ALLOWED_ORIGIN.to_string()),
    )
    .await;

    assert_eq!(event.event, AgentEventName::AgentLanPairingStatusReported);
    assert_eq!(
        event.payload.get(constants::field::LAN_AUDIT_EVENT_TYPE),
        Some(&LogFieldValue::String(
            constants::value::LAN_AUDIT_LAN_AI_PROVIDER_ADVERTISED.to_string()
        ))
    );
    assert_eq!(
        event.payload.get(constants::field::LAN_PARENT_AUTHORITY),
        Some(&LogFieldValue::String(
            constants::value::LAN_PARENT_AUTHORITY_OBSERVER.to_string()
        ))
    );
    assert_eq!(
        event.payload.get(constants::field::LAN_AI_PROVIDER_STATUS),
        Some(&LogFieldValue::String(
            constants::value::LAN_AI_PROVIDER_STATUS_UNAVAILABLE.to_string()
        ))
    );
    assert_no_raw_lan_ai_markers(&event.payload);
}

#[tokio::test]
async fn authorized_lan_ai_job_submit_returns_degraded_result_when_provider_is_unavailable() {
    let runtime = paired_runtime().await;
    let event = handle_command_text_for_test(
        &serialize_command(command_for_target(
            AgentCommandName::AgentLanAiJobSubmit,
            local_network_target(constants::lan_pairing::CHILD_DEVICE_ID),
            lan_ai_job_payload(constants::value::LAN_PARENT_AUTHORITY_ACTIVE_CONTROLLER),
        )),
        runtime,
        Some(constants::lan_pairing::ALLOWED_ORIGIN.to_string()),
    )
    .await;

    assert_eq!(event.event, AgentEventName::AgentLanAiJobReported);
    assert_eq!(
        event.payload.get(constants::field::LAN_CONTROL_STATE),
        Some(&LogFieldValue::String(
            constants::value::LAN_CONTROL_ACCEPTED.to_string()
        ))
    );
    assert_eq!(
        event.payload.get(constants::field::LAN_AUDIT_EVENT_TYPE),
        Some(&LogFieldValue::String(
            constants::value::LAN_AUDIT_LAN_AI_JOB_DEGRADED.to_string()
        ))
    );
    assert_eq!(
        event.payload.get(constants::field::LAN_AI_JOB_STATE),
        Some(&LogFieldValue::String(
            constants::value::LAN_AI_JOB_STATE_DEGRADED.to_string()
        ))
    );
    assert_eq!(
        event
            .payload
            .get(constants::field::LOCAL_AI_UNAVAILABLE_REASON),
        Some(&LogFieldValue::String(
            constants::local_ai_runtime::UNAVAILABLE_REASON_UNCONFIGURED.to_string()
        ))
    );
    assert_no_raw_lan_ai_markers(&event.payload);
}

#[tokio::test]
async fn observer_lan_ai_job_submit_is_rejected_before_provider_routing() {
    let runtime = paired_runtime().await;
    let event = handle_command_text_for_test(
        &serialize_command(command_for_target(
            AgentCommandName::AgentLanAiJobSubmit,
            local_network_target(constants::lan_pairing::CHILD_DEVICE_ID),
            lan_ai_job_payload(constants::value::LAN_PARENT_AUTHORITY_OBSERVER),
        )),
        runtime,
        Some(constants::lan_pairing::ALLOWED_ORIGIN.to_string()),
    )
    .await;

    assert_rejection_with_audit(
        &event,
        constants::value::LAN_REASON_OBSERVER_READ_ONLY,
        constants::value::LAN_AUDIT_LAN_AI_JOB_REJECTED,
    );
}

fn lan_ai_job_payload(authority: &str) -> LogFields {
    let mut payload = intent_payload_for_kind(
        constants::lan_pairing::LAN_AI_JOB_INTENT_ID,
        constants::lan_pairing::CHILD_DEVICE_ID,
        constants::lan_pairing::PROOF_DIGEST,
        constants::lan_pairing::EXPIRES_AT,
        constants::value::LAN_INTENT_LAN_AI_JOB_SUBMIT,
    );
    payload.insert(
        constants::field::LAN_PARENT_AUTHORITY.to_string(),
        LogFieldValue::String(authority.to_string()),
    );
    payload.insert(
        constants::field::LAN_AI_JOB_ID.to_string(),
        LogFieldValue::String(constants::lan_pairing::LAN_AI_JOB_ID.to_string()),
    );
    payload
}

fn with_parent_authority(mut payload: LogFields, authority: &str) -> LogFields {
    payload.insert(
        constants::field::LAN_PARENT_AUTHORITY.to_string(),
        LogFieldValue::String(authority.to_string()),
    );
    payload
}

fn assert_no_raw_lan_ai_markers(payload: &LogFields) {
    for marker in [
        constants::lan_pairing::RAW_MARKER_ACTIVITY_SQLITE,
        constants::lan_pairing::RAW_MARKER_ACTIVITY_NDJSON,
        constants::lan_pairing::RAW_MARKER_DECRYPTED_EVIDENCE,
        constants::lan_pairing::RAW_MARKER_JOURNAL_PATH,
        constants::lan_pairing::RAW_MARKER_RAW_EVIDENCE,
        constants::lan_pairing::RAW_MARKER_RAW_PROMPT,
        constants::lan_pairing::RAW_MARKER_RAW_PROOF_SECRET,
        constants::lan_pairing::RAW_MARKER_RAW_TOKEN,
        constants::lan_pairing::RAW_MARKER_SQLITE_PATH,
    ] {
        assert!(!payload_contains_marker(payload, marker));
    }
}

fn payload_contains_marker(payload: &LogFields, marker: &str) -> bool {
    payload.iter().any(|(key, value)| {
        key.contains(marker)
            || match value {
                LogFieldValue::String(value) => value.contains(marker),
                LogFieldValue::Number(_) | LogFieldValue::Boolean(_) | LogFieldValue::Null(_) => {
                    false
                }
            }
    })
}
