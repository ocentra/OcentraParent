use crate::test_text::TestText;
use ocentra_parent_agent_protocol::activity_query::ActivityRecentSummary;
use ocentra_parent_agent_protocol::activity_surface::{
    ActivityHistoricalReportList, ActivityReportDocument, ActivityReportRequest,
    ActivitySurfaceRequest,
};
use std::fs::remove_dir_all;
use std::path::Path as TestPath;

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

pub fn save_activity_report_document_for_test(
    report: ActivityReportDocument,
) -> ActivityReportDocument {
    crate::activity_surface_report_store::logic::save_report_document_to_dir(
        report,
        crate::activity_surface_report_store::logic::activity_report_storage_dir(),
    )
}

pub fn save_activity_report_document_to_dir_for_test(
    report: ActivityReportDocument,
    directory: impl AsRef<TestPath>,
) -> ActivityReportDocument {
    crate::activity_surface_report_store::logic::save_report_document_to_dir(
        report,
        crate::activity_surface_report_store::ReportStorageDir(directory.as_ref().to_path_buf()),
    )
}

pub fn history_list_from_dir_for_test(
    request: ActivitySurfaceRequest,
    directory: impl AsRef<TestPath>,
) -> ActivityHistoricalReportList {
    crate::activity_surface_report_store::logic::history_list_from_dir(
        request,
        crate::activity_surface_report_store::ReportStorageDir(directory.as_ref().to_path_buf()),
    )
}

pub fn cleanup_report_dir(path: impl AsRef<TestPath>) {
    let _ = remove_dir_all(path.as_ref());
}

pub async fn load_activity_recent_summary_from_store_path_for_test(
    store_path: impl AsRef<TestPath>,
) -> Option<ActivityRecentSummary> {
    let store_path = store_path.as_ref().to_path_buf();
    tokio::task::spawn_blocking(move || {
        let store = match ocentra_parent_agent_core::activity_store::ActivityStore::open(store_path)
        {
            Ok(store) => store,
            Err(_error) => return None,
        };
        store
            .recent_summary(
                ocentra_parent_agent_protocol::constants::activity_store::DEFAULT_RECENT_LIMIT,
            )
            .ok()
    })
    .await
    .ok()?
}
