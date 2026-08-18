pub const NAMESPACE: &str = "parent_controller";
pub const EVENT_SCHEMA_VERSION: u16 = 1;

pub const EVENT_PARENT_ACTION_RECEIVED: &str = "parent_controller.parent_action.received";
pub const EVENT_COMMAND_VALIDATED: &str = "parent_controller.command.validated";
pub const EVENT_COMMAND_REJECTED: &str = "parent_controller.command.rejected";
pub const EVENT_CHILD_COMMAND_FORWARD_REQUESTED: &str =
    "parent_controller.child_command.forward_requested";
pub const EVENT_CHILD_COMMAND_FORWARDED: &str = "parent_controller.child_command.forwarded";
pub const EVENT_READ_MODEL_PROJECTED: &str = "parent_controller.read_model.projected";

pub const CORRELATION_PARENT_CHILD_RUNTIME_PREFIX: &str = "correlation.parent-child-runtime.";
pub const AGGREGATE_PARENT_CHILD_RUNTIME_PREFIX: &str = "aggregate.parent-child-runtime.";
pub const IDEMPOTENCY_PARENT_CHILD_RUNTIME_PREFIX: &str = "idempotency.parent-child-runtime.";
pub const RUNTIME_COMPONENT_PARENT_CHILD_SPINE: &str = "parent-child-runtime-spine";
pub const RUNTIME_INSTANCE_LOCAL_PARENT_CONTROLLER: &str = "local-parent-controller";
pub const EVENTING_JOURNAL_EXTENSION: &str = "parent-runtime-intent.eventing.ndjson";
pub const INGRESS_REJECTION_INVALID_REQUEST: &str = "invalid-parent-runtime-intent-request";
pub const INGRESS_NO_CLAIM_AUTHENTICATED_SESSION_UNAVAILABLE: &str =
    "parent-runtime-intent-authenticated-session-unavailable-manual-required";
pub const INGRESS_NO_CLAIM_INVALID_SOURCE: &str =
    "parent-runtime-intent-source-is-not-trusted-local-portal";
pub const INGRESS_NO_CLAIM_JOURNAL_UNAVAILABLE: &str = "parent-runtime-intent-journal-unavailable";
pub const INGRESS_NO_CLAIM_EVENTING_UNAVAILABLE: &str =
    "parent-runtime-intent-eventing-unavailable";
pub const INGRESS_NO_CLAIM_DISPATCH_BLOCKED: &str =
    "parent-runtime-intent-dispatch-blocked-manual-required";
pub const REF_PARENT_COMMAND_SUFFIX: &str = "parent-command";
pub const REF_CHILD_COMMAND_SUFFIX: &str = "child-command";
pub const REF_TRANSPORT_MESSAGE_SUFFIX: &str = "transport-message";
pub const REF_PARENT_READ_MODEL_SUFFIX: &str = "parent-read-model";
pub const SUBSCRIBER_PARENT_ACTION_VALIDATOR: &str =
    "subscriber.parent-controller.action-validator";
pub const SUBSCRIBER_PARENT_COMMAND_VALIDATOR: &str =
    "subscriber.parent-controller.command-validator";
pub const SUBSCRIBER_PARENT_CHILD_TRANSPORT: &str = "subscriber.parent-controller.child-transport";
pub const SUBSCRIBER_PARENT_READ_MODEL_PROJECTOR: &str =
    "subscriber.parent-controller.read-model-projector";
pub const TARGET_PARENT_ACTION_VALIDATOR: &str = "target.parent-controller.action-validator";
pub const TARGET_PARENT_COMMAND_VALIDATOR: &str = "target.parent-controller.command-validator";
pub const TARGET_PARENT_CHILD_TRANSPORT: &str = "target.parent-controller.child-transport";
pub const TARGET_PARENT_READ_MODEL_PROJECTOR: &str =
    "target.parent-controller.read-model-projector";

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
pub const ERROR_PARENT_CHILD_RUNTIME_PUBLISHES: &str = "parent child runtime publishes";
pub const ERROR_PARENT_CHILD_RUNTIME_PAYLOAD_DECODES: &str = "parent child runtime payload decodes";
