use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingText;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanBrowserAddDeviceReadModel, LanDiscoveryEventHistory, LanDiscoveryEventHistoryState,
    LanDiscoveryEventRow,
};
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};
use serde::{ser::SerializeStruct, Serialize, Serializer};
use serde_json::Value;

#[path = "lan_runtime_stream_payload/labels.rs"]
mod labels;

use crate::{fields::fields_from_pairs, json_contract};
use labels::{discovery_event_kind_label, discovery_history_state_label};

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct LanRuntimeServiceStreamReport {
    pub(crate) generated_at: String,
    pub(crate) observed_events: usize,
    pub(crate) streamed_events: usize,
    pub(crate) failed_events: usize,
    pub(crate) event_history_state: String,
    pub(crate) manual_required_state: bool,
    pub(crate) latest_event_id: Option<String>,
    pub(crate) latest_observed_at: Option<String>,
    pub(crate) entries: Vec<LanRuntimeServiceStreamEntry>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct LanRuntimeServiceStreamEntry {
    pub(crate) stream_type: String,
    pub(crate) event_ref: String,
    pub(crate) payload: Value,
}

impl Serialize for LanRuntimeServiceStreamEntry {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut entry =
            serializer.serialize_struct(constants::field::LAN_RUNTIME_EVENT_CHAIN_STREAM, 3)?;
        entry.serialize_field(constants::field::EVENT_TYPE, &self.stream_type)?;
        entry.serialize_field(constants::field::EVENT_REF, &self.event_ref)?;
        entry.serialize_field(constants::field::PAYLOAD, &self.payload)?;
        entry.end()
    }
}

pub(crate) fn stream_lan_runtime_event_chain_for_read_model(
    read_model: &LanBrowserAddDeviceReadModel,
) -> LanRuntimeServiceStreamReport {
    stream_lan_runtime_event_chain_for_history(
        &read_model.discovery_event_history,
        LanPairingText(read_model.generated_at.clone()),
    )
}

pub(crate) fn stream_lan_runtime_event_chain_for_history(
    history: &LanDiscoveryEventHistory,
    generated_at: LanPairingText,
) -> LanRuntimeServiceStreamReport {
    let entries = history
        .rows
        .iter()
        .map(stream_entry_from_row)
        .collect::<Vec<_>>();
    LanRuntimeServiceStreamReport {
        generated_at: generated_at.0,
        observed_events: history.rows.len(),
        streamed_events: entries.len(),
        failed_events: 0,
        event_history_state: discovery_history_state_label(&history.state).0,
        manual_required_state: matches!(
            history.state,
            LanDiscoveryEventHistoryState::ManualRequired
        ),
        latest_event_id: history.latest_event_id.clone(),
        latest_observed_at: history.latest_observed_at.clone(),
        entries,
    }
}

pub(crate) fn lan_runtime_event_chain_stream_payload(
    report: &LanRuntimeServiceStreamReport,
) -> LogFields {
    fields_from_pairs(vec![
        (
            constants::field::GENERATED_AT,
            LogFieldValue::String(report.generated_at.clone()),
        ),
        (
            constants::field::LAN_RUNTIME_OBSERVED_EVENTS,
            count_value(report.observed_events),
        ),
        (
            constants::field::LAN_RUNTIME_STREAMED_EVENTS,
            count_value(report.streamed_events),
        ),
        (
            constants::field::LAN_RUNTIME_FAILED_EVENTS,
            count_value(report.failed_events),
        ),
        (
            constants::field::LAN_RUNTIME_EVENT_HISTORY_STATE,
            LogFieldValue::String(report.event_history_state.clone()),
        ),
        (
            constants::field::LAN_RUNTIME_MANUAL_REQUIRED_STATE,
            LogFieldValue::Boolean(report.manual_required_state),
        ),
        (
            constants::field::LATEST_EVENT_ID,
            LogFieldValue::String(report.latest_event_id.clone().unwrap_or_default()),
        ),
        (
            constants::field::LATEST_OBSERVED_AT,
            LogFieldValue::String(report.latest_observed_at.clone().unwrap_or_default()),
        ),
        (
            constants::field::LAN_RUNTIME_EVENT_CHAIN_STREAM,
            LogFieldValue::String(json_contract::serialize_json_string(&report.entries).0),
        ),
    ])
}

fn stream_entry_from_row(row: &LanDiscoveryEventRow) -> LanRuntimeServiceStreamEntry {
    LanRuntimeServiceStreamEntry {
        stream_type: discovery_event_kind_label(&row.event_kind).0,
        event_ref: row.event_id.clone(),
        payload: json_contract::serialize_json_value(row),
    }
}

fn count_value(value: usize) -> LogFieldValue {
    LogFieldValue::Number(value as f64)
}
