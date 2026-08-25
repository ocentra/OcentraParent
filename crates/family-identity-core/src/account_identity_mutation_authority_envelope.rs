use crate::account_identity_authority::VerifiedAccountIdentityAuthority;

use super::protocol::{provider_label, role_label, support_revocation_label, support_scope_label};
use super::{AccountIdentityMutationAuthorityRequest, ResolvedAccountIdentityMutationTarget};
use crate::account_identity_mutation_authority_error::AccountIdentityMutationAuthorityError;

pub(crate) const ENVELOPE_VERSION: &str = "ocentra.account-mutation.v1";
pub(crate) const SIGNATURE_ALGORITHM: &str = "ed25519";
pub(crate) const AUDIENCE: &str = "ocentra.account.mutation";
pub(crate) const ENVIRONMENT: &str = "account-owned";
pub(crate) const CANONICAL_FIELD_COUNT: usize = 44;
pub(crate) const MAX_CANONICAL_FIELD_BYTES: usize = 1024;
pub(crate) const MAX_CANONICAL_PAYLOAD_BYTES: usize = 8 * 1024;

pub(crate) struct CanonicalMutationEnvelope {
    pub(crate) key_id: String,
    pub(crate) provider: String,
    pub(crate) provider_subject: String,
    pub(crate) account_id: String,
    pub(crate) household_id: String,
    pub(crate) member_id: String,
    pub(crate) role: String,
    pub(crate) device_id: String,
    pub(crate) child_profile_id: String,
    pub(crate) child_device_id: String,
    pub(crate) session_id: String,
    pub(crate) support_receipt_id: String,
    pub(crate) support_provider_subject: String,
    pub(crate) support_account_id: String,
    pub(crate) support_member_id: String,
    pub(crate) support_household_id: String,
    pub(crate) support_device_id: String,
    pub(crate) support_child_profile_id: String,
    pub(crate) support_child_device_id: String,
    pub(crate) support_scope: String,
    pub(crate) support_issuer: String,
    pub(crate) support_issued_at: String,
    pub(crate) support_expires_at: String,
    pub(crate) support_revocation_state: String,
    pub(crate) support_audit_identity: String,
    pub(crate) action: String,
    pub(crate) target_kind: String,
    pub(crate) target_id: String,
    pub(crate) target_child_profile_id: String,
    pub(crate) target_child_device_id: String,
    pub(crate) target_household_id: String,
    pub(crate) target_owner_member_id: String,
    pub(crate) target_state: String,
    pub(crate) target_support_channel: String,
    pub(crate) target_support_authorization_id: String,
    pub(crate) target_support_authorization_issuer: String,
    pub(crate) target_support_authorization_scope: String,
    pub(crate) idempotency_key: String,
    pub(crate) issued_at: String,
    pub(crate) expires_at: String,
    pub(crate) session_generation: u64,
    pub(crate) authority_generation: u64,
    pub(crate) binding_generation: u64,
    pub(crate) target_expires_at_epoch_millis: i64,
    pub(crate) target_support_authorization_expires_at_epoch_millis: i64,
}

pub(crate) fn from_resolved(
    key_id: &str,
    authority: &VerifiedAccountIdentityAuthority,
    request: &AccountIdentityMutationAuthorityRequest,
    target: &ResolvedAccountIdentityMutationTarget,
    issued_at: &str,
    expires_at: &str,
) -> CanonicalMutationEnvelope {
    let mut envelope = base_envelope(key_id, authority, request, target, issued_at, expires_at);
    if let Some(receipt) = authority.support_receipt() {
        envelope.support_receipt_id = receipt.receipt_id.as_str().to_owned();
        envelope.support_provider_subject = receipt.provider_subject.as_str().to_owned();
        envelope.support_account_id = receipt.account_id.to_string();
        envelope.support_member_id = receipt.member_id.as_str().to_owned();
        envelope.support_household_id = receipt.household_id.to_string();
        envelope.support_device_id = receipt.device_id.as_str().to_owned();
        envelope.support_child_profile_id = receipt.child_profile_id.to_string();
        envelope.support_child_device_id = receipt.child_device_id.as_str().to_owned();
        envelope.support_scope = support_scope_label(receipt.scope).to_owned();
        envelope.support_issuer = receipt.issuer.as_str().to_owned();
        envelope.support_issued_at.clone_from(&receipt.issued_at);
        envelope.support_expires_at.clone_from(&receipt.expires_at);
        envelope.support_revocation_state =
            support_revocation_label(receipt.revocation_state).to_owned();
        envelope.support_audit_identity = receipt.audit_identity.as_str().to_owned();
    }
    envelope
}

fn base_envelope(
    key_id: &str,
    authority: &VerifiedAccountIdentityAuthority,
    request: &AccountIdentityMutationAuthorityRequest,
    target: &ResolvedAccountIdentityMutationTarget,
    issued_at: &str,
    expires_at: &str,
) -> CanonicalMutationEnvelope {
    CanonicalMutationEnvelope {
        key_id: key_id.to_owned(),
        provider: provider_label(authority.provider()).to_owned(),
        provider_subject: authority.provider_subject().as_str().to_owned(),
        account_id: authority.account_id().to_string(),
        household_id: authority.household_id().to_string(),
        member_id: authority.member_id().as_str().to_owned(),
        role: role_label(authority.role()).to_owned(),
        device_id: authority.device_id().as_str().to_owned(),
        child_profile_id: authority.child_profile_id().to_string(),
        child_device_id: authority.child_device_id().as_str().to_owned(),
        session_id: authority.session_id().as_str().to_owned(),
        support_receipt_id: String::new(),
        support_provider_subject: String::new(),
        support_account_id: String::new(),
        support_member_id: String::new(),
        support_household_id: String::new(),
        support_device_id: String::new(),
        support_child_profile_id: String::new(),
        support_child_device_id: String::new(),
        support_scope: String::new(),
        support_issuer: String::new(),
        support_issued_at: String::new(),
        support_expires_at: String::new(),
        support_revocation_state: String::new(),
        support_audit_identity: String::new(),
        action: request.action().as_str().to_owned(),
        target_kind: target.kind.clone(),
        target_id: target.target_id.clone(),
        target_child_profile_id: target.child_profile_id.clone(),
        target_child_device_id: target.child_device_id.clone(),
        target_household_id: target.household_id.clone(),
        target_owner_member_id: target.owner_member_id.clone(),
        target_state: target.state.clone(),
        target_support_channel: target.support_channel.clone(),
        target_support_authorization_id: target.support_authorization_id.clone(),
        target_support_authorization_issuer: target.support_authorization_issuer.clone(),
        target_support_authorization_scope: target.support_authorization_scope.clone(),
        idempotency_key: request.idempotency_key().to_owned(),
        issued_at: issued_at.to_owned(),
        expires_at: expires_at.to_owned(),
        session_generation: authority.session_generation(),
        authority_generation: authority.authority_generation(),
        binding_generation: authority.current_binding().authority_generation,
        target_expires_at_epoch_millis: target.expires_at_epoch_millis,
        target_support_authorization_expires_at_epoch_millis: target
            .support_authorization_expires_at_epoch_millis,
    }
}

pub(crate) fn encode(
    envelope: &CanonicalMutationEnvelope,
) -> Result<Vec<u8>, AccountIdentityMutationAuthorityError> {
    super::parse::validate_issued_envelope(envelope)?;
    let fields = string_fields(envelope);
    let capacity = encoded_capacity(&fields)?;
    let mut bytes = Vec::with_capacity(capacity);
    for field in fields {
        append_string(&mut bytes, field)?;
    }
    for number in [
        envelope.session_generation,
        envelope.authority_generation,
        envelope.binding_generation,
    ] {
        bytes.extend_from_slice(&number.to_be_bytes());
    }
    for number in [
        envelope.target_expires_at_epoch_millis,
        envelope.target_support_authorization_expires_at_epoch_millis,
    ] {
        bytes.extend_from_slice(&number.to_be_bytes());
    }
    (bytes.len() <= MAX_CANONICAL_PAYLOAD_BYTES)
        .then_some(bytes)
        .ok_or(AccountIdentityMutationAuthorityError::InvalidEnvelope)
}

pub(super) fn string_fields(envelope: &CanonicalMutationEnvelope) -> [&str; CANONICAL_FIELD_COUNT] {
    [
        ENVELOPE_VERSION,
        SIGNATURE_ALGORITHM,
        AUDIENCE,
        ENVIRONMENT,
        &envelope.key_id,
        &envelope.provider,
        &envelope.provider_subject,
        &envelope.account_id,
        &envelope.household_id,
        &envelope.member_id,
        &envelope.role,
        &envelope.device_id,
        &envelope.child_profile_id,
        &envelope.child_device_id,
        &envelope.session_id,
        &envelope.support_receipt_id,
        &envelope.support_provider_subject,
        &envelope.support_account_id,
        &envelope.support_member_id,
        &envelope.support_household_id,
        &envelope.support_device_id,
        &envelope.support_child_profile_id,
        &envelope.support_child_device_id,
        &envelope.support_scope,
        &envelope.support_issuer,
        &envelope.support_issued_at,
        &envelope.support_expires_at,
        &envelope.support_revocation_state,
        &envelope.support_audit_identity,
        &envelope.action,
        &envelope.target_kind,
        &envelope.target_id,
        &envelope.target_child_profile_id,
        &envelope.target_child_device_id,
        &envelope.target_household_id,
        &envelope.target_owner_member_id,
        &envelope.target_state,
        &envelope.target_support_channel,
        &envelope.target_support_authorization_id,
        &envelope.target_support_authorization_issuer,
        &envelope.target_support_authorization_scope,
        &envelope.idempotency_key,
        &envelope.issued_at,
        &envelope.expires_at,
    ]
}

fn encoded_capacity(
    fields: &[&str; CANONICAL_FIELD_COUNT],
) -> Result<usize, AccountIdentityMutationAuthorityError> {
    let mut total = 5_usize
        .checked_mul(8)
        .ok_or(AccountIdentityMutationAuthorityError::InvalidEnvelope)?;
    for field in fields {
        if field.len() > MAX_CANONICAL_FIELD_BYTES {
            return Err(AccountIdentityMutationAuthorityError::InvalidEnvelope);
        }
        u32::try_from(field.len())
            .map_err(|_| AccountIdentityMutationAuthorityError::InvalidEnvelope)?;
        total = total
            .checked_add(4)
            .and_then(|value| value.checked_add(field.len()))
            .ok_or(AccountIdentityMutationAuthorityError::InvalidEnvelope)?;
    }
    (total <= MAX_CANONICAL_PAYLOAD_BYTES)
        .then_some(total)
        .ok_or(AccountIdentityMutationAuthorityError::InvalidEnvelope)
}

fn append_string(
    bytes: &mut Vec<u8>,
    value: &str,
) -> Result<(), AccountIdentityMutationAuthorityError> {
    let length = u32::try_from(value.len())
        .map_err(|_| AccountIdentityMutationAuthorityError::InvalidEnvelope)?;
    bytes.extend_from_slice(&length.to_be_bytes());
    bytes.extend_from_slice(value.as_bytes());
    Ok(())
}
