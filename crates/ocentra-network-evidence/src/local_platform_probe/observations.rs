mod refs;

use crate::{NetworkAdapterCapabilityStatusProof, NetworkAdapterCapabilityStatusState};

use self::refs::normalized_refs;

use super::{
    normalize_ref, NetworkLocalPlatformProbeError, NetworkLocalPlatformProbeObservation,
    NetworkLocalPlatformProbeState, NetworkPlatformClaimTarget,
};

pub(super) fn normalize_observations(
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

pub(super) fn validate_observations(
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

fn reject_observation_claims(
    observation: &NetworkLocalPlatformProbeObservation,
) -> Result<(), NetworkLocalPlatformProbeError> {
    [
        (
            observation.adapter_execution_attempted,
            NetworkLocalPlatformProbeError::AdapterExecutionAttemptRejected(observation.target),
        ),
        (
            observation.exact_url_claimed,
            NetworkLocalPlatformProbeError::ExactUrlClaimRejected,
        ),
        (
            observation.decrypted_payload_claimed,
            NetworkLocalPlatformProbeError::DecryptedPayloadClaimRejected,
        ),
        (
            observation.page_content_claimed,
            NetworkLocalPlatformProbeError::PageContentClaimRejected,
        ),
        (
            observation.production_platform_support_claimed,
            NetworkLocalPlatformProbeError::ProductionPlatformSupportClaimRejected,
        ),
    ]
    .into_iter()
    .find_map(|(claimed, error)| claimed.then_some(error))
    .map_or_else(
        || {
            if observation.probe_state == NetworkLocalPlatformProbeState::ReadOnlyObserved
                && !observation.read_only_probe_executed
            {
                Err(
                    NetworkLocalPlatformProbeError::ReadOnlyProbeExecutionRefMissing(
                        observation.target,
                    ),
                )
            } else {
                Ok(())
            }
        },
        Err,
    )
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
