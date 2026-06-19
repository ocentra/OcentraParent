use ocentra_parent_agent_protocol::{
    constants, AgentCommandName, AgentEventName, DeviceRoleRuntimeReadModel,
    DeviceRuntimeAiProviderState, DeviceRuntimeLocalAiClaim, DeviceRuntimeRole,
    DeviceRuntimeRoleEntry, DeviceRuntimeRoleState, DeviceRuntimeRouteState, DeviceRuntimeSurface,
    LanPairingParentAuthority, LogFieldValue, LogFields,
};

use crate::{
    lan_pairing::LanPairingRuntime,
    lan_pairing_test_commands::{
        command_for_target, intent_payload_for_kind, local_network_target, paired_runtime,
        serialize_command,
    },
    websocket::handle_command_text_for_test,
};

#[tokio::test]
async fn authorized_lan_ai_job_submit_reports_child_owned_mesh_route_metadata() {
    let runtime = lan_ai_provider_runtime().await;
    let event = handle_command_text_for_test(
        &serialize_command(command_for_target(
            AgentCommandName::AgentLanAiJobSubmit,
            local_network_target(constants::lan_pairing::CHILD_DEVICE_ID),
            lan_ai_job_payload(constants::local_ai_runtime::CAPABILITY_SUMMARIZATION),
        )),
        runtime,
        Some(constants::lan_pairing::ALLOWED_ORIGIN.to_string()),
    )
    .await;

    assert_eq!(event.event, AgentEventName::AgentLanAiJobReported);
    assert_eq!(
        event
            .payload
            .get(constants::field::LAN_AI_SELECTED_PROVIDER_PEER_ID),
        Some(&LogFieldValue::String(
            constants::local_ai_runtime::PHYSICAL_DEVICE_LOCAL.to_string()
        ))
    );
    assert_eq!(
        event
            .payload
            .get(constants::field::LAN_AI_SELECTED_ROUTE_REASON),
        Some(&LogFieldValue::String(
            constants::household_mesh::ROUTE_REASON_SELECTED_DESKTOP.to_string()
        ))
    );
    assert_child_owned_custody_fields(&event.payload);
    assert_eq!(
        event.payload.get(constants::field::LAN_AI_CLAIM_ID),
        Some(&LogFieldValue::String(lan_ai_claim_id()))
    );
    assert_eq!(
        event.payload.get(constants::field::LAN_AI_LEASE_ID),
        Some(&LogFieldValue::String(lan_ai_lease_id()))
    );
    assert_no_raw_lan_ai_markers(&event.payload);
}

fn assert_child_owned_custody_fields(payload: &LogFields) {
    assert_eq!(
        payload.get(constants::field::LAN_AI_PROVIDER_POLICY_AUTHORITY),
        Some(&LogFieldValue::String(
            constants::household_mesh::POLICY_AUTHORITY_CHILD_AGENT_ONLY.to_string()
        ))
    );
    assert_eq!(
        payload.get(constants::field::LAN_AI_PROVIDER_CAN_PUBLISH_POLICY),
        Some(&LogFieldValue::Boolean(false))
    );
    assert_eq!(
        payload.get(constants::field::LAN_AI_RAW_SCREEN_TRANSFERRED),
        Some(&LogFieldValue::Boolean(false))
    );
    assert_eq!(
        payload.get(constants::field::LAN_AI_CHILD_VALIDATES_PROVIDER_RESULT),
        Some(&LogFieldValue::Boolean(true))
    );
}

fn lan_ai_job_payload(capability: &str) -> LogFields {
    let mut payload = intent_payload_for_kind(
        constants::lan_pairing::LAN_AI_JOB_INTENT_ID,
        constants::lan_pairing::CHILD_DEVICE_ID,
        constants::lan_pairing::PROOF_DIGEST,
        constants::lan_pairing::EXPIRES_AT,
        constants::value::LAN_INTENT_LAN_AI_JOB_SUBMIT,
    );
    payload.insert(
        constants::field::LAN_PARENT_AUTHORITY.to_string(),
        LogFieldValue::String(constants::value::LAN_PARENT_AUTHORITY_ACTIVE_CONTROLLER.to_string()),
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

async fn lan_ai_provider_runtime() -> LanPairingRuntime {
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
    runtime.lan_ai_provider_capabilities =
        vec![constants::local_ai_runtime::CAPABILITY_SUMMARIZATION.to_string()];
    runtime
}

fn role_entry(role: DeviceRuntimeRole) -> DeviceRuntimeRoleEntry {
    DeviceRuntimeRoleEntry {
        role,
        state: DeviceRuntimeRoleState::Implemented,
    }
}

fn lan_ai_claim_id() -> String {
    prefixed_id(constants::lan_pairing::LAN_AI_CLAIM_ID_PREFIX)
}

fn lan_ai_lease_id() -> String {
    prefixed_id(constants::lan_pairing::LAN_AI_LEASE_ID_PREFIX)
}

fn prefixed_id(prefix: &str) -> String {
    let mut value = String::from(prefix);
    value.push_str(constants::lan_pairing::LAN_AI_JOB_ID);
    value
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
