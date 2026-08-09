use std::fmt::{Display, Formatter};

use ocentra_eventing::error::EventingError;
use ocentra_parent_agent_protocol::activity::policy_preview::PolicyRequestStatus;
use ocentra_policy_control_core::policy_request::{ChildPolicyRequest, PolicyTemporaryOverride};

#[derive(Clone, Debug)]
pub(crate) struct CommandId(pub(crate) String);

#[derive(Clone, Debug)]
pub(crate) struct AuditEventId(pub(crate) String);

#[derive(Clone, Debug)]
pub(crate) struct RequestIdText(pub(crate) String);

#[derive(Clone, Debug)]
pub(crate) struct RejectionReason(pub(crate) String);

#[derive(Clone, Debug)]
pub(crate) struct ErrorMessage(pub(crate) String);

#[derive(Clone, Debug)]
pub(crate) struct FieldName(pub(crate) &'static str);

#[derive(Clone, Debug)]
pub(crate) struct FieldText(pub(crate) String);

#[derive(Clone, Debug)]
pub(crate) struct PreviousResolution {
    pub(crate) request: ChildPolicyRequest,
    pub(crate) temporary_override: Option<PolicyTemporaryOverride>,
}

#[derive(Clone, Debug)]
pub(crate) struct ResolutionSnapshot {
    pub(crate) confirmed_request: ChildPolicyRequest,
    pub(crate) previous_resolution: Option<PreviousResolution>,
}

#[derive(Debug)]
pub(crate) struct ResolutionError(pub(crate) String);

#[derive(Debug)]
pub(crate) struct SnapshotError {
    pub(crate) reason: RejectionReason,
    pub(crate) lookup_claimed: bool,
    pub(crate) request_id: Option<RequestIdText>,
    pub(crate) status: PolicyRequestStatus,
}

impl SnapshotError {
    pub(crate) fn new(
        reason: RejectionReason,
        lookup_claimed: bool,
        request_id: Option<RequestIdText>,
        status: PolicyRequestStatus,
    ) -> Self {
        Self {
            reason,
            lookup_claimed,
            request_id,
            status,
        }
    }
}

impl ResolutionError {
    pub(crate) fn from_message(message: ErrorMessage) -> Self {
        Self(message.0)
    }

    pub(crate) fn into_reason(self) -> RejectionReason {
        RejectionReason(self.0)
    }
}

impl From<EventingError> for ResolutionError {
    fn from(error: EventingError) -> Self {
        Self(error.to_string())
    }
}

impl Display for ResolutionError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(formatter)
    }
}
