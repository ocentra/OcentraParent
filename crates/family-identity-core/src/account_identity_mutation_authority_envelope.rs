use crate::account_identity_authority::VerifiedAccountIdentityAuthority;

use super::{AccountIdentityMutationAuthorityRequest, AccountIdentityMutationTarget};

pub(super) const ENVELOPE_VERSION: &str = "ocentra.account-mutation.v1";
pub(super) const SIGNATURE_ALGORITHM: &str = "ed25519";
pub(super) const AUDIENCE: &str = "ocentra.account.mutation";
pub(super) const ENVIRONMENT: &str = "account-owned";

pub(super) struct CanonicalMutationEnvelope<'a> {
    pub(super) key_id: &'a str,
    pub(super) provider: &'a str,
    pub(super) provider_subject: &'a str,
    pub(super) account_id: &'a str,
    pub(super) household_id: &'a str,
    pub(super) member_id: &'a str,
    pub(super) device_id: &'a str,
    pub(super) child_profile_id: &'a str,
    pub(super) child_device_id: &'a str,
    pub(super) session_id: &'a str,
    pub(super) session_generation: u64,
    pub(super) authority_generation: u64,
    pub(super) binding_generation: u64,
    pub(super) support_receipt: Option<&'a str>,
    pub(super) action: &'a str,
    pub(super) target_kind: &'a str,
    pub(super) target_id: &'a str,
    pub(super) target_child_profile_id: &'a str,
    pub(super) target_child_device_id: &'a str,
    pub(super) idempotency_key: &'a str,
    pub(super) issued_at: &'a str,
    pub(super) expires_at: &'a str,
}

pub(super) fn from_request<'a>(
    key_id: &'a str,
    authority: &'a VerifiedAccountIdentityAuthority,
    request: &'a AccountIdentityMutationAuthorityRequest,
    issued_at: &'a str,
    expires_at: &'a str,
) -> CanonicalMutationEnvelope<'a> {
    let (target_kind, target_id, target_child_profile_id, target_child_device_id) =
        target_parts(&request.target);
    CanonicalMutationEnvelope {
        key_id,
        provider: provider_label(authority.provider()),
        provider_subject: authority.provider_subject().as_str(),
        account_id: authority.account_id().as_str(),
        household_id: authority.household_id().as_str(),
        member_id: authority.member_id().as_str(),
        device_id: authority.device_id().as_str(),
        child_profile_id: authority.child_profile_id().as_str(),
        child_device_id: authority.child_device_id().as_str(),
        session_id: authority.session_id().as_str(),
        session_generation: authority.session_generation(),
        authority_generation: authority.authority_generation(),
        binding_generation: authority.current_binding().authority_generation,
        support_receipt: authority
            .support_audit_identity()
            .map(|value| value.as_str()),
        action: request.action.as_str(),
        target_kind,
        target_id,
        target_child_profile_id,
        target_child_device_id,
        idempotency_key: request.idempotency_key.as_str(),
        issued_at,
        expires_at,
    }
}

pub(super) fn encode(envelope: &CanonicalMutationEnvelope<'_>) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(1024);
    for field in [
        ENVELOPE_VERSION,
        SIGNATURE_ALGORITHM,
        AUDIENCE,
        ENVIRONMENT,
        envelope.key_id,
        envelope.provider,
        envelope.provider_subject,
        envelope.account_id,
        envelope.household_id,
        envelope.member_id,
        envelope.device_id,
        envelope.child_profile_id,
        envelope.child_device_id,
        envelope.session_id,
        envelope.support_receipt.unwrap_or_default(),
        envelope.action,
        envelope.target_kind,
        envelope.target_id,
        envelope.target_child_profile_id,
        envelope.target_child_device_id,
        envelope.idempotency_key,
        envelope.issued_at,
        envelope.expires_at,
    ] {
        append_string(&mut bytes, field);
    }
    for number in [
        envelope.session_generation,
        envelope.authority_generation,
        envelope.binding_generation,
    ] {
        bytes.extend_from_slice(&number.to_be_bytes());
    }
    bytes
}

fn target_parts(target: &AccountIdentityMutationTarget) -> (&str, &str, &str, &str) {
    match target {
        AccountIdentityMutationTarget::ChildDevice {
            child_profile_id,
            child_device_id,
        } => ("child-device", "", child_profile_id, child_device_id),
        AccountIdentityMutationTarget::SetupInvite(invite_id) => {
            ("setup-invite", invite_id, "", "")
        }
        AccountIdentityMutationTarget::Recovery(recovery_id) => ("recovery", recovery_id, "", ""),
    }
}

fn append_string(bytes: &mut Vec<u8>, value: &str) {
    bytes.extend_from_slice(&(value.len() as u32).to_be_bytes());
    bytes.extend_from_slice(value.as_bytes());
}

fn provider_label(
    provider: &ocentra_schema::account_identity_authority::AccountIdentityProvider,
) -> &'static str {
    match provider {
        ocentra_schema::account_identity_authority::AccountIdentityProvider::Authjs => "authjs",
        ocentra_schema::account_identity_authority::AccountIdentityProvider::Firebase => "firebase",
    }
}
