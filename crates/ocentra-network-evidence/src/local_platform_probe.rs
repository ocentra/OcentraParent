use serde::{Deserialize, Serialize};

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

fn reject_input_claims(
    claims: &NetworkLocalPlatformProbeUnsupportedClaims,
) -> Result<(), NetworkLocalPlatformProbeError> {
    if claims.exact_url_claimed {
        return Err(NetworkLocalPlatformProbeError::ExactUrlClaimRejected);
    }
    if claims.decrypted_payload_claimed {
        return Err(NetworkLocalPlatformProbeError::DecryptedPayloadClaimRejected);
    }
    if claims.page_content_claimed {
        return Err(NetworkLocalPlatformProbeError::PageContentClaimRejected);
    }
    if claims.live_adapter_execution_claimed {
        return Err(NetworkLocalPlatformProbeError::LiveAdapterExecutionClaimRejected);
    }
    if claims.enforcement_command_claimed {
        return Err(NetworkLocalPlatformProbeError::EnforcementCommandClaimRejected);
    }
    if claims.ui_policy_authority_claimed {
        return Err(NetworkLocalPlatformProbeError::UiPolicyAuthorityClaimRejected);
    }
    if claims.production_platform_support_claimed {
        return Err(NetworkLocalPlatformProbeError::ProductionPlatformSupportClaimRejected);
    }
    Ok(())
}

fn reject_status_claims(
    status: &NetworkAdapterCapabilityStatusProof,
) -> Result<(), NetworkLocalPlatformProbeError> {
    if !status.no_live_adapter_execution_claimed {
        return Err(NetworkLocalPlatformProbeError::AdapterStatusClaimsLiveExecution);
    }
    if !status.no_enforcement_commands_published {
        return Err(NetworkLocalPlatformProbeError::AdapterStatusPublishesEnforcementCommand);
    }
    if !status.ui_has_no_policy_authority {
        return Err(NetworkLocalPlatformProbeError::AdapterStatusAllowsUiPolicyAuthority);
    }
    Ok(())
}

fn normalize_observations(
    observations: Vec<NetworkLocalPlatformProbeObservation>,
) -> Result<Vec<NetworkLocalPlatformProbeObservation>, NetworkLocalPlatformProbeError> {
    let mut normalized = Vec::new();
    for mut observation in observations {
        if normalized
            .iter()
            .any(|current: &NetworkLocalPlatformProbeObservation| {
                current.target == observation.target
            })
        {
            return Err(NetworkLocalPlatformProbeError::DuplicateTargetObservation(
                observation.target,
            ));
        }
        observation.evidence_refs =
            normalized_refs(observation.target, &observation.evidence_refs)?;
        reject_observation_claims(&observation)?;
        normalized.push(observation);
    }
    Ok(normalized)
}

fn reject_observation_claims(
    observation: &NetworkLocalPlatformProbeObservation,
) -> Result<(), NetworkLocalPlatformProbeError> {
    if observation.adapter_execution_attempted {
        return Err(
            NetworkLocalPlatformProbeError::AdapterExecutionAttemptRejected(observation.target),
        );
    }
    if observation.exact_url_claimed {
        return Err(NetworkLocalPlatformProbeError::ExactUrlClaimRejected);
    }
    if observation.decrypted_payload_claimed {
        return Err(NetworkLocalPlatformProbeError::DecryptedPayloadClaimRejected);
    }
    if observation.page_content_claimed {
        return Err(NetworkLocalPlatformProbeError::PageContentClaimRejected);
    }
    if observation.production_platform_support_claimed {
        return Err(NetworkLocalPlatformProbeError::ProductionPlatformSupportClaimRejected);
    }
    if observation.probe_state == NetworkLocalPlatformProbeState::ReadOnlyObserved
        && !observation.read_only_probe_executed
    {
        return Err(
            NetworkLocalPlatformProbeError::ReadOnlyProbeExecutionRefMissing(observation.target),
        );
    }
    Ok(())
}

fn validate_observations(
    observations: &[NetworkLocalPlatformProbeObservation],
    status: &NetworkAdapterCapabilityStatusProof,
) -> Result<(), NetworkLocalPlatformProbeError> {
    for observation in observations {
        let status_entry = status
            .entries
            .iter()
            .find(|entry| entry.target == observation.target)
            .ok_or(NetworkLocalPlatformProbeError::MissingAdapterStatusEntry(
                observation.target,
            ))?;
        if status_entry.capability_status != observation.capability_status {
            return Err(NetworkLocalPlatformProbeError::CapabilityStatusMismatch(
                observation.target,
            ));
        }
        if !probe_state_supports_status(observation.probe_state, observation.capability_status) {
            return Err(
                NetworkLocalPlatformProbeError::ProbeStateDoesNotSupportCapability(
                    observation.target,
                ),
            );
        }
    }
    Ok(())
}

fn probe_state_supports_status(
    state: NetworkLocalPlatformProbeState,
    status: NetworkAdapterCapabilityStatusState,
) -> bool {
    match state {
        NetworkLocalPlatformProbeState::ReadOnlyObserved => {
            status == NetworkAdapterCapabilityStatusState::DryRun
        }
        NetworkLocalPlatformProbeState::LabReady => matches!(
            status,
            NetworkAdapterCapabilityStatusState::LabReady
                | NetworkAdapterCapabilityStatusState::DistroReady
        ),
        NetworkLocalPlatformProbeState::ManualRequired => {
            status == NetworkAdapterCapabilityStatusState::ManualRequired
        }
        NetworkLocalPlatformProbeState::Unavailable | NetworkLocalPlatformProbeState::CiOnly => {
            status == NetworkAdapterCapabilityStatusState::Unavailable
        }
    }
}

fn count_host(
    observations: &[NetworkLocalPlatformProbeObservation],
    host: NetworkLocalPlatformProbeHost,
) -> usize {
    observations
        .iter()
        .filter(|observation| observation.host == host)
        .count()
}

fn count_apple_ci_unavailable(observations: &[NetworkLocalPlatformProbeObservation]) -> usize {
    observations
        .iter()
        .filter(|observation| {
            matches!(
                observation.host,
                NetworkLocalPlatformProbeHost::MacOsCi | NetworkLocalPlatformProbeHost::IosCi
            ) && observation.probe_state == NetworkLocalPlatformProbeState::CiOnly
        })
        .count()
}

fn normalized_refs(
    target: NetworkPlatformClaimTarget,
    refs: &[String],
) -> Result<Vec<String>, NetworkLocalPlatformProbeError> {
    let mut normalized = Vec::new();
    for value in refs {
        let Some(ref_value) = normalize_ref(value) else {
            return Err(NetworkLocalPlatformProbeError::EmptyObservationEvidenceRef(
                target,
            ));
        };
        if !normalized.contains(&ref_value) {
            normalized.push(ref_value);
        }
    }
    if normalized.is_empty() {
        return Err(NetworkLocalPlatformProbeError::EmptyObservationEvidenceRef(
            target,
        ));
    }
    Ok(normalized)
}

fn normalize_ref(value: &str) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_owned())
    }
}
