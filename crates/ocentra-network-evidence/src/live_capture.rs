use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkLiveCapturePlatform {
    WindowsNpcap,
    LinuxLibpcap,
    MacosBpfLibpcap,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkLiveCaptureProofState {
    ProofReady,
    ManualRequired,
    Unavailable,
    Degraded,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkLiveCaptureRequiredArtifact {
    DriverProof,
    InterfaceEnumeration,
    PermissionProof,
    BoundedCaptureProof,
    CleanStopProof,
    QuotaRotationProof,
    RetentionDeleteExportProof,
    CustodyProof,
    PrivateTrafficExclusionProof,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkLiveCaptureProofInput {
    pub capture_proof_ref: String,
    pub platform: NetworkLiveCapturePlatform,
    pub interface_ref: Option<String>,
    pub driver_proof_ref: Option<String>,
    pub permission_proof_ref: Option<String>,
    pub bounded_capture_ref: Option<String>,
    pub clean_stop_ref: Option<String>,
    pub quota_rotation_ref: Option<String>,
    pub retention_delete_export_ref: Option<String>,
    pub custody_ref: Option<String>,
    pub private_traffic_exclusion_ref: Option<String>,
    pub platform_available: bool,
    pub driver_available: bool,
    pub permission_granted: bool,
    pub interface_enumerated: bool,
    pub bounded_capture_succeeded: bool,
    pub clean_stop_succeeded: bool,
    pub adapter_degraded: bool,
    pub live_capture_execution_claimed: bool,
    pub unbounded_capture_claimed: bool,
    pub raw_pcap_without_custody_claimed: bool,
    pub exact_url_claimed: bool,
    pub decrypted_payload_claimed: bool,
    pub page_content_claimed: bool,
    pub private_message_claimed: bool,
    pub search_query_claimed: bool,
    pub policy_authority_claimed: bool,
    pub adapter_authority_claimed: bool,
    pub enforcement_command_claimed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkLiveCaptureProof {
    pub capture_proof_ref: String,
    pub platform: NetworkLiveCapturePlatform,
    pub proof_state: NetworkLiveCaptureProofState,
    pub missing_artifacts: Vec<NetworkLiveCaptureRequiredArtifact>,
    pub interface_ref: Option<String>,
    pub driver_proof_ref: Option<String>,
    pub permission_proof_ref: Option<String>,
    pub bounded_capture_ref: Option<String>,
    pub clean_stop_ref: Option<String>,
    pub quota_rotation_ref: Option<String>,
    pub retention_delete_export_ref: Option<String>,
    pub custody_ref: Option<String>,
    pub private_traffic_exclusion_ref: Option<String>,
    pub capture_ready: bool,
    pub driver_invoked: bool,
    pub live_capture_executed: bool,
    pub raw_pcap_without_custody_available: bool,
    pub exact_url_available: bool,
    pub decrypted_payload_available: bool,
    pub page_content_available: bool,
    pub private_message_available: bool,
    pub search_query_available: bool,
    pub policy_authority: bool,
    pub adapter_authority: bool,
    pub enforcement_commands_published: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkLiveCaptureProofError {
    EmptyCaptureProofRef,
    EmptyArtifactRef,
    LiveCaptureExecutionClaimRejected,
    UnboundedCaptureClaimRejected,
    RawPcapWithoutCustodyClaimRejected,
    ExactUrlClaimRejected,
    DecryptedPayloadClaimRejected,
    PageContentClaimRejected,
    PrivateMessageClaimRejected,
    SearchQueryClaimRejected,
    PolicyAuthorityClaimRejected,
    AdapterAuthorityClaimRejected,
    EnforcementCommandClaimRejected,
}

pub fn plan_network_live_capture_proof(
    input: NetworkLiveCaptureProofInput,
) -> Result<NetworkLiveCaptureProof, NetworkLiveCaptureProofError> {
    validate_input(&input)?;

    let missing_artifacts = missing_artifacts(&input);
    let proof_state = if !input.platform_available {
        NetworkLiveCaptureProofState::Unavailable
    } else if input.adapter_degraded {
        NetworkLiveCaptureProofState::Degraded
    } else if missing_artifacts.is_empty() {
        NetworkLiveCaptureProofState::ProofReady
    } else {
        NetworkLiveCaptureProofState::ManualRequired
    };

    Ok(NetworkLiveCaptureProof {
        capture_proof_ref: input.capture_proof_ref,
        platform: input.platform,
        proof_state,
        missing_artifacts,
        interface_ref: input.interface_ref,
        driver_proof_ref: input.driver_proof_ref,
        permission_proof_ref: input.permission_proof_ref,
        bounded_capture_ref: input.bounded_capture_ref,
        clean_stop_ref: input.clean_stop_ref,
        quota_rotation_ref: input.quota_rotation_ref,
        retention_delete_export_ref: input.retention_delete_export_ref,
        custody_ref: input.custody_ref,
        private_traffic_exclusion_ref: input.private_traffic_exclusion_ref,
        capture_ready: proof_state == NetworkLiveCaptureProofState::ProofReady,
        driver_invoked: false,
        live_capture_executed: false,
        raw_pcap_without_custody_available: false,
        exact_url_available: false,
        decrypted_payload_available: false,
        page_content_available: false,
        private_message_available: false,
        search_query_available: false,
        policy_authority: false,
        adapter_authority: false,
        enforcement_commands_published: 0,
    })
}

fn validate_input(
    input: &NetworkLiveCaptureProofInput,
) -> Result<(), NetworkLiveCaptureProofError> {
    if input.capture_proof_ref.trim().is_empty() {
        return Err(NetworkLiveCaptureProofError::EmptyCaptureProofRef);
    }
    for artifact_ref in artifact_refs(input).into_iter().flatten() {
        if artifact_ref.trim().is_empty() {
            return Err(NetworkLiveCaptureProofError::EmptyArtifactRef);
        }
    }
    validate_claims(input)
}

fn validate_claims(
    input: &NetworkLiveCaptureProofInput,
) -> Result<(), NetworkLiveCaptureProofError> {
    if input.live_capture_execution_claimed {
        return Err(NetworkLiveCaptureProofError::LiveCaptureExecutionClaimRejected);
    }
    if input.unbounded_capture_claimed {
        return Err(NetworkLiveCaptureProofError::UnboundedCaptureClaimRejected);
    }
    if input.raw_pcap_without_custody_claimed {
        return Err(NetworkLiveCaptureProofError::RawPcapWithoutCustodyClaimRejected);
    }
    if input.exact_url_claimed {
        return Err(NetworkLiveCaptureProofError::ExactUrlClaimRejected);
    }
    if input.decrypted_payload_claimed {
        return Err(NetworkLiveCaptureProofError::DecryptedPayloadClaimRejected);
    }
    if input.page_content_claimed {
        return Err(NetworkLiveCaptureProofError::PageContentClaimRejected);
    }
    if input.private_message_claimed {
        return Err(NetworkLiveCaptureProofError::PrivateMessageClaimRejected);
    }
    if input.search_query_claimed {
        return Err(NetworkLiveCaptureProofError::SearchQueryClaimRejected);
    }
    if input.policy_authority_claimed {
        return Err(NetworkLiveCaptureProofError::PolicyAuthorityClaimRejected);
    }
    if input.adapter_authority_claimed {
        return Err(NetworkLiveCaptureProofError::AdapterAuthorityClaimRejected);
    }
    if input.enforcement_command_claimed {
        return Err(NetworkLiveCaptureProofError::EnforcementCommandClaimRejected);
    }
    Ok(())
}

fn missing_artifacts(
    input: &NetworkLiveCaptureProofInput,
) -> Vec<NetworkLiveCaptureRequiredArtifact> {
    let mut missing = Vec::new();
    require(
        input.driver_available && input.driver_proof_ref.is_some(),
        NetworkLiveCaptureRequiredArtifact::DriverProof,
        &mut missing,
    );
    require(
        input.interface_enumerated && input.interface_ref.is_some(),
        NetworkLiveCaptureRequiredArtifact::InterfaceEnumeration,
        &mut missing,
    );
    require(
        input.permission_granted && input.permission_proof_ref.is_some(),
        NetworkLiveCaptureRequiredArtifact::PermissionProof,
        &mut missing,
    );
    require(
        input.bounded_capture_succeeded && input.bounded_capture_ref.is_some(),
        NetworkLiveCaptureRequiredArtifact::BoundedCaptureProof,
        &mut missing,
    );
    require(
        input.clean_stop_succeeded && input.clean_stop_ref.is_some(),
        NetworkLiveCaptureRequiredArtifact::CleanStopProof,
        &mut missing,
    );
    require(
        input.quota_rotation_ref.is_some(),
        NetworkLiveCaptureRequiredArtifact::QuotaRotationProof,
        &mut missing,
    );
    require(
        input.retention_delete_export_ref.is_some(),
        NetworkLiveCaptureRequiredArtifact::RetentionDeleteExportProof,
        &mut missing,
    );
    require(
        input.custody_ref.is_some(),
        NetworkLiveCaptureRequiredArtifact::CustodyProof,
        &mut missing,
    );
    require(
        input.private_traffic_exclusion_ref.is_some(),
        NetworkLiveCaptureRequiredArtifact::PrivateTrafficExclusionProof,
        &mut missing,
    );
    missing
}

fn require(
    condition: bool,
    artifact: NetworkLiveCaptureRequiredArtifact,
    missing: &mut Vec<NetworkLiveCaptureRequiredArtifact>,
) {
    if !condition {
        missing.push(artifact);
    }
}

fn artifact_refs(input: &NetworkLiveCaptureProofInput) -> [Option<&str>; 9] {
    [
        input.interface_ref.as_deref(),
        input.driver_proof_ref.as_deref(),
        input.permission_proof_ref.as_deref(),
        input.bounded_capture_ref.as_deref(),
        input.clean_stop_ref.as_deref(),
        input.quota_rotation_ref.as_deref(),
        input.retention_delete_export_ref.as_deref(),
        input.custody_ref.as_deref(),
        input.private_traffic_exclusion_ref.as_deref(),
    ]
}
