#![forbid(unsafe_code)]

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    ocentra_parent_agent_service::run_agent_service().await;
}
