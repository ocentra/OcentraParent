#[path = "lease_authorization.rs"]
mod authorization;

use ocentra_parent_logging_core::local_artifact_mutation::LocalArtifactMutationSession;
use serde_json::json;

use super::{
    success, FailureDisposition, LeaseRequirement, LeaseState, OperationExecution, ProviderError,
};
use crate::protocol::{self, ProviderIdentifier, ValidatedOperation, ValidatedRequest};

impl LeaseState {
    pub(super) fn begin(&mut self) -> Result<ProviderIdentifier, ProviderError> {
        if self.current.is_some() {
            return Err(ProviderError::new(
                protocol::text::LEASE_ALREADY_ACTIVE,
                FailureDisposition::Continue,
            ));
        }
        let lease_id = super::random_identifier()?;
        // CLONE-JUSTIFICATION: the lease is retained by the session while a
        // second owned value is encoded into the begin response.
        self.current = Some(lease_id.clone());
        Ok(lease_id)
    }

    pub(super) fn end(&mut self, lease_id: &ProviderIdentifier) {
        if self.current.as_ref() == Some(lease_id) {
            self.current = None;
        }
    }
}

pub(super) fn execute(
    session: &LocalArtifactMutationSession<'_>,
    lease: &mut LeaseState,
    request: &ValidatedRequest,
    operation: &ValidatedOperation,
) -> Result<OperationExecution, ProviderError> {
    match operation {
        ValidatedOperation::BeginLease => begin_lease(session, lease, request),
        ValidatedOperation::EndLease { lease_id } => end_lease(session, lease, request, lease_id),
        ValidatedOperation::Shutdown => shutdown(session, lease, request),
        _ => Err(ProviderError::new(
            protocol::text::UNSUPPORTED_OPERATION,
            FailureDisposition::Continue,
        )),
    }
}

fn begin_lease(
    session: &LocalArtifactMutationSession<'_>,
    lease: &mut LeaseState,
    request: &ValidatedRequest,
) -> Result<OperationExecution, ProviderError> {
    if request.lease_id().is_some() {
        return Err(ProviderError::new(
            protocol::text::LEASE_BEGIN_ARGUMENT,
            FailureDisposition::Continue,
        ));
    }
    verify_session_current(session)?;
    let lease_id = lease.begin()?;
    let lease_text = lease_id.text();
    Ok(success(protocol::text::object(vec![(
        protocol::text::TextId::LeaseIdKey,
        json!(lease_text),
    )])))
}

fn end_lease(
    session: &LocalArtifactMutationSession<'_>,
    lease: &mut LeaseState,
    request: &ValidatedRequest,
    requested: &ProviderIdentifier,
) -> Result<OperationExecution, ProviderError> {
    authorize_lease(lease, request.lease_id(), LeaseRequirement::Required)?;
    if lease.current.as_ref() != Some(requested) {
        return Err(ProviderError::new(
            protocol::text::REQUESTED_LEASE_NOT_CURRENT,
            FailureDisposition::Continue,
        ));
    }
    verify_session_current(session)?;
    lease.end(requested);
    Ok(success(protocol::text::object(vec![(
        protocol::text::TextId::ReleasedKey,
        json!(true),
    )])))
}

fn shutdown(
    session: &LocalArtifactMutationSession<'_>,
    lease: &LeaseState,
    request: &ValidatedRequest,
) -> Result<OperationExecution, ProviderError> {
    if lease.current.is_some() || request.lease_id().is_some() {
        return Err(ProviderError::new(
            protocol::text::SHUTDOWN_LEASE_ACTIVE,
            FailureDisposition::Continue,
        ));
    }
    verify_session_current(session)?;
    Ok(OperationExecution {
        result: protocol::text::object(vec![(protocol::text::TextId::ShutdownKey, json!(true))]),
        disposition: super::ExecutionDisposition::Shutdown,
    })
}

pub(super) fn authorize_lease(
    lease: &LeaseState,
    supplied: Option<&ProviderIdentifier>,
    requirement: LeaseRequirement,
) -> Result<(), ProviderError> {
    authorization::authorize_lease(lease, supplied, requirement)
}

pub(super) fn verify_session_current(
    session: &LocalArtifactMutationSession<'_>,
) -> Result<(), ProviderError> {
    authorization::verify_session_current(session)
}
