use ocentra_eventing::error::EventingError;

use super::remote_delivery_status::NetworkRuntimeRemoteDeliveryStatusError;
use super::NetworkRuntimeEventPayload;

pub type NetworkRuntimeRemoteEventChainJournalReport =
    ocentra_parent_agent_protocol::network_flow::remote_delivery_reports::NetworkRuntimeRemoteEventChainJournalReport;

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
    pub raw_pcap_available_count: usize,
    pub exact_url_available_count: usize,
    pub decrypted_payload_available_count: usize,
    pub page_content_available_count: usize,
    pub video_content_available_count: usize,
    pub private_message_content_available_count: usize,
    pub search_query_available_count: usize,
}

impl UnsupportedClaimCounts {
    pub(super) fn has_any(&self) -> bool {
        self.enforcement_command_event_count
            + self.adapter_action_executed_count
            + self.raw_pcap_available_count
            + self.exact_url_available_count
            + self.decrypted_payload_available_count
            + self.page_content_available_count
            + self.video_content_available_count
            + self.private_message_content_available_count
            + self.search_query_available_count
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
