use serde::{Deserialize, Serialize};

use crate::live_capture::{NetworkLiveCapturePlatform, NetworkLiveCaptureProof};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkLiveCaptureExecutionSource {
    WindowsNpcapDriver,
    LinuxLibpcapDriver,
    MacosBpfLibpcapDriver,
    MetadataSnapshotOnly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkLiveCaptureExecutionState {
    BoundedExecuted,
    ManualRequired,
    Unavailable,
    Degraded,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkLiveCaptureExecutionRequiredArtifact {
    ProofReadyLiveCapture,
    DriverInvocation,
    InterfaceObservation,
    Permission,
    BoundedWindow,
    CleanStop,
    Custody,
    RetentionDeleteExport,
    MetadataOnlySanitization,
    PrivateTrafficExclusion,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkLiveCaptureExecutionInput {
    pub execution_ref: String,
    pub live_capture_proof: NetworkLiveCaptureProof,
    pub source: NetworkLiveCaptureExecutionSource,
    pub driver_invocation_ref: Option<String>,
    pub interface_observation_ref: Option<String>,
    pub permission_ref: Option<String>,
    pub bounded_window_ref: Option<String>,
    pub clean_stop_ref: Option<String>,
    pub custody_ref: Option<String>,
    pub retention_delete_export_ref: Option<String>,
    pub metadata_only_sanitization_ref: Option<String>,
    pub private_traffic_exclusion_ref: Option<String>,
    pub driver_invoked: bool,
    pub live_capture_executed: bool,
    pub metadata_snapshot_executed: bool,
    pub captured_packet_count: usize,
    pub raw_artifact_created: bool,
    pub netstat_metadata_substitution_claimed: bool,
    pub unbounded_capture_claimed: bool,
    pub raw_pcap_without_custody_claimed: bool,
    pub exact_url_claimed: bool,
    pub decrypted_payload_claimed: bool,
    pub page_content_claimed: bool,
    pub private_message_claimed: bool,
    pub search_query_claimed: bool,
    pub policy_authority_claimed: bool,
    pub adapter_authority_claimed: bool,
    pub host_filtering_claimed: bool,
    pub enforcement_command_claimed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkLiveCaptureExecutionProof {
    pub execution_ref: String,
    pub capture_proof_ref: String,
    pub platform: NetworkLiveCapturePlatform,
    pub source: NetworkLiveCaptureExecutionSource,
    pub execution_state: NetworkLiveCaptureExecutionState,
    pub missing_artifacts: Vec<NetworkLiveCaptureExecutionRequiredArtifact>,
    pub driver_invocation_ref: Option<String>,
    pub interface_observation_ref: Option<String>,
    pub permission_ref: Option<String>,
    pub bounded_window_ref: Option<String>,
    pub clean_stop_ref: Option<String>,
    pub custody_ref: Option<String>,
    pub retention_delete_export_ref: Option<String>,
    pub metadata_only_sanitization_ref: Option<String>,
    pub private_traffic_exclusion_ref: Option<String>,
    pub driver_invoked: bool,
    pub live_capture_executed: bool,
    pub metadata_snapshot_executed: bool,
    pub captured_packet_count: usize,
    pub raw_artifact_created: bool,
    pub raw_pcap_without_custody_available: bool,
    pub exact_url_available: bool,
    pub decrypted_payload_available: bool,
    pub page_content_available: bool,
    pub private_message_available: bool,
    pub search_query_available: bool,
    pub policy_authority: bool,
    pub adapter_authority: bool,
    pub host_filtering_claimed: bool,
    pub enforcement_commands_published: usize,
    pub netstat_metadata_substituted_for_live_capture: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkLiveCaptureExecutionError {
    EmptyExecutionRef,
    EmptyArtifactRef,
    SourcePlatformMismatch,
    MetadataSnapshotCannotClaimDriverExecution,
    DriverExecutionRequiresPacketObservation,
    RawArtifactCreationRejected,
    NetstatSubstitutionClaimRejected,
    UnboundedCaptureClaimRejected,
    RawPcapWithoutCustodyClaimRejected,
    ExactUrlClaimRejected,
    DecryptedPayloadClaimRejected,
    PageContentClaimRejected,
    PrivateMessageClaimRejected,
    SearchQueryClaimRejected,
    PolicyAuthorityClaimRejected,
    AdapterAuthorityClaimRejected,
    HostFilteringClaimRejected,
    EnforcementCommandClaimRejected,
}
