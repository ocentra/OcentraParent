#[path = "error_code.rs"]
mod code;
#[path = "error_display.rs"]
mod display;
#[path = "error_io.rs"]
mod io;
#[path = "error_code_secondary.rs"]
mod secondary_code;

const IO_FAILURE_CODE: &str = "io-failure";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ErrorCode(&'static str);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArtifactError {
    UnsupportedPlatform,
    UnsupportedOperation(&'static str),
    InvalidPath(&'static str),
    InvalidRequestId,
    RequestIdConflict,
    RootIdentityChanged,
    AncestorIdentityChanged,
    LinkOrReparseDetected,
    HardlinkDetected,
    OwnershipChanged,
    LockConflict,
    NotFound,
    AlreadyExists,
    SizeLimit,
    DurabilityFailure,
    DurabilityFailureWith(String),
    RecoveryRequired,
    Io(String),
}

impl ArtifactError {
    pub fn code(&self) -> ErrorCode {
        code::primary(self)
            .or_else(|| secondary_code::secondary(self))
            .unwrap_or(ErrorCode(IO_FAILURE_CODE))
    }
}

pub(crate) fn io_error(error: std::io::Error) -> ArtifactError {
    io::io_error(error)
}
