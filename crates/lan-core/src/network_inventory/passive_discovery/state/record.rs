use super::super::{
    labels, LanPassiveDiscoveryDeviceId, LanPassiveDiscoveryEventRow,
    LanPassiveDiscoveryListenerState, LanPassiveDiscoveryRecordOutcome,
    LanPassiveDiscoveryScanSessionId,
};
use super::PassiveDiscoveryEventInput;

pub(super) fn record_event(
    state: &mut LanPassiveDiscoveryListenerState,
    input: PassiveDiscoveryEventInput<'_>,
) -> LanPassiveDiscoveryRecordOutcome {
    let PassiveDiscoveryEventInput {
        event_kind,
        source,
        trigger_reason,
        observed_at,
        device_id,
        scan_session_id,
        summary,
    } = input;
    if !state.is_running() {
        return LanPassiveDiscoveryRecordOutcome::Stopped;
    }

    let event_id = labels::passive_event_id(
        &event_kind,
        source.as_ref(),
        &trigger_reason,
        observed_at,
        device_id,
        scan_session_id,
    );
    if state
        .rows
        .iter()
        .any(|row| row.event_id.as_str().eq_ignore_ascii_case(&event_id))
    {
        return LanPassiveDiscoveryRecordOutcome::Deduplicated;
    }

    if state.rows.len() >= state.max_rows {
        let _ = state.rows.pop_front();
        state.dropped_row_count = state.dropped_row_count.saturating_add(1);
    }

    let previous_event_id = state.rows.back().map(|row| row.event_id.clone());
    state.rows.push_back(LanPassiveDiscoveryEventRow {
        schema_version: LanPassiveDiscoveryListenerState::SCHEMA_VERSION,
        event_id: event_id.into(),
        event_kind,
        observed_at: observed_at.to_string(),
        previous_event_id,
        source,
        trigger_reason,
        device_id: device_id.map(LanPassiveDiscoveryDeviceId::from),
        scan_session_id: scan_session_id.map(LanPassiveDiscoveryScanSessionId::from),
        summary,
    });
    LanPassiveDiscoveryRecordOutcome::Recorded
}
