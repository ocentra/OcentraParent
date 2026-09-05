use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::{
    DeviceRoleRuntimeReadModel, DeviceRuntimeAiProviderState, DeviceRuntimeLocalAiClaim,
    DeviceRuntimeRole, DeviceRuntimeRoleEntry, DeviceRuntimeRoleState, DeviceRuntimeRouteState,
    DeviceRuntimeSurface,
};
use ocentra_parent_agent_protocol::lan_pairing_authority::LanPairingParentAuthority;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};
use ocentra_parent_agent_protocol::transport::AgentCommandName;

use crate::{
    app::{lan_pairing::LanPairingRuntime, websocket::handle_command_text_for_test},
    lan_pairing_test_assertions::assert_rejection_with_audit,
    lan_pairing_test_commands::{
        command_for_target, intent_payload_for_kind, local_network_target, serialize_command,
    },
    test_text::TestText,
};

#[test]
fn crate_owned_lan_ai_provider_state_reports_available_and_degraded_without_pairing_claims() {
    let available = lan_ai_provider_runtime(DeviceRuntimeAiProviderState::Available);
    let degraded = lan_ai_provider_runtime(DeviceRuntimeAiProviderState::Degraded);

    assert_eq!(
        available.lan_ai_provider_status_value().0,
        constants::value::LAN_AI_PROVIDER_STATUS_AVAILABLE
    );
    assert_eq!(
        degraded.lan_ai_provider_status_value().0,
        constants::value::LAN_AI_PROVIDER_STATUS_DEGRADED
    );
    assert_eq!(available.trusted_device_count(), 0);
    assert_eq!(degraded.trusted_device_count(), 0);
}

#[tokio::test]
async fn unpaired_lan_ai_provider_status_request_fails_closed_before_advertisement() {
    let runtime = lan_ai_provider_runtime(DeviceRuntimeAiProviderState::Available);
    let event = handle_command_text_for_test(
        serialize_command(command_for_target(
            AgentCommandName::AgentLanAiProviderStatusGet,
            local_network_target(constants::lan_pairing::CHILD_DEVICE_ID),
            lan_ai_intent_payload(
                constants::lan_pairing::LAN_AI_PROVIDER_STATUS_INTENT_ID,
                constants::value::LAN_INTENT_LAN_AI_PROVIDER_STATUS,
                constants::value::LAN_PARENT_AUTHORITY_OBSERVER,
                constants::local_ai_runtime::CAPABILITY_CHAT_COMPLETION,
            ),
        )),
        runtime,
        Some(TestText::from_display(
            constants::lan_pairing::ALLOWED_ORIGIN,
        )),
    )
    .await;

    assert_rejection_with_audit(
        &event,
        constants::value::LAN_REASON_ANONYMOUS,
        constants::value::LAN_AUDIT_LAN_AI_JOB_REJECTED,
    );
    assert_no_raw_lan_ai_markers(&event.payload);
}

#[tokio::test]
async fn advertised_local_provider_cannot_bypass_unpaired_job_authority() {
    let runtime = lan_ai_provider_runtime(DeviceRuntimeAiProviderState::Available);
    let event = lan_ai_job_event(
        runtime,
        constants::value::LAN_PARENT_AUTHORITY_ACTIVE_CONTROLLER,
        constants::local_ai_runtime::CAPABILITY_CHAT_COMPLETION,
    )
    .await;

    assert_rejection_with_audit(
        &event,
        constants::value::LAN_REASON_ANONYMOUS,
        constants::value::LAN_AUDIT_LAN_AI_JOB_REJECTED,
    );
    assert_eq!(
        event.payload.get(constants::field::LAN_AI_LEASE_STATE),
        None
    );
    assert_eq!(
        event.payload.get(constants::field::LOCAL_AI_OUTPUT_TEXT),
        None
    );
    assert_no_raw_lan_ai_markers(&event.payload);
}

#[tokio::test]
async fn repeated_unpaired_lan_ai_jobs_remain_anonymous_without_allocating_replay_state() {
    let runtime = lan_ai_provider_runtime(DeviceRuntimeAiProviderState::Available);
    let first = lan_ai_job_event(
        runtime.clone(),
        constants::value::LAN_PARENT_AUTHORITY_ACTIVE_CONTROLLER,
        constants::local_ai_runtime::CAPABILITY_CHAT_COMPLETION,
    )
    .await;
    let second = lan_ai_job_event(
        runtime,
        constants::value::LAN_PARENT_AUTHORITY_ACTIVE_CONTROLLER,
        constants::local_ai_runtime::CAPABILITY_CHAT_COMPLETION,
    )
    .await;

    for event in [&first, &second] {
        assert_rejection_with_audit(
            event,
            constants::value::LAN_REASON_ANONYMOUS,
            constants::value::LAN_AUDIT_LAN_AI_JOB_REJECTED,
        );
        assert_eq!(
            event.payload.get(constants::field::LAN_AI_LEASE_STATE),
            None
        );
    }
}

#[tokio::test]
async fn observer_lan_ai_job_is_rejected_before_provider_routing() {
    let event = lan_ai_job_event(
        LanPairingRuntime::empty(),
        constants::value::LAN_PARENT_AUTHORITY_OBSERVER,
        constants::local_ai_runtime::CAPABILITY_CHAT_COMPLETION,
    )
    .await;

    assert_rejection_with_audit(
        &event,
        constants::value::LAN_REASON_OBSERVER_READ_ONLY,
        constants::value::LAN_AUDIT_LAN_AI_JOB_REJECTED,
    );
}

async fn lan_ai_job_event(
    runtime: LanPairingRuntime,
    authority: &'static str,
    capability: &'static str,
) -> ocentra_parent_agent_protocol::transport::AgentEventEnvelope {
    handle_command_text_for_test(
        serialize_command(command_for_target(
            AgentCommandName::AgentLanAiJobSubmit,
            local_network_target(constants::lan_pairing::CHILD_DEVICE_ID),
            lan_ai_intent_payload(
                constants::lan_pairing::LAN_AI_JOB_INTENT_ID,
                constants::value::LAN_INTENT_LAN_AI_JOB_SUBMIT,
                authority,
                capability,
            ),
        )),
        runtime,
        Some(TestText::from_display(
            constants::lan_pairing::ALLOWED_ORIGIN,
        )),
    )
    .await
}

fn lan_ai_intent_payload(
    intent_id: &'static str,
    intent_kind: &'static str,
    authority: &'static str,
    capability: &'static str,
) -> LogFields {
    let mut payload = intent_payload_for_kind(
        intent_id,
        constants::lan_pairing::CHILD_DEVICE_ID,
        constants::lan_pairing::PROOF_DIGEST,
        constants::lan_pairing::EXPIRES_AT,
        intent_kind,
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

fn lan_ai_provider_runtime(provider_state: DeviceRuntimeAiProviderState) -> LanPairingRuntime {
    let mut runtime = LanPairingRuntime::empty();
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
        lan_ai_provider_state: provider_state,
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
        assert!(!payload.iter().any(|(key, value)| {
            key.contains(marker)
                || matches!(value, LogFieldValue::String(value) if value.contains(marker))
        }));
    }
}
