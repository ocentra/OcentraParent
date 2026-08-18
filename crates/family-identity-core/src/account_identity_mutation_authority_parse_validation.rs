use super::super::envelope::{AUDIENCE, ENVELOPE_VERSION, ENVIRONMENT, SIGNATURE_ALGORITHM};
use super::FIELD_COUNT;
use crate::account_identity_mutation_authority_error::AccountIdentityMutationAuthorityError;

pub(super) fn validate_fields(
    fields: &[String; FIELD_COUNT],
) -> Result<(), AccountIdentityMutationAuthorityError> {
    if fields[0] != ENVELOPE_VERSION
        || fields[1] != SIGNATURE_ALGORITHM
        || fields[2] != AUDIENCE
        || fields[3] != ENVIRONMENT
        || !valid_key_id(&fields[4])
    {
        return Err(AccountIdentityMutationAuthorityError::InvalidEnvelope);
    }
    if fields[5..15].iter().any(|value| value.trim().is_empty())
        || !["Authjs", "Firebase"].contains(&fields[5].as_str())
        || !["ParentOwner", "CoParentGuardian"].contains(&fields[10].as_str())
    {
        return Err(AccountIdentityMutationAuthorityError::InvalidEnvelope);
    }
    validate_support_fields(&fields[15..29])?;
    validate_target_fields(fields)
}

fn validate_support_fields(fields: &[String]) -> Result<(), AccountIdentityMutationAuthorityError> {
    let all_empty = fields.iter().all(String::is_empty);
    let all_present = fields.iter().all(|value| !value.trim().is_empty());
    if !all_empty && !all_present {
        return Err(AccountIdentityMutationAuthorityError::InvalidEnvelope);
    }
    if all_present
        && (!["ReadOnly", "Household", "DeviceControl"].contains(&fields[8].as_str())
            || fields[12] != "Active")
    {
        return Err(AccountIdentityMutationAuthorityError::InvalidEnvelope);
    }
    Ok(())
}

fn validate_target_fields(
    fields: &[String; FIELD_COUNT],
) -> Result<(), AccountIdentityMutationAuthorityError> {
    let known_pair = matches!(
        (fields[29].as_str(), fields[30].as_str()),
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
