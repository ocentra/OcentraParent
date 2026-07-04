use std::collections::VecDeque;

use super::{
    labels, LanPassiveDiscoveryDeviceId, LanPassiveDiscoveryEventHistory,
    LanPassiveDiscoveryEventKind, LanPassiveDiscoveryEventRow,
    LanPassiveDiscoveryListenerLifecycleState, LanPassiveDiscoveryListenerState,
    LanPassiveDiscoveryPacket, LanPassiveDiscoveryPacketIngestOutcome,
    LanPassiveDiscoveryRecordOutcome, LanPassiveDiscoveryScanSessionId, LanPassiveDiscoverySource,
    LanPassiveDiscoveryTriggerReason,
};

struct PassiveDiscoveryEventInput<'a> {
    event_kind: LanPassiveDiscoveryEventKind,
    source: Option<LanPassiveDiscoverySource>,
    trigger_reason: LanPassiveDiscoveryTriggerReason,
    observed_at: &'a str,
    device_id: Option<&'a str>,
    scan_session_id: Option<&'a str>,
    summary: String,
}

impl LanPassiveDiscoveryListenerState {
    pub const SCHEMA_VERSION: u16 = 1;
    pub const DEFAULT_MAX_ROWS: usize = 128;

    pub fn running(generated_at: String) -> Self {
        Self::with_capacity(generated_at, Self::DEFAULT_MAX_ROWS)
    }

    pub fn with_capacity(generated_at: String, max_rows: usize) -> Self {
        Self {
            generated_at,
            lifecycle_state: LanPassiveDiscoveryListenerLifecycleState::Running,
            max_rows: max_rows.max(1),
            dropped_row_count: 0,
            rows: VecDeque::new(),
        }
    }

    pub fn lifecycle_state(&self) -> LanPassiveDiscoveryListenerLifecycleState {
        self.lifecycle_state.clone()
    }

    pub fn is_running(&self) -> bool {
        self.lifecycle_state == LanPassiveDiscoveryListenerLifecycleState::Running
    }

    pub fn stop(&mut self) {
        self.lifecycle_state = LanPassiveDiscoveryListenerLifecycleState::Stopped;
    }

    pub fn ingest_udp_packet(&mut self, payload: &[u8]) -> LanPassiveDiscoveryPacketIngestOutcome {
        if !self.is_running() {
            return LanPassiveDiscoveryPacketIngestOutcome::Stopped;
        }

        match super::packet::parse_passive_discovery_packet(payload) {
            Ok(packet) => match self.record_passive_packet(packet) {
                LanPassiveDiscoveryRecordOutcome::Recorded => {
                    LanPassiveDiscoveryPacketIngestOutcome::Recorded
                }
                LanPassiveDiscoveryRecordOutcome::Deduplicated => {
                    LanPassiveDiscoveryPacketIngestOutcome::Deduplicated
                }
                LanPassiveDiscoveryRecordOutcome::Stopped => {
                    LanPassiveDiscoveryPacketIngestOutcome::Stopped
                }
            },
            Err(error) => LanPassiveDiscoveryPacketIngestOutcome::Rejected(error),
        }
    }

    pub fn record_passive_packet(
        &mut self,
        packet: LanPassiveDiscoveryPacket,
    ) -> LanPassiveDiscoveryRecordOutcome {
        self.record_passive_update(
            packet.source,
            packet.trigger_reason,
            &packet.observed_at,
            packet
                .device_id
                .as_ref()
                .map(LanPassiveDiscoveryDeviceId::as_str),
            packet
                .scan_session_id
                .as_ref()
                .map(LanPassiveDiscoveryScanSessionId::as_str),
            packet.summary,
        )
    }

    pub fn record_passive_update(
        &mut self,
        source: LanPassiveDiscoverySource,
        trigger_reason: LanPassiveDiscoveryTriggerReason,
        observed_at: &str,
        device_id: Option<&str>,
        scan_session_id: Option<&str>,
        summary: impl Into<String>,
    ) -> LanPassiveDiscoveryRecordOutcome {
        self.record_event(PassiveDiscoveryEventInput {
            event_kind: LanPassiveDiscoveryEventKind::PassiveUpdate,
            source: Some(source),
            trigger_reason,
            observed_at,
            device_id,
            scan_session_id,
            summary: summary.into(),
        })
    }

    pub fn record_rescan_trigger(
        &mut self,
        trigger_reason: LanPassiveDiscoveryTriggerReason,
        observed_at: &str,
        summary: impl Into<String>,
    ) -> LanPassiveDiscoveryRecordOutcome {
        self.record_event(PassiveDiscoveryEventInput {
            event_kind: LanPassiveDiscoveryEventKind::RescanTrigger,
            source: None,
            trigger_reason,
            observed_at,
            device_id: None,
            scan_session_id: None,
            summary: summary.into(),
        })
    }

    pub fn snapshot(&self) -> LanPassiveDiscoveryEventHistory {
        let latest = self.rows.back();
        LanPassiveDiscoveryEventHistory {
            schema_version: Self::SCHEMA_VERSION,
            generated_at: self.generated_at.clone(),
            lifecycle_state: self.lifecycle_state.clone(),
            max_rows: self.max_rows,
            dropped_row_count: self.dropped_row_count,
            latest_event_id: latest.map(|row| row.event_id.clone()),
            latest_observed_at: latest.map(|row| row.observed_at.clone()),
            rows: self.rows.iter().cloned().collect(),
        }
    }

    pub fn rows(&self) -> Vec<LanPassiveDiscoveryEventRow> {
        self.rows.iter().cloned().collect()
    }

    fn record_event(
        &mut self,
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
        if !self.is_running() {
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

        if self
            .rows
            .iter()
            .any(|row| row.event_id.as_str().eq_ignore_ascii_case(&event_id))
        {
            return LanPassiveDiscoveryRecordOutcome::Deduplicated;
        }

        if self.rows.len() >= self.max_rows {
            let _ = self.rows.pop_front();
            self.dropped_row_count = self.dropped_row_count.saturating_add(1);
        }

        let previous_event_id = self.rows.back().map(|row| row.event_id.clone());
        self.rows.push_back(LanPassiveDiscoveryEventRow {
            schema_version: Self::SCHEMA_VERSION,
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
}
