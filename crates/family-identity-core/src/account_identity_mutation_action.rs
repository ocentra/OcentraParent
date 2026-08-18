use super::AccountIdentityMutationAction;

impl AccountIdentityMutationAction {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::RevokeChildDevice => "revoke-child-device",
            Self::RevokeSetupInvite => "revoke-setup-invite",
            Self::RevokeRecovery => "revoke-recovery",
        }
    }

    pub(crate) fn parse(value: &str) -> Option<Self> {
        match value {
            "revoke-child-device" => Some(Self::RevokeChildDevice),
            "revoke-setup-invite" => Some(Self::RevokeSetupInvite),
            "revoke-recovery" => Some(Self::RevokeRecovery),
            _ => None,
        }
    }
}
