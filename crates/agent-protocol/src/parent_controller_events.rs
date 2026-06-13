use serde::{Deserialize, Serialize};

pub trait ParentControllerEventContract {
    const EVENT_TYPE: &'static str;
    const SCHEMA_VERSION: u16 = crate::constants::parent_controller::EVENT_SCHEMA_VERSION;
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentActionReceivedEvent {
    pub schema_version: u16,
    pub parent_action_event_ref: String,
    pub received_at: String,
    pub parent_intent_ref: String,
    pub parent_profile_ref: String,
    pub device_ref: String,
    pub action_kind: ParentControllerActionKind,
    pub source: ParentControllerSource,
    pub custody: String,
    pub idempotency_key: String,
}

impl ParentControllerEventContract for ParentActionReceivedEvent {
    const EVENT_TYPE: &'static str =
        crate::constants::parent_controller::EVENT_PARENT_ACTION_RECEIVED;
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentCommandValidatedEvent {
    pub schema_version: u16,
    pub command_validated_event_ref: String,
    pub parent_action_event_ref: String,
    pub parent_command_ref: String,
    pub child_command_ref: Option<String>,
    pub validated_at: String,
    pub validation_state: ParentCommandValidationState,
    pub causation_event_ref: String,
    pub idempotency_key: String,
}

impl ParentControllerEventContract for ParentCommandValidatedEvent {
    const EVENT_TYPE: &'static str = crate::constants::parent_controller::EVENT_COMMAND_VALIDATED;
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentCommandRejectedEvent {
    pub schema_version: u16,
    pub command_rejected_event_ref: String,
    pub parent_action_event_ref: String,
    pub rejected_at: String,
    pub rejection_reason_code: String,
    pub causation_event_ref: String,
    pub idempotency_key: String,
}

impl ParentControllerEventContract for ParentCommandRejectedEvent {
    const EVENT_TYPE: &'static str = crate::constants::parent_controller::EVENT_COMMAND_REJECTED;
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentChildCommandForwardRequestedEvent {
    pub schema_version: u16,
    pub forward_requested_event_ref: String,
    pub parent_command_ref: String,
    pub child_command_ref: String,
    pub device_ref: String,
    pub requested_at: String,
    pub transport_boundary: ParentChildCommandTransportBoundary,
    pub causation_event_ref: String,
    pub idempotency_key: String,
}

impl ParentControllerEventContract for ParentChildCommandForwardRequestedEvent {
    const EVENT_TYPE: &'static str =
        crate::constants::parent_controller::EVENT_CHILD_COMMAND_FORWARD_REQUESTED;
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentChildCommandForwardedEvent {
    pub schema_version: u16,
    pub forwarded_event_ref: String,
    pub child_command_ref: String,
    pub transport_message_ref: String,
    pub forwarded_at: String,
    pub delivery_state: ParentChildCommandDeliveryState,
    pub causation_event_ref: String,
}

impl ParentControllerEventContract for ParentChildCommandForwardedEvent {
    const EVENT_TYPE: &'static str =
        crate::constants::parent_controller::EVENT_CHILD_COMMAND_FORWARDED;
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentReadModelProjectedEvent {
    pub schema_version: u16,
    pub read_model_projected_event_ref: String,
    pub read_model_ref: String,
    pub previous_event_ref: String,
    pub projected_at: String,
    pub projection_kind: ParentProjectionKind,
    pub visible_to_portal: bool,
}

impl ParentControllerEventContract for ParentReadModelProjectedEvent {
    const EVENT_TYPE: &'static str =
        crate::constants::parent_controller::EVENT_READ_MODEL_PROJECTED;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ParentControllerActionKind {
    Allow,
    Block,
    Ask,
    Review,
    UpdateTrackingConfig,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ParentControllerSource {
    PortalTypedIntent,
    LocalServiceApi,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ParentCommandValidationState {
    Validated,
    Rejected,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ParentChildCommandTransportBoundary {
    TypedLocalServiceTransport,
    BrokerTransport,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ParentChildCommandDeliveryState {
    Forwarded,
    Queued,
    Unavailable,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ParentProjectionKind {
    ParentIntentStatus,
    ChildCommandStatus,
    CapabilityState,
    TrackingConfigState,
}
