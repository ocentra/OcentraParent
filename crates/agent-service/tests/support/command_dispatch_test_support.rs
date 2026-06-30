use std::path::Path;

use ocentra_parent_agent_protocol::transport::AgentEventEnvelope;

pub async fn handle_local_command_text_for_test(body: &str) -> AgentEventEnvelope {
    crate::agent_service_lib::websocket::dispatch_local_command_text(body).await
}

pub async fn handle_local_command_text_with_browser_policy_store_for_test(
    body: &str,
    store_path: &Path,
) -> AgentEventEnvelope {
    crate::agent_service_lib::websocket::dispatch_local_command_text_with_browser_policy_store(
        body, store_path,
    )
    .await
}
