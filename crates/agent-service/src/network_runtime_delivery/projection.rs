use std::collections::BTreeSet;

use ocentra_eventing::ids::EventId;
use ocentra_parent_agent_core::network_event_runtime::{
    network_runtime_event_ids_for_source_event, NetworkRuntimeEventPayload, NetworkRuntimeReport,
};
use ocentra_parent_agent_protocol::network_flow::ActivityNetworkFlowReadModel;

use super::NetworkRuntimeProjectionRowCounts;

#[derive(Clone, Debug, Default)]
pub(super) struct RetainedEventIds(BTreeSet<EventId>);

impl RetainedEventIds {
    pub(super) fn contains(&self, event_id: &EventId) -> bool {
        self.0.contains(event_id)
    }
}

#[derive(Clone, Copy)]
pub(super) struct EventNameRef<'a>(pub(super) &'a str);

pub(super) fn retained_event_ids_for_rows(
    read_model: &ActivityNetworkFlowReadModel,
) -> RetainedEventIds {
    RetainedEventIds(
        read_model
            .rows
            .iter()
            .flat_map(expected_event_ids_for_row)
            .collect(),
    )
}

pub(super) fn network_runtime_projection_row_counts(
    read_model: &ActivityNetworkFlowReadModel,
    report: &NetworkRuntimeReport,
) -> NetworkRuntimeProjectionRowCounts {
    let delivered_rows = read_model
        .rows
        .iter()
        .filter(|row| row_has_all_expected_events(row, report))
        .count();
    let manual_required_rows = read_model
        .rows
        .iter()
        .filter(|row| row_has_manual_required_event(row, report))
        .count();
    NetworkRuntimeProjectionRowCounts {
        delivered_rows,
        failed_rows: read_model.rows.len().saturating_sub(delivered_rows),
        manual_required_rows,
    }
}

pub(super) fn count_event_type(
    report: &NetworkRuntimeReport,
    event_name: EventNameRef<'_>,
) -> usize {
    report
        .stored_events
        .iter()
        .filter(|event| event.contract.event_type.as_str() == event_name.0)
        .count()
}

fn expected_event_ids_for_row(
    row: &ocentra_parent_agent_protocol::network_flow::ActivityNetworkFlowObservation,
) -> Vec<EventId> {
    EventId::parse(row.event_id.clone())
        .ok()
        .and_then(|source_event_id| {
            network_runtime_event_ids_for_source_event(
                &source_event_id,
                &super::network_runtime_observation_from_row(row).ok()?,
            )
            .ok()
        })
        .unwrap_or_default()
}

fn row_has_all_expected_events(
    row: &ocentra_parent_agent_protocol::network_flow::ActivityNetworkFlowObservation,
    report: &NetworkRuntimeReport,
) -> bool {
    let expected = expected_event_ids_for_row(row);
    !expected.is_empty()
        && expected.iter().all(|event_id| {
            report
                .stored_events
                .iter()
                .any(|event| event.event_id.as_str() == event_id.as_str())
        })
}

fn row_has_manual_required_event(
    row: &ocentra_parent_agent_protocol::network_flow::ActivityNetworkFlowObservation,
    report: &NetworkRuntimeReport,
) -> bool {
    expected_event_ids_for_row(row).iter().any(|event_id| {
        report.stored_events.iter().any(|event| {
            event.event_id.as_str() == event_id.as_str()
                && event
                    .decode::<NetworkRuntimeEventPayload>()
                    .map(|payload| {
                        payload.payload.intervention_state
                            == ocentra_parent_agent_protocol::network_flow::NetworkInterventionState::ManualRequired
                    })
                    .unwrap_or(false)
        })
    })
}
