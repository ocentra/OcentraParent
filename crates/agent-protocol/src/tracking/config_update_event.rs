use ocentra_eventing::{
    AggregateKey, DomainEvent, EventContract, EventResponseContract, EventType, EventingError,
    IdempotencyKey, RequestEvent, RequestId, SchemaVersion,
};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{
    constants, AgentCommandEnvelope, AgentRoute, TrackingRetentionSettingsWriteRequest,
    AGENT_PROTOCOL_SCHEMA_VERSION,
};
use super::{
    TrackingDurableSettingsPersistenceState,
    TrackingRetentionCommandId, TrackingSourceMessageId, TrackingSourcePeerId,
    TrackingTargetDeviceId, TrackingTargetPlatform,
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TrackingConfigUpdateTargetScope {
    Family,
    ChildProfile,
    ChildDevice,
    DeviceGroup,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TrackingConfigUpdateEventName {
    Parent,
    Child,
    Applied,
}

impl TrackingConfigUpdateEventName {
    pub fn as_contract_text(&self) -> &'static str {
        match self {
            Self::Parent => constants::tracking_config_update::PARENT_EVENT_TYPE,
            Self::Child => constants::tracking_config_update::CHILD_EVENT_TYPE,
            Self::Applied => constants::tracking_config_update::APPLIED_EVENT_TYPE,
        }
    }
}

impl Serialize for TrackingConfigUpdateEventName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_contract_text())
    }
}

impl<'de> Deserialize<'de> for TrackingConfigUpdateEventName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            constants::tracking_config_update::PARENT_EVENT_TYPE => Ok(Self::Parent),
            constants::tracking_config_update::CHILD_EVENT_TYPE => Ok(Self::Child),
            constants::tracking_config_update::APPLIED_EVENT_TYPE => Ok(Self::Applied),
            _ => Err(serde::de::Error::unknown_variant(
                value.as_str(),
                &[
                    constants::tracking_config_update::PARENT_EVENT_TYPE,
                    constants::tracking_config_update::CHILD_EVENT_TYPE,
                    constants::tracking_config_update::APPLIED_EVENT_TYPE,
                ],
            )),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrackingConfigUpdateResponseState {
    #[serde(rename = "applied")]
    Applied,
    #[serde(rename = "rejected")]
    Rejected,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrackingConfigEffectiveState {
    #[serde(rename = "enabled")]
    Enabled,
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "degraded")]
    Degraded,
}

impl TrackingConfigUpdateTargetScope {
    fn as_contract_text(&self) -> &'static str {
        match self {
            Self::Family => constants::tracking_config_update::TARGET_SCOPE_FAMILY,
            Self::ChildProfile => constants::tracking_config_update::TARGET_SCOPE_CHILD_PROFILE,
            Self::ChildDevice => constants::tracking_config_update::TARGET_SCOPE_CHILD_DEVICE,
            Self::DeviceGroup => constants::tracking_config_update::TARGET_SCOPE_DEVICE_GROUP,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackingConfigUpdateTarget {
    pub scope: TrackingConfigUpdateTargetScope,
    pub device_id: TrackingTargetDeviceId,
    pub platform: TrackingTargetPlatform,
    pub route: AgentRoute,
}

impl TrackingConfigUpdateTarget {
    pub fn from_command(command: &AgentCommandEnvelope) -> Self {
        Self {
            scope: TrackingConfigUpdateTargetScope::ChildDevice,
            device_id: TrackingTargetDeviceId::parse(command.target.device_id.clone())
                .expect(constants::peer::LOCAL_DEV_AGENT),
            platform: TrackingTargetPlatform::parse(command.target.platform.clone())
                .expect(constants::tracking_config_update::TARGET_SCOPE_CHILD_DEVICE),
            route: command.target.route.clone(),
        }
    }

    fn aggregate_key_text(&self) -> String {
        format!("{}:{}", self.scope.as_contract_text(), self.device_id)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentTrackingConfigUpdatedEvent {
    pub source_command_id: TrackingRetentionCommandId,
    pub source_message_id: TrackingSourceMessageId,
    pub source_peer_id: TrackingSourcePeerId,
    pub target: TrackingConfigUpdateTarget,
    pub config: TrackingRetentionSettingsWriteRequest,
}

impl DomainEvent for ParentTrackingConfigUpdatedEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        Ok(EventContract::new(
            EventType::parse(constants::tracking_config_update::PARENT_EVENT_TYPE)?,
            SchemaVersion::new(AGENT_PROTOCOL_SCHEMA_VERSION)?,
        ))
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        AggregateKey::parse(self.target.aggregate_key_text())
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        IdempotencyKey::parse(format!(
            "{}:{}",
            self.source_command_id,
            self.target.aggregate_key_text()
        ))
    }
}

impl RequestEvent for ParentTrackingConfigUpdatedEvent {
    type Response = TrackingConfigUpdateResponse;

    fn request_id(&self) -> Result<RequestId, EventingError> {
        RequestId::parse(self.source_command_id.as_str())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChildTrackingConfigUpdatedEvent {
    pub parent_event_type: TrackingConfigUpdateEventName,
    pub source_command_id: TrackingRetentionCommandId,
    pub target: TrackingConfigUpdateTarget,
    pub config: TrackingRetentionSettingsWriteRequest,
}

impl DomainEvent for ChildTrackingConfigUpdatedEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        Ok(EventContract::new(
            EventType::parse(constants::tracking_config_update::CHILD_EVENT_TYPE)?,
            SchemaVersion::new(AGENT_PROTOCOL_SCHEMA_VERSION)?,
        ))
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        AggregateKey::parse(self.target.aggregate_key_text())
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        IdempotencyKey::parse(format!(
            "{}:{}:{}",
            self.parent_event_type.as_contract_text(),
            self.source_command_id,
            self.target.aggregate_key_text()
        ))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackingConfigUpdateAppliedEvent {
    pub parent_event_type: TrackingConfigUpdateEventName,
    pub child_event_type: TrackingConfigUpdateEventName,
    pub source_command_id: TrackingRetentionCommandId,
    pub target: TrackingConfigUpdateTarget,
    pub response_state: TrackingConfigUpdateResponseState,
    pub effective_tracking_state: TrackingConfigEffectiveState,
    pub local_service_state_revision: u64,
    pub durable_settings_persistence_state: TrackingDurableSettingsPersistenceState,
}

impl DomainEvent for TrackingConfigUpdateAppliedEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        Ok(EventContract::new(
            EventType::parse(constants::tracking_config_update::APPLIED_EVENT_TYPE)?,
            SchemaVersion::new(AGENT_PROTOCOL_SCHEMA_VERSION)?,
        ))
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        AggregateKey::parse(self.target.aggregate_key_text())
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        IdempotencyKey::parse(format!(
            "{}:{}:{}",
            TrackingConfigUpdateEventName::Applied.as_contract_text(),
            self.source_command_id,
            self.target.aggregate_key_text()
        ))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackingConfigUpdateResponse {
    pub schema_version: u16,
    pub source_command_id: TrackingRetentionCommandId,
    pub response_state: TrackingConfigUpdateResponseState,
    pub effective_tracking_state: TrackingConfigEffectiveState,
    pub child_event_type: TrackingConfigUpdateEventName,
    pub target: TrackingConfigUpdateTarget,
    pub local_service_state_revision: Option<u64>,
    pub durable_settings_persistence_state: TrackingDurableSettingsPersistenceState,
}

impl EventResponseContract for TrackingConfigUpdateResponse {
    fn validate(&self) -> Result<(), EventingError> {
        if self.schema_version != AGENT_PROTOCOL_SCHEMA_VERSION {
            return Err(EventingError::InvalidVersion);
        }
        Ok(())
    }
}

pub fn parent_tracking_config_updated_event_from_command(
    command: &AgentCommandEnvelope,
    request: TrackingRetentionSettingsWriteRequest,
) -> ParentTrackingConfigUpdatedEvent {
    ParentTrackingConfigUpdatedEvent {
        source_command_id: request.command_id.clone(),
        source_message_id: TrackingSourceMessageId::parse(command.message_id.clone())
            .expect(constants::tracking_retention_settings_write::COMMAND_ID),
        source_peer_id: TrackingSourcePeerId::parse(command.source.peer_id.clone())
            .expect(constants::peer::PORTAL_DEV),
        target: TrackingConfigUpdateTarget::from_command(command),
        config: request,
    }
}

pub fn child_tracking_config_updated_event_from_parent(
    parent_event: &ParentTrackingConfigUpdatedEvent,
) -> ChildTrackingConfigUpdatedEvent {
    ChildTrackingConfigUpdatedEvent {
        parent_event_type: TrackingConfigUpdateEventName::Parent,
        source_command_id: parent_event.source_command_id.clone(),
        target: parent_event.target.clone(),
        config: parent_event.config.clone(),
    }
}

pub fn tracking_config_update_applied_event_from_child(
    child_event: &ChildTrackingConfigUpdatedEvent,
    response_state: TrackingConfigUpdateResponseState,
    effective_tracking_state: TrackingConfigEffectiveState,
    local_service_state_revision: u64,
    durable_settings_persistence_state: TrackingDurableSettingsPersistenceState,
) -> TrackingConfigUpdateAppliedEvent {
    TrackingConfigUpdateAppliedEvent {
        parent_event_type: child_event.parent_event_type.clone(),
        child_event_type: TrackingConfigUpdateEventName::Child,
        source_command_id: child_event.source_command_id.clone(),
        target: child_event.target.clone(),
        response_state,
        effective_tracking_state,
        local_service_state_revision,
        durable_settings_persistence_state,
    }
}
