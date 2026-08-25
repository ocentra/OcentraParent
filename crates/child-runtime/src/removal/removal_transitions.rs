use std::io;

use super::{
    removal_record::{append_audit, status_from_record},
    ChildAgentCleanupState, ChildAgentRemovalAction, ChildAgentRemovalAuthorizationAction,
    ChildAgentRemovalBoundary, ChildAgentRemovalStatus, ChildAgentTrustState,
    VerifiedParentRemovalAuthorization,
};

impl ChildAgentRemovalBoundary {
    pub fn revoke_with_parent_authorization(
        &self,
        authorization: VerifiedParentRemovalAuthorization,
    ) -> io::Result<ChildAgentRemovalStatus> {
        self.with_locked_record(|record| {
            self.require_identity(&authorization, ChildAgentRemovalAuthorizationAction::Revoke)?;
            if record.trust_state == ChildAgentTrustState::Revoked {
                return Ok(status_from_record(record));
            }
            append_audit(
                record,
                ChildAgentRemovalAction::Revoked,
                authorization.reference().to_owned(),
                authorization.identity().clone(),
            )?;
            record.trust_state = ChildAgentTrustState::Revoked;
            record.cleanup_state = ChildAgentCleanupState::ManualPlatformRemovalRequired;
            Ok(status_from_record(record))
        })
    }

    pub fn reauthorize_with_parent_authorization(
        &self,
        authorization: VerifiedParentRemovalAuthorization,
    ) -> io::Result<ChildAgentRemovalStatus> {
        self.with_locked_record(|record| {
            self.require_identity(
                &authorization,
                ChildAgentRemovalAuthorizationAction::Reauthorize,
            )?;
            if !record.tamper_signals.is_empty() {
                return Err(io::Error::new(
                    io::ErrorKind::PermissionDenied,
                    "tamper evidence requires an explicit trusted resolution before reauthorization",
                ));
            }
            append_audit(
                record,
                ChildAgentRemovalAction::Reauthorized,
                authorization.reference().to_owned(),
                authorization.identity().clone(),
            )?;
            record.trust_state = ChildAgentTrustState::Active;
            record.cleanup_state = ChildAgentCleanupState::NotRequired;
            Ok(status_from_record(record))
        })
    }
}
