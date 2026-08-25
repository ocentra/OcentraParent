use ocentra_eventing::error::EventingError;
use ocentra_parent_agent_core::browser_event_runtime::BrowserRuntimeReport;

use super::BrowserRuntimeServiceStreamReport;

pub(super) fn record_browser_runtime_publication(
    stream: &mut BrowserRuntimeServiceStreamReport,
    publication: Result<BrowserRuntimeReport, EventingError>,
    evidence_requires_manual_review: bool,
) {
    let Ok(report) = publication else {
        stream.failed_rows += 1;
        stream.manual_required_rows += 1;
        return;
    };
    if !stream.record_publication(&report) || evidence_requires_manual_review {
        stream.manual_required_rows += 1;
    }
}
