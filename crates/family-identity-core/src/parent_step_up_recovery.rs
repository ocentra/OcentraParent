use crate::device_trust_lifecycle::{DeviceTrustLifecycleError, DeviceTrustLifecycleRepository};
use crate::device_trust_signer_registration::{
    CurrentSignerAuthority, SignerRegistrationAuthorization,
};
use crate::parent_presence::{
    ParentPresenceVerificationFailureReason, ParentPresenceVerificationPort,
};
use crate::parent_presence_store::StoredParentStepUpIntent;

use super::{ParentStepUpCeremonyError, RegisterLanSignerAnchorIntent};

pub(super) fn recover(
    presence: &mut ParentPresenceVerificationPort,
    challenge_ref: &str,
    lifecycle: &mut DeviceTrustLifecycleRepository,
) -> Result<CurrentSignerAuthority, ParentStepUpCeremonyError> {
    let stored = presence
        .parent_step_up_intent(challenge_ref)
        .map_err(|_error| ParentStepUpCeremonyError::StorageUnavailable)?
        .ok_or(ParentStepUpCeremonyError::Presence(
            ParentPresenceVerificationFailureReason::ChallengeNotIssued,
        ))?;
    let challenge_lifecycle = presence
        .store
        .parent_step_up_challenge_lifecycle(challenge_ref)
        .map_err(|_error| ParentStepUpCeremonyError::StorageUnavailable)?;
    if challenge_lifecycle.as_deref() != Some("consumed") {
        return Err(ParentStepUpCeremonyError::Presence(
            ParentPresenceVerificationFailureReason::ChallengeNotIssued,
        ));
    }
    if stored.lifecycle_state != "consumed"
        || !matches!(stored.registration_state.as_str(), "pending" | "completed")
    {
        return Err(ParentStepUpCeremonyError::Presence(
            ParentPresenceVerificationFailureReason::ChallengeNotIssued,
        ));
    }
    let signer_public_key = signer_key(&stored)?;
    let authorization = authorization(&stored, signer_public_key)?;
    let authority = resolve_authority(lifecycle, &stored, signer_public_key, authorization)?;
    if stored.registration_state == "pending" {
        presence
            .complete_parent_step_up_registration(challenge_ref)
            .map_err(|_error| ParentStepUpCeremonyError::StorageUnavailable)?;
    }
    Ok(authority)
}

fn signer_key(stored: &StoredParentStepUpIntent) -> Result<[u8; 32], ParentStepUpCeremonyError> {
    stored
        .signer_public_key
        .as_slice()
        .try_into()
        .map_err(|_error| ParentStepUpCeremonyError::StorageUnavailable)
}

fn authorization(
    stored: &StoredParentStepUpIntent,
    signer_public_key: [u8; 32],
) -> Result<SignerRegistrationAuthorization, ParentStepUpCeremonyError> {
    let parent_presence_receipt = stored
        .parent_presence_receipt
        .as_deref()
        .ok_or(ParentStepUpCeremonyError::StorageUnavailable)?;
    let credential_id = stored
        .credential_id
        .as_deref()
        .ok_or(ParentStepUpCeremonyError::StorageUnavailable)?;
    let credential_algorithm = stored
        .credential_algorithm
        .ok_or(ParentStepUpCeremonyError::StorageUnavailable)?;
    let credential_sign_count = u32::try_from(
        stored
            .credential_sign_count
            .ok_or(ParentStepUpCeremonyError::StorageUnavailable)?,
    )
    .map_err(|_error| ParentStepUpCeremonyError::StorageUnavailable)?;
    let lifecycle_generation = generation(stored.lifecycle_generation)?;
    let installation_binding_generation = generation(stored.installation_binding_generation)?;
    let authority_generation = generation(stored.authority_generation)?;
    let intent = RegisterLanSignerAnchorIntent::new(
        stored.family_id.clone(),
        stored.trust_subject.clone(),
        stored.parent_account_id.clone(),
        stored.parent_device_id.clone(),
        stored.child_device_id.clone(),
        stored.installation_id.clone(),
        stored.pairing_id.clone(),
        stored.route_id.clone(),
        signer_public_key,
        lifecycle_generation,
        installation_binding_generation,
        authority_generation,
        stored.correlation_id.clone(),
    )
    .map_err(|_error| ParentStepUpCeremonyError::StorageUnavailable)?;
    if intent.intent_digest != stored.intent_digest {
        return Err(ParentStepUpCeremonyError::StorageUnavailable);
    }
    SignerRegistrationAuthorization::from_verified_parent_step_up(
        &stored.family_id,
        &stored.trust_subject,
        &stored.parent_device_id,
        &stored.child_device_id,
        &stored.installation_id,
        &signer_public_key,
        &stored.correlation_id,
        parent_presence_receipt,
        &stored.intent_digest,
        &stored.route_id,
        credential_id,
        credential_algorithm,
        credential_sign_count,
        lifecycle_generation,
        installation_binding_generation,
        authority_generation,
    )
    .map_err(ParentStepUpCeremonyError::Lifecycle)
}

fn generation(value: i64) -> Result<u64, ParentStepUpCeremonyError> {
    u64::try_from(value).map_err(|_error| ParentStepUpCeremonyError::StorageUnavailable)
}

fn resolve_authority(
    lifecycle: &mut DeviceTrustLifecycleRepository,
    stored: &StoredParentStepUpIntent,
    signer_public_key: [u8; 32],
    authorization: SignerRegistrationAuthorization,
) -> Result<CurrentSignerAuthority, ParentStepUpCeremonyError> {
    match lifecycle.current_signer_authority(
        &stored.family_id,
        &stored.trust_subject,
        &stored.parent_device_id,
        &stored.child_device_id,
    ) {
        Ok(authority) => {
            validate_existing_authority(&authority, stored, signer_public_key)?;
            Ok(authority)
        }
        Err(DeviceTrustLifecycleError::SignerRegistrationMissing)
            if stored.registration_state == "pending" =>
        {
            lifecycle
                .register_signer_anchor(authorization)
                .map_err(ParentStepUpCeremonyError::Lifecycle)
        }
        Err(DeviceTrustLifecycleError::SignerRegistrationMissing) => Err(
            ParentStepUpCeremonyError::Lifecycle(DeviceTrustLifecycleError::RegistrationMissing),
        ),
        Err(error) => Err(ParentStepUpCeremonyError::Lifecycle(error)),
    }
}

fn validate_existing_authority(
    authority: &CurrentSignerAuthority,
    stored: &StoredParentStepUpIntent,
    signer_public_key: [u8; 32],
) -> Result<(), ParentStepUpCeremonyError> {
    let receipt = stored
        .parent_presence_receipt
        .as_deref()
        .ok_or(ParentStepUpCeremonyError::StorageUnavailable)?;
    let credential_id = stored
        .credential_id
        .as_deref()
        .ok_or(ParentStepUpCeremonyError::StorageUnavailable)?;
    let credential_algorithm = stored
        .credential_algorithm
        .ok_or(ParentStepUpCeremonyError::StorageUnavailable)?;
    let credential_sign_count = u32::try_from(
        stored
            .credential_sign_count
            .ok_or(ParentStepUpCeremonyError::StorageUnavailable)?,
    )
    .map_err(|_error| ParentStepUpCeremonyError::StorageUnavailable)?;
    authority
        .matches_registration(
            &stored.intent_digest,
            receipt,
            &stored.route_id,
            signer_public_key,
            &stored.installation_id,
            credential_id,
            credential_algorithm,
            credential_sign_count,
        )
        .then_some(())
        .ok_or(ParentStepUpCeremonyError::Lifecycle(
            DeviceTrustLifecycleError::RegistrationMissing,
        ))
}
