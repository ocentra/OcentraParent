use std::fs;
use std::io;

use super::{
    removal_record::status_from_record,
    removal_validation::{current_unix_seconds, non_empty_signal_ref},
    ChildAgentRemovalBoundary, ChildAgentRemovalStatus, ChildAgentServiceIdentity,
    ChildAgentTamperSignal, ChildAgentTamperSignalKind,
};

impl ChildAgentRemovalBoundary {
    pub(crate) fn open_with_identity(
        path: impl Into<std::path::PathBuf>,
        identity: Option<ChildAgentServiceIdentity>,
    ) -> io::Result<Self> {
        let path = path.into();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        if let Ok(metadata) = fs::symlink_metadata(&path) {
            if metadata.file_type().is_symlink() {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "child removal state must not be a symlink",
                ));
            }
        }
        let boundary = Self { path, identity };
        boundary.status()?;
        Ok(boundary)
    }

    pub fn status(&self) -> io::Result<ChildAgentRemovalStatus> {
        self.with_locked_record(|record| Ok(status_from_record(record)))
    }

    pub fn record_tamper_signal(
        &self,
        signal_ref: impl Into<String>,
        kind: ChildAgentTamperSignalKind,
    ) -> io::Result<ChildAgentRemovalStatus> {
        let signal_ref = non_empty_signal_ref(&signal_ref.into())?;
        let observed_at_unix_seconds = current_unix_seconds()?;
        self.with_locked_record(|record| {
            if record
                .tamper_signals
                .iter()
                .any(|signal| signal.signal_ref == signal_ref)
            {
                return Ok(status_from_record(record));
            }
            record.tamper_signals.push(ChildAgentTamperSignal {
                signal_ref,
                kind,
                observed_at_unix_seconds,
            });
            Ok(status_from_record(record))
        })
    }

    pub(super) fn require_identity(
        &self,
        authorization: &super::VerifiedParentRemovalAuthorization,
        expected_action: super::ChildAgentRemovalAuthorizationAction,
    ) -> io::Result<()> {
        if authorization.action() != expected_action {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "parent removal authorization action does not match the requested transition",
            ));
        }
        if authorization.authority_generation == 0
            || authorization.authority_nonce.trim().is_empty()
        {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "current parent authority freshness binding is unavailable",
            ));
        }
        let Some(identity) = &self.identity else {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "child service identity is required before parent removal authorization",
            ));
        };
        if identity != authorization.identity() {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "parent removal authorization is bound to a different child identity",
            ));
        }
        Ok(())
    }
}
