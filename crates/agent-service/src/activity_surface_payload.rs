use ocentra_parent_agent_protocol::{
    constants, ActivityHistoricalReportList, ActivityReadModelState, ActivityReportDocument,
    ActivityReportFrequency, LogFieldValue, LogFields,
};

use crate::fields::fields_from_pairs;

pub(crate) fn activity_report_document_payload(report: &ActivityReportDocument) -> LogFields {
    fields_from_pairs(vec![
        (
            constants::field::ACTIVITY_REPORT_ID,
            LogFieldValue::String(report.report_id.clone()),
        ),
        (
            constants::field::ACTIVITY_REPORT_FREQUENCY,
            LogFieldValue::String(report_frequency_value(report.frequency).to_string()),
        ),
        (
            constants::field::ACTIVITY_SURFACE_STATE,
            LogFieldValue::String(report_state(report).to_string()),
        ),
        (
            constants::field::ACTIVITY_REPORT_DOCUMENT,
            LogFieldValue::String(json_string(report)),
        ),
    ])
}

pub(crate) fn activity_history_payload(history: &ActivityHistoricalReportList) -> LogFields {
    fields_from_pairs(vec![
        (
            constants::field::ACTIVITY_SURFACE_STATE,
            LogFieldValue::String(read_model_state_value(history.state).to_string()),
        ),
        (
            constants::field::RETURNED,
            LogFieldValue::Number(history.reports.len() as f64),
        ),
        (
            constants::field::ACTIVITY_REPORTS,
            LogFieldValue::String(history_json_string(history)),
        ),
    ])
}

pub(crate) fn activity_read_model_payload(
    read_model_kind: &'static str,
    state: ActivityReadModelState,
    row_count: usize,
    read_model_json: String,
) -> LogFields {
    fields_from_pairs(vec![
        (
            constants::field::ACTIVITY_READ_MODEL_KIND,
            LogFieldValue::String(read_model_kind.to_string()),
        ),
        (
            constants::field::ACTIVITY_SURFACE_STATE,
            LogFieldValue::String(read_model_state_value(state).to_string()),
        ),
        (
            constants::field::RETURNED,
            LogFieldValue::Number(row_count as f64),
        ),
        (
            constants::field::ACTIVITY_READ_MODEL,
            LogFieldValue::String(read_model_json),
        ),
    ])
}

fn report_state(report: &ActivityReportDocument) -> &'static str {
    if report
        .sections
        .iter()
        .any(|section| section.state == ActivityReadModelState::Ready)
    {
        return read_model_state_value(ActivityReadModelState::Ready);
    }

    report
        .sections
        .first()
        .map(|section| read_model_state_value(section.state))
        .unwrap_or_else(|| read_model_state_value(ActivityReadModelState::Empty))
}

fn read_model_state_value(state: ActivityReadModelState) -> &'static str {
    match state {
        ActivityReadModelState::Ready => constants::activity_surface::STATE_READY,
        ActivityReadModelState::Empty => constants::activity_surface::STATE_EMPTY,
        ActivityReadModelState::Unavailable => constants::activity_surface::STATE_UNAVAILABLE,
        ActivityReadModelState::Offline => constants::activity_surface::STATE_OFFLINE,
        ActivityReadModelState::Stale => constants::activity_surface::STATE_STALE,
        ActivityReadModelState::Degraded => constants::activity_surface::STATE_DEGRADED,
        ActivityReadModelState::ManualRequired => {
            constants::activity_surface::STATE_MANUAL_REQUIRED
        }
        ActivityReadModelState::PermissionRequired => {
            constants::activity_surface::STATE_PERMISSION_REQUIRED
        }
        ActivityReadModelState::ScaffoldOnly => constants::activity_surface::STATE_SCAFFOLD_ONLY,
    }
}

fn report_frequency_value(frequency: ActivityReportFrequency) -> &'static str {
    match frequency {
        ActivityReportFrequency::Daily => constants::activity_surface::FREQUENCY_DAILY,
        ActivityReportFrequency::Weekly => constants::activity_surface::FREQUENCY_WEEKLY,
        ActivityReportFrequency::Monthly => constants::activity_surface::FREQUENCY_MONTHLY,
    }
}

fn json_string(value: &ActivityReportDocument) -> String {
    serde_json::to_string(value).expect(constants::error::AGENT_EVENT_SERIALIZES)
}

fn history_json_string(value: &ActivityHistoricalReportList) -> String {
    serde_json::to_string(value).expect(constants::error::AGENT_EVENT_SERIALIZES)
}
