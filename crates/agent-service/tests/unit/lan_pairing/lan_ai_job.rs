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
use ocentra_parent_agent_protocol::transport::AgentEventEnvelope;
use ocentra_parent_agent_protocol::transport::AgentEventName;

use crate::{
    app::{lan_pairing::LanPairingRuntime, websocket::handle_command_text_for_test},
    lan_pairing_test_assertions::assert_rejection_with_audit,
    lan_pairing_test_commands::{
        command_for_target, intent_payload, intent_payload_for_kind, local_network_target,
        paired_runtime, serialize_command,
    },
    lan_pairing_test_multidevice_commands::route_revoke_command,
    test_text::TestText,
};

#[tokio::test]
async fn lan_ai_provider_status_advertises_capability_to_observer_without_raw_activity() {
    let runtime = paired_runtime().await;
    let event = handle_command_text_for_test(
        serialize_command(command_for_target(
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
                LanParentAuthorityExpectation {
                    authority: constants::value::LAN_PARENT_AUTHORITY_OBSERVER,
                },
            ),
        )),
        runtime,
        Some(TestText::from_display(
            constants::lan_pairing::ALLOWED_ORIGIN,
        )),
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
        serialize_command(command_for_target(
            AgentCommandName::AgentLanAiJobSubmit,
            local_network_target(constants::lan_pairing::CHILD_DEVICE_ID),
            lan_ai_job_payload_for_capability(LanAiJobPayloadExpectation {
                authority: constants::value::LAN_PARENT_AUTHORITY_ACTIVE_CONTROLLER,
                capability: constants::local_ai_runtime::CAPABILITY_CHAT_COMPLETION,
            }),
        )),
        runtime,
        Some(TestText::from_display(
            constants::lan_pairing::ALLOWED_ORIGIN,
        )),
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
        serialize_command(command_for_target(
            AgentCommandName::AgentLanAiJobSubmit,
            local_network_target(constants::lan_pairing::CHILD_DEVICE_ID),
            lan_ai_job_payload_for_capability(LanAiJobPayloadExpectation {
                authority: constants::value::LAN_PARENT_AUTHORITY_ACTIVE_CONTROLLER,
                capability: constants::local_ai_runtime::CAPABILITY_CHAT_COMPLETION,
            }),
        )),
        runtime,
        Some(TestText::from_display(
            constants::lan_pairing::ALLOWED_ORIGIN,
        )),
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
async fn repeated_lan_ai_job_submit_reuses_completed_job_without_raw_transfer() {
    let runtime = lan_ai_provider_runtime().await;
    let first_command = lan_ai_job_submit_command(LanAiJobSubmitExpectation {
        message_id: "lan-ai-job-command-first",
        intent_id: "lan-ai-job-intent-first",
        authority: constants::value::LAN_PARENT_AUTHORITY_ACTIVE_CONTROLLER,
        capability: constants::local_ai_runtime::CAPABILITY_CHAT_COMPLETION,
    });
    let second_command = lan_ai_job_submit_command(LanAiJobSubmitExpectation {
        message_id: "lan-ai-job-command-second",
        intent_id: "lan-ai-job-intent-second",
        authority: constants::value::LAN_PARENT_AUTHORITY_ACTIVE_CONTROLLER,
        capability: constants::local_ai_runtime::CAPABILITY_CHAT_COMPLETION,
    });

    let first = handle_command_text_for_test(
        first_command,
        runtime.clone(),
        Some(TestText::from_display(
            constants::lan_pairing::ALLOWED_ORIGIN,
        )),
    )
    .await;
    let second = handle_command_text_for_test(
        second_command,
        runtime,
        Some(TestText::from_display(
            constants::lan_pairing::ALLOWED_ORIGIN,
        )),
    )
    .await;

    assert_eq!(first.event, AgentEventName::AgentLanAiJobReported);
    assert_eq!(second.event, AgentEventName::AgentLanAiJobReported);
    assert_eq!(
        first.payload.get(constants::field::LAN_AI_JOB_ID),
        second.payload.get(constants::field::LAN_AI_JOB_ID)
    );
    assert_eq!(
        second.payload.get(constants::field::LAN_AUDIT_EVENT_TYPE),
        Some(&LogFieldValue::String(
            constants::value::LAN_AUDIT_LAN_AI_JOB_COMPLETED.to_string()
        ))
    );
    assert_eq!(
        second.payload.get(constants::field::LAN_AI_JOB_STATE),
        Some(&LogFieldValue::String(
            constants::value::LAN_AI_JOB_STATE_COMPLETED.to_string()
        ))
    );
    assert_no_raw_lan_ai_markers(&second.payload);
}

#[tokio::test]
async fn degraded_lan_ai_provider_routes_as_degraded_policy_state() {
    let runtime = lan_ai_provider_runtime_with_state(DeviceRuntimeAiProviderState::Degraded).await;
    let event = handle_command_text_for_test(
        serialize_command(command_for_target(
            AgentCommandName::AgentLanAiJobSubmit,
            local_network_target(constants::lan_pairing::CHILD_DEVICE_ID),
            lan_ai_job_payload_for_capability(LanAiJobPayloadExpectation {
                authority: constants::value::LAN_PARENT_AUTHORITY_ACTIVE_CONTROLLER,
                capability: constants::local_ai_runtime::CAPABILITY_CHAT_COMPLETION,
            }),
        )),
        runtime,
        Some(TestText::from_display(
            constants::lan_pairing::ALLOWED_ORIGIN,
        )),
    )
    .await;

    assert_eq!(event.event, AgentEventName::AgentLanAiJobReported);
    assert_eq!(
        event.payload.get(constants::field::LAN_AI_PROVIDER_STATUS),
        Some(&LogFieldValue::String(
            constants::value::LAN_AI_PROVIDER_STATUS_DEGRADED.to_string()
        ))
    );
    assert_eq!(
        event
            .payload
            .get(constants::field::LAN_AI_PROVIDER_ROUTING_STATE),
        Some(&LogFieldValue::String(
            constants::value::LAN_AI_PROVIDER_ROUTING_DEGRADED.to_string()
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
            constants::local_ai_runtime::DEGRADED_PROVIDER_UNAVAILABLE.to_string()
        ))
    );
    assert_no_raw_lan_ai_markers(&event.payload);
}

#[tokio::test]
async fn unsupported_lan_ai_capability_is_rejected_after_authority_checks() {
    let runtime = lan_ai_provider_runtime().await;
    let event = handle_command_text_for_test(
        serialize_command(command_for_target(
            AgentCommandName::AgentLanAiJobSubmit,
            local_network_target(constants::lan_pairing::CHILD_DEVICE_ID),
            lan_ai_job_payload_for_capability(LanAiJobPayloadExpectation {
                authority: constants::value::LAN_PARENT_AUTHORITY_ACTIVE_CONTROLLER,
                capability: constants::local_ai_runtime::CAPABILITY_CLASSIFICATION,
            }),
        )),
        runtime,
        Some(TestText::from_display(
            constants::lan_pairing::ALLOWED_ORIGIN,
        )),
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
async fn lan_ai_job_submit_reports_provider_unavailable_for_stale_offline_and_revoked_routes() {
    let stale_runtime = lan_ai_provider_runtime().await;
    assert!(stale_runtime.mark_selected_stale_for_test());
    let stale_event = lan_ai_job_event(stale_runtime).await;

    let offline_runtime = lan_ai_provider_runtime().await;
    assert!(offline_runtime.mark_selected_offline_for_test());
    let offline_event = lan_ai_job_event(offline_runtime).await;

    let revoked_runtime = lan_ai_provider_runtime().await;
    let _ = handle_command_text_for_test(
        serialize_command(route_revoke_command(intent_payload(
            constants::lan_pairing::REVOKE_INTENT_ID,
            constants::lan_pairing::CHILD_DEVICE_ID,
            constants::lan_pairing::PROOF_DIGEST,
            constants::lan_pairing::EXPIRES_AT,
        ))),
        revoked_runtime.clone(),
        Some(TestText::from_display(
            constants::lan_pairing::ALLOWED_ORIGIN,
        )),
    )
    .await;
    let revoked_event = lan_ai_job_event(revoked_runtime).await;

    assert_route_blocked_provider(
        &stale_event,
        LanAiJobBlockExpectation {
            reason: constants::value::LAN_REASON_STALE,
        },
    );
    assert_route_blocked_provider(
        &offline_event,
        LanAiJobBlockExpectation {
            reason: constants::value::LAN_REASON_OFFLINE,
        },
    );
    assert_route_blocked_provider(
        &revoked_event,
        LanAiJobBlockExpectation {
            reason: constants::value::LAN_REASON_REVOKED,
        },
    );
}

#[tokio::test]
async fn observer_lan_ai_job_submit_is_rejected_before_provider_routing() {
    let runtime = paired_runtime().await;
    let event = handle_command_text_for_test(
        serialize_command(command_for_target(
            AgentCommandName::AgentLanAiJobSubmit,
            local_network_target(constants::lan_pairing::CHILD_DEVICE_ID),
            lan_ai_job_payload_for_capability(LanAiJobPayloadExpectation {
                authority: constants::value::LAN_PARENT_AUTHORITY_OBSERVER,
                capability: constants::local_ai_runtime::CAPABILITY_CHAT_COMPLETION,
            }),
        )),
        runtime,
        Some(TestText::from_display(
            constants::lan_pairing::ALLOWED_ORIGIN,
        )),
    )
    .await;

    assert_rejection_with_audit(
        &event,
        constants::value::LAN_REASON_OBSERVER_READ_ONLY,
        constants::value::LAN_AUDIT_LAN_AI_JOB_REJECTED,
    );
}

#[derive(Clone, Copy)]
struct LanAiJobSubmitExpectation {
    message_id: &'static str,
    intent_id: &'static str,
    authority: &'static str,
    capability: &'static str,
}

#[derive(Clone, Copy)]
struct LanAiJobPayloadExpectation {
    authority: &'static str,
    capability: &'static str,
}

fn lan_ai_job_payload_for_capability(expectation: LanAiJobPayloadExpectation) -> LogFields {
    let mut payload = intent_payload_for_kind(
        constants::lan_pairing::LAN_AI_JOB_INTENT_ID,
        constants::lan_pairing::CHILD_DEVICE_ID,
        constants::lan_pairing::PROOF_DIGEST,
        constants::lan_pairing::EXPIRES_AT,
        constants::value::LAN_INTENT_LAN_AI_JOB_SUBMIT,
    );
    payload.insert(
        constants::field::LAN_PARENT_AUTHORITY.to_string(),
        LogFieldValue::String(expectation.authority.to_string()),
    );
    payload.insert(
        constants::field::LAN_AI_JOB_ID.to_string(),
        LogFieldValue::String(constants::lan_pairing::LAN_AI_JOB_ID.to_string()),
    );
    payload.insert(
        constants::field::LOCAL_AI_CAPABILITY_FLAGS.to_string(),
        LogFieldValue::String(expectation.capability.to_string()),
    );
    payload
}

fn lan_ai_job_submit_command(expectation: LanAiJobSubmitExpectation) -> TestText {
    let mut command = command_for_target(
        AgentCommandName::AgentLanAiJobSubmit,
        local_network_target(constants::lan_pairing::CHILD_DEVICE_ID),
        lan_ai_job_payload_for_capability(LanAiJobPayloadExpectation {
            authority: expectation.authority,
            capability: expectation.capability,
        }),
    );
    command.message_id = expectation.message_id.to_string();
    command.payload.insert(
        constants::field::LAN_INTENT_ID.to_string(),
        LogFieldValue::String(expectation.intent_id.to_string()),
    );
    serialize_command(command)
}

async fn lan_ai_job_event(runtime: LanPairingRuntime) -> AgentEventEnvelope {
    handle_command_text_for_test(
        serialize_command(command_for_target(
            AgentCommandName::AgentLanAiJobSubmit,
            local_network_target(constants::lan_pairing::CHILD_DEVICE_ID),
            lan_ai_job_payload_for_capability(LanAiJobPayloadExpectation {
                authority: constants::value::LAN_PARENT_AUTHORITY_ACTIVE_CONTROLLER,
                capability: constants::local_ai_runtime::CAPABILITY_CHAT_COMPLETION,
            }),
        )),
        runtime,
        Some(TestText::from_display(
            constants::lan_pairing::ALLOWED_ORIGIN,
        )),
    )
    .await
}

async fn lan_ai_provider_runtime() -> LanPairingRuntime {
    lan_ai_provider_runtime_with_state(DeviceRuntimeAiProviderState::Available).await
}

async fn lan_ai_provider_runtime_with_state(
    provider_state: DeviceRuntimeAiProviderState,
) -> LanPairingRuntime {
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

#[derive(Clone, Copy)]
struct LanAiJobBlockExpectation {
    reason: &'static str,
}

fn assert_route_blocked_provider(
    event: &AgentEventEnvelope,
    expectation: LanAiJobBlockExpectation,
) {
    assert_rejection_with_audit(
        event,
        expectation.reason,
        constants::value::LAN_AUDIT_LAN_AI_JOB_REJECTED,
    );
    assert_eq!(
        event
            .payload
            .get(constants::field::LAN_AI_PROVIDER_ROUTING_STATE),
        Some(&LogFieldValue::String(
            constants::value::LAN_AI_PROVIDER_ROUTING_UNAVAILABLE.to_string()
        ))
    );
    assert_eq!(
        event.payload.get(constants::field::LAN_AI_JOB_STATUS),
        Some(&LogFieldValue::String(
            constants::value::LAN_AI_JOB_STATE_REJECTED.to_string()
        ))
    );
    assert_no_raw_lan_ai_markers(&event.payload);
}

fn role_entry(role: DeviceRuntimeRole) -> DeviceRuntimeRoleEntry {
    DeviceRuntimeRoleEntry {
        role,
        state: DeviceRuntimeRoleState::Implemented,
    }
}

#[derive(Clone, Copy)]
struct LanParentAuthorityExpectation {
    authority: &'static str,
}

fn with_parent_authority(
    mut payload: LogFields,
    expectation: LanParentAuthorityExpectation,
) -> LogFields {
    payload.insert(
        constants::field::LAN_PARENT_AUTHORITY.to_string(),
        LogFieldValue::String(expectation.authority.to_string()),
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
        assert!(!payload.iter().any(|(key, value)| {
            key.contains(marker)
                || matches!(value, LogFieldValue::String(value) if value.contains(marker))
        }));
    }
}
