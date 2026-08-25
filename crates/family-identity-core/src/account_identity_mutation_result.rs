use super::AccountIdentityMutationResult;

impl AccountIdentityMutationResult {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::SetupInviteRevoked => "setup-invite-revoked",
            Self::RecoveryRevoked => "recovery-revoked",
        }
    }

    pub(crate) fn parse(value: &str) -> Option<Self> {
        match value {
            "setup-invite-revoked" => Some(Self::SetupInviteRevoked),
            "recovery-revoked" => Some(Self::RecoveryRevoked),
            _ => None,
        }
    }
}
