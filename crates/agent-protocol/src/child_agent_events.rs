use ocentra_eventing::error::EventingError;
use serde::de::{Deserializer, Error};
use serde::{Deserialize, Serialize};

pub trait ChildAgentEventContract {
    const EVENT_TYPE: &'static str;
    const SCHEMA_VERSION: u16 = crate::constants::child_agent::EVENT_SCHEMA_VERSION;

    fn validate(&self) -> Result<(), EventingError>;
}

fn deserialize_child_schema_version<'de, D>(deserializer: D) -> Result<u16, D::Error>
where
    D: Deserializer<'de>,
{
    let version = u16::deserialize(deserializer)?;
    (version == crate::constants::child_agent::EVENT_SCHEMA_VERSION)
        .then_some(version)
        .ok_or_else(|| D::Error::custom("unsupported child-agent event schema version"))
}

fn deserialize_child_non_empty_text<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let value = String::deserialize(deserializer)?;
    (!value.trim().is_empty())
        .then_some(value)
        .ok_or_else(|| D::Error::custom("child-agent event text must not be blank"))
}

fn validate_child_schema_version(version: u16) -> Result<(), EventingError> {
    (version == crate::constants::child_agent::EVENT_SCHEMA_VERSION)
        .then_some(())
        .ok_or(EventingError::InvalidVersion)
}

fn validate_child_text(value: &str, field: &'static str) -> Result<(), EventingError> {
    (!value.trim().is_empty())
        .then_some(())
        .ok_or(EventingError::EmptyValue { field })
}

fn validate_child_event(
    schema_version: u16,
    fields: &[(&str, &'static str)],
) -> Result<(), EventingError> {
    validate_child_schema_version(schema_version)?;
    for (value, field) in fields.iter().copied() {
        validate_child_text(value, field)?;
    }
    Ok(())
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct ChildCommandReceivedEvent {
    #[serde(deserialize_with = "deserialize_child_schema_version")]
    pub schema_version: u16,
    #[serde(deserialize_with = "deserialize_child_non_empty_text")]
    pub command_received_event_ref: String,
    #[serde(deserialize_with = "deserialize_child_non_empty_text")]
    pub child_command_ref: String,
    #[serde(deserialize_with = "deserialize_child_non_empty_text")]
    pub received_at: String,
    #[serde(deserialize_with = "deserialize_child_non_empty_text")]
    pub device_ref: String,
    #[serde(deserialize_with = "deserialize_child_non_empty_text")]
    pub parent_controller_event_ref: String,
    #[serde(deserialize_with = "deserialize_child_non_empty_text")]
    pub transport_message_ref: String,
    pub command_kind: ChildCommandKind,
}

impl ChildAgentEventContract for ChildCommandReceivedEvent {
    const EVENT_TYPE: &'static str = crate::constants::child_agent::EVENT_COMMAND_RECEIVED;

    fn validate(&self) -> Result<(), EventingError> {
        validate_child_event(
            self.schema_version,
            &[
                (
                    &self.command_received_event_ref,
                    "command_received_event_ref",
                ),
                (&self.child_command_ref, "child_command_ref"),
                (&self.received_at, "received_at"),
                (&self.device_ref, "device_ref"),
                (
                    &self.parent_controller_event_ref,
                    "parent_controller_event_ref",
                ),
                (&self.transport_message_ref, "transport_message_ref"),
            ],
        )
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct ChildCommandAcceptedEvent {
    #[serde(deserialize_with = "deserialize_child_schema_version")]
    pub schema_version: u16,
    #[serde(deserialize_with = "deserialize_child_non_empty_text")]
    pub command_accepted_event_ref: String,
    #[serde(deserialize_with = "deserialize_child_non_empty_text")]
    pub child_command_ref: String,
    #[serde(deserialize_with = "deserialize_child_non_empty_text")]
    pub accepted_at: String,
    pub decision: ChildCommandDecision,
    #[serde(deserialize_with = "deserialize_child_non_empty_text")]
    pub causation_event_ref: String,
    #[serde(deserialize_with = "deserialize_child_non_empty_text")]
    pub idempotency_key: String,
}

impl ChildAgentEventContract for ChildCommandAcceptedEvent {
    const EVENT_TYPE: &'static str = crate::constants::child_agent::EVENT_COMMAND_ACCEPTED;

    fn validate(&self) -> Result<(), EventingError> {
        validate_child_event(
            self.schema_version,
            &[
                (
                    &self.command_accepted_event_ref,
                    "command_accepted_event_ref",
                ),
                (&self.child_command_ref, "child_command_ref"),
                (&self.accepted_at, "accepted_at"),
                (&self.causation_event_ref, "causation_event_ref"),
                (&self.idempotency_key, "idempotency_key"),
            ],
        )
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct ChildCommandRejectedEvent {
    #[serde(deserialize_with = "deserialize_child_schema_version")]
    pub schema_version: u16,
    #[serde(deserialize_with = "deserialize_child_non_empty_text")]
    pub command_rejected_event_ref: String,
    #[serde(deserialize_with = "deserialize_child_non_empty_text")]
    pub child_command_ref: String,
    #[serde(deserialize_with = "deserialize_child_non_empty_text")]
    pub rejected_at: String,
    #[serde(deserialize_with = "deserialize_child_non_empty_text")]
    pub rejection_reason_code: String,
    #[serde(deserialize_with = "deserialize_child_non_empty_text")]
    pub causation_event_ref: String,
    #[serde(deserialize_with = "deserialize_child_non_empty_text")]
    pub idempotency_key: String,
}

impl ChildAgentEventContract for ChildCommandRejectedEvent {
    const EVENT_TYPE: &'static str = crate::constants::child_agent::EVENT_COMMAND_REJECTED;

    fn validate(&self) -> Result<(), EventingError> {
        validate_child_event(
            self.schema_version,
            &[
                (
                    &self.command_rejected_event_ref,
                    "command_rejected_event_ref",
                ),
                (&self.child_command_ref, "child_command_ref"),
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
pub struct ChildCapabilityStateUpdatedEvent {
    #[serde(deserialize_with = "deserialize_child_schema_version")]
    pub schema_version: u16,
    #[serde(deserialize_with = "deserialize_child_non_empty_text")]
    pub capability_state_event_ref: String,
    #[serde(deserialize_with = "deserialize_child_non_empty_text")]
    pub device_ref: String,
    #[serde(deserialize_with = "deserialize_child_non_empty_text")]
    pub updated_at: String,
    #[serde(deserialize_with = "deserialize_child_non_empty_text")]
    pub capability_ref: String,
    pub capability_state: ChildCapabilityState,
    #[serde(deserialize_with = "deserialize_child_non_empty_text")]
    pub previous_event_ref: String,
    #[serde(deserialize_with = "deserialize_child_non_empty_text")]
    pub custody: String,
}

impl ChildAgentEventContract for ChildCapabilityStateUpdatedEvent {
    const EVENT_TYPE: &'static str = crate::constants::child_agent::EVENT_CAPABILITY_STATE_UPDATED;

    fn validate(&self) -> Result<(), EventingError> {
        validate_child_event(
            self.schema_version,
            &[
                (
                    &self.capability_state_event_ref,
                    "capability_state_event_ref",
                ),
                (&self.device_ref, "device_ref"),
                (&self.updated_at, "updated_at"),
                (&self.capability_ref, "capability_ref"),
                (&self.previous_event_ref, "previous_event_ref"),
                (&self.custody, "custody"),
            ],
        )
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct ChildRuntimeHealthUpdatedEvent {
    #[serde(deserialize_with = "deserialize_child_schema_version")]
    pub schema_version: u16,
    #[serde(deserialize_with = "deserialize_child_non_empty_text")]
    pub runtime_health_event_ref: String,
    #[serde(deserialize_with = "deserialize_child_non_empty_text")]
    pub device_ref: String,
    #[serde(deserialize_with = "deserialize_child_non_empty_text")]
    pub updated_at: String,
    pub health_state: ChildRuntimeHealthState,
    #[serde(deserialize_with = "deserialize_child_non_empty_text")]
    pub previous_event_ref: String,
    #[serde(deserialize_with = "deserialize_child_non_empty_text")]
    pub custody: String,
}

impl ChildAgentEventContract for ChildRuntimeHealthUpdatedEvent {
    const EVENT_TYPE: &'static str = crate::constants::child_agent::EVENT_RUNTIME_HEALTH_UPDATED;

    fn validate(&self) -> Result<(), EventingError> {
        validate_child_event(
            self.schema_version,
            &[
                (&self.runtime_health_event_ref, "runtime_health_event_ref"),
                (&self.device_ref, "device_ref"),
                (&self.updated_at, "updated_at"),
                (&self.previous_event_ref, "previous_event_ref"),
                (&self.custody, "custody"),
            ],
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ChildCommandKind {
    ObserveNetwork,
    ApplyPolicy,
    ApplyTrackingConfig,
    BrowserActionIntentHandoff,
    ReportHealth,
    RefreshCapability,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ChildCommandDecision {
    Accepted,
    ManualRequired,
    Unsupported,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ChildCapabilityState {
    Ready,
    ManualRequired,
    Unavailable,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ChildRuntimeHealthState {
    Healthy,
    Degraded,
    Unavailable,
}
