use ocentra_parent_agent_protocol::activity_surface::ActivityReportDocument;
use ocentra_parent_agent_protocol::activity_surface::ActivitySavedReportState;
use ocentra_parent_agent_protocol::constants;

use super::ParentAssistantTextRef;

pub(super) fn saved_state_label(
    report: &ActivityReportDocument,
) -> ParentAssistantTextRef<'static> {
    match report
        .saved_metadata
        .as_ref()
        .map(|metadata| metadata.saved_state)
        .unwrap_or(ActivitySavedReportState::Draft)
    {
        ActivitySavedReportState::Draft => {
            ParentAssistantTextRef(constants::activity_surface::SAVED_STATE_DRAFT)
        }
        ActivitySavedReportState::Saved => {
            ParentAssistantTextRef(constants::activity_surface::SAVED_STATE_SAVED)
        }
        ActivitySavedReportState::StorageUnavailable => {
            ParentAssistantTextRef(constants::activity_surface::SAVED_STATE_STORAGE_UNAVAILABLE)
        }
        ActivitySavedReportState::Degraded => {
            ParentAssistantTextRef(constants::activity_surface::SAVED_STATE_DEGRADED)
        }
        ActivitySavedReportState::ScaffoldOnly => {
            ParentAssistantTextRef(constants::activity_surface::SAVED_STATE_SCAFFOLD_ONLY)
        }
    }
}
