pub const NAMESPACE: &str = "child_agent";
pub const EVENT_SCHEMA_VERSION: u16 = 1;

pub const EVENT_COMMAND_RECEIVED: &str = "child_agent.command.received";
pub const EVENT_COMMAND_ACCEPTED: &str = "child_agent.command.accepted";
pub const EVENT_COMMAND_REJECTED: &str = "child_agent.command.rejected";
pub const EVENT_CAPABILITY_STATE_UPDATED: &str = "child_agent.capability_state.updated";
pub const EVENT_RUNTIME_HEALTH_UPDATED: &str = "child_agent.runtime_health.updated";

pub const EVENT_TYPES: [&str; 5] = [
    EVENT_COMMAND_RECEIVED,
    EVENT_COMMAND_ACCEPTED,
    EVENT_COMMAND_REJECTED,
    EVENT_CAPABILITY_STATE_UPDATED,
    EVENT_RUNTIME_HEALTH_UPDATED,
];

pub const RUNTIME_COMPONENT_CHILD_AGENT: &str = "child-agent-runtime";
pub const RUNTIME_INSTANCE_LOCAL_CHILD_AGENT: &str = "local-child-agent";
pub const SUBSCRIBER_CHILD_COMMAND_RECEIVER: &str = "subscriber.child-agent.command-receiver";
pub const SUBSCRIBER_CHILD_COMMAND_DECIDER: &str = "subscriber.child-agent.command-decider";
pub const SUBSCRIBER_CHILD_CAPABILITY_PROJECTOR: &str =
    "subscriber.child-agent.capability-projector";
pub const SUBSCRIBER_CHILD_HEALTH_PROJECTOR: &str = "subscriber.child-agent.health-projector";
pub const TARGET_CHILD_COMMAND_RECEIVER: &str = "target.child-agent.command-receiver";
pub const TARGET_CHILD_COMMAND_DECIDER: &str = "target.child-agent.command-decider";
pub const TARGET_CHILD_CAPABILITY_PROJECTOR: &str = "target.child-agent.capability-projector";
pub const TARGET_CHILD_HEALTH_PROJECTOR: &str = "target.child-agent.health-projector";

pub const TEST_COMMAND_RECEIVED_EVENT_REF: &str = "event.child-agent.command.received.1";
pub const TEST_COMMAND_ACCEPTED_EVENT_REF: &str = "event.child-agent.command.accepted.1";
pub const TEST_COMMAND_REJECTED_EVENT_REF: &str = "event.child-agent.command.rejected.1";
pub const TEST_CAPABILITY_STATE_EVENT_REF: &str = "event.child-agent.capability-state.updated.1";
pub const TEST_RUNTIME_HEALTH_EVENT_REF: &str = "event.child-agent.runtime-health.updated.1";
pub const TEST_CHILD_COMMAND_REF: &str = "command.child-agent.1";
pub const TEST_PARENT_CONTROLLER_EVENT_REF: &str =
    "event.parent-controller.child-command.forwarded.1";
pub const TEST_TRANSPORT_MESSAGE_REF: &str = "transport.parent-child.1";
pub const TEST_DEVICE_REF: &str = "device.child.windows-1";
pub const TEST_CAPABILITY_REF: &str = "capability.child-agent.network-observer";
pub const TEST_IDEMPOTENCY_KEY: &str = "idempotency.child-command.1";
pub const TEST_REJECTION_CODE: &str = "unsupported-child-command";
pub const CUSTODY_CHILD_AGENT_RUNTIME: &str = "child-agent-runtime";
