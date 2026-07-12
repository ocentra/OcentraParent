use ocentra_parent_agent_protocol::activity_surface::{
    ActivityHistoricalReportList, ActivityReadModelState, ActivityReportDocument,
    ActivityReportFrequency,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};

use crate::fields::fields_from_pairs;

#[path = "activity_surface_payload/read_model_state_value.rs"]
mod read_model_state_value;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct JsonText(pub(crate) String);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct SurfaceText(pub(crate) &'static str);

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ReadModelKind(pub(crate) String);

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ReadModelJson(pub(crate) String);

pub(crate) fn activity_report_document_payload(report: &ActivityReportDocument) -> LogFields {
    fields_from_pairs(vec![
        (
            constants::field::ACTIVITY_REPORT_ID,
            LogFieldValue::String(report.report_id.clone()),
        ),
        (
            constants::field::ACTIVITY_REPORT_FREQUENCY,
            LogFieldValue::String(report_frequency_value(report.frequency).0.to_string()),
        ),
        (
            constants::field::ACTIVITY_SURFACE_STATE,
            LogFieldValue::String(report_state(report).0.to_string()),
        ),
        (
            constants::field::ACTIVITY_REPORT_DOCUMENT,
            LogFieldValue::String(json_string(report).0),
        ),
    ])
}

pub(crate) fn activity_history_payload(history: &ActivityHistoricalReportList) -> LogFields {
    fields_from_pairs(vec![
        (
            constants::field::ACTIVITY_SURFACE_STATE,
            LogFieldValue::String(
                read_model_state_value::read_model_state_value(history.state)
                    .0
                    .to_string(),
            ),
        ),
        (
            constants::field::RETURNED,
            LogFieldValue::Number(history.reports.len() as f64),
        ),
        (
            constants::field::ACTIVITY_REPORTS,
            LogFieldValue::String(history_json_string(history).0),
        ),
    ])
}

pub(crate) fn activity_read_model_payload(
    read_model_kind: ReadModelKind,
    state: ActivityReadModelState,
    row_count: usize,
    read_model_json: ReadModelJson,
) -> LogFields {
    fields_from_pairs(vec![
        (
            constants::field::ACTIVITY_READ_MODEL_KIND,
            LogFieldValue::String(read_model_kind.0),
        ),
        (
            constants::field::ACTIVITY_SURFACE_STATE,
            LogFieldValue::String(
                read_model_state_value::read_model_state_value(state)
                    .0
                    .to_string(),
            ),
        ),
        (
            constants::field::RETURNED,
            LogFieldValue::Number(row_count as f64),
        ),
        (
            constants::field::ACTIVITY_READ_MODEL,
            LogFieldValue::String(read_model_json.0),
        ),
    ])
}

fn report_state(report: &ActivityReportDocument) -> SurfaceText {
    if report
        .sections
        .iter()
        .any(|section| section.state == ActivityReadModelState::Ready)
    {
        return read_model_state_value::read_model_state_value(ActivityReadModelState::Ready);
    }

    report
        .sections
        .first()
        .map(|section| read_model_state_value::read_model_state_value(section.state))
        .unwrap_or_else(|| {
            read_model_state_value::read_model_state_value(ActivityReadModelState::Empty)
        })
}

fn report_frequency_value(frequency: ActivityReportFrequency) -> SurfaceText {
    match frequency {
        ActivityReportFrequency::Daily => SurfaceText(constants::activity_surface::FREQUENCY_DAILY),
        ActivityReportFrequency::Weekly => {
            SurfaceText(constants::activity_surface::FREQUENCY_WEEKLY)
        }
        ActivityReportFrequency::Monthly => {
            SurfaceText(constants::activity_surface::FREQUENCY_MONTHLY)
        }
    }
}

fn json_string(value: &ActivityReportDocument) -> JsonText {
    serialized_json(value)
}

fn history_json_string(value: &ActivityHistoricalReportList) -> JsonText {
    serialized_json(value)
}

fn serialized_json<T>(value: &T) -> JsonText
where
    T: serde::Serialize,
{
    JsonText(serde_json::to_string(value).unwrap_or_else(|_| {
        serde_json::Value::String(constants::error::AGENT_EVENT_SERIALIZES.to_string()).to_string()
    }))
}
