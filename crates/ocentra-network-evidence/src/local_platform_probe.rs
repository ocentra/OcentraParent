mod claims;
mod metrics;
mod observations;

use serde::{Deserialize, Serialize};

use self::{
    claims::{reject_input_claims, reject_status_claims},
    metrics::{count_apple_ci_unavailable, count_host},
    observations::{normalize_observations, validate_observations},
};

use crate::{
    NetworkAdapterCapabilityStatusProof, NetworkAdapterCapabilityStatusState,
    NetworkPlatformClaimTarget,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkLocalPlatformProbeHost {
    Windows,
    AndroidSdk,
    LinuxWsl,
    MacOsCi,
    IosCi,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkLocalPlatformProbeState {
    ReadOnlyObserved,
    LabReady,
    ManualRequired,
    Unavailable,
    CiOnly,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkLocalPlatformProbeUnsupportedClaims {
    pub exact_url_claimed: bool,
    pub decrypted_payload_claimed: bool,
    pub page_content_claimed: bool,
    pub live_adapter_execution_claimed: bool,
    pub enforcement_command_claimed: bool,
    pub ui_policy_authority_claimed: bool,
    pub production_platform_support_claimed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkLocalPlatformProbeObservation {
    pub target: NetworkPlatformClaimTarget,
    pub host: NetworkLocalPlatformProbeHost,
    pub probe_state: NetworkLocalPlatformProbeState,
    pub capability_status: NetworkAdapterCapabilityStatusState,
    pub evidence_refs: Vec<String>,
    pub read_only_probe_executed: bool,
    pub adapter_execution_attempted: bool,
    pub exact_url_claimed: bool,
    pub decrypted_payload_claimed: bool,
    pub page_content_claimed: bool,
    pub production_platform_support_claimed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkLocalPlatformProbeInput {
    pub probe_ref: String,
    pub adapter_status: NetworkAdapterCapabilityStatusProof,
    pub observations: Vec<NetworkLocalPlatformProbeObservation>,
    pub unsupported_claims: NetworkLocalPlatformProbeUnsupportedClaims,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkLocalPlatformProbeProof {
    pub probe_ref: String,
    pub adapter_status_ref: String,
    pub observations: Vec<NetworkLocalPlatformProbeObservation>,
    pub target_count: usize,
    pub windows_probe_count: usize,
    pub android_probe_count: usize,
    pub linux_probe_count: usize,
    pub apple_ci_unavailable_count: usize,
    pub every_observation_matches_adapter_status: bool,
    pub read_only_probes_do_not_execute_adapters: bool,
    pub local_platform_support_claimed: bool,
    pub no_live_adapter_execution_claimed: bool,
    pub no_enforcement_commands_published: bool,
    pub ui_has_no_policy_authority: bool,
    pub exact_url_available: bool,
    pub decrypted_payload_available: bool,
    pub page_content_available: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkLocalPlatformProbeError {
    EmptyProbeRef,
    EmptyAdapterStatusRef,
    EmptyObservationSet,
    EmptyObservationEvidenceRef(NetworkPlatformClaimTarget),
    DuplicateTargetObservation(NetworkPlatformClaimTarget),
    MissingAdapterStatusEntry(NetworkPlatformClaimTarget),
    CapabilityStatusMismatch(NetworkPlatformClaimTarget),
    ProbeStateDoesNotSupportCapability(NetworkPlatformClaimTarget),
    ReadOnlyProbeExecutionRefMissing(NetworkPlatformClaimTarget),
    AdapterExecutionAttemptRejected(NetworkPlatformClaimTarget),
    ExactUrlClaimRejected,
    DecryptedPayloadClaimRejected,
    PageContentClaimRejected,
    LiveAdapterExecutionClaimRejected,
    EnforcementCommandClaimRejected,
    UiPolicyAuthorityClaimRejected,
    ProductionPlatformSupportClaimRejected,
    AdapterStatusClaimsLiveExecution,
    AdapterStatusPublishesEnforcementCommand,
    AdapterStatusAllowsUiPolicyAuthority,
}

pub fn build_network_local_platform_probe_proof(
    input: NetworkLocalPlatformProbeInput,
) -> Result<NetworkLocalPlatformProbeProof, NetworkLocalPlatformProbeError> {
    reject_input_claims(&input.unsupported_claims)?;
    reject_status_claims(&input.adapter_status)?;
    let probe_ref =
        normalize_ref(&input.probe_ref).ok_or(NetworkLocalPlatformProbeError::EmptyProbeRef)?;
    let adapter_status_ref = normalize_ref(&input.adapter_status.status_ref)
        .ok_or(NetworkLocalPlatformProbeError::EmptyAdapterStatusRef)?;
    if input.observations.is_empty() {
        return Err(NetworkLocalPlatformProbeError::EmptyObservationSet);
    }

    let observations = normalize_observations(input.observations)?;
    validate_observations(&observations, &input.adapter_status)?;
    Ok(NetworkLocalPlatformProbeProof {
        windows_probe_count: count_host(&observations, NetworkLocalPlatformProbeHost::Windows),
        android_probe_count: count_host(&observations, NetworkLocalPlatformProbeHost::AndroidSdk),
        linux_probe_count: count_host(&observations, NetworkLocalPlatformProbeHost::LinuxWsl),
        apple_ci_unavailable_count: count_apple_ci_unavailable(&observations),
        target_count: observations.len(),
        probe_ref,
        adapter_status_ref,
        observations,
        every_observation_matches_adapter_status: true,
        read_only_probes_do_not_execute_adapters: true,
        local_platform_support_claimed: false,
        no_live_adapter_execution_claimed: true,
        no_enforcement_commands_published: true,
        ui_has_no_policy_authority: true,
        exact_url_available: false,
        decrypted_payload_available: false,
        page_content_available: false,
    })
}

pub(super) fn normalize_ref(value: &str) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_owned())
    }
}
