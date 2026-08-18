use std::fmt;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum AccountIdentityMutationAuthorityError {
    InvalidRequest,
    InvalidEnvelope,
    InvalidAuthority,
    AuthorityExpired,
    RoleNotAuthorized,
    TargetMismatch,
    TargetNotCurrent,
    StepUpUnavailable,
    SignerCustodyUnavailable,
    VerificationKeyUnavailable,
    SignatureInvalid,
    ClockUnavailable,
    IdempotencyConflict,
    EffectPending,
    EffectStateInvalid,
    RepositoryUnavailable,
}

impl fmt::Display for AccountIdentityMutationAuthorityError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl std::error::Error for AccountIdentityMutationAuthorityError {}
