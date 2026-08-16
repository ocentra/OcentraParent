use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFieldValue;

use super::{ScreenAiAnalysisEventRecord, ScreenAnalysisFieldEntry};

pub(super) fn screen_analysis_redaction_fields(
    record: &ScreenAiAnalysisEventRecord,
) -> Vec<ScreenAnalysisFieldEntry> {
    let join_values = |values: &[String]| values.join(&constants::delimiter::LIST.to_string());
    let mut fields = Vec::new();
    for (key, values) in [
        (
            constants::field::SCREEN_OCR_TEXT_SNIPPETS,
            &record.ocr_text_snippets,
        ),
        (
            constants::field::SCREEN_REDACTION_NOTES,
            &record.redaction_notes,
        ),
    ] {
        if !values.is_empty() {
            fields.push(ScreenAnalysisFieldEntry {
                key,
                value: LogFieldValue::String(join_values(values)),
            });
        }
    }
    fields
}
