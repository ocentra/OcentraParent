#![forbid(unsafe_code)]

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), ocentra_child_runtime::service::ChildAgentServiceError> {
    ocentra_child_runtime::service::run_child_agent_service().await
}
