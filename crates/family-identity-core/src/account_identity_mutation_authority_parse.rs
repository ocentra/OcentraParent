use super::envelope::{
    encode, CanonicalMutationEnvelope, CANONICAL_FIELD_COUNT, MAX_CANONICAL_PAYLOAD_BYTES,
};
use crate::account_identity_mutation_authority_error::AccountIdentityMutationAuthorityError;

pub(super) const FIELD_COUNT: usize = CANONICAL_FIELD_COUNT;

#[path = "account_identity_mutation_authority_parse_cursor.rs"]
mod cursor;
#[path = "account_identity_mutation_authority_parse_target_validation.rs"]
mod target_validation;
#[path = "account_identity_mutation_authority_parse_validation.rs"]
mod validation;

pub(crate) struct ParsedSignedMutationAuthority {
    pub(crate) payload: Vec<u8>,
    pub(crate) signature: [u8; 64],
    pub(crate) envelope: CanonicalMutationEnvelope,
}

pub(crate) fn parse_wire(
    wire: &[u8],
) -> Result<ParsedSignedMutationAuthority, AccountIdentityMutationAuthorityError> {
    let payload_length = wire
        .get(..4)
        .and_then(|value| <[u8; 4]>::try_from(value).ok())
        .map(u32::from_be_bytes)
        .and_then(|value| usize::try_from(value).ok())
        .ok_or(AccountIdentityMutationAuthorityError::InvalidEnvelope)?;
    if payload_length == 0 || payload_length > MAX_CANONICAL_PAYLOAD_BYTES {
        return Err(AccountIdentityMutationAuthorityError::InvalidEnvelope);
    }
    let expected_length = 4_usize
        .checked_add(payload_length)
        .and_then(|value| value.checked_add(64))
        .ok_or(AccountIdentityMutationAuthorityError::InvalidEnvelope)?;
    if wire.len() != expected_length {
        return Err(AccountIdentityMutationAuthorityError::InvalidEnvelope);
    }
    let payload = wire[4..4 + payload_length].to_vec();
    let signature = <[u8; 64]>::try_from(&wire[4 + payload_length..])
        .map_err(|_error| AccountIdentityMutationAuthorityError::InvalidEnvelope)?;
    let envelope = parse_payload(&payload)?;
    if encode(&envelope)? != payload {
        return Err(AccountIdentityMutationAuthorityError::InvalidEnvelope);
    }
    Ok(ParsedSignedMutationAuthority {
        payload,
        signature,
        envelope,
    })
}

fn parse_payload(
    payload: &[u8],
) -> Result<CanonicalMutationEnvelope, AccountIdentityMutationAuthorityError> {
    let mut cursor = cursor::Cursor::new(payload);
    let fields = cursor.read_strings::<FIELD_COUNT>()?;
    let numbers = cursor.read_u64s::<3>()?;
    let signed_numbers = cursor.read_i64s::<2>()?;
    cursor.finish()?;
    validation::validate_fields(&fields)?;
    let envelope = CanonicalMutationEnvelope {
        key_id: fields[4].clone(),
        provider: fields[5].clone(),
        provider_subject: fields[6].clone(),
        account_id: fields[7].clone(),
        household_id: fields[8].clone(),
        member_id: fields[9].clone(),
        role: fields[10].clone(),
        device_id: fields[11].clone(),
        child_profile_id: fields[12].clone(),
        child_device_id: fields[13].clone(),
        session_id: fields[14].clone(),
        support_receipt_id: fields[15].clone(),
        support_provider_subject: fields[16].clone(),
        support_account_id: fields[17].clone(),
        support_member_id: fields[18].clone(),
        support_household_id: fields[19].clone(),
        support_device_id: fields[20].clone(),
        support_child_profile_id: fields[21].clone(),
        support_child_device_id: fields[22].clone(),
        support_scope: fields[23].clone(),
        support_issuer: fields[24].clone(),
        support_issued_at: fields[25].clone(),
        support_expires_at: fields[26].clone(),
        support_revocation_state: fields[27].clone(),
        support_audit_identity: fields[28].clone(),
        action: fields[29].clone(),
        target_kind: fields[30].clone(),
        target_id: fields[31].clone(),
        target_child_profile_id: fields[32].clone(),
        target_child_device_id: fields[33].clone(),
        target_household_id: fields[34].clone(),
        target_owner_member_id: fields[35].clone(),
        target_state: fields[36].clone(),
        target_support_channel: fields[37].clone(),
        target_support_authorization_id: fields[38].clone(),
        target_support_authorization_issuer: fields[39].clone(),
        target_support_authorization_scope: fields[40].clone(),
        idempotency_key: fields[41].clone(),
        issued_at: fields[42].clone(),
        expires_at: fields[43].clone(),
        session_generation: numbers[0],
        authority_generation: numbers[1],
        binding_generation: numbers[2],
        target_expires_at_epoch_millis: signed_numbers[0],
        target_support_authorization_expires_at_epoch_millis: signed_numbers[1],
    };
    validate_issued_envelope(&envelope)?;
    Ok(envelope)
}

pub(crate) fn validate_issued_envelope(
    envelope: &CanonicalMutationEnvelope,
) -> Result<(), AccountIdentityMutationAuthorityError> {
    validation::validate_envelope(envelope)?;
    target_validation::validate_envelope(envelope)
}
