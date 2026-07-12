use std::path::Path as TestPath;

use ocentra_parent_agent_protocol::transport::AgentEventEnvelope;

use crate::test_text::TestText;

pub async fn handle_local_command_text_for_test(body: TestText) -> AgentEventEnvelope {
    crate::agent_service_lib::websocket::dispatch_local_command_text(
        crate::agent_service_lib::websocket::WebsocketCommandText(body.0),
    )
    .await
}

pub async fn handle_local_command_text_with_browser_policy_store_for_test(
    body: TestText,
    store_path: &TestPath,
) -> AgentEventEnvelope {
    crate::agent_service_lib::websocket::dispatch_local_command_text_with_browser_policy_store(
        crate::agent_service_lib::websocket::WebsocketCommandText(body.0),
        crate::agent_service_lib::websocket::WebsocketBrowserPolicyStorePath(
            store_path.to_path_buf(),
        ),
    )
    .await
}
