use ocentra_eventing::error::EventingError;
use ocentra_eventing::ids::{CorrelationId, EventId, EventType, SourceComponent};

use super::{
    NetworkRuntimeRemoteDeliveryProviderChildReadinessError,
    NetworkRuntimeRemoteDeliveryProviderChildReadinessReport,
    NetworkRuntimeRemoteDeliveryProviderChildReadinessState,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NetworkRuntimeRemoteDeliveryCrossProcessCustodyReadinessState {
    ManualRequiredUnavailable,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NetworkRuntimeRemoteDeliveryCrossProcessCustodyReadinessRecord {
    pub sequence: u64,
    pub event_id: EventId,
    pub event_type: EventType,
    pub correlation_id: CorrelationId,
    pub source_provider_state: NetworkRuntimeRemoteDeliveryProviderChildReadinessState,
    pub source_child_device_state: NetworkRuntimeRemoteDeliveryProviderChildReadinessState,
    pub fixture_ack_ref: SourceComponent,
    pub provider_readiness_ref: SourceComponent,
    pub child_device_readiness_ref: SourceComponent,
    pub cross_process_custody_status_ref: SourceComponent,
    pub cross_process_replay_readiness_ref: SourceComponent,
    pub remote_retention_readiness_ref: SourceComponent,
    pub remote_delete_custody_readiness_ref: SourceComponent,
    pub remote_export_custody_readiness_ref: SourceComponent,
    pub custody_state: NetworkRuntimeRemoteDeliveryCrossProcessCustodyReadinessState,
}

#[derive(Clone, Debug)]
pub struct NetworkRuntimeRemoteDeliveryCrossProcessCustodyReadinessReport {
    pub provider_child_readiness: NetworkRuntimeRemoteDeliveryProviderChildReadinessReport,
    pub cross_process_custody_status_ref: SourceComponent,
    pub cross_process_replay_readiness_ref: SourceComponent,
    pub remote_retention_readiness_ref: SourceComponent,
    pub remote_delete_custody_readiness_ref: SourceComponent,
    pub remote_export_custody_readiness_ref: SourceComponent,
    pub custody_state: NetworkRuntimeRemoteDeliveryCrossProcessCustodyReadinessState,
    pub source_provider_child_readiness_record_count: usize,
    pub cross_process_replay_readiness_record_count: usize,
    pub remote_retention_readiness_record_count: usize,
    pub remote_delete_custody_readiness_record_count: usize,
    pub remote_export_custody_readiness_record_count: usize,
    pub cross_process_replay_artifact_count: usize,
    pub remote_retention_artifact_count: usize,
    pub remote_delete_custody_artifact_count: usize,
    pub remote_export_custody_artifact_count: usize,
    pub custody_records_match_provider_child_readiness: bool,
    pub broker_delivery_implemented: bool,
    pub family_hub_delivery_implemented: bool,
    pub remote_delivery_ack_implemented: bool,
    pub provider_delivery_implemented: bool,
    pub child_device_delivery_implemented: bool,
    pub cross_process_replay_implemented: bool,
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
    pub records: Vec<NetworkRuntimeRemoteDeliveryCrossProcessCustodyReadinessRecord>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NetworkRuntimeRemoteDeliveryCrossProcessCustodyReadinessError {
    ProviderChildReadiness(NetworkRuntimeRemoteDeliveryProviderChildReadinessError),
    Eventing(EventingError),
    EmptyProviderChildReadiness,
    UnsupportedClaim,
    CustodyRecordMismatch,
}

impl From<EventingError> for NetworkRuntimeRemoteDeliveryCrossProcessCustodyReadinessError {
    fn from(error: EventingError) -> Self {
        Self::Eventing(error)
    }
}
