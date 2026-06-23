use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFieldValue;

use super::ScreenAiAnalysisEventRecord;

pub(super) fn screen_analysis_redaction_fields(
    record: &ScreenAiAnalysisEventRecord,
) -> Vec<(&'static str, LogFieldValue)> {
    [
        optional_string_list_field(
            constants::field::SCREEN_OCR_TEXT_SNIPPETS,
            &record.ocr_text_snippets,
        ),
        optional_string_list_field(
            constants::field::SCREEN_REDACTION_NOTES,
            &record.redaction_notes,
        ),
    ]
    .into_iter()
    .flatten()
    .collect()
}

fn optional_string_list_field(
    key: &'static str,
    values: &[String],
) -> Option<(&'static str, LogFieldValue)> {
    if values.is_empty() {
        None
    } else {
        Some((
            key,
            LogFieldValue::String(values.join(&constants::delimiter::LIST.to_string())),
        ))
    }
}
