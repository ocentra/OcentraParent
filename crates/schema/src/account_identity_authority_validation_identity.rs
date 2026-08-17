use super::super::{
    AccountIdentityCurrentMemberDeviceAuthorityHandoff, AccountIdentityMappingStatus,
    AccountIdentityMemberAuthoritySchemaVersion, AccountIdentityMemberAuthorityValidationError,
};

pub(super) fn validate(
    handoff: &AccountIdentityCurrentMemberDeviceAuthorityHandoff,
) -> Result<(), AccountIdentityMemberAuthorityValidationError> {
    (handoff.schema_version == AccountIdentityMemberAuthoritySchemaVersion::V0_1)
        .then_some(())
        .ok_or(AccountIdentityMemberAuthorityValidationError::SchemaVersionMismatch)?;
    (handoff.mapping.status == AccountIdentityMappingStatus::Active)
        .then_some(())
        .ok_or(AccountIdentityMemberAuthorityValidationError::InactiveProviderMapping)?;
    (handoff.mapping.account_id == handoff.member.account_id)
        .then_some(())
        .ok_or(AccountIdentityMemberAuthorityValidationError::MappingAccountMismatch)?;
    (handoff.member.account_id == handoff.binding.account_id)
        .then_some(())
        .ok_or(AccountIdentityMemberAuthorityValidationError::MemberAccountMismatch)?;
    (handoff.member.household_id == handoff.binding.household_id)
        .then_some(())
        .ok_or(AccountIdentityMemberAuthorityValidationError::MemberHouseholdMismatch)?;
    (handoff.binding.account_id == handoff.member.account_id)
        .then_some(())
        .ok_or(AccountIdentityMemberAuthorityValidationError::BindingAccountMismatch)?;
    (handoff.binding.household_id == handoff.member.household_id)
        .then_some(())
        .ok_or(AccountIdentityMemberAuthorityValidationError::BindingHouseholdMismatch)
}
