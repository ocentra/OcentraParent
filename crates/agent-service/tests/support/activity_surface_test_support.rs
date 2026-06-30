use ocentra_parent_agent_protocol::activity_query::ActivityRecentSummary;
use ocentra_parent_agent_protocol::activity_surface::ActivityReportDocument;
use ocentra_parent_agent_protocol::activity_surface::ActivityReportRequest;
use ocentra_parent_agent_protocol::transport::AgentEventEnvelope;
use std::fs::remove_dir_all;

pub async fn handle_local_command_text_for_test(body: &str) -> AgentEventEnvelope {
    crate::agent_service_lib::websocket::dispatch_local_command_text(body).await
}

pub async fn lock_activity_report_env_for_test() -> tokio::sync::MutexGuard<'static, ()> {
    crate::activity_report_env_lock::REPORT_ENV_LOCK
        .lock()
        .await
}

pub fn build_activity_report_document_for_test(
    request: ActivityReportRequest,
) -> ActivityReportDocument {
    crate::activity_surface_report::report_document(request, None, Vec::new())
}

pub fn cleanup_report_dir(path: &std::path::Path) {
    let _ = remove_dir_all(path);
}

pub async fn load_activity_recent_summary_from_store_path_for_test(
    store_path: &std::path::Path,
) -> Option<ActivityRecentSummary> {
    let store_path = store_path.to_path_buf();
    tokio::task::spawn_blocking(move || {
        let store =
            ocentra_parent_agent_core::activity_store::ActivityStore::open(store_path).ok()?;
        store
            .recent_summary(
                ocentra_parent_agent_protocol::constants::activity_store::DEFAULT_RECENT_LIMIT,
            )
            .ok()
    })
    .await
    .ok()
    .flatten()
}
