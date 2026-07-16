use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::transport::AgentEventName;

use crate::test_text::TestText;

use crate::app::{lan_pairing::LanPairingRuntime, websocket::handle_command_text_for_test};

#[tokio::test]
async fn lan_api_boundary_rejects_malformed_command_body() {
    let malformed = handle_command_text_for_test(
        TestText::from_display("{"),
        LanPairingRuntime::empty(),
        Some(TestText::from_display(
            constants::lan_pairing::ALLOWED_ORIGIN,
        )),
    )
    .await;
    assert_eq!(malformed.event, AgentEventName::AgentCommandRejected);
    assert!(matches!(
        malformed.payload.get(constants::field::REASON),
        Some(LogFieldValue::String(reason)) if reason.contains("EOF while parsing")
    ));
}

#[path = "lan_api_boundary_extra.rs"]
mod lan_api_boundary_extra_tests;
