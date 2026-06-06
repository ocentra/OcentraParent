use ocentra_parent_agent_protocol::constants;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScreenMeshPayloadMode {
    RedactedSummary,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScreenMeshClaimState {
    NotRequested,
    Requested,
    Granted,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScreenMeshLeaseState {
    NotCreated,
    Active,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScreenMeshProviderResultState {
    NotReturned,
    Returned,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScreenMeshChildValidationState {
    NotReady,
    Requested,
    Accepted,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScreenMeshPolicyState {
    NotReady,
    Ready,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScreenMeshResultRejectionReason {
    DuplicateResult,
    ExpiredLease,
    WrongProvider,
    WrongClaim,
    EvidenceMismatch,
    CustodyMismatch,
    RawImageTransfer,
    ProviderAuthorityViolation,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScreenMeshCustodyBoundary {
    pub raw_screenshot_transferred: bool,
    pub raw_screenshot_retained_by_provider: bool,
    pub provider_can_publish_policy: bool,
    pub provider_can_publish_enforcement: bool,
    pub child_agent_validates_before_policy: bool,
}

impl ScreenMeshCustodyBoundary {
    pub(crate) fn child_owned_worker_only() -> Self {
        Self {
            raw_screenshot_transferred: false,
            raw_screenshot_retained_by_provider: false,
            provider_can_publish_policy: false,
            provider_can_publish_enforcement: false,
            child_agent_validates_before_policy: true,
        }
    }
}

pub(crate) fn custody_label() -> &'static str {
    constants::value::LAN_PROVIDER_CUSTODY_LOCAL_NETWORK_AI_PROVIDER
}
