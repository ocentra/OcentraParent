use ocentra_eventing::{EventCustody, RuntimeRole};
use ocentra_parent_agent_protocol::constants;
use serde::{Deserialize, Serialize};

const PARENT_CHILD_RUNTIME_PHASES: [ParentChildRuntimePhase; 9] = [
    ParentChildRuntimePhase::ParentActionReceived,
    ParentChildRuntimePhase::ParentCommandValidated,
    ParentChildRuntimePhase::ParentChildCommandForwardRequested,
    ParentChildRuntimePhase::ParentChildCommandForwarded,
    ParentChildRuntimePhase::ChildCommandReceived,
    ParentChildRuntimePhase::ChildCommandAccepted,
    ParentChildRuntimePhase::ChildCapabilityStateUpdated,
    ParentChildRuntimePhase::ChildRuntimeHealthUpdated,
    ParentChildRuntimePhase::ParentReadModelProjected,
];

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParentChildRuntimePhase {
    ParentActionReceived,
    ParentCommandValidated,
    ParentChildCommandForwardRequested,
    ParentChildCommandForwarded,
    ChildCommandReceived,
    ChildCommandAccepted,
    ChildCapabilityStateUpdated,
    ChildRuntimeHealthUpdated,
    ParentReadModelProjected,
}

impl ParentChildRuntimePhase {
    pub fn ordered_chain() -> &'static [Self] {
        &PARENT_CHILD_RUNTIME_PHASES
    }

    pub(crate) fn event_type(self) -> &'static str {
        match self {
            Self::ParentActionReceived => {
                constants::parent_controller::EVENT_PARENT_ACTION_RECEIVED
            }
            Self::ParentCommandValidated => constants::parent_controller::EVENT_COMMAND_VALIDATED,
            Self::ParentChildCommandForwardRequested => {
                constants::parent_controller::EVENT_CHILD_COMMAND_FORWARD_REQUESTED
            }
            Self::ParentChildCommandForwarded => {
                constants::parent_controller::EVENT_CHILD_COMMAND_FORWARDED
            }
            Self::ChildCommandReceived => constants::child_agent::EVENT_COMMAND_RECEIVED,
            Self::ChildCommandAccepted => constants::child_agent::EVENT_COMMAND_ACCEPTED,
            Self::ChildCapabilityStateUpdated => {
                constants::child_agent::EVENT_CAPABILITY_STATE_UPDATED
            }
            Self::ChildRuntimeHealthUpdated => constants::child_agent::EVENT_RUNTIME_HEALTH_UPDATED,
            Self::ParentReadModelProjected => {
                constants::parent_controller::EVENT_READ_MODEL_PROJECTED
            }
        }
    }

    pub(crate) fn schema_version(self) -> u16 {
        if self.is_child_agent_phase() {
            constants::child_agent::EVENT_SCHEMA_VERSION
        } else {
            constants::parent_controller::EVENT_SCHEMA_VERSION
        }
    }

    pub(crate) fn subscriber_id(self) -> &'static str {
        match self {
            Self::ParentActionReceived => {
                constants::parent_controller::SUBSCRIBER_PARENT_ACTION_VALIDATOR
            }
            Self::ParentCommandValidated => {
                constants::parent_controller::SUBSCRIBER_PARENT_COMMAND_VALIDATOR
            }
            Self::ParentChildCommandForwardRequested | Self::ParentChildCommandForwarded => {
                constants::parent_controller::SUBSCRIBER_PARENT_CHILD_TRANSPORT
            }
            Self::ChildCommandReceived => constants::child_agent::SUBSCRIBER_CHILD_COMMAND_RECEIVER,
            Self::ChildCommandAccepted => constants::child_agent::SUBSCRIBER_CHILD_COMMAND_DECIDER,
            Self::ChildCapabilityStateUpdated => {
                constants::child_agent::SUBSCRIBER_CHILD_CAPABILITY_PROJECTOR
            }
            Self::ChildRuntimeHealthUpdated => {
                constants::child_agent::SUBSCRIBER_CHILD_HEALTH_PROJECTOR
            }
            Self::ParentReadModelProjected => {
                constants::parent_controller::SUBSCRIBER_PARENT_READ_MODEL_PROJECTOR
            }
        }
    }

    pub(crate) fn target_handler(self) -> &'static str {
        match self {
            Self::ParentActionReceived => {
                constants::parent_controller::TARGET_PARENT_ACTION_VALIDATOR
            }
            Self::ParentCommandValidated => {
                constants::parent_controller::TARGET_PARENT_COMMAND_VALIDATOR
            }
            Self::ParentChildCommandForwardRequested | Self::ParentChildCommandForwarded => {
                constants::parent_controller::TARGET_PARENT_CHILD_TRANSPORT
            }
            Self::ChildCommandReceived => constants::child_agent::TARGET_CHILD_COMMAND_RECEIVER,
            Self::ChildCommandAccepted => constants::child_agent::TARGET_CHILD_COMMAND_DECIDER,
            Self::ChildCapabilityStateUpdated => {
                constants::child_agent::TARGET_CHILD_CAPABILITY_PROJECTOR
            }
            Self::ChildRuntimeHealthUpdated => {
                constants::child_agent::TARGET_CHILD_HEALTH_PROJECTOR
            }
            Self::ParentReadModelProjected => {
                constants::parent_controller::TARGET_PARENT_READ_MODEL_PROJECTOR
            }
        }
    }

    pub(crate) fn runtime_role(self) -> RuntimeRole {
        let value = if self.is_child_agent_phase() {
            constants::eventing_source::ROLE_AGENT
        } else if self == Self::ParentReadModelProjected {
            constants::eventing_source::ROLE_READ_MODEL
        } else {
            constants::eventing_source::ROLE_CONTROLLER
        };
        RuntimeRole::parse(value)
            .expect(constants::eventing_source::ERROR_RUNTIME_ROLE_CONSTANT_PARSES)
    }

    pub(crate) fn custody(self) -> EventCustody {
        let value = if self.is_child_agent_phase() {
            constants::eventing_source::CUSTODY_LOCAL_JOURNAL
        } else {
            constants::eventing_source::CUSTODY_COORDINATOR_CACHE
        };
        EventCustody::parse(value)
            .expect(constants::eventing_source::ERROR_EVENT_CUSTODY_CONSTANT_PARSES)
    }

    pub(crate) fn is_child_agent_phase(self) -> bool {
        matches!(
            self,
            Self::ChildCommandReceived
                | Self::ChildCommandAccepted
                | Self::ChildCapabilityStateUpdated
                | Self::ChildRuntimeHealthUpdated
        )
    }
}
