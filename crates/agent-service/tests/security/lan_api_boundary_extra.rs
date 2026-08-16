use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};
use ocentra_parent_agent_protocol::transport::{AgentEventEnvelope, AgentEventName};

use crate::test_text::TestText;

use crate::{
    app::{lan_pairing::LanPairingRuntime, websocket::handle_command_text_for_test},
    lan_pairing_test_commands::{
        health_command, intent_payload, paired_runtime, serialize_command,
    },
    test_invariants::require_ok,
};

fn assert_rejection(event: &AgentEventEnvelope, reason: impl std::fmt::Display) {
    let reason = reason.to_string();
    assert_eq!(event.event, AgentEventName::AgentCommandRejected);
    assert_eq!(
        event.payload.get(constants::field::LAN_CONTROL_STATE),
        Some(&LogFieldValue::String(
            constants::value::LAN_CONTROL_REJECTED.to_string()
        ))
    );
    assert_eq!(
        event.payload.get(constants::field::LAN_REJECTION_REASON),
        Some(&LogFieldValue::String(reason))
    );
    assert_eq!(
        event
            .payload
            .get(constants::field::LAN_AUTHENTICATION_STATE),
        Some(&LogFieldValue::String(
            constants::value::LAN_AUTH_UNAUTHENTICATED.to_string()
        ))
    );
}

#[tokio::test]
async fn lan_api_boundary_rejects_unknown_command_body() {
    let unknown = handle_command_text_for_test(
        unknown_lan_command_body(),
        LanPairingRuntime::empty(),
        Some(TestText::from_display(
            constants::lan_pairing::ALLOWED_ORIGIN,
        )),
    )
    .await;
    assert_eq!(unknown.event, AgentEventName::AgentCommandRejected);
    assert!(matches!(
        unknown.payload.get(constants::field::REASON),
        Some(LogFieldValue::String(reason))
            if reason.contains("unknown variant")
                && reason.contains("agent.lan-pairing.unknown")
    ));
}

#[tokio::test]
async fn lan_api_boundary_rejects_oversized_command_bodies() {
    let oversized = handle_command_text_for_test(
        oversized_lan_command_body(),
        LanPairingRuntime::empty(),
        Some(TestText::from_display(
            constants::lan_pairing::ALLOWED_ORIGIN,
        )),
    )
    .await;
    assert_eq!(oversized.event, AgentEventName::AgentCommandRejected);
    assert_eq!(
        oversized.payload.get(constants::field::LAN_CONTROL_STATE),
        Some(&LogFieldValue::String(
            constants::value::LAN_CONTROL_REJECTED.to_string()
        ))
    );
    assert_eq!(
        oversized
            .payload
            .get(constants::field::LAN_REJECTION_REASON),
        Some(&LogFieldValue::String(
            constants::value::LAN_REASON_PAYLOAD_TOO_LARGE.to_string()
        ))
    );
    assert_eq!(
        oversized
            .payload
            .get(constants::field::LAN_AUTHENTICATION_STATE),
        Some(&LogFieldValue::String(
            constants::value::LAN_AUTH_UNAUTHENTICATED.to_string()
        ))
    );
}

#[tokio::test]
async fn lan_api_boundary_rejects_origin_header_injection_without_child_control() {
    let runtime = paired_runtime().await;
    let injected_origin = format!(
        "{}\r\nx-forwarded-host: attacker.invalid",
        constants::lan_pairing::ALLOWED_ORIGIN
    );
    let rejected = handle_command_text_for_test(
        serialize_command(health_command(intent_payload(
            "intent-origin-injection",
            constants::lan_pairing::CHILD_DEVICE_ID,
            constants::lan_pairing::PROOF_DIGEST,
            constants::lan_pairing::EXPIRES_AT,
        ))),
        runtime,
        Some(TestText::from_display(injected_origin.clone())),
    )
    .await;

    assert_rejection(&rejected, constants::value::LAN_REASON_WRONG_ORIGIN);
    assert_eq!(
        rejected.payload.get(constants::field::ORIGIN),
        Some(&LogFieldValue::String(injected_origin))
    );
}

#[tokio::test]
async fn lan_api_boundary_rejects_missing_signed_intent_without_pairing_upgrade() {
    let runtime = paired_runtime().await;
    let rejected = handle_command_text_for_test(
        serialize_command(health_command(LogFields::new())),
        runtime,
        Some(TestText::from_display(
            constants::lan_pairing::ALLOWED_ORIGIN,
        )),
    )
    .await;

    assert_rejection(&rejected, constants::value::LAN_REASON_ANONYMOUS);
    assert_eq!(
        rejected
            .payload
            .get(constants::field::LAN_AUTHENTICATION_STATE),
        Some(&LogFieldValue::String(
            constants::value::LAN_AUTH_UNAUTHENTICATED.to_string()
        ))
    );
}

fn unknown_lan_command_body() -> TestText {
    let mut command_value: serde_json::Value = require_ok(
        serde_json::from_str(
            serialize_command(health_command(LogFields::new()))
                .0
                .as_str(),
        ),
        "serialized LAN command is JSON",
    );
    command_value["command"] = serde_json::json!("agent.lan-pairing.unknown");

    TestText::from_display(require_ok(
        serde_json::to_string(&command_value),
        "mutated LAN command serializes",
    ))
}

fn oversized_lan_command_body() -> TestText {
    TestText::from_display("x".repeat(constants::lan_pairing::LAN_WEBSOCKET_COMMAND_MAX_BYTES + 1))
}
