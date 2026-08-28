use ocentra_eventing::error::EventingError;
use serde::de::{Deserializer, Error};
use serde::{Deserialize, Serialize};

pub trait ParentControllerEventContract {
    const EVENT_TYPE: &'static str;
    const SCHEMA_VERSION: u16 = crate::constants::parent_controller::EVENT_SCHEMA_VERSION;

    fn validate(&self) -> Result<(), EventingError>;
}

fn deserialize_parent_schema_version<'de, D>(deserializer: D) -> Result<u16, D::Error>
where
    D: Deserializer<'de>,
{
    let version = u16::deserialize(deserializer)?;
    (version == crate::constants::parent_controller::EVENT_SCHEMA_VERSION)
        .then_some(version)
        .ok_or_else(|| D::Error::custom("unsupported parent-controller event schema version"))
}

fn deserialize_parent_non_empty_text<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let value = String::deserialize(deserializer)?;
    (!value.trim().is_empty())
        .then_some(value)
        .ok_or_else(|| D::Error::custom("parent-controller event text must not be blank"))
}

fn deserialize_parent_optional_non_empty_text<'de, D>(
    deserializer: D,
) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<String>::deserialize(deserializer)?;
    if matches!(value.as_deref(), Some(value) if value.trim().is_empty()) {
        return Err(D::Error::custom(
            "parent-controller optional event text must not be blank",
        ));
    }
    Ok(value)
}

fn validate_parent_schema_version(version: u16) -> Result<(), EventingError> {
    (version == crate::constants::parent_controller::EVENT_SCHEMA_VERSION)
        .then_some(())
        .ok_or(EventingError::InvalidVersion)
}

fn validate_parent_text(value: &str, field: &'static str) -> Result<(), EventingError> {
    (!value.trim().is_empty())
        .then_some(())
        .ok_or(EventingError::EmptyValue { field })
}

fn validate_parent_optional_text(
    value: Option<&str>,
    field: &'static str,
) -> Result<(), EventingError> {
    if let Some(value) = value {
        validate_parent_text(value, field)?;
    }
    Ok(())
}

fn validate_parent_event(
    schema_version: u16,
    fields: &[(&str, &'static str)],
) -> Result<(), EventingError> {
    validate_parent_schema_version(schema_version)?;
    for (value, field) in fields.iter().copied() {
        validate_parent_text(value, field)?;
    }
    Ok(())
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct ParentActionReceivedEvent {
    #[serde(deserialize_with = "deserialize_parent_schema_version")]
    pub schema_version: u16,
    #[serde(deserialize_with = "deserialize_parent_non_empty_text")]
    pub parent_action_event_ref: String,
    #[serde(deserialize_with = "deserialize_parent_non_empty_text")]
    pub received_at: String,
    #[serde(deserialize_with = "deserialize_parent_non_empty_text")]
    pub parent_intent_ref: String,
    #[serde(deserialize_with = "deserialize_parent_non_empty_text")]
    pub parent_profile_ref: String,
    #[serde(deserialize_with = "deserialize_parent_non_empty_text")]
    pub device_ref: String,
    pub action_kind: ParentControllerActionKind,
    pub source: ParentControllerSource,
    #[serde(deserialize_with = "deserialize_parent_non_empty_text")]
    pub custody: String,
    #[serde(deserialize_with = "deserialize_parent_non_empty_text")]
    pub idempotency_key: String,
}

impl ParentControllerEventContract for ParentActionReceivedEvent {
    const EVENT_TYPE: &'static str =
        crate::constants::parent_controller::EVENT_PARENT_ACTION_RECEIVED;

    fn validate(&self) -> Result<(), EventingError> {
        validate_parent_event(
            self.schema_version,
            &[
                (&self.parent_action_event_ref, "parent_action_event_ref"),
                (&self.received_at, "received_at"),
                (&self.parent_intent_ref, "parent_intent_ref"),
                (&self.parent_profile_ref, "parent_profile_ref"),
                (&self.device_ref, "device_ref"),
                (&self.custody, "custody"),
                (&self.idempotency_key, "idempotency_key"),
            ],
        )
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct ParentCommandValidatedEvent {
    #[serde(deserialize_with = "deserialize_parent_schema_version")]
    pub schema_version: u16,
    #[serde(deserialize_with = "deserialize_parent_non_empty_text")]
    pub command_validated_event_ref: String,
    #[serde(deserialize_with = "deserialize_parent_non_empty_text")]
    pub parent_action_event_ref: String,
    #[serde(deserialize_with = "deserialize_parent_non_empty_text")]
    pub parent_command_ref: String,
    #[serde(deserialize_with = "deserialize_parent_optional_non_empty_text")]
    pub child_command_ref: Option<String>,
    #[serde(deserialize_with = "deserialize_parent_non_empty_text")]
    pub validated_at: String,
    pub validation_state: ParentCommandValidationState,
    #[serde(deserialize_with = "deserialize_parent_non_empty_text")]
    pub causation_event_ref: String,
    #[serde(deserialize_with = "deserialize_parent_non_empty_text")]
    pub idempotency_key: String,
}

impl ParentControllerEventContract for ParentCommandValidatedEvent {
    const EVENT_TYPE: &'static str = crate::constants::parent_controller::EVENT_COMMAND_VALIDATED;

    fn validate(&self) -> Result<(), EventingError> {
        validate_parent_event(
            self.schema_version,
            &[
                (
                    &self.command_validated_event_ref,
                    "command_validated_event_ref",
                ),
                (&self.parent_action_event_ref, "parent_action_event_ref"),
                (&self.parent_command_ref, "parent_command_ref"),
                (&self.validated_at, "validated_at"),
                (&self.causation_event_ref, "causation_event_ref"),
                (&self.idempotency_key, "idempotency_key"),
            ],
        )
        .and_then(|_| {
            validate_parent_optional_text(self.child_command_ref.as_deref(), "child_command_ref")
        })
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct ParentCommandRejectedEvent {
    #[serde(deserialize_with = "deserialize_parent_schema_version")]
    pub schema_version: u16,
    #[serde(deserialize_with = "deserialize_parent_non_empty_text")]
    pub command_rejected_event_ref: String,
    #[serde(deserialize_with = "deserialize_parent_non_empty_text")]
    pub parent_action_event_ref: String,
    #[serde(deserialize_with = "deserialize_parent_non_empty_text")]
    pub rejected_at: String,
    #[serde(deserialize_with = "deserialize_parent_non_empty_text")]
    pub rejection_reason_code: String,
    #[serde(deserialize_with = "deserialize_parent_non_empty_text")]
    pub causation_event_ref: String,
    #[serde(deserialize_with = "deserialize_parent_non_empty_text")]
    pub idempotency_key: String,
}

impl ParentControllerEventContract for ParentCommandRejectedEvent {
    const EVENT_TYPE: &'static str = crate::constants::parent_controller::EVENT_COMMAND_REJECTED;

    fn validate(&self) -> Result<(), EventingError> {
        validate_parent_event(
            self.schema_version,
            &[
                (
                    &self.command_rejected_event_ref,
                    "command_rejected_event_ref",
                ),
                (&self.parent_action_event_ref, "parent_action_event_ref"),
                (&self.rejected_at, "rejected_at"),
                (&self.rejection_reason_code, "rejection_reason_code"),
                (&self.causation_event_ref, "causation_event_ref"),
                (&self.idempotency_key, "idempotency_key"),
            ],
        )
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct ParentChildCommandForwardRequestedEvent {
    #[serde(deserialize_with = "deserialize_parent_schema_version")]
    pub schema_version: u16,
    #[serde(deserialize_with = "deserialize_parent_non_empty_text")]
    pub forward_requested_event_ref: String,
    #[serde(deserialize_with = "deserialize_parent_non_empty_text")]
    pub parent_command_ref: String,
    #[serde(deserialize_with = "deserialize_parent_non_empty_text")]
    pub child_command_ref: String,
    #[serde(deserialize_with = "deserialize_parent_non_empty_text")]
    pub device_ref: String,
    #[serde(deserialize_with = "deserialize_parent_non_empty_text")]
    pub requested_at: String,
    pub transport_boundary: ParentChildCommandTransportBoundary,
    #[serde(deserialize_with = "deserialize_parent_non_empty_text")]
    pub causation_event_ref: String,
    #[serde(deserialize_with = "deserialize_parent_non_empty_text")]
    pub idempotency_key: String,
}

impl ParentControllerEventContract for ParentChildCommandForwardRequestedEvent {
    const EVENT_TYPE: &'static str =
        crate::constants::parent_controller::EVENT_CHILD_COMMAND_FORWARD_REQUESTED;

    fn validate(&self) -> Result<(), EventingError> {
        validate_parent_event(
            self.schema_version,
            &[
                (
                    &self.forward_requested_event_ref,
                    "forward_requested_event_ref",
                ),
                (&self.parent_command_ref, "parent_command_ref"),
                (&self.child_command_ref, "child_command_ref"),
                (&self.device_ref, "device_ref"),
                (&self.requested_at, "requested_at"),
                (&self.causation_event_ref, "causation_event_ref"),
                (&self.idempotency_key, "idempotency_key"),
            ],
        )
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct ParentChildCommandForwardedEvent {
    #[serde(deserialize_with = "deserialize_parent_schema_version")]
    pub schema_version: u16,
    #[serde(deserialize_with = "deserialize_parent_non_empty_text")]
    pub forwarded_event_ref: String,
    #[serde(deserialize_with = "deserialize_parent_non_empty_text")]
    pub child_command_ref: String,
    #[serde(deserialize_with = "deserialize_parent_non_empty_text")]
    pub transport_message_ref: String,
    #[serde(deserialize_with = "deserialize_parent_non_empty_text")]
    pub forwarded_at: String,
    pub delivery_state: ParentChildCommandDeliveryState,
    #[serde(deserialize_with = "deserialize_parent_non_empty_text")]
    pub causation_event_ref: String,
}

impl ParentControllerEventContract for ParentChildCommandForwardedEvent {
    const EVENT_TYPE: &'static str =
        crate::constants::parent_controller::EVENT_CHILD_COMMAND_FORWARDED;

    fn validate(&self) -> Result<(), EventingError> {
        validate_parent_event(
            self.schema_version,
            &[
                (&self.forwarded_event_ref, "forwarded_event_ref"),
                (&self.child_command_ref, "child_command_ref"),
                (&self.transport_message_ref, "transport_message_ref"),
                (&self.forwarded_at, "forwarded_at"),
                (&self.causation_event_ref, "causation_event_ref"),
            ],
        )
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct ParentReadModelProjectedEvent {
    #[serde(deserialize_with = "deserialize_parent_schema_version")]
    pub schema_version: u16,
    #[serde(deserialize_with = "deserialize_parent_non_empty_text")]
    pub read_model_projected_event_ref: String,
    #[serde(deserialize_with = "deserialize_parent_non_empty_text")]
    pub read_model_ref: String,
    #[serde(deserialize_with = "deserialize_parent_non_empty_text")]
    pub previous_event_ref: String,
    #[serde(deserialize_with = "deserialize_parent_non_empty_text")]
    pub projected_at: String,
    pub projection_kind: ParentProjectionKind,
    pub visible_to_portal: bool,
}

impl ParentControllerEventContract for ParentReadModelProjectedEvent {
    const EVENT_TYPE: &'static str =
        crate::constants::parent_controller::EVENT_READ_MODEL_PROJECTED;

    fn validate(&self) -> Result<(), EventingError> {
        validate_parent_event(
            self.schema_version,
            &[
                (
                    &self.read_model_projected_event_ref,
                    "read_model_projected_event_ref",
                ),
                (&self.read_model_ref, "read_model_ref"),
                (&self.previous_event_ref, "previous_event_ref"),
                (&self.projected_at, "projected_at"),
            ],
        )
    }
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
