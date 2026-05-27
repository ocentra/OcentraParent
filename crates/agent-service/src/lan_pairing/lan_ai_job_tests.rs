use ocentra_parent_agent_protocol::{
    constants, AgentCommandName, AgentEventName, DeviceRoleRuntimeReadModel,
    DeviceRuntimeAiProviderState, DeviceRuntimeLocalAiClaim, DeviceRuntimeRole,
    DeviceRuntimeRoleEntry, DeviceRuntimeRoleState, DeviceRuntimeRouteState, DeviceRuntimeSurface,
    LanPairingParentAuthority, LogFieldValue, LogFields,
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
async fn authorized_lan_ai_job_submit_routes_to_opted_in_provider() {
    let runtime = lan_ai_provider_runtime().await;
    let event = handle_command_text_for_test(
        &serialize_command(command_for_target(
            AgentCommandName::AgentLanAiJobSubmit,
            local_network_target(constants::lan_pairing::CHILD_DEVICE_ID),
            lan_ai_job_payload_for_capability(
                constants::value::LAN_PARENT_AUTHORITY_ACTIVE_CONTROLLER,
                constants::local_ai_runtime::CAPABILITY_CHAT_COMPLETION,
            ),
        )),
        runtime,
        Some(constants::lan_pairing::ALLOWED_ORIGIN.to_string()),
    )
    .await;

    assert_eq!(event.event, AgentEventName::AgentLanAiJobReported);
    assert_eq!(
        event.payload.get(constants::field::LAN_AUDIT_EVENT_TYPE),
        Some(&LogFieldValue::String(
            constants::value::LAN_AUDIT_LAN_AI_JOB_COMPLETED.to_string()
        ))
    );
    assert_eq!(
        event.payload.get(constants::field::LAN_AI_PROVIDER_STATUS),
        Some(&LogFieldValue::String(
            constants::value::LAN_AI_PROVIDER_STATUS_AVAILABLE.to_string()
        ))
    );
    assert_eq!(
        event.payload.get(constants::field::LAN_AI_JOB_STATE),
        Some(&LogFieldValue::String(
            constants::value::LAN_AI_JOB_STATE_COMPLETED.to_string()
        ))
    );
    assert_eq!(
        event.payload.get(constants::field::LOCAL_AI_OUTPUT_TEXT),
        Some(&LogFieldValue::String(
            constants::value::LAN_AI_PROVIDER_RESULT_REDACTED.to_string()
        ))
    );
    assert_no_raw_lan_ai_markers(&event.payload);
}

#[tokio::test]
async fn unsupported_lan_ai_capability_is_rejected_after_authority_checks() {
    let runtime = lan_ai_provider_runtime().await;
    let event = handle_command_text_for_test(
        &serialize_command(command_for_target(
            AgentCommandName::AgentLanAiJobSubmit,
            local_network_target(constants::lan_pairing::CHILD_DEVICE_ID),
            lan_ai_job_payload_for_capability(
                constants::value::LAN_PARENT_AUTHORITY_ACTIVE_CONTROLLER,
                constants::local_ai_runtime::CAPABILITY_CLASSIFICATION,
            ),
        )),
        runtime,
        Some(constants::lan_pairing::ALLOWED_ORIGIN.to_string()),
    )
    .await;

    assert_rejection_with_audit(
        &event,
        constants::value::LAN_REASON_LAN_AI_JOB_UNAUTHORIZED,
        constants::value::LAN_AUDIT_LAN_AI_JOB_REJECTED,
    );
    assert_eq!(
        event.payload.get(constants::field::LAN_AI_JOB_STATUS),
        Some(&LogFieldValue::String(
            constants::value::LAN_AI_JOB_STATE_REJECTED.to_string()
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
    lan_ai_job_payload_for_capability(
        authority,
        constants::local_ai_runtime::CAPABILITY_CHAT_COMPLETION,
    )
}

fn lan_ai_job_payload_for_capability(authority: &str, capability: &str) -> LogFields {
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
    payload.insert(
        constants::field::LOCAL_AI_CAPABILITY_FLAGS.to_string(),
        LogFieldValue::String(capability.to_string()),
    );
    payload
}

async fn lan_ai_provider_runtime() -> crate::lan_pairing::LanPairingRuntime {
    let mut runtime = paired_runtime().await;
    runtime.device_roles = DeviceRoleRuntimeReadModel {
        schema_version: constants::lan_pairing::SCHEMA_VERSION_TEXT.to_string(),
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
    runtime.lan_ai_provider_capabilities = vec![
        constants::local_ai_runtime::CAPABILITY_CHAT_COMPLETION.to_string(),
        constants::local_ai_runtime::CAPABILITY_SUMMARIZATION.to_string(),
    ];
    runtime
}

fn role_entry(role: DeviceRuntimeRole) -> DeviceRuntimeRoleEntry {
    DeviceRuntimeRoleEntry {
        role,
        state: DeviceRuntimeRoleState::Implemented,
    }
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
