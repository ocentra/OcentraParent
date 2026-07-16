use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::logging::LogFields;
use ocentra_parent_agent_protocol::transport::AgentEventName;

use crate::test_text::TestText;

use crate::{
    app::{lan_pairing::LanPairingRuntime, websocket::handle_command_text_for_test},
    lan_pairing_test_commands::{serialize_command, status_command},
};

#[tokio::test]
async fn empty_status_get_returns_base_pairing_status_read_model() {
    let event = handle_command_text_for_test(
        serialize_command(status_command(LogFields::new())),
        LanPairingRuntime::empty(),
        Some(TestText::from_display(
            constants::lan_pairing::ALLOWED_ORIGIN,
        )),
    )
    .await;

    assert_eq!(event.event, AgentEventName::AgentLanPairingStatusReported);
    assert!(matches!(
        event.payload.get(constants::field::LAN_ADD_DEVICE_READ_MODEL),
        Some(LogFieldValue::String(value)) if !value.is_empty()
    ));
}
