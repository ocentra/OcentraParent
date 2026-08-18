use std::fmt;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum AccountIdentityMutationAuthorityError {
    InvalidRequest,
    InvalidAuthority,
    AuthorityExpired,
    RoleNotAuthorized,
    TargetMismatch,
    StepUpUnavailable,
    EntropyUnavailable,
    SignatureUnavailable,
}

impl fmt::Display for AccountIdentityMutationAuthorityError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::InvalidRequest => "mutation authority request rejected",
            Self::InvalidAuthority => "current account authority is invalid",
            Self::AuthorityExpired => "current account authority is expired",
            Self::RoleNotAuthorized => "account role cannot issue this mutation",
            Self::TargetMismatch => "mutation target does not match current account binding",
            Self::StepUpUnavailable => "parent step-up authority is unavailable",
            Self::EntropyUnavailable => "account mutation signer entropy unavailable",
            Self::SignatureUnavailable => "account mutation signature unavailable",
        })
    }
}

impl std::error::Error for AccountIdentityMutationAuthorityError {}
