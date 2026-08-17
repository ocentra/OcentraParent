use ocentra_lan_core::lan_pairing::verify_lan_signed_child_agent_envelope;
use ocentra_lan_core::lan_pairing::LanMdnsAdvertisementLifecycleDecision;
use ocentra_lan_core::lan_pairing::LanMdnsAdvertisementLifecycleInput;
use ocentra_lan_core::lan_pairing::LanMdnsAdvertisementPlatformSupport;
use ocentra_lan_core::lan_pairing::LanSignedChildAgentVerificationContext;
use ocentra_lan_core::lan_pairing::LanSignedChildAgentVerificationError;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingRejectionReason;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingText;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingTrustState;
use ocentra_parent_agent_protocol::lan_pairing::LanSelectedRouteTarget;
use ocentra_parent_agent_protocol::lan_pairing::LanSignedChildAgentClaim;
use ocentra_parent_agent_protocol::lan_pairing::LanSignedChildAgentEnvelope;
use std::fmt::Display;

use crate::{
    lan_pairing::{LanPairingChallengeState, LanPairingRuntime},
    time::timestamp_now,
};

#[path = "lan_pairing_runtime_state/challenge_validation.rs"]
mod challenge_validation;
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
#[path = "lan_pairing_runtime_state/registry_persistence.rs"]
mod registry_persistence;
#[path = "lan_pairing_runtime_state/rejection_reason.rs"]
mod rejection_reason;
#[path = "lan_pairing_runtime_state/runtime_config.rs"]
mod runtime_config;
#[path = "lan_pairing_runtime_state/signed_child_passive_observation.rs"]
mod signed_child_passive_observation;

use self::rejection_reason::signed_child_agent_rejection_reason;

impl LanPairingRuntime {
    pub fn trusted_device_count(&self) -> usize {
        self.registry
            .lock()
            .map(|registry| registry.trusted_device_count())
            .unwrap_or(0)
    }

    pub fn selected_target(&self) -> Option<LanSelectedRouteTarget> {
        let observed_at: String = timestamp_now();
        self.registry
            .lock()
            .ok()
            .and_then(|registry| registry.selected_target_at(&observed_at))
    }

    pub fn trusted_device_ids(&self) -> Vec<LanPairingText> {
        self.registry
            .lock()
            .map(|registry| {
                registry
                    .trusted_device_ids()
                    .into_iter()
                    .map(LanPairingText)
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn revoked_device_ids(&self) -> Vec<LanPairingText> {
        self.registry
            .lock()
            .map(|registry| {
                registry
                    .revoked_device_ids()
                    .into_iter()
                    .map(LanPairingText)
                    .collect()
            })
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
        observed_at: &impl Display,
        context: &LanSignedChildAgentVerificationContext,
    ) -> Result<LanSignedChildAgentClaim, LanSignedChildAgentVerificationError> {
        let observed_at = LanPairingText(observed_at.to_string());
        let mut replay_guard = self
            .signed_child_agent_replay_guard
            .lock()
            .map_err(|error| {
                drop(error);
                LanSignedChildAgentVerificationError::SignatureRejected
            })?;
        verify_lan_signed_child_agent_envelope(
            envelope,
            observed_at.0.as_str(),
            context,
            &mut replay_guard,
        )
    }

    pub fn observe_signed_child_agent_envelope(
        &self,
        envelope: &LanSignedChildAgentEnvelope,
        observed_at: &impl Display,
    ) -> Result<LanSignedChildAgentClaim, LanPairingRejectionReason> {
        let observed_at = LanPairingText(observed_at.to_string());
        let context = self.signed_child_agent_verification_context()?;
        let claim = self
            .verify_signed_child_agent_envelope(envelope, &observed_at, &context)
            .map_err(|reason| signed_child_agent_rejection_reason(&reason))?;
        self.record_signed_child_agent_passive_observation(&claim, &observed_at);
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

    pub(crate) fn remember_challenge(&self, challenge: LanPairingChallengeState) {
        if let Ok(mut challenges) = self.challenges.lock() {
            challenges.retain(|candidate| candidate.challenge_id != challenge.challenge_id);
            challenges.push(challenge);
        }
    }
}
