use core::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AppGameWindowsLocalPolicyError {
    UnsupportedPlatform,
    SystemDirectoryUnavailable,
    TrustedExecutableUnavailable,
    TrustedExecutableChanged,
    ReparsePointRejected,
    UntrustedOwner,
    UntrustedAcl,
    ProcessSpawn(u32),
    ProcessIdentityMismatch,
    ProcessTimeout,
    ProcessKill(u32),
    ProcessReap(u32),
    ProcessFailed(i32),
    UnexpectedStandardError,
    OutputTooLarge,
    OutputInvalidUtf8,
    OutputInvalidJson,
    OutputInvalidSchemaVersion,
    OutputInvalidInvariant,
    WindowsApi(u32),
}

impl fmt::Display for AppGameWindowsLocalPolicyError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, formatter)
    }
}

impl std::error::Error for AppGameWindowsLocalPolicyError {}
