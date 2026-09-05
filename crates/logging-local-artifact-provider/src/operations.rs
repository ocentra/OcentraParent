//! Validated provider operations over the native mutation session.

use ocentra_parent_logging_core::local_artifact_mutation::{
    LocalArtifactIdentity, LocalArtifactMutationError, LocalArtifactMutationSession,
};

use crate::protocol::text::ErrorText;
use crate::protocol::types::ResponseResult;
use crate::protocol::{ProviderIdentifier, ValidatedRequest, WireIdentity};

mod dispatch;
mod errors;
mod lease;
mod mutations;
mod values;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum FailureDisposition {
    Continue,
    Terminate,
}

#[derive(Debug)]
pub(crate) struct ProviderError {
    text: ErrorText,
    disposition: FailureDisposition,
}

impl ProviderError {
    pub(crate) fn new(text: ErrorText, disposition: FailureDisposition) -> Self {
        Self { text, disposition }
    }

    pub(crate) const fn text(&self) -> ErrorText {
        self.text
    }

    pub(crate) const fn disposition(&self) -> FailureDisposition {
        self.disposition
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ExecutionDisposition {
    Continue,
    Shutdown,
}

#[derive(Debug)]
pub(crate) struct OperationExecution {
    result: ResponseResult,
    disposition: ExecutionDisposition,
}

impl OperationExecution {
    pub(crate) fn into_result(self) -> ResponseResult {
        self.result
    }

    pub(crate) const fn disposition(&self) -> ExecutionDisposition {
        self.disposition
    }
}

#[derive(Debug, Default)]
pub(crate) struct LeaseState {
    current: Option<ProviderIdentifier>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum LeaseRequirement {
    Optional,
    Required,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum PayloadKind {
    Append,
    Replace,
}

/// Length of a payload that has already passed the operation decoder.
///
/// BRAND-INVARIANT: the value is measured from the exact bytes sent to the
/// native owner and is used only for receipt verification and response shape.
#[derive(Clone, Copy)]
pub(super) struct PayloadLength(usize);

/// Count of mutations submitted to one native transaction.
///
/// BRAND-INVARIANT: the value is the length of the validated mutation vector.
#[derive(Clone, Copy)]
pub(super) struct MutationCount(usize);

impl PayloadKind {
    pub(crate) const fn maximum_bytes(self) -> usize {
        match self {
            Self::Append => crate::protocol::MAXIMUM_APPEND_BYTES,
            Self::Replace => crate::protocol::MAXIMUM_REPLACE_BYTES,
        }
    }

    pub(crate) const fn allows_empty(self) -> bool {
        matches!(self, Self::Replace)
    }
}

pub(crate) fn map_owner_error(error: &LocalArtifactMutationError) -> ProviderError {
    errors::map_owner_error(error)
}

pub(crate) fn random_identifier() -> Result<ProviderIdentifier, ProviderError> {
    errors::random_identifier()
}

pub(crate) fn wire_identity(identity: LocalArtifactIdentity) -> WireIdentity {
    values::wire_identity(identity)
}

pub(crate) fn execute(
    session: &mut LocalArtifactMutationSession<'_>,
    lease: &mut LeaseState,
    request: &ValidatedRequest,
) -> Result<OperationExecution, ProviderError> {
    dispatch::execute(session, lease, request)
}

fn success(result: ResponseResult) -> OperationExecution {
    OperationExecution {
        result,
        disposition: ExecutionDisposition::Continue,
    }
}
