use std::fmt;

use super::ChildAgentServiceError;

impl fmt::Display for ChildAgentServiceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Configuration(error) => {
                write!(formatter, "child service configuration failed: {error}")
            }
            Self::Runtime(error) => write!(formatter, "child runtime operation failed: {error}"),
            Self::Storage(error) => {
                write!(formatter, "child service durable storage failed: {error}")
            }
            Self::Shutdown(error) => {
                write!(formatter, "child service shutdown signal failed: {error}")
            }
            Self::RecoveryPending(readiness) => {
                write!(formatter, "child service is not ready: {readiness:?}")
            }
            Self::TrustBindingManualRequired => {
                write!(
                    formatter,
                    "child service identity binding requires manual setup"
                )
            }
            Self::TamperManualRequired { signal_ref } => {
                write!(
                    formatter,
                    "child service tamper evidence requires manual review: {signal_ref:?}"
                )
            }
            Self::TrustRevoked { audit_ref } => {
                write!(formatter, "child service trust is revoked: {audit_ref:?}")
            }
            Self::UnknownDomain(domain) => {
                write!(
                    formatter,
                    "child service has no runtime flow for {domain:?}"
                )
            }
        }
    }
}
