use chrono::{DateTime, Utc};
use ocentra_schema::account_identity_authority::{
    AccountIdentityCurrentMemberDeviceAuthorityHandoff, AccountIdentityProviderSubject,
    AccountIdentityRole, AccountIdentitySessionFreshnessState,
    AccountIdentitySupportReceiptRevocationState, ACCOUNT_IDENTITY_AUTHORITY_MAX_GENERATION,
};

pub(super) fn validate_current_session(
    handoff: &AccountIdentityCurrentMemberDeviceAuthorityHandoff,
) -> Result<(), ()> {
    if handoff.member.session_freshness_state != AccountIdentitySessionFreshnessState::Fresh
        || handoff.member.session_generation == 0
        || handoff.member.session_generation > ACCOUNT_IDENTITY_AUTHORITY_MAX_GENERATION
    {
        return Err(());
    }
    let expires_at = DateTime::parse_from_rfc3339(&handoff.member.session_expires_at)
        .map_err(|_| ())?
        .with_timezone(&Utc);
    (expires_at > Utc::now()).then_some(()).ok_or(())
}

pub(super) fn validate_support_receipt(
    handoff: &AccountIdentityCurrentMemberDeviceAuthorityHandoff,
    provider_subject: &AccountIdentityProviderSubject,
) -> Result<(), ()> {
    let Some(receipt) = handoff.member.support_receipt.as_ref() else {
        return (handoff.member.role != AccountIdentityRole::SupportAdmin)
            .then_some(())
            .ok_or(());
    };
    if receipt.provider_subject != *provider_subject
        || receipt.account_id != handoff.member.account_id
        || receipt.member_id != handoff.member.member_id
        || receipt.household_id != handoff.member.household_id
        || receipt.device_id != handoff.member.device_id
        || receipt.child_profile_id != handoff.binding.child_profile_id
        || receipt.child_device_id != handoff.binding.child_device_id
        || receipt.revocation_state != AccountIdentitySupportReceiptRevocationState::Active
    {
        return Err(());
    }
    let issued_at = DateTime::parse_from_rfc3339(&receipt.issued_at)
        .map_err(|_| ())?
        .with_timezone(&Utc);
    let expires_at = DateTime::parse_from_rfc3339(&receipt.expires_at)
        .map_err(|_| ())?
        .with_timezone(&Utc);
    let now = Utc::now();
    (issued_at <= now && now < expires_at)
        .then_some(())
        .ok_or(())
}
