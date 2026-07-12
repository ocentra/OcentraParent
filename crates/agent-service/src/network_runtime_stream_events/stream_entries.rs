use ocentra_parent_agent_core::network_event_runtime::NetworkRuntimeReport;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::network_flow::NetworkRuntimeEventPayload;

use crate::network_runtime_stream_event_values as values;

use super::event_kind;
use super::protocol_payload;
use super::NetworkRuntimeServiceStreamEntry;

pub(crate) fn stream_entries_from_report(
    report: &NetworkRuntimeReport,
) -> Vec<NetworkRuntimeServiceStreamEntry> {
    report
        .stored_events
        .iter()
        .filter_map(|event| {
            let decoded = event.decode::<NetworkRuntimeEventPayload>().ok()?;
            let stream_type = event.contract.event_type.as_str().to_string();
            let event_kind = event_kind::from_event_type(&event.contract.event_type)?;
            let mut event_ref = String::from(event.correlation_id.as_str());
            event_ref.push(constants::delimiter::HYPHEN);
            event_ref.push_str(event.contract.event_type.as_str());
            let event_ref = values::NetworkRuntimeStreamRef(event_ref);
            Some(NetworkRuntimeServiceStreamEntry {
                stream_type,
                event_ref: event_ref.0.clone(),
                payload: protocol_payload::protocol_payload(
                    event_kind,
                    &event_ref,
                    &decoded.payload,
                ),
            })
        })
        .collect()
}
