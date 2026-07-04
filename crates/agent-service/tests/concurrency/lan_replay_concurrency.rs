use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::transport::AgentEventName;

use crate::{
    app::websocket::handle_command_text_for_test,
    lan_pairing_test_commands::{
        health_command, intent_payload, paired_runtime, serialize_command,
    },
    test_invariants::require_ok,
    test_text::TestText,
};

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn concurrent_duplicate_lan_intents_accept_once_and_reject_replay_once() {
    let runtime = paired_runtime().await;
    let command = serialize_command(health_command(intent_payload(
        "intent-concurrent-replay",
        constants::lan_pairing::CHILD_DEVICE_ID,
        constants::lan_pairing::PROOF_DIGEST,
        constants::lan_pairing::EXPIRES_AT,
    )));

    let first = spawn_control(runtime.clone(), command.clone());
    let second = spawn_control(runtime, command);
    let events = vec![
        require_ok(first.await, "first concurrent LAN command completes"),
        require_ok(second.await, "second concurrent LAN command completes"),
    ];

    assert_eq!(accepted_count(&events), 1);
    assert_eq!(replayed_rejection_count(&events), 1);
}

fn spawn_control(
    runtime: crate::app::lan_pairing::LanPairingRuntime,
    command: TestText,
) -> tokio::task::JoinHandle<ocentra_parent_agent_protocol::transport::AgentEventEnvelope> {
    tokio::spawn(async move {
        handle_command_text_for_test(
            command,
            runtime,
            Some(TestText::from_display(
                constants::lan_pairing::ALLOWED_ORIGIN,
            )),
        )
        .await
    })
}

fn accepted_count(
    events: &[ocentra_parent_agent_protocol::transport::AgentEventEnvelope],
) -> usize {
    events
        .iter()
        .filter(|event| event.event == AgentEventName::AgentHealthReported)
        .count()
}

fn replayed_rejection_count(
    events: &[ocentra_parent_agent_protocol::transport::AgentEventEnvelope],
) -> usize {
    events
        .iter()
        .filter(|event| {
            event.event == AgentEventName::AgentCommandRejected
                && event.payload.get(constants::field::LAN_REJECTION_REASON)
                    == Some(&LogFieldValue::String(
                        constants::value::LAN_REASON_REPLAYED.to_string(),
                    ))
        })
        .count()
}
