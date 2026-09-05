use super::AccountIdentityMutationAction;

impl AccountIdentityMutationAction {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::RevokeChildDevice => "revoke-child-device",
            Self::RevokeSetupInvite => "revoke-setup-invite",
            Self::RevokeRecovery => "revoke-recovery",
        }
    }
}
