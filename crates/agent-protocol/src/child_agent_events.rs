use serde::{Deserialize, Serialize};

pub trait ChildAgentEventContract {
    const EVENT_TYPE: &'static str;
    const SCHEMA_VERSION: u16 = crate::constants::child_agent::EVENT_SCHEMA_VERSION;
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChildCommandReceivedEvent {
    pub schema_version: u16,
    pub command_received_event_ref: String,
    pub child_command_ref: String,
    pub received_at: String,
    pub device_ref: String,
    pub parent_controller_event_ref: String,
    pub transport_message_ref: String,
    pub command_kind: ChildCommandKind,
}

impl ChildAgentEventContract for ChildCommandReceivedEvent {
    const EVENT_TYPE: &'static str = crate::constants::child_agent::EVENT_COMMAND_RECEIVED;
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChildCommandAcceptedEvent {
    pub schema_version: u16,
    pub command_accepted_event_ref: String,
    pub child_command_ref: String,
    pub accepted_at: String,
    pub decision: ChildCommandDecision,
    pub causation_event_ref: String,
    pub idempotency_key: String,
}

impl ChildAgentEventContract for ChildCommandAcceptedEvent {
    const EVENT_TYPE: &'static str = crate::constants::child_agent::EVENT_COMMAND_ACCEPTED;
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChildCommandRejectedEvent {
    pub schema_version: u16,
    pub command_rejected_event_ref: String,
    pub child_command_ref: String,
    pub rejected_at: String,
    pub rejection_reason_code: String,
    pub causation_event_ref: String,
    pub idempotency_key: String,
}

impl ChildAgentEventContract for ChildCommandRejectedEvent {
    const EVENT_TYPE: &'static str = crate::constants::child_agent::EVENT_COMMAND_REJECTED;
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChildCapabilityStateUpdatedEvent {
    pub schema_version: u16,
    pub capability_state_event_ref: String,
    pub device_ref: String,
    pub updated_at: String,
    pub capability_ref: String,
    pub capability_state: ChildCapabilityState,
    pub previous_event_ref: String,
    pub custody: String,
}

impl ChildAgentEventContract for ChildCapabilityStateUpdatedEvent {
    const EVENT_TYPE: &'static str = crate::constants::child_agent::EVENT_CAPABILITY_STATE_UPDATED;
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChildRuntimeHealthUpdatedEvent {
    pub schema_version: u16,
    pub runtime_health_event_ref: String,
    pub device_ref: String,
    pub updated_at: String,
    pub health_state: ChildRuntimeHealthState,
    pub previous_event_ref: String,
    pub custody: String,
}

impl ChildAgentEventContract for ChildRuntimeHealthUpdatedEvent {
    const EVENT_TYPE: &'static str = crate::constants::child_agent::EVENT_RUNTIME_HEALTH_UPDATED;
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
