use thiserror::Error;

pub(crate) const RECORD_NAMESPACE: &[u8] = b"ocentra.protected-capability-custody.v1";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SecurityLevel {
    Unavailable,
    InProcessOnly,
    SameUserIsolated,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlatformAttestation {
    pub security_level: SecurityLevel,
    pub key_epoch: u64,
    pub writer_epoch: u64,
    pub anti_rollback_watermark: u64,
}

impl PlatformAttestation {
    pub fn new(
        security_level: SecurityLevel,
        key_epoch: u64,
        writer_epoch: u64,
        anti_rollback_watermark: u64,
    ) -> Result<Self, PlatformError> {
        if key_epoch == 0 || writer_epoch == 0 || anti_rollback_watermark == 0 {
            return Err(PlatformError::InvalidAttestation);
        }
        Ok(Self {
            security_level,
            key_epoch,
            writer_epoch,
            anti_rollback_watermark,
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum SealedState {
    Prepared = 1,
    CommitAmbiguous = 2,
    AbortAmbiguous = 3,
    Committed = 4,
    Aborted = 5,
}

#[derive(Clone, Copy)]
pub struct SealContext<'a> {
    pub record_namespace: &'a [u8],
    pub canonical_binding: &'a [u8],
    pub state: SealedState,
    pub sequence: u64,
    pub key_epoch: u64,
    pub writer_epoch: u64,
    pub anti_rollback_watermark: u64,
}

#[derive(Debug, Error)]
pub enum PlatformError {
    #[error("platform custody is unavailable")]
    Unavailable,
    #[error("platform custody rejected the request")]
    Rejected,
    #[error("platform custody detected tampering")]
    Tampered,
    #[error("platform custody detected a wrong binding")]
    WrongBinding,
    #[error("platform custody key material rotated")]
    Rotated,
    #[error("platform custody detected a conflicting writer")]
    Conflict,
    #[error("platform custody anti-rollback watermark regressed")]
    AntiRollback,
    #[error("platform returned an invalid attestation")]
    InvalidAttestation,
}

/// The production implementation must be an authenticated broker or platform
/// adapter. In-process sealing is intentionally rejected by the core.
pub trait PlatformCustodyPort: Send + Sync {
    fn attest(&self) -> Result<PlatformAttestation, PlatformError>;

    /// Seal the versioned namespace, complete canonical binding, and transition
    /// metadata as authenticated associated data. The adapter must authenticate
    /// the isolated writer.
    fn seal(&self, context: SealContext<'_>) -> Result<Vec<u8>, PlatformError>;

    /// Verify the sealed record against every field in `context`; this must not
    /// return success for a different binding, state, sequence, or epoch.
    fn open(&self, context: SealContext<'_>, sealed: &[u8]) -> Result<(), PlatformError>;
}
