use std::{fmt, path::Path, time::SystemTime};

use chrono::{TimeDelta, Utc};
use ed25519_dalek::VerifyingKey;
use getrandom::fill;
use ocentra_eventing::ids::CorrelationId;
use sha2::{Digest, Sha256};

use crate::{
    device_trust_lifecycle::{DeviceTrustLifecycleError, DeviceTrustLifecycleRepository},
    device_trust_signer_registration::{CurrentSignerAuthority, SignerRegistrationAuthorization},
    household_authority::{HouseholdAuthorityAction, ParentStepUpAssertionSnapshot},
    parent_presence::{
        ParentPresenceChallenge, ParentPresenceStorageFailureReason,
        ParentPresenceVerificationFailureReason, ParentPresenceVerificationInput,
    },
    parent_presence_port::ParentPresenceVerificationPort,
    parent_presence_store::StoredParentStepUpIntent,
    parent_step_up_challenge_codec::encode_base64url,
    parent_step_up_platform::{
        PlatformPasskeyAssertion, PlatformPasskeyCredential, PlatformPasskeyError,
    },
};

#[path = "parent_step_up_recovery.rs"]
mod recovery;

const CEREMONY_LIFETIME_SECONDS: i64 = 5 * 60;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ParentStepUpCeremonyError {
    InvalidIntent,
    RandomnessUnavailable,
    StorageUnavailable,
    Platform(PlatformPasskeyError),
    Presence(ParentPresenceVerificationFailureReason),
    InvalidCorrelation,
    Lifecycle(DeviceTrustLifecycleError),
}

/// The exact high-risk registration action. Its digest is carried in the
/// durable nonce identity, so the challenge cannot be redirected to another
/// child, installation, route, or signer key after restart.
pub(crate) struct RegisterLanSignerAnchorIntent {
    family_id: String,
    trust_subject: String,
    parent_account_id: String,
    parent_device_id: String,
    child_device_id: String,
    installation_id: String,
    pairing_id: String,
    route_id: String,
    signer_public_key: [u8; 32],
    lifecycle_generation: u64,
    installation_binding_generation: u64,
    authority_generation: u64,
    correlation_id: String,
    intent_digest: String,
}

impl fmt::Debug for RegisterLanSignerAnchorIntent {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RegisterLanSignerAnchorIntent")
            .field("redaction", &"sensitive-fields-omitted")
            .field("lifecycle_generation", &self.lifecycle_generation)
            .field(
                "installation_binding_generation",
                &self.installation_binding_generation,
            )
            .field("authority_generation", &self.authority_generation)
            .finish()
    }
}

impl RegisterLanSignerAnchorIntent {
    pub(crate) fn new(
        family_id: String,
        trust_subject: String,
        parent_account_id: String,
        parent_device_id: String,
        child_device_id: String,
        installation_id: String,
        pairing_id: String,
        route_id: String,
        signer_public_key: [u8; 32],
        lifecycle_generation: u64,
        installation_binding_generation: u64,
        authority_generation: u64,
        correlation_id: String,
    ) -> Result<Self, ParentStepUpCeremonyError> {
        let values = [
            family_id.as_str(),
            trust_subject.as_str(),
            parent_account_id.as_str(),
            parent_device_id.as_str(),
            child_device_id.as_str(),
            installation_id.as_str(),
            pairing_id.as_str(),
            route_id.as_str(),
            correlation_id.as_str(),
        ];
        if values
            .iter()
            .any(|value| value.trim().is_empty() || value.len() > 512)
            || lifecycle_generation == 0
            || installation_binding_generation == 0
            || authority_generation == 0
            || VerifyingKey::from_bytes(&signer_public_key)
                .map(|key| key.is_weak())
                .unwrap_or(true)
        {
            return Err(ParentStepUpCeremonyError::InvalidIntent);
        }
        let intent_digest = intent_digest(
            &values,
            &signer_public_key,
            lifecycle_generation,
            installation_binding_generation,
            authority_generation,
        );
        Ok(Self {
            family_id,
            trust_subject,
            parent_account_id,
            parent_device_id,
            child_device_id,
            installation_id,
            pairing_id,
            route_id,
            signer_public_key,
            lifecycle_generation,
            installation_binding_generation,
            authority_generation,
            correlation_id,
            intent_digest,
        })
    }

    pub(crate) fn intent_digest(&self) -> &str {
        &self.intent_digest
    }
}

pub(crate) struct ParentStepUpChallenge {
    challenge_ref: String,
    nonce_ref: String,
    expires_at: String,
    intent: RegisterLanSignerAnchorIntent,
}

impl fmt::Debug for ParentStepUpChallenge {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ParentStepUpChallenge")
            .field("challenge_ref", &"[redacted]")
            .field("nonce_ref", &"[redacted]")
            .field("expires_at", &self.expires_at)
            .finish()
    }
}

impl ParentStepUpChallenge {
    pub(crate) fn challenge_ref(&self) -> &str {
        &self.challenge_ref
    }

    pub(crate) fn expires_at(&self) -> &str {
        &self.expires_at
    }
}

pub(crate) struct ParentStepUpCeremony {
    presence: ParentPresenceVerificationPort,
}

impl ParentStepUpCeremony {
    pub(crate) fn open(store_path: impl AsRef<Path>) -> Result<Self, ParentStepUpCeremonyError> {
        let presence =
            ParentPresenceVerificationPort::open_for_parent_step_up(
                store_path.as_ref().to_path_buf(),
                || {
                    crate::parent_presence::ParentPresenceObservedAt::from_system_time(
                        SystemTime::now(),
                    )
                },
            )
            .map_err(map_storage)?;
        Ok(Self { presence })
    }

    pub(crate) fn issue_register_lan_signer_anchor(
        &mut self,
        intent: RegisterLanSignerAnchorIntent,
    ) -> Result<ParentStepUpChallenge, ParentStepUpCeremonyError> {
        let challenge_ref = random_challenge_ref()?;
        let random_nonce = random_identifier("nonce")?;
        let nonce_ref = format!("intent:{}:{random_nonce}", intent.intent_digest);
        let expires_at = (Utc::now() + TimeDelta::seconds(CEREMONY_LIFETIME_SECONDS))
            .to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
        self.presence
            .issue_challenge_with_parent_step_up_intent(
                ParentPresenceChallenge {
                    challenge_ref: challenge_ref.clone(),
                    nonce_ref: nonce_ref.clone(),
                    family_id: intent.family_id.clone(),
                    parent_account_id: intent.parent_account_id.clone(),
                    privileged_action: HouseholdAuthorityAction::RegisterLanSignerAnchor,
                    action_device_id: intent.parent_device_id.clone(),
                    action_device_child_profile_id: Some(intent.pairing_id.clone()),
                    target_child_profile_id: Some(intent.child_device_id.clone()),
                    expires_at: expires_at.clone(),
                },
                StoredParentStepUpIntent {
                    challenge_ref: challenge_ref.clone(),
                    nonce_ref: nonce_ref.clone(),
                    intent_digest: intent.intent_digest.clone(),
                    family_id: intent.family_id.clone(),
                    trust_subject: intent.trust_subject.clone(),
                    parent_account_id: intent.parent_account_id.clone(),
                    parent_device_id: intent.parent_device_id.clone(),
                    child_device_id: intent.child_device_id.clone(),
                    installation_id: intent.installation_id.clone(),
                    pairing_id: intent.pairing_id.clone(),
                    route_id: intent.route_id.clone(),
                    signer_public_key: intent.signer_public_key.to_vec(),
                    lifecycle_generation: i64::try_from(intent.lifecycle_generation)
                        .map_err(|_error| ParentStepUpCeremonyError::InvalidIntent)?,
                    installation_binding_generation: i64::try_from(
                        intent.installation_binding_generation,
                    )
                    .map_err(|_error| ParentStepUpCeremonyError::InvalidIntent)?,
                    authority_generation: i64::try_from(intent.authority_generation)
                        .map_err(|_error| ParentStepUpCeremonyError::InvalidIntent)?,
                    correlation_id: intent.correlation_id.clone(),
                    expires_at: expires_at.clone(),
                    lifecycle_state: "issued".to_owned(),
                    registration_state: "pending".to_owned(),
                    parent_presence_receipt: None,
                    credential_id: None,
                    credential_algorithm: None,
                    credential_sign_count: None,
                },
            )
            .map_err(|_error| ParentStepUpCeremonyError::StorageUnavailable)?;
        Ok(ParentStepUpChallenge {
            challenge_ref,
            nonce_ref,
            expires_at,
            intent,
        })
    }

    pub(crate) fn resume_register_lan_signer_anchor(
        &self,
        challenge_ref: &str,
    ) -> Result<ParentStepUpChallenge, ParentStepUpCeremonyError> {
        let stored = self
            .presence
            .parent_step_up_intent(challenge_ref)
            .map_err(|_error| ParentStepUpCeremonyError::StorageUnavailable)?
            .ok_or(ParentStepUpCeremonyError::Presence(
                ParentPresenceVerificationFailureReason::ChallengeNotIssued,
            ))?;
        if stored.lifecycle_state != "issued" {
            return Err(ParentStepUpCeremonyError::Presence(
                ParentPresenceVerificationFailureReason::ReplayRejected,
            ));
        }
        let signer_public_key: [u8; 32] = stored
            .signer_public_key
            .as_slice()
            .try_into()
            .map_err(|_error| ParentStepUpCeremonyError::StorageUnavailable)?;
        let intent = RegisterLanSignerAnchorIntent::new(
            stored.family_id,
            stored.trust_subject,
            stored.parent_account_id,
            stored.parent_device_id,
            stored.child_device_id,
            stored.installation_id,
            stored.pairing_id,
            stored.route_id,
            signer_public_key,
            u64::try_from(stored.lifecycle_generation)
                .map_err(|_error| ParentStepUpCeremonyError::StorageUnavailable)?,
            u64::try_from(stored.installation_binding_generation)
                .map_err(|_error| ParentStepUpCeremonyError::StorageUnavailable)?,
            u64::try_from(stored.authority_generation)
                .map_err(|_error| ParentStepUpCeremonyError::StorageUnavailable)?,
            stored.correlation_id,
        )
        .map_err(|_error| ParentStepUpCeremonyError::StorageUnavailable)?;
        if intent.intent_digest != stored.intent_digest {
            return Err(ParentStepUpCeremonyError::StorageUnavailable);
        }
        Ok(ParentStepUpChallenge {
            challenge_ref: stored.challenge_ref,
            nonce_ref: stored.nonce_ref,
            expires_at: stored.expires_at,
            intent,
        })
    }

    pub(crate) fn consume_and_register_lan_signer_anchor(
        &mut self,
        challenge: ParentStepUpChallenge,
        credential: &PlatformPasskeyCredential,
        assertion: PlatformPasskeyAssertion,
        lifecycle: &mut DeviceTrustLifecycleRepository,
    ) -> Result<CurrentSignerAuthority, ParentStepUpCeremonyError> {
        let verified_assertion = assertion
            .verify_for_challenge(&challenge.challenge_ref, credential)
            .map_err(ParentStepUpCeremonyError::Platform)?;
        let correlation_id = CorrelationId::parse(challenge.intent.correlation_id.clone())
            .map_err(|_error| ParentStepUpCeremonyError::InvalidCorrelation)?;
        let assertion = ParentStepUpAssertionSnapshot {
            family_id: challenge.intent.family_id.clone(),
            parent_account_id: challenge.intent.parent_account_id.clone(),
            action_device_id: challenge.intent.parent_device_id.clone(),
            action_device_child_profile_id: Some(challenge.intent.pairing_id.clone()),
            target_child_profile_id: Some(challenge.intent.child_device_id.clone()),
            action: HouseholdAuthorityAction::RegisterLanSignerAnchor,
            nonce: challenge.nonce_ref.clone(),
            expires_at: challenge.expires_at.clone(),
        };
        let accepted = self
            .presence
            .verify_and_consume_step_up(
                ParentPresenceVerificationInput {
                    correlation_id,
                    challenge_ref: challenge.challenge_ref.clone(),
                    assertion,
                },
                verified_assertion.credential_id(),
                verified_assertion.algorithm(),
                verified_assertion.sign_count(),
            )
            .map_err(ParentStepUpCeremonyError::Presence)?;
        require_current_generation(lifecycle, &challenge.intent)?;
        let authorization = SignerRegistrationAuthorization::from_verified_parent_step_up(
            &challenge.intent.family_id,
            &challenge.intent.trust_subject,
            &challenge.intent.parent_device_id,
            &challenge.intent.child_device_id,
            &challenge.intent.installation_id,
            &challenge.intent.signer_public_key,
            challenge.intent.correlation_id.as_str(),
            accepted.receipt_ref().as_str(),
            &challenge.intent.intent_digest,
            &challenge.intent.route_id,
            verified_assertion.credential_id(),
            verified_assertion.algorithm(),
            verified_assertion.sign_count(),
            challenge.intent.lifecycle_generation,
            challenge.intent.installation_binding_generation,
            challenge.intent.authority_generation,
        )
        .map_err(ParentStepUpCeremonyError::Lifecycle)?;
        let _receipt_ref = accepted.receipt_ref();
        let authority = lifecycle
            .register_signer_anchor(authorization)
            .map_err(ParentStepUpCeremonyError::Lifecycle)?;
        self.presence
            .complete_parent_step_up_registration(&challenge.challenge_ref)
            .map_err(|_error| ParentStepUpCeremonyError::StorageUnavailable)?;
        Ok(authority)
    }

    pub(crate) fn recover_register_lan_signer_anchor(
        &mut self,
        challenge_ref: &str,
        lifecycle: &mut DeviceTrustLifecycleRepository,
    ) -> Result<CurrentSignerAuthority, ParentStepUpCeremonyError> {
        recovery::recover(&mut self.presence, challenge_ref, lifecycle)
    }
}

fn require_current_generation(
    lifecycle: &mut DeviceTrustLifecycleRepository,
    intent: &RegisterLanSignerAnchorIntent,
) -> Result<(), ParentStepUpCeremonyError> {
    let row = DeviceTrustLifecycleRepository::row(
        &lifecycle.connection,
        &intent.family_id,
        &intent.trust_subject,
        &intent.parent_device_id,
    )
    .map_err(ParentStepUpCeremonyError::Lifecycle)?
    .ok_or(ParentStepUpCeremonyError::Lifecycle(
        DeviceTrustLifecycleError::RegistrationMissing,
    ))?;
    let (
        state,
        lifecycle_generation,
        installation_id,
        installation_generation,
        authority_generation,
    ) = row;
    if state != "trusted"
        || lifecycle_generation != intent.lifecycle_generation
        || installation_id != intent.installation_id
        || installation_generation != intent.installation_binding_generation
        || authority_generation != intent.authority_generation
        || !lifecycle.external_authority.matches(
            &intent.family_id,
            &intent.trust_subject,
            &intent.parent_device_id,
            intent.authority_generation,
        )
    {
        return Err(ParentStepUpCeremonyError::Lifecycle(
            DeviceTrustLifecycleError::ParentReauthorizationRequired,
        ));
    }
    Ok(())
}

fn intent_digest(
    values: &[&str; 9],
    signer_public_key: &[u8; 32],
    lifecycle_generation: u64,
    installation_binding_generation: u64,
    authority_generation: u64,
) -> String {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(b"ocentra-register-lan-signer-anchor-v1\0");
    for value in values {
        bytes.extend_from_slice(&(value.len() as u64).to_be_bytes());
        bytes.extend_from_slice(value.as_bytes());
    }
    bytes.extend_from_slice(signer_public_key);
    bytes.extend_from_slice(&lifecycle_generation.to_be_bytes());
    bytes.extend_from_slice(&installation_binding_generation.to_be_bytes());
    bytes.extend_from_slice(&authority_generation.to_be_bytes());
    hex(&Sha256::digest(bytes))
}

fn random_identifier(prefix: &str) -> Result<String, ParentStepUpCeremonyError> {
    let mut bytes = [0_u8; 32];
    fill(&mut bytes).map_err(|_error| ParentStepUpCeremonyError::RandomnessUnavailable)?;
    Ok(format!("{prefix}:{}", hex(&bytes)))
}

fn random_challenge_ref() -> Result<String, ParentStepUpCeremonyError> {
    let mut bytes = [0_u8; 32];
    fill(&mut bytes).map_err(|_error| ParentStepUpCeremonyError::RandomnessUnavailable)?;
    Ok(encode_base64url(&bytes))
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn map_storage(_error: ParentPresenceStorageFailureReason) -> ParentStepUpCeremonyError {
    ParentStepUpCeremonyError::StorageUnavailable
}
