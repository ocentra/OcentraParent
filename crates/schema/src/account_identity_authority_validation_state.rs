use super::super::{
    AccountIdentityAccountState, AccountIdentityCurrentMemberDeviceAuthorityHandoff,
    AccountIdentityDeviceTrustState, AccountIdentityMemberAuthorityValidationError,
    AccountIdentityMembershipState, AccountIdentityRole, AccountIdentitySessionFreshnessState,
    AccountIdentitySupportReceiptRevocationState, ACCOUNT_IDENTITY_AUTHORITY_MAX_GENERATION,
};

pub(super) fn validate(
    handoff: &AccountIdentityCurrentMemberDeviceAuthorityHandoff,
) -> Result<(), AccountIdentityMemberAuthorityValidationError> {
    (handoff.member.account_state == AccountIdentityAccountState::Active)
        .then_some(())
        .ok_or(AccountIdentityMemberAuthorityValidationError::InactiveAccount)?;
    (handoff.member.membership_state == AccountIdentityMembershipState::Active)
        .then_some(())
        .ok_or(AccountIdentityMemberAuthorityValidationError::InactiveMembership)?;
    (handoff.member.device_trust_state == AccountIdentityDeviceTrustState::Trusted)
        .then_some(())
        .ok_or(AccountIdentityMemberAuthorityValidationError::UntrustedDevice)?;
    (handoff.member.session_freshness_state == AccountIdentitySessionFreshnessState::Fresh)
        .then_some(())
        .ok_or(AccountIdentityMemberAuthorityValidationError::StaleSession)?;
    (handoff.member.session_generation > 0
        && handoff.member.session_generation <= ACCOUNT_IDENTITY_AUTHORITY_MAX_GENERATION)
        .then_some(())
        .ok_or(AccountIdentityMemberAuthorityValidationError::SessionGenerationInvalid)?;
    (!handoff.member.session_expires_at.trim().is_empty())
        .then_some(())
        .ok_or(AccountIdentityMemberAuthorityValidationError::SessionExpiryMissing)?;
    validate_support_receipt(handoff)
}

fn validate_support_receipt(
    handoff: &AccountIdentityCurrentMemberDeviceAuthorityHandoff,
) -> Result<(), AccountIdentityMemberAuthorityValidationError> {
    if handoff.member.role == AccountIdentityRole::SupportAdmin
        && handoff.member.support_receipt.is_none()
    {
        return Err(AccountIdentityMemberAuthorityValidationError::SupportReceiptRequired);
    }
    if let Some(receipt) = handoff.member.support_receipt.as_ref() {
        if receipt.issued_at.trim().is_empty() || receipt.expires_at.trim().is_empty() {
            return Err(AccountIdentityMemberAuthorityValidationError::SupportReceiptInvalid);
        }
        if receipt.revocation_state == AccountIdentitySupportReceiptRevocationState::Revoked {
            return Err(AccountIdentityMemberAuthorityValidationError::SupportReceiptRevoked);
        }
    }
    Ok(())
}
