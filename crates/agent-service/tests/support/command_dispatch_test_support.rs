use std::path::Path as TestPath;

use ocentra_parent_agent_protocol::transport::AgentEventEnvelope;

use crate::test_text::TestText;

pub async fn handle_local_command_text_for_test(body: TestText) -> AgentEventEnvelope {
    crate::agent_service_lib::websocket::dispatch_local_command_text(body.0.as_str()).await
}

pub async fn handle_local_command_text_with_browser_policy_store_for_test(
    body: TestText,
    store_path: &TestPath,
) -> AgentEventEnvelope {
    crate::agent_service_lib::websocket::dispatch_local_command_text_with_browser_policy_store(
        body.0.as_str(),
        store_path,
    )
    .await
}
