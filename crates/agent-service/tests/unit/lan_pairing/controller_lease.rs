use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::logging::LogFields;
use ocentra_parent_agent_protocol::transport::AgentEventEnvelope;

use crate::{
    app::websocket::handle_command_text_for_test,
    lan_pairing_test_assertions::{assert_rejection, assert_rejection_with_audit},
    lan_pairing_test_commands::{
        command_for_target, health_command, intent_payload, intent_payload_for_kind,
        local_network_target, paired_runtime, serialize_command,
    },
    test_text::TestText,
};

#[tokio::test]
async fn lan_pairing_requires_single_active_controller_lease_before_control() {
    let runtime = paired_runtime().await;
    let missing_lease =
        rejected_controller_lease_control(runtime.clone(), missing_controller_lease_payload())
            .await;
    let expired_lease =
        rejected_controller_lease_control(runtime.clone(), expired_controller_lease_payload())
            .await;
    let wrong_controller =
        rejected_controller_lease_control(runtime, second_controller_payload()).await;

    assert_rejection(
        &missing_lease,
        constants::value::LAN_REASON_CONTROLLER_LEASE_MISSING,
    );
    assert_rejection(
        &expired_lease,
        constants::value::LAN_REASON_CONTROLLER_LEASE_EXPIRED,
    );
    assert_rejection(
        &wrong_controller,
        constants::value::LAN_REASON_WRONG_CONTROLLER,
    );
    assert_eq!(
        wrong_controller
            .payload
            .get(constants::field::LAN_CONTROLLER_LEASE_ID),
        Some(&LogFieldValue::String(
            constants::lan_pairing::SECOND_CONTROLLER_LEASE_ID.to_string()
        ))
    );
}

#[tokio::test]
async fn lan_pairing_allows_observer_reads_and_rejects_observer_writes() {
    let runtime = paired_runtime().await;
    let observer_read = handle_command_text_for_test(
        serialize_command(health_command(with_parent_authority(
            intent_payload_for_kind(
                constants::lan_pairing::OBSERVER_RULE_QUERY_INTENT_ID,
                constants::lan_pairing::CHILD_DEVICE_ID,
                constants::lan_pairing::PROOF_DIGEST,
                constants::lan_pairing::EXPIRES_AT,
                constants::value::LAN_INTENT_RULE_QUERY,
            ),
        ))),
        runtime.clone(),
        Some(TestText::from_display(
            constants::lan_pairing::ALLOWED_ORIGIN,
        )),
    )
    .await;
    let observer_write = handle_command_text_for_test(
        serialize_command(health_command(with_parent_authority(
            intent_payload_for_kind(
                constants::lan_pairing::OBSERVER_RULE_UPDATE_INTENT_ID,
                constants::lan_pairing::CHILD_DEVICE_ID,
                constants::lan_pairing::PROOF_DIGEST,
                constants::lan_pairing::EXPIRES_AT,
                constants::value::LAN_INTENT_RULE_UPDATE,
            ),
        ))),
        runtime,
        Some(TestText::from_display(
            constants::lan_pairing::ALLOWED_ORIGIN,
        )),
    )
    .await;

    assert_eq!(
        observer_read.event,
        ocentra_parent_agent_protocol::transport::AgentEventName::AgentHealthReported
    );
    assert_eq!(
        observer_read
            .payload
            .get(constants::field::LAN_PARENT_AUTHORITY),
        Some(&LogFieldValue::String(
            constants::value::LAN_PARENT_AUTHORITY_OBSERVER.to_string()
        ))
    );
    assert_rejection(
        &observer_write,
        constants::value::LAN_REASON_OBSERVER_READ_ONLY,
    );
}

#[tokio::test]
async fn lan_pairing_controller_lease_lifecycle_renews_releases_and_takes_over() {
    let runtime = paired_runtime().await;
    let renewed = lease_lifecycle_command(
        runtime.clone(),
        ocentra_parent_agent_protocol::transport::AgentCommandName::AgentLanPairingControllerLeaseRenew,
        ControllerLeaseLifecycleExpectation {
            message_id: constants::lan_pairing::CONTROLLER_LEASE_RENEW_INTENT_ID,
            intent_kind: constants::value::LAN_INTENT_CONTROLLER_LEASE_RENEW,
        },
        controller_lease_payload(ControllerLeasePayloadExpectation {
            intent_id: constants::lan_pairing::CONTROLLER_LEASE_RENEW_INTENT_ID,
        }),
    )
    .await;
    let released = lease_lifecycle_command(
        runtime.clone(),
        ocentra_parent_agent_protocol::transport::AgentCommandName::AgentLanPairingControllerLeaseRelease,
        ControllerLeaseLifecycleExpectation {
            message_id: constants::lan_pairing::CONTROLLER_LEASE_RELEASE_INTENT_ID,
            intent_kind: constants::value::LAN_INTENT_CONTROLLER_LEASE_RELEASE,
        },
        controller_lease_payload(ControllerLeasePayloadExpectation {
            intent_id: constants::lan_pairing::CONTROLLER_LEASE_RELEASE_INTENT_ID,
        }),
    )
    .await;
    let takeover = lease_lifecycle_command(
        runtime.clone(),
        ocentra_parent_agent_protocol::transport::AgentCommandName::AgentLanPairingControllerLeaseTakeover,
        ControllerLeaseLifecycleExpectation {
            message_id: constants::lan_pairing::CONTROLLER_LEASE_TAKEOVER_INTENT_ID,
            intent_kind: constants::value::LAN_INTENT_CONTROLLER_LEASE_TAKEOVER,
        },
        second_controller_payload_for_kind(
            ControllerLeasePayloadKindExpectation {
                intent_id: constants::lan_pairing::CONTROLLER_LEASE_TAKEOVER_INTENT_ID,
                intent_kind: constants::value::LAN_INTENT_CONTROLLER_LEASE_TAKEOVER,
            },
        ),
    )
    .await;
    let old_controller_after_takeover = rejected_controller_lease_control(
        runtime,
        controller_lease_payload(ControllerLeasePayloadExpectation {
            intent_id: constants::lan_pairing::OLD_CONTROLLER_AFTER_TAKEOVER_INTENT_ID,
        }),
    )
    .await;

    assert_audit_type(
        &renewed,
        AuditTypeExpectation {
            audit_type: constants::value::LAN_AUDIT_CONTROLLER_LEASE_RENEWED,
        },
    );
    assert_audit_type(
        &released,
        AuditTypeExpectation {
            audit_type: constants::value::LAN_AUDIT_CONTROLLER_LEASE_RELEASED,
        },
    );
    assert_audit_type(
        &takeover,
        AuditTypeExpectation {
            audit_type: constants::value::LAN_AUDIT_CONTROLLER_LEASE_TAKEOVER_ACCEPTED,
        },
    );
    assert_rejection(
        &old_controller_after_takeover,
        constants::value::LAN_REASON_WRONG_CONTROLLER,
    );
}

#[tokio::test]
async fn lan_pairing_controller_lease_takeover_is_rejected_while_active_controller_is_current() {
    let runtime = paired_runtime().await;
    let denied = lease_lifecycle_command(
        runtime,
        ocentra_parent_agent_protocol::transport::AgentCommandName::AgentLanPairingControllerLeaseTakeover,
        ControllerLeaseLifecycleExpectation {
            message_id: constants::lan_pairing::CONTROLLER_LEASE_TAKEOVER_INTENT_ID,
            intent_kind: constants::value::LAN_INTENT_CONTROLLER_LEASE_TAKEOVER,
        },
        second_controller_payload_for_kind(
            ControllerLeasePayloadKindExpectation {
                intent_id: constants::lan_pairing::CONTROLLER_LEASE_TAKEOVER_INTENT_ID,
                intent_kind: constants::value::LAN_INTENT_CONTROLLER_LEASE_TAKEOVER,
            },
        ),
    )
    .await;

    assert_rejection_with_audit(
        &denied,
        constants::value::LAN_REASON_TAKEOVER_DENIED,
        constants::value::LAN_AUDIT_CONTROLLER_LEASE_TAKEOVER_REJECTED,
    );
}

async fn rejected_controller_lease_control(
    runtime: crate::app::lan_pairing::LanPairingRuntime,
    payload: LogFields,
) -> AgentEventEnvelope {
    handle_command_text_for_test(
        serialize_command(health_command(payload)),
        runtime,
        Some(TestText::from_display(
            constants::lan_pairing::ALLOWED_ORIGIN,
        )),
    )
    .await
}

async fn lease_lifecycle_command(
    runtime: crate::app::lan_pairing::LanPairingRuntime,
    command_name: ocentra_parent_agent_protocol::transport::AgentCommandName,
    expectation: ControllerLeaseLifecycleExpectation,
    mut payload: LogFields,
) -> AgentEventEnvelope {
    payload.insert(
        constants::field::LAN_INTENT_KIND.to_string(),
        LogFieldValue::String(expectation.intent_kind.to_string()),
    );
    let mut command = command_for_target(
        command_name,
        local_network_target(constants::lan_pairing::CHILD_DEVICE_ID),
        payload,
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

fn with_parent_authority(mut payload: LogFields) -> LogFields {
    payload.insert(
        constants::field::LAN_PARENT_AUTHORITY.to_string(),
        LogFieldValue::String(constants::value::LAN_PARENT_AUTHORITY_OBSERVER.to_string()),
    );
    payload
}

#[derive(Clone, Copy)]
struct ControllerLeasePayloadKindExpectation {
    intent_id: &'static str,
    intent_kind: &'static str,
}

fn second_controller_payload_for_kind(
    expectation: ControllerLeasePayloadKindExpectation,
) -> LogFields {
    let mut payload = intent_payload_for_kind(
        expectation.intent_id,
        constants::lan_pairing::CHILD_DEVICE_ID,
        constants::lan_pairing::PROOF_DIGEST,
        constants::lan_pairing::EXPIRES_AT,
        expectation.intent_kind,
    );
    payload.insert(
        constants::field::LAN_CONTROLLER_LEASE_ID.to_string(),
        LogFieldValue::String(constants::lan_pairing::SECOND_CONTROLLER_LEASE_ID.to_string()),
    );
    payload.insert(
        constants::field::LAN_CONTROLLER_DEVICE_ID.to_string(),
        LogFieldValue::String(constants::lan_pairing::SECOND_PARENT_DEVICE_ID.to_string()),
    );
    payload.insert(
        constants::field::LAN_PARENT_ACTOR_ID.to_string(),
        LogFieldValue::String(constants::lan_pairing::SECOND_PARENT_ACTOR_ID.to_string()),
    );
    payload
}

#[derive(Clone, Copy)]
struct AuditTypeExpectation {
    audit_type: &'static str,
}

fn assert_audit_type(event: &AgentEventEnvelope, expectation: AuditTypeExpectation) {
    assert_eq!(
        event.payload.get(constants::field::LAN_AUDIT_EVENT_TYPE),
        Some(&LogFieldValue::String(expectation.audit_type.to_string()))
    );
}

fn missing_controller_lease_payload() -> LogFields {
    let mut payload = controller_lease_payload(ControllerLeasePayloadExpectation {
        intent_id: constants::lan_pairing::RULE_QUERY_INTENT_ID,
    });
    payload = without_fields(
        payload,
        [
            TestText::from_display(constants::field::LAN_CONTROLLER_LEASE_ID),
            TestText::from_display(constants::field::LAN_CONTROLLER_DEVICE_ID),
            TestText::from_display(constants::field::LAN_PARENT_ACTOR_ID),
            TestText::from_display(constants::field::LAN_CONTROLLER_LEASE_ISSUED_AT),
            TestText::from_display(constants::field::LAN_CONTROLLER_LEASE_EXPIRES_AT),
        ],
    );
    payload
}

fn expired_controller_lease_payload() -> LogFields {
    let mut payload = controller_lease_payload(ControllerLeasePayloadExpectation {
        intent_id: constants::lan_pairing::RULE_UPDATE_INTENT_ID,
    });
    payload.insert(
        constants::field::LAN_CONTROLLER_LEASE_EXPIRES_AT.to_string(),
        LogFieldValue::String(constants::lan_pairing::CONTROLLER_LEASE_EXPIRED_AT.to_string()),
    );
    payload
}

fn second_controller_payload() -> LogFields {
    let mut payload = controller_lease_payload(ControllerLeasePayloadExpectation {
        intent_id: constants::lan_pairing::APPROVAL_DECISION_INTENT_ID,
    });
    payload.insert(
        constants::field::LAN_CONTROLLER_LEASE_ID.to_string(),
        LogFieldValue::String(constants::lan_pairing::SECOND_CONTROLLER_LEASE_ID.to_string()),
    );
    payload.insert(
        constants::field::LAN_CONTROLLER_DEVICE_ID.to_string(),
        LogFieldValue::String(constants::lan_pairing::SECOND_PARENT_DEVICE_ID.to_string()),
    );
    payload.insert(
        constants::field::LAN_PARENT_ACTOR_ID.to_string(),
        LogFieldValue::String(constants::lan_pairing::SECOND_PARENT_ACTOR_ID.to_string()),
    );
    payload
}

#[derive(Clone, Copy)]
struct ControllerLeasePayloadExpectation {
    intent_id: &'static str,
}

fn controller_lease_payload(expectation: ControllerLeasePayloadExpectation) -> LogFields {
    intent_payload(
        expectation.intent_id,
        constants::lan_pairing::CHILD_DEVICE_ID,
        constants::lan_pairing::PROOF_DIGEST,
        constants::lan_pairing::EXPIRES_AT,
    )
}

fn without_fields<I, T>(payload: LogFields, keys: I) -> LogFields
where
    I: IntoIterator<Item = T>,
    T: Into<TestText>,
{
    let mut inner = payload.into_inner();
    for key in keys {
        let key: TestText = key.into();
        inner.remove(key.0.as_str());
    }
    LogFields::from(inner)
}

struct ControllerLeaseLifecycleExpectation {
    message_id: &'static str,
    intent_kind: &'static str,
}
