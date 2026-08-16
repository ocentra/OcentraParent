use std::path::Path;

use ocentra_eventing::bus::reports::handler::HandlerOutcome;
use ocentra_parent_agent_core::{
    activity_store::ActivityStore, screen_event_runtime::ScreenRuntimeReport,
    screen_evidence_queue::ScreenEvidenceExpiredQueueEntry,
};

use crate::activity_surface_read_models::activity_screen_row_from_result;
use crate::screen_ai_service_event_subscription::{ObservedAtText, ScreenAiServiceEventRuntime};

#[path = "screen_ai_retention_sweeper_deletion_events/conversions.rs"]
mod conversions;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ScreenAiRetentionSweeperDeletionEventOutcome {
    pub(crate) queue_job_id: String,
    pub(crate) downstream_event_count: usize,
    pub(crate) raw_image_escaped: bool,
}

pub(crate) async fn publish_screen_retention_deletion_events(
    runtime: &ScreenAiServiceEventRuntime,
    store_path: &Path,
    expired_entries: &[ScreenEvidenceExpiredQueueEntry],
    observed_at: impl Into<ScreenRetentionObservedAt>,
) -> Vec<ScreenAiRetentionSweeperDeletionEventOutcome> {
    let observed_at = observed_at.into();
    let mut outcomes = Vec::new();
    let Ok(store) = ActivityStore::open(store_path) else {
        return outcomes;
    };
    let delivery_journal_path = store_path.with_extension(
        ocentra_parent_agent_protocol::constants::activity_store::DEFAULT_JOURNAL_FILE_NAME,
    );
    for entry in expired_entries {
        let Ok(Some(result)) = store.screen_evidence_result_for_queue_job(&entry.queue_job_id)
        else {
            continue;
        };
        if let Ok(report) = runtime
            .publish_deletion_row(
                activity_screen_row_from_result(result),
                ObservedAtText(observed_at.0.clone()),
                &delivery_journal_path,
            )
            .await
        {
            if !terminal_deletion_delivery_handled(&report) {
                continue;
            }
            outcomes.push(ScreenAiRetentionSweeperDeletionEventOutcome {
                queue_job_id: entry.queue_job_id.clone(),
                downstream_event_count: report.stored_events.len(),
                raw_image_escaped: report.raw_image_escaped(),
            });
        }
    }
    outcomes
}

fn terminal_deletion_delivery_handled(report: &ScreenRuntimeReport) -> bool {
    let [publish] = report.publish_reports.as_slice() else {
        return false;
    };
    report.stored_events.len() == 1
        && report.dead_letters.is_empty()
        && !report.raw_image_escaped()
        && publish.subscriber_count == 1
        && publish.handled_count == 1
        && publish.dead_letter_count == 0
        && publish.handler_reports.len() == 1
        && publish.handler_reports[0].outcome == HandlerOutcome::Handled
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ScreenRetentionObservedAt(String);
