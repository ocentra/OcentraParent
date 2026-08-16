use ocentra_parent_agent_protocol::transport::AgentEventEnvelope;

use crate::test_text::TestText;

pub(crate) async fn handle_local_command_text_for_test(body: TestText) -> AgentEventEnvelope {
    crate::agent_service_lib::websocket::dispatch_local_command_text(
        crate::agent_service_lib::websocket::WebsocketCommandText(body.0),
    )
    .await
}
