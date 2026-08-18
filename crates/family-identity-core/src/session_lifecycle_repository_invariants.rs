#![forbid(unsafe_code)]

use crate::session_lifecycle::SessionActivityState;
use crate::session_lifecycle_custody::record::{SessionCredentialClass, SessionCredentialRecord};
use crate::session_lifecycle_custody::storage_values::{
    digest_is_valid, SessionRefreshFamilyId, SESSION_ACCESS_DIGEST_DOMAIN,
    SESSION_CREDENTIAL_CLASS, SESSION_DIGEST_ALGORITHM, SESSION_REFRESH_DIGEST_DOMAIN,
};

use super::labels;
use super::SessionLifecycleRepositoryError;

pub(crate) fn validate_record(
    record: &SessionCredentialRecord,
) -> Result<(), SessionLifecycleRepositoryError> {
    validate_contract_labels(record)?;
    validate_digests(record)?;
    validate_authority_binding(record)?;
    validate_generations(record)?;
    validate_temporal_state(record)?;
    Ok(())
}

fn validate_contract_labels(
    record: &SessionCredentialRecord,
) -> Result<(), SessionLifecycleRepositoryError> {
    let class_is_browser = record.credential_class == SessionCredentialClass::BrowserUserSession;
    if !class_is_browser
        || labels::credential_class_label(record.credential_class).0 != SESSION_CREDENTIAL_CLASS
        || record.digest_algorithm != SESSION_DIGEST_ALGORITHM
        || record.access_digest_domain != SESSION_ACCESS_DIGEST_DOMAIN
        || record.refresh_digest_domain != SESSION_REFRESH_DIGEST_DOMAIN
    {
        return Err(SessionLifecycleRepositoryError::InvalidStoredSession);
    }
    Ok(())
}

fn validate_digests(
    record: &SessionCredentialRecord,
) -> Result<(), SessionLifecycleRepositoryError> {
    let family_round_trip =
        SessionRefreshFamilyId::parse(record.refresh_family_id.as_str().to_owned()).is_some();
    if !digest_is_valid(record.access_digest.as_str())
        || !digest_is_valid(record.refresh_digest.as_str())
        || record.access_digest.as_str() == record.refresh_digest.as_str()
        || !family_round_trip
    {
        return Err(SessionLifecycleRepositoryError::InvalidStoredSession);
    }
    Ok(())
}

fn validate_authority_binding(
    record: &SessionCredentialRecord,
) -> Result<(), SessionLifecycleRepositoryError> {
    let binding = &record.binding;
    let identifiers_are_present = !record.session_id.as_str().trim().is_empty()
        && !binding.account_id.to_string().trim().is_empty()
        && !binding.provider_subject.as_str().trim().is_empty()
        && !binding.household_id.to_string().trim().is_empty()
        && !binding.member_id.as_str().trim().is_empty()
        && !binding.device_id.as_str().trim().is_empty()
        && !binding.authority_session_id.as_str().trim().is_empty();
    if !identifiers_are_present
        || record.session_id.as_str() != binding.authority_session_id.as_str()
    {
        return Err(SessionLifecycleRepositoryError::InvalidStoredSession);
    }
    Ok(())
}

fn validate_generations(
    record: &SessionCredentialRecord,
) -> Result<(), SessionLifecycleRepositoryError> {
    if record.refresh_generation == 0
        || record.global_revoke_epoch == 0
        || record.binding.authority_generation == 0
        || record.binding.authority_session_generation == 0
    {
        return Err(SessionLifecycleRepositoryError::InvalidStoredSession);
    }
    Ok(())
}

fn validate_temporal_state(
    record: &SessionCredentialRecord,
) -> Result<(), SessionLifecycleRepositoryError> {
    let ordered = record.issued_at_epoch_millis > 0
        && record.fresh_until_epoch_millis > 0
        && record.fresh_until_epoch_millis <= record.access_expires_at_epoch_millis
        && record.access_expires_at_epoch_millis <= record.refresh_expires_at_epoch_millis
        && record.refresh_expires_at_epoch_millis
            <= record.binding.authority_expires_at_epoch_millis
        && record.last_transition_at_epoch_millis >= record.issued_at_epoch_millis;
    let lifecycle_is_consistent = match record.activity_state {
        SessionActivityState::Active => {
            record.last_transition_at_epoch_millis == record.issued_at_epoch_millis
        }
        SessionActivityState::LoggedOut
        | SessionActivityState::Revoked
        | SessionActivityState::GloballyRevoked => {
            record.last_transition_at_epoch_millis > record.issued_at_epoch_millis
        }
    };
    if !ordered || !lifecycle_is_consistent {
        return Err(SessionLifecycleRepositoryError::InvalidStoredSession);
    }
    Ok(())
}
