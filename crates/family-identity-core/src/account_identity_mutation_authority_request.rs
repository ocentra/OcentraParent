use super::{
    validation, AccountIdentityMutationAction, AccountIdentityMutationAuthorityRequest,
    AccountIdentityMutationTarget,
};
use crate::account_identity_mutation_authority_error::AccountIdentityMutationAuthorityError;
use crate::family_identity::{RecoveryId, SetupInviteId};

impl AccountIdentityMutationAuthorityRequest {
    pub fn revoke_child_device(
        child_profile_id: &crate::family_identity::ChildProfileId,
        child_device_id: &ocentra_schema::account_identity_authority::AccountIdentityChildDeviceId,
        idempotency_key: impl Into<String>,
    ) -> Result<Self, AccountIdentityMutationAuthorityError> {
        Self::validated(
            AccountIdentityMutationAction::RevokeChildDevice,
            AccountIdentityMutationTarget::ChildDevice {
                child_profile_id: child_profile_id.as_str().to_owned(),
                child_device_id: child_device_id.as_str().to_owned(),
            },
            idempotency_key.into(),
        )
    }

    pub fn revoke_setup_invite(
        invite_id: &SetupInviteId,
        idempotency_key: impl Into<String>,
    ) -> Result<Self, AccountIdentityMutationAuthorityError> {
        Self::validated(
            AccountIdentityMutationAction::RevokeSetupInvite,
            AccountIdentityMutationTarget::SetupInvite(invite_id.as_str().to_owned()),
            idempotency_key.into(),
        )
    }

    pub fn revoke_recovery(
        recovery_id: &RecoveryId,
        idempotency_key: impl Into<String>,
    ) -> Result<Self, AccountIdentityMutationAuthorityError> {
        Self::validated(
            AccountIdentityMutationAction::RevokeRecovery,
            AccountIdentityMutationTarget::Recovery(recovery_id.as_str().to_owned()),
            idempotency_key.into(),
        )
    }

    fn validated(
        action: AccountIdentityMutationAction,
        target: AccountIdentityMutationTarget,
        idempotency_key: String,
    ) -> Result<Self, AccountIdentityMutationAuthorityError> {
        let request = Self {
            action,
            target,
            idempotency_key,
        };
        validation::validate_request(&request)?;
        Ok(request)
    }

    pub fn action(&self) -> AccountIdentityMutationAction {
        self.action
    }

    pub fn idempotency_key(&self) -> &str {
        self.idempotency_key.as_str()
    }

    pub(crate) fn target(&self) -> &AccountIdentityMutationTarget {
        &self.target
    }
}
