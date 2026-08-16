#[path = "network_runtime_stream_events/event_kind.rs"]
mod event_kind;
#[path = "network_runtime_stream_events/protocol_payload.rs"]
mod protocol_payload;
#[path = "network_runtime_stream_events/stream_entries.rs"]
mod stream_entries;

use serde::{ser::SerializeStruct, Serialize, Serializer};
use serde_json::Value;

use ocentra_parent_agent_protocol::constants;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct NetworkRuntimeServiceStreamEntry {
    pub(crate) stream_type: String,
    pub(crate) event_ref: String,
    pub(crate) payload: Value,
}

impl Serialize for NetworkRuntimeServiceStreamEntry {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut entry =
            serializer.serialize_struct(constants::field::NETWORK_RUNTIME_EVENT_CHAIN_STREAM, 3)?;
        entry.serialize_field(constants::field::EVENT_TYPE, &self.stream_type)?;
        entry.serialize_field(constants::field::EVENT_REF, &self.event_ref)?;
        entry.serialize_field(constants::field::PAYLOAD, &self.payload)?;
        entry.end()
    }
}

pub(crate) fn stream_entries_from_report(
    report: &ocentra_parent_agent_core::network_event_runtime::NetworkRuntimeReport,
) -> Vec<NetworkRuntimeServiceStreamEntry> {
    stream_entries::stream_entries_from_report(report)
}
