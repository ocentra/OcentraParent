use ocentra_eventing::{EventingError, ReplayMode, SourceComponent};

use super::remote_delivery_status::NetworkRuntimeRemoteDeliveryStatusReport;
use super::{NetworkRuntimeEventPayload, NetworkRuntimeRemoteDeliveryStatusError};

#[derive(Clone, Debug)]
pub struct NetworkRuntimeRemoteEventChainJournalReport {
    pub remote_delivery_status: NetworkRuntimeRemoteDeliveryStatusReport,
    pub event_chain_journal_ref: SourceComponent,
    pub event_chain_replay_ref: SourceComponent,
    pub event_chain_export_ref: SourceComponent,
    pub event_chain_support_status_ref: SourceComponent,
    pub stored_event_count: usize,
    pub journal_entry_count: usize,
    pub projection_replay_record_count: usize,
    pub replay_cursor_next_sequence: u64,
    pub exported_event_type_count: usize,
    pub exportable_remote_envelope_count: usize,
    pub unavailable_event_count: usize,
    pub enforcement_command_event_count: usize,
    pub adapter_action_executed_count: usize,
    pub exact_url_available_count: usize,
    pub decrypted_payload_available_count: usize,
    pub page_content_available_count: usize,
    pub projection_replay_mode: ReplayMode,
    pub durable_envelope_ready: bool,
    pub broker_delivery_implemented: bool,
    pub family_hub_delivery_implemented: bool,
    pub provider_delivery_implemented: bool,
    pub child_device_delivery_implemented: bool,
    pub product_ready_claimed: bool,
    pub policy_authority: bool,
    pub side_effect_authority: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NetworkRuntimeRemoteEventChainJournalError {
    RemoteDeliveryStatus(NetworkRuntimeRemoteDeliveryStatusError),
    Eventing(EventingError),
    EmptyJournal,
    ReplayMismatch,
    UnsupportedClaim,
}

pub(super) struct UnsupportedClaimCounts {
    pub enforcement_command_event_count: usize,
    pub adapter_action_executed_count: usize,
    pub exact_url_available_count: usize,
    pub decrypted_payload_available_count: usize,
    pub page_content_available_count: usize,
}

impl UnsupportedClaimCounts {
    pub(super) fn has_any(&self) -> bool {
        self.enforcement_command_event_count
            + self.adapter_action_executed_count
            + self.exact_url_available_count
            + self.decrypted_payload_available_count
            + self.page_content_available_count
            > 0
    }
}

impl From<EventingError> for NetworkRuntimeRemoteEventChainJournalError {
    fn from(error: EventingError) -> Self {
        Self::Eventing(error)
    }
}

pub(super) fn count_payloads(
    payloads: &[NetworkRuntimeEventPayload],
    predicate: impl Fn(&NetworkRuntimeEventPayload) -> bool,
) -> usize {
    payloads.iter().filter(|payload| predicate(payload)).count()
}
