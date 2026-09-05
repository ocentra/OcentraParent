use super::super::envelope::{
    string_fields, CanonicalMutationEnvelope, AUDIENCE, ENVELOPE_VERSION, ENVIRONMENT,
    SIGNATURE_ALGORITHM,
};
use super::FIELD_COUNT;
use crate::account_identity_mutation_authority_error::AccountIdentityMutationAuthorityError;
use ocentra_schema::account_identity_authority::ACCOUNT_IDENTITY_AUTHORITY_MAX_GENERATION;

pub(super) fn validate_fields(
    fields: &[String; FIELD_COUNT],
) -> Result<(), AccountIdentityMutationAuthorityError> {
    let fields = std::array::from_fn(|index| fields[index].as_str());
    validate_values(&fields)
}

pub(super) fn validate_envelope(
    envelope: &CanonicalMutationEnvelope,
) -> Result<(), AccountIdentityMutationAuthorityError> {
    validate_values(&string_fields(envelope))?;
    if envelope.session_generation == 0
        || envelope.session_generation > ACCOUNT_IDENTITY_AUTHORITY_MAX_GENERATION
        || envelope.authority_generation == 0
        || envelope.authority_generation > ACCOUNT_IDENTITY_AUTHORITY_MAX_GENERATION
        || envelope.binding_generation == 0
        || envelope.binding_generation > ACCOUNT_IDENTITY_AUTHORITY_MAX_GENERATION
    {
        return Err(AccountIdentityMutationAuthorityError::InvalidEnvelope);
    }
    Ok(())
}

fn validate_values(
    fields: &[&str; FIELD_COUNT],
) -> Result<(), AccountIdentityMutationAuthorityError> {
    if fields[0] != ENVELOPE_VERSION
        || fields[1] != SIGNATURE_ALGORITHM
        || fields[2] != AUDIENCE
        || fields[3] != ENVIRONMENT
        || !valid_key_id(fields[4])
    {
        return Err(AccountIdentityMutationAuthorityError::InvalidEnvelope);
    }
    if fields[5..15].iter().any(|value| value.trim().is_empty())
        || !["authjs", "firebase"].contains(&fields[5])
        || !["parent-owner", "co-parent-guardian"].contains(&fields[10])
    {
        return Err(AccountIdentityMutationAuthorityError::InvalidEnvelope);
    }
    validate_support_fields(&fields[15..29])?;
    validate_target_fields(fields)
}

fn validate_support_fields(fields: &[&str]) -> Result<(), AccountIdentityMutationAuthorityError> {
    let all_empty = fields.iter().all(|value| value.is_empty());
    let all_present = fields.iter().all(|value| !value.trim().is_empty());
    if !all_empty && !all_present {
        return Err(AccountIdentityMutationAuthorityError::InvalidEnvelope);
    }
    if all_present
        && (!["read-only", "household", "device-control"].contains(&fields[8])
            || fields[12] != "active"
            || chrono::DateTime::parse_from_rfc3339(fields[10]).is_err()
            || chrono::DateTime::parse_from_rfc3339(fields[11]).is_err())
    {
        return Err(AccountIdentityMutationAuthorityError::InvalidEnvelope);
    }
    Ok(())
}

fn validate_target_fields(
    fields: &[&str; FIELD_COUNT],
) -> Result<(), AccountIdentityMutationAuthorityError> {
    let known_pair = matches!(
        (fields[29], fields[30]),
        ("revoke-child-device", "child-device")
            | ("revoke-setup-invite", "setup-invite")
            | ("revoke-recovery", "recovery")
    );
    if !known_pair
        || [34, 35, 36, 41, 42, 43]
            .iter()
            .any(|index| fields[*index].trim().is_empty())
        || fields[41].len() > super::super::validation::MAX_IDEMPOTENCY_KEY_BYTES
        || fields[41].bytes().any(|byte| byte.is_ascii_control())
        || chrono::DateTime::parse_from_rfc3339(fields[42]).is_err()
        || chrono::DateTime::parse_from_rfc3339(fields[43]).is_err()
        || [31, 32, 33]
            .iter()
            .any(|index| fields[*index].len() > super::super::validation::MAX_TARGET_ID_BYTES)
    {
        return Err(AccountIdentityMutationAuthorityError::InvalidEnvelope);
    }
    Ok(())
}

fn valid_key_id(value: &str) -> bool {
    value.len() == 71
        && value.starts_with("sha256:")
        && value[7..]
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}
