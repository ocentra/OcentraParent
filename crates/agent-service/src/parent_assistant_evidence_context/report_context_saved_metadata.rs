use ocentra_parent_agent_protocol::activity_surface::ActivityReportDocument;
use ocentra_parent_agent_protocol::constants;

use super::ParentAssistantText;

pub(super) enum SavedMetadataValue {
    FileName,
    SavedAt,
    StorageReason,
}

pub(super) fn saved_metadata_value(
    report: &ActivityReportDocument,
    value: &SavedMetadataValue,
) -> ParentAssistantText {
    let Some(metadata) = report.saved_metadata.as_ref() else {
        return ParentAssistantText(
            constants::parent_assistant::ACTIVITY_REPORT_SUMMARY_NONE.to_string(),
        );
    };

    match value {
        SavedMetadataValue::FileName => ParentAssistantText(metadata.file_name.clone()),
        SavedMetadataValue::SavedAt => metadata
            .saved_at
            .clone()
            .map(ParentAssistantText)
            .unwrap_or_else(|| {
                ParentAssistantText(
                    constants::parent_assistant::ACTIVITY_REPORT_SUMMARY_NONE.to_string(),
                )
            }),
        SavedMetadataValue::StorageReason => metadata
            .storage_reason
            .clone()
            .map(ParentAssistantText)
            .unwrap_or_else(|| {
                ParentAssistantText(
                    constants::parent_assistant::ACTIVITY_REPORT_SUMMARY_NONE.to_string(),
                )
            }),
    }
}
