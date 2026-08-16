use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::DeviceRoleRuntimeReadModel;
use ocentra_parent_agent_protocol::lan_pairing::DeviceRuntimeAiProviderState;
use ocentra_parent_agent_protocol::lan_pairing::DeviceRuntimeLocalAiClaim;
use ocentra_parent_agent_protocol::lan_pairing::DeviceRuntimeRole;
use ocentra_parent_agent_protocol::lan_pairing::DeviceRuntimeRoleEntry;
use ocentra_parent_agent_protocol::lan_pairing::DeviceRuntimeRoleState;
use ocentra_parent_agent_protocol::lan_pairing::DeviceRuntimeRouteState;
use ocentra_parent_agent_protocol::lan_pairing::DeviceRuntimeSurface;
use ocentra_parent_agent_protocol::lan_pairing_authority::LanPairingParentAuthority;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::logging::LogFields;
use ocentra_parent_agent_protocol::transport::AgentCommandName;
use ocentra_parent_agent_protocol::transport::AgentEventName;

use crate::{
    app::{lan_pairing::LanPairingRuntime, websocket::handle_command_text_for_test},
    lan_pairing_test_assertions::assert_rejection_with_audit,
    lan_pairing_test_commands::{
        command_for_target, intent_payload_for_kind, local_network_target, paired_runtime,
        serialize_command,
    },
    test_text::TestText,
};

#[tokio::test]
async fn lan_ai_job_submit_completes_and_records_service_owned_lease() {
    let runtime = lan_ai_provider_runtime().await;
    let event = lan_ai_job_event(runtime).await;

    assert_eq!(event.event, AgentEventName::AgentLanAiJobReported);
    assert_eq!(
        event.payload.get(constants::field::LAN_AI_LEASE_STATE),
        Some(&LogFieldValue::String(
            constants::value::LAN_AI_LEASE_STATE_COMPLETED.to_string()
        ))
    );
    assert_eq!(
        event
            .payload
            .get(constants::field::LAN_AI_LEASE_ATTEMPT_COUNT),
        Some(&LogFieldValue::Number(1.0))
    );
    assert_no_raw_lan_ai_markers(&event.payload);
}

#[tokio::test]
async fn duplicate_completed_lan_ai_job_submit_is_idempotent() {
    let runtime = lan_ai_provider_runtime().await;
    let first_event = lan_ai_job_event_with_ids(
        runtime.clone(),
        LanAiJobEventExpectation {
            message_id: constants::lan_pairing::INTENT_ID,
            intent_id: constants::lan_pairing::LAN_AI_JOB_INTENT_ID,
        },
    )
    .await;
    let duplicate_event = lan_ai_job_event_with_ids(
        runtime,
        LanAiJobEventExpectation {
            message_id: constants::lan_pairing::SECOND_SELECT_INTENT_ID,
            intent_id: constants::lan_pairing::SECOND_SELECT_INTENT_ID,
        },
    )
    .await;

    assert_eq!(
        first_event.payload.get(constants::field::LAN_AI_LEASE_ID),
        duplicate_event
            .payload
            .get(constants::field::LAN_AI_LEASE_ID)
    );
    assert_eq!(
        duplicate_event
            .payload
            .get(constants::field::LAN_AI_LEASE_STATE),
        Some(&LogFieldValue::String(
            constants::value::LAN_AI_LEASE_STATE_COMPLETED.to_string()
        ))
    );
    assert_eq!(
        duplicate_event
            .payload
            .get(constants::field::LAN_AI_LEASE_ATTEMPT_COUNT),
        Some(&LogFieldValue::Number(1.0))
    );
    assert_no_raw_lan_ai_markers(&duplicate_event.payload);
}

#[tokio::test]
async fn duplicate_active_lan_ai_job_submit_is_rejected_before_provider_result() {
    let runtime = lan_ai_provider_runtime().await;
    runtime.seed_lan_ai_job_lease_for_test(
        constants::lan_pairing::LAN_AI_JOB_ID,
        constants::value::LAN_AI_LEASE_STATE_CLAIMED,
        1,
        constants::lan_pairing::EXPIRES_AT,
    );
    let event = lan_ai_job_event(runtime).await;

    assert_rejection_with_audit(
        &event,
        constants::value::LAN_REASON_LAN_AI_JOB_UNAUTHORIZED,
        constants::value::LAN_AUDIT_LAN_AI_JOB_REJECTED,
    );
    assert_eq!(
        event.payload.get(constants::field::LAN_AI_LEASE_STATE),
        Some(&LogFieldValue::String(
            constants::value::LAN_AI_LEASE_STATE_DUPLICATE_REJECTED.to_string()
        ))
    );
    assert_eq!(
        event.payload.get(constants::field::LOCAL_AI_OUTPUT_TEXT),
        None
    );
    assert_no_raw_lan_ai_markers(&event.payload);
}

#[tokio::test]
async fn expired_lan_ai_job_lease_requeues_without_provider_output() {
    let runtime = lan_ai_provider_runtime().await;
    runtime.seed_lan_ai_job_lease_for_test(
        constants::lan_pairing::LAN_AI_JOB_ID,
        constants::value::LAN_AI_LEASE_STATE_CLAIMED,
        1,
        constants::lan_pairing::EXPIRED_AT,
    );
    let event = lan_ai_job_event(runtime).await;

    assert_eq!(event.event, AgentEventName::AgentLanAiJobReported);
    assert_eq!(
        event.payload.get(constants::field::LAN_AI_LEASE_STATE),
        Some(&LogFieldValue::String(
            constants::value::LAN_AI_LEASE_STATE_EXPIRED_REQUEUED.to_string()
        ))
    );
    assert_eq!(
        event
            .payload
            .get(constants::field::LAN_AI_LEASE_ATTEMPT_COUNT),
        Some(&LogFieldValue::Number(2.0))
    );
    assert_eq!(
        event.payload.get(constants::field::LOCAL_AI_OUTPUT_TEXT),
        None
    );
    assert_no_raw_lan_ai_markers(&event.payload);
}

#[tokio::test]
async fn expired_lan_ai_job_lease_dead_letters_after_max_attempts() {
    let runtime = lan_ai_provider_runtime().await;
    runtime.seed_lan_ai_job_lease_for_test(
        constants::lan_pairing::LAN_AI_JOB_ID,
        constants::value::LAN_AI_LEASE_STATE_CLAIMED,
        2,
        constants::lan_pairing::EXPIRED_AT,
    );
    let event = lan_ai_job_event(runtime).await;

    assert_eq!(event.event, AgentEventName::AgentLanAiJobReported);
    assert_eq!(
        event.payload.get(constants::field::LAN_AI_LEASE_STATE),
        Some(&LogFieldValue::String(
            constants::value::LAN_AI_LEASE_STATE_DEAD_LETTERED.to_string()
        ))
    );
    assert_eq!(
        event
            .payload
            .get(constants::field::LAN_AI_LEASE_ATTEMPT_COUNT),
        Some(&LogFieldValue::Number(3.0))
    );
    assert_eq!(
        event
            .payload
            .get(constants::field::LAN_AI_DEAD_LETTER_REASON),
        Some(&LogFieldValue::String(
            constants::value::LAN_AI_DEAD_LETTER_REASON_MAX_ATTEMPTS.to_string()
        ))
    );
    assert_no_raw_lan_ai_markers(&event.payload);
}

async fn lan_ai_job_event(
    runtime: LanPairingRuntime,
) -> ocentra_parent_agent_protocol::transport::AgentEventEnvelope {
    lan_ai_job_event_with_ids(
        runtime,
        LanAiJobEventExpectation {
            message_id: constants::lan_pairing::INTENT_ID,
            intent_id: constants::lan_pairing::LAN_AI_JOB_INTENT_ID,
        },
    )
    .await
}

async fn lan_ai_job_event_with_ids(
    runtime: LanPairingRuntime,
    expectation: LanAiJobEventExpectation,
) -> ocentra_parent_agent_protocol::transport::AgentEventEnvelope {
    let mut command = command_for_target(
        AgentCommandName::AgentLanAiJobSubmit,
        local_network_target(constants::lan_pairing::CHILD_DEVICE_ID),
        lan_ai_job_payload_for_intent(LanAiJobIntentExpectation {
            intent_id: expectation.intent_id,
        }),
    );
    command.message_id = expectation.message_id.to_string();
    handle_command_text_for_test(
        serialize_command(command),
        runtime,
        Some(TestText::from_display(
            constants::lan_pairing::ALLOWED_ORIGIN,
        )),
    )
    .await
}

#[derive(Clone, Copy)]
struct LanAiJobIntentExpectation {
    intent_id: &'static str,
}

struct LanAiJobEventExpectation {
    message_id: &'static str,
    intent_id: &'static str,
}

fn lan_ai_job_payload_for_intent(expectation: LanAiJobIntentExpectation) -> LogFields {
    let mut payload = intent_payload_for_kind(
        expectation.intent_id,
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
        LogFieldValue::String(constants::local_ai_runtime::CAPABILITY_CHAT_COMPLETION.to_string()),
    );
    payload
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
