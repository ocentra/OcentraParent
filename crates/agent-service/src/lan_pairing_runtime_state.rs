use ocentra_lan_core::lan_pairing::verify_lan_signed_child_agent_envelope;
use ocentra_lan_core::lan_pairing::LanMdnsAdvertisementLifecycleDecision;
use ocentra_lan_core::lan_pairing::LanMdnsAdvertisementLifecycleInput;
use ocentra_lan_core::lan_pairing::LanMdnsAdvertisementPlatformSupport;
use ocentra_lan_core::lan_pairing::LanSignedChildAgentVerificationContext;
use ocentra_lan_core::lan_pairing::LanSignedChildAgentVerificationError;
use ocentra_lan_core::network_inventory::passive_discovery::LanPassiveDiscoverySource;
use ocentra_lan_core::network_inventory::passive_discovery::LanPassiveDiscoveryTriggerReason;
use ocentra_parent_agent_core::trusted_device_registry::TrustedDeviceRegistry;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingProof;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingRejectionReason;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingTrustState;
use ocentra_parent_agent_protocol::lan_pairing::LanSelectedRouteTarget;
use ocentra_parent_agent_protocol::lan_pairing::LanSignedChildAgentClaim;
use ocentra_parent_agent_protocol::lan_pairing::LanSignedChildAgentEnvelope;

use crate::{
    lan_pairing::{LanPairingChallengeState, LanPairingRegistryPersistence, LanPairingRuntime},
    time::timestamp_now,
};

#[path = "lan_pairing_runtime_state/device_roles.rs"]
mod device_roles;
#[path = "lan_pairing_runtime_state/job_leases.rs"]
pub(crate) mod job_leases;
#[path = "lan_pairing_runtime_state/mdns_advertisement.rs"]
pub(crate) mod mdns_advertisement;
#[path = "lan_pairing_runtime_state/passive_discovery.rs"]
pub(crate) mod passive_discovery;
#[path = "lan_pairing_runtime_state/provider_heartbeat.rs"]
pub(crate) mod provider_heartbeat;
#[path = "lan_pairing_runtime_state/provider_routing.rs"]
mod provider_routing;
#[path = "lan_pairing_runtime_state/runtime_config.rs"]
mod runtime_config;

const SIGNED_CHILD_MESSAGE_KIND_HELLO: &str = "hello";
const SIGNED_CHILD_MESSAGE_KIND_HEARTBEAT: &str = "heartbeat";
const SIGNED_CHILD_OBSERVED_SUMMARY_PREFIX: &str = "signed child ";
const SIGNED_CHILD_OBSERVED_SUMMARY_ROUTE_SEPARATOR: &str = " observed: route=";
const SIGNED_CHILD_OBSERVED_SUMMARY_INSTALL_SEPARATOR: &str = "; install-id=";

impl LanPairingRuntime {
    pub fn trusted_device_count(&self) -> usize {
        self.registry
            .lock()
            .map(|registry| registry.trusted_device_count())
            .unwrap_or(0)
    }

    pub fn selected_target(&self) -> Option<LanSelectedRouteTarget> {
        let observed_at = timestamp_now();
        self.registry
            .lock()
            .ok()
            .and_then(|registry| registry.selected_target_at(&observed_at))
    }

    pub fn trusted_device_ids(&self) -> Vec<String> {
        self.registry
            .lock()
            .map(|registry| registry.trusted_device_ids())
            .unwrap_or_default()
    }

    pub fn revoked_device_ids(&self) -> Vec<String> {
        self.registry
            .lock()
            .map(|registry| registry.revoked_device_ids())
            .unwrap_or_default()
    }

    pub fn mdns_advertisement_lifecycle(
        desired_present: bool,
        running: bool,
        platform_support: LanMdnsAdvertisementPlatformSupport,
    ) -> LanMdnsAdvertisementLifecycleDecision {
        ocentra_lan_core::lan_pairing::evaluate_lan_mdns_advertisement_lifecycle(
            LanMdnsAdvertisementLifecycleInput {
                desired_present,
                running,
                platform_support,
            },
        )
    }

    pub fn signed_child_agent_replay_observation_count(&self) -> usize {
        self.signed_child_agent_replay_guard
            .lock()
            .map(|guard| guard.observed_count())
            .unwrap_or(0)
    }

    pub fn verify_signed_child_agent_envelope(
        &self,
        envelope: &LanSignedChildAgentEnvelope,
        observed_at: &str,
        context: &LanSignedChildAgentVerificationContext,
    ) -> Result<LanSignedChildAgentClaim, LanSignedChildAgentVerificationError> {
        let mut replay_guard = self
            .signed_child_agent_replay_guard
            .lock()
            .map_err(|error| {
                drop(error);
                LanSignedChildAgentVerificationError::SignatureRejected
            })?;
        verify_lan_signed_child_agent_envelope(envelope, observed_at, context, &mut replay_guard)
    }

    pub fn observe_signed_child_agent_envelope(
        &self,
        envelope: &LanSignedChildAgentEnvelope,
        observed_at: &str,
    ) -> Result<LanSignedChildAgentClaim, LanPairingRejectionReason> {
        let context = self.signed_child_agent_verification_context()?;
        let claim = self
            .verify_signed_child_agent_envelope(envelope, observed_at, &context)
            .map_err(|reason| signed_child_agent_rejection_reason(&reason))?;
        self.record_signed_child_agent_passive_observation(&claim, observed_at);
        Ok(claim)
    }

    fn signed_child_agent_verification_context(
        &self,
    ) -> Result<LanSignedChildAgentVerificationContext, LanPairingRejectionReason> {
        let expected_parent_device_id = self
            .signed_child_agent_parent_device_id
            .clone()
            .ok_or(LanPairingRejectionReason::SignedChildAgentContextUnavailable)?;
        let expected_family_hash = self
            .signed_child_agent_family_hash
            .clone()
            .ok_or(LanPairingRejectionReason::SignedChildAgentContextUnavailable)?;
        let expected_child_device_id = self
            .local_child_device_id
            .clone()
            .ok_or(LanPairingRejectionReason::SignedChildAgentContextUnavailable)?;
        Ok(LanSignedChildAgentVerificationContext {
            expected_parent_device_id,
            expected_family_hash,
            expected_route_id: self.signed_child_agent_route_id.clone(),
            expected_child_device_id: Some(expected_child_device_id),
        })
    }

    pub fn has_revoked_pairing(&self) -> bool {
        self.registry
            .lock()
            .map(|registry| registry.has_revoked_pairing())
            .unwrap_or(false)
    }

    pub(crate) fn mdns_pairing_state(&self) -> LanPairingTrustState {
        if self.trusted_device_count() > 0 {
            self.selected_target()
                .map(|target| target.trust_state)
                .unwrap_or(LanPairingTrustState::Paired)
        } else {
            LanPairingTrustState::Unpaired
        }
    }

    fn record_signed_child_agent_passive_observation(
        &self,
        claim: &LanSignedChildAgentClaim,
        observed_at: &str,
    ) {
        let message_kind = match claim.message_kind {
            ocentra_parent_agent_protocol::lan_pairing::LanSignedChildAgentMessageKind::Hello => {
                SIGNED_CHILD_MESSAGE_KIND_HELLO
            }
            ocentra_parent_agent_protocol::lan_pairing::LanSignedChildAgentMessageKind::Heartbeat => {
                SIGNED_CHILD_MESSAGE_KIND_HEARTBEAT
            }
        };
        let summary =
            signed_child_observed_summary(message_kind, &claim.route_id, &claim.install_id);
        if let Ok(mut state) = self.passive_discovery_listener_state.lock() {
            let _ = state.record_passive_update(
                LanPassiveDiscoverySource::OcentraBeacon,
                LanPassiveDiscoveryTriggerReason::PassivePacketObserved,
                observed_at,
                Some(claim.child_device_id.as_str()),
                None,
                summary,
            );
        }
    }

    pub(crate) fn remember_challenge(&self, challenge: LanPairingChallengeState) {
        if let Ok(mut challenges) = self.challenges.lock() {
            challenges.retain(|candidate| candidate.challenge_id != challenge.challenge_id);
            challenges.push(challenge);
        }
    }

    pub(crate) fn validate_challenge_proof(
        &self,
        proof: &LanPairingProof,
        observed_at: &str,
    ) -> Result<(), LanPairingRejectionReason> {
        let mut challenges = self.challenges.lock().map_err(|error| {
            let _ = error;
            LanPairingRejectionReason::Malformed
        })?;
        if challenges.is_empty() {
            return Ok(());
        }

        let challenge = challenges
            .iter_mut()
            .find(|candidate| candidate.challenge_id == proof.challenge_id)
            .ok_or(LanPairingRejectionReason::Malformed)?;
        if challenge.accepted {
            return Err(LanPairingRejectionReason::Replayed);
        }
        if challenge.child_device_id != proof.child_device_id {
            return Err(LanPairingRejectionReason::WrongDevice);
        }
        if challenge.parent_device_id != proof.parent_device_id {
            return Err(LanPairingRejectionReason::Malformed);
        }
        if challenge.route_id != proof.route_id {
            return Err(LanPairingRejectionReason::UnsupportedRoute);
        }
        if challenge.origin != proof.origin {
            return Err(LanPairingRejectionReason::WrongOrigin);
        }
        if challenge.proof_digest != proof.proof_digest {
            return Err(LanPairingRejectionReason::Malformed);
        }
        if observed_at > challenge.expires_at.as_str() || observed_at > proof.expires_at.as_str() {
            return Err(LanPairingRejectionReason::Stale);
        }

        challenge.accepted = true;
        Ok(())
    }

    pub(crate) fn persistence_mode(&self) -> &'static str {
        match &self.persistence {
            LanPairingRegistryPersistence::InMemory => {
                constants::value::LAN_PERSISTENCE_IN_MEMORY_FAIL_CLOSED
            }
            LanPairingRegistryPersistence::LocalJsonRegistry(_) => {
                constants::value::LAN_PERSISTENCE_LOCAL_JSON_REGISTRY
            }
        }
    }

    pub(crate) fn restart_behavior(&self) -> &'static str {
        match &self.persistence {
            LanPairingRegistryPersistence::InMemory => {
                constants::value::LAN_RESTART_FAIL_CLOSED_UNPAIRED
            }
            LanPairingRegistryPersistence::LocalJsonRegistry(_)
                if self.selected_target().is_some() =>
            {
                constants::value::LAN_RESTART_RESTORE_TRUSTED_REGISTRY_SELECTED_ROUTE
            }
            LanPairingRegistryPersistence::LocalJsonRegistry(_) => {
                constants::value::LAN_RESTART_RESTORE_TRUSTED_REGISTRY_UNSELECTED
            }
        }
    }

    pub(crate) fn persist_registry(&self, registry: &TrustedDeviceRegistry) -> bool {
        match &self.persistence {
            LanPairingRegistryPersistence::InMemory => true,
            LanPairingRegistryPersistence::LocalJsonRegistry(path) => {
                registry.save_json(path.as_path()).is_ok()
            }
        }
    }
}

fn signed_child_observed_summary(message_kind: &str, route_id: &str, install_id: &str) -> String {
    let mut summary = String::from(SIGNED_CHILD_OBSERVED_SUMMARY_PREFIX);
    summary.push_str(message_kind);
    summary.push_str(SIGNED_CHILD_OBSERVED_SUMMARY_ROUTE_SEPARATOR);
    summary.push_str(route_id);
    summary.push_str(SIGNED_CHILD_OBSERVED_SUMMARY_INSTALL_SEPARATOR);
    summary.push_str(install_id);
    summary
}

pub(crate) fn signed_child_agent_rejection_reason(
    reason: &LanSignedChildAgentVerificationError,
) -> LanPairingRejectionReason {
    match reason {
        LanSignedChildAgentVerificationError::Replayed => LanPairingRejectionReason::Replayed,
        LanSignedChildAgentVerificationError::Expired
        | LanSignedChildAgentVerificationError::FutureIssuedAt => {
            LanPairingRejectionReason::Expired
        }
        LanSignedChildAgentVerificationError::WrongRoute => {
            LanPairingRejectionReason::UnsupportedRoute
        }
        LanSignedChildAgentVerificationError::WrongFamily
        | LanSignedChildAgentVerificationError::WrongParentDevice
        | LanSignedChildAgentVerificationError::WrongChildDevice => {
            LanPairingRejectionReason::WrongDevice
        }
        LanSignedChildAgentVerificationError::UnsupportedSchemaVersion
        | LanSignedChildAgentVerificationError::EmptyRequiredField
        | LanSignedChildAgentVerificationError::InvalidMetadata
        | LanSignedChildAgentVerificationError::MalformedTimestamp
        | LanSignedChildAgentVerificationError::UnsupportedAlgorithm
        | LanSignedChildAgentVerificationError::InvalidPublicKey
        | LanSignedChildAgentVerificationError::PublicKeyIdMismatch
        | LanSignedChildAgentVerificationError::InvalidSignature
        | LanSignedChildAgentVerificationError::SignatureRejected
        | LanSignedChildAgentVerificationError::SerializationFailed => {
            LanPairingRejectionReason::Malformed
        }
    }
}
