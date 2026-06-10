use ocentra_eventing::{EventingError, SourceComponent};

use super::{
    NetworkRuntimeRemoteDeliveryDispatchReadinessError,
    NetworkRuntimeRemoteDeliveryDispatchReadinessReport,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NetworkRuntimeRemoteDeliveryNoEnforcementInvariantState {
    AvailableMetadataNonEnforcing,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NetworkRuntimeRemoteDeliveryNoEnforcementStage {
    RemoteDeliveryStatus,
    EventChainJournal,
    ReceiptLedger,
    DurableEnvelope,
    OutboxHandoff,
    DispatchReadiness,
}

#[derive(Clone, Debug)]
pub struct NetworkRuntimeRemoteDeliveryNoEnforcementInvariantReport {
    pub dispatch_readiness: NetworkRuntimeRemoteDeliveryDispatchReadinessReport,
    pub invariant_ref: SourceComponent,
    pub available_metadata_ref: SourceComponent,
    pub state: NetworkRuntimeRemoteDeliveryNoEnforcementInvariantState,
    pub stages: Vec<NetworkRuntimeRemoteDeliveryNoEnforcementStage>,
    pub remote_metadata_stage_count: usize,
    pub available_metadata_refs: Vec<SourceComponent>,
    pub available_metadata_ref_count: usize,
    pub manual_required_candidate_count: usize,
    pub dispatch_ready_candidate_count: usize,
    pub dispatch_attempt_count: usize,
    pub remote_ack_count: usize,
    pub broker_delivery_implemented: bool,
    pub family_hub_delivery_implemented: bool,
    pub remote_delivery_ack_implemented: bool,
    pub provider_delivery_implemented: bool,
    pub child_device_delivery_implemented: bool,
    pub remote_delete_export_propagation_implemented: bool,
    pub product_ready_remote_delivery: bool,
    pub policy_authority: bool,
    pub side_effect_authority: bool,
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NetworkRuntimeRemoteDeliveryNoEnforcementInvariantError {
    DispatchReadiness(NetworkRuntimeRemoteDeliveryDispatchReadinessError),
    Eventing(EventingError),
    MissingAvailableMetadata,
    UnsupportedClaim,
}

impl From<EventingError> for NetworkRuntimeRemoteDeliveryNoEnforcementInvariantError {
    fn from(error: EventingError) -> Self {
        Self::Eventing(error)
    }
}
