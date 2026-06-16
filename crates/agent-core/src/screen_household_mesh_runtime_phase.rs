use ocentra_eventing::ids::RuntimeRole;
use ocentra_parent_agent_protocol::constants;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScreenHouseholdMeshPhase {
    WorkQueued,
    OfferPublished,
    ClaimRequested,
    ClaimGranted,
    LeaseCreated,
    ProviderResultReturned,
    ChildResultAccepted,
    PolicyRequested,
}

impl ScreenHouseholdMeshPhase {
    pub(crate) fn ordered_chain() -> &'static [Self] {
        &[
            Self::WorkQueued,
            Self::OfferPublished,
            Self::ClaimRequested,
            Self::ClaimGranted,
            Self::LeaseCreated,
            Self::ProviderResultReturned,
            Self::ChildResultAccepted,
            Self::PolicyRequested,
        ]
    }

    pub(crate) fn event_type(self) -> &'static str {
        match self {
            Self::WorkQueued => constants::screen_flow::EVENT_SCREEN_MESH_WORK_QUEUED,
            Self::OfferPublished => constants::screen_flow::EVENT_SCREEN_MESH_OFFER_PUBLISHED,
            Self::ClaimRequested => constants::screen_flow::EVENT_SCREEN_MESH_CLAIM_REQUESTED,
            Self::ClaimGranted => constants::screen_flow::EVENT_SCREEN_MESH_CLAIM_GRANTED,
            Self::LeaseCreated => constants::screen_flow::EVENT_SCREEN_MESH_LEASE_CREATED,
            Self::ProviderResultReturned => {
                constants::screen_flow::EVENT_SCREEN_MESH_PROVIDER_RESULT_RETURNED
            }
            Self::ChildResultAccepted => {
                constants::screen_flow::EVENT_SCREEN_MESH_CHILD_RESULT_ACCEPTED
            }
            Self::PolicyRequested => constants::screen_flow::EVENT_SCREEN_MESH_POLICY_REQUESTED,
        }
    }

    pub(crate) fn subscriber_id(self) -> &'static str {
        match self {
            Self::WorkQueued => constants::screen_flow::SUBSCRIBER_SCREEN_MESH_WORK_QUEUE,
            Self::OfferPublished => constants::screen_flow::SUBSCRIBER_SCREEN_MESH_OFFER,
            Self::ClaimRequested => constants::screen_flow::SUBSCRIBER_SCREEN_MESH_CLAIM_REQUEST,
            Self::ClaimGranted => constants::screen_flow::SUBSCRIBER_SCREEN_MESH_CLAIM_GRANT,
            Self::LeaseCreated => constants::screen_flow::SUBSCRIBER_SCREEN_MESH_LEASE,
            Self::ProviderResultReturned => {
                constants::screen_flow::SUBSCRIBER_SCREEN_MESH_PROVIDER_RESULT
            }
            Self::ChildResultAccepted => {
                constants::screen_flow::SUBSCRIBER_SCREEN_MESH_CHILD_VALIDATION
            }
            Self::PolicyRequested => constants::screen_flow::SUBSCRIBER_SCREEN_MESH_POLICY_REQUEST,
        }
    }

    pub(crate) fn target_handler(self) -> &'static str {
        match self {
            Self::WorkQueued | Self::ClaimGranted | Self::LeaseCreated => {
                constants::screen_flow::TARGET_SCREEN_MESH_CHILD_LEDGER
            }
            Self::OfferPublished | Self::ClaimRequested => {
                constants::screen_flow::TARGET_SCREEN_MESH_BRIDGE
            }
            Self::ProviderResultReturned => {
                constants::screen_flow::TARGET_SCREEN_MESH_PROVIDER_WORKER
            }
            Self::ChildResultAccepted => constants::screen_flow::TARGET_SCREEN_MESH_CHILD_VALIDATOR,
            Self::PolicyRequested => constants::screen_flow::TARGET_SCREEN_POLICY_ENGINE,
        }
    }

    pub(crate) fn runtime_role(self) -> RuntimeRole {
        let value = match self {
            Self::OfferPublished | Self::ClaimRequested => constants::eventing_source::ROLE_AGENT,
            Self::ProviderResultReturned => constants::eventing_source::ROLE_ANALYZER,
            Self::PolicyRequested => constants::eventing_source::ROLE_DECISION_ENGINE,
            _ => constants::eventing_source::ROLE_AUDIT_WRITER,
        };
        match RuntimeRole::parse(value) {
            Ok(role) => role,
            Err(_) => std::process::abort(),
        }
    }
}
