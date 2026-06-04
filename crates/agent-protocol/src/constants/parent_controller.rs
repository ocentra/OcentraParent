pub const NAMESPACE: &str = "parent_controller";
pub const EVENT_SCHEMA_VERSION: u16 = 1;

pub const EVENT_PARENT_ACTION_RECEIVED: &str = "parent_controller.parent_action.received";
pub const EVENT_COMMAND_VALIDATED: &str = "parent_controller.command.validated";
pub const EVENT_COMMAND_REJECTED: &str = "parent_controller.command.rejected";
pub const EVENT_CHILD_COMMAND_FORWARD_REQUESTED: &str =
    "parent_controller.child_command.forward_requested";
pub const EVENT_CHILD_COMMAND_FORWARDED: &str = "parent_controller.child_command.forwarded";
pub const EVENT_READ_MODEL_PROJECTED: &str = "parent_controller.read_model.projected";

pub const TEST_PARENT_ACTION_EVENT_REF: &str = "event.parent-controller.parent-action.received.1";
pub const TEST_COMMAND_VALIDATED_EVENT_REF: &str = "event.parent-controller.command.validated.1";
pub const TEST_COMMAND_REJECTED_EVENT_REF: &str = "event.parent-controller.command.rejected.1";
pub const TEST_FORWARD_REQUESTED_EVENT_REF: &str =
    "event.parent-controller.child-command.forward-requested.1";
pub const TEST_FORWARDED_EVENT_REF: &str = "event.parent-controller.child-command.forwarded.1";
pub const TEST_READ_MODEL_PROJECTED_EVENT_REF: &str =
    "event.parent-controller.read-model.projected.1";
pub const TEST_PARENT_INTENT_REF: &str = "intent.parent.portal.1";
pub const TEST_PARENT_COMMAND_REF: &str = "command.parent-controller.1";
pub const TEST_CHILD_COMMAND_REF: &str = "command.child-agent.1";
pub const TEST_PARENT_PROFILE_REF: &str = "profile.child.1";
pub const TEST_DEVICE_REF: &str = "device.child.windows-1";
pub const TEST_IDEMPOTENCY_KEY: &str = "idempotency.parent-command.1";
pub const TEST_TRANSPORT_MESSAGE_REF: &str = "transport.parent-child.1";
pub const TEST_READ_MODEL_REF: &str = "read-model.parent-controller.1";
pub const TEST_VALIDATION_REJECTION_CODE: &str = "invalid-parent-intent";
pub const CUSTODY_LOCAL_SERVICE_VALIDATION: &str = "local-service-validation";
pub const TRANSPORT_BOUNDARY_LOCAL_SERVICE: &str = "typed-local-service-transport";
