use ocentra_eventing::{
    AggregateKey, DomainEvent, EventContract, EventResponseContract, EventType, EventingError,
    IdempotencyKey, RequestEvent, RequestId, SchemaVersion,
};
use serde::{Deserialize, Serialize};

use crate::{
    constants, AgentCommandEnvelope, AgentRoute, TrackingRetentionSettingsWriteRequest,
    AGENT_PROTOCOL_SCHEMA_VERSION,
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TrackingConfigUpdateTargetScope {
    Family,
    ChildProfile,
    ChildDevice,
    DeviceGroup,
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
    pub device_id: String,
    pub platform: String,
    pub route: String,
}

impl TrackingConfigUpdateTarget {
    pub fn from_command(command: &AgentCommandEnvelope) -> Self {
        Self {
            scope: TrackingConfigUpdateTargetScope::ChildDevice,
            device_id: command.target.device_id.clone(),
            platform: command.target.platform.clone(),
            route: route_contract_text(&command.target.route).to_string(),
        }
    }

    fn aggregate_key_text(&self) -> String {
        format!("{}:{}", self.scope.as_contract_text(), self.device_id)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentTrackingConfigUpdatedEvent {
    pub source_command_id: String,
    pub source_message_id: String,
    pub source_peer_id: String,
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
        RequestId::parse(self.source_command_id.clone())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChildTrackingConfigUpdatedEvent {
    pub parent_event_type: String,
    pub source_command_id: String,
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
            self.parent_event_type,
            self.source_command_id,
            self.target.aggregate_key_text()
        ))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackingConfigUpdateResponse {
    pub schema_version: u16,
    pub source_command_id: String,
    pub response_state: String,
    pub effective_tracking_state: String,
    pub child_event_type: String,
    pub target: TrackingConfigUpdateTarget,
    pub local_service_state_revision: Option<u64>,
    pub durable_settings_persisted: bool,
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
        source_message_id: command.message_id.clone(),
        source_peer_id: command.source.peer_id.clone(),
        target: TrackingConfigUpdateTarget::from_command(command),
        config: request,
    }
}

pub fn child_tracking_config_updated_event_from_parent(
    parent_event: &ParentTrackingConfigUpdatedEvent,
) -> ChildTrackingConfigUpdatedEvent {
    ChildTrackingConfigUpdatedEvent {
        parent_event_type: constants::tracking_config_update::PARENT_EVENT_TYPE.to_string(),
        source_command_id: parent_event.source_command_id.clone(),
        target: parent_event.target.clone(),
        config: parent_event.config.clone(),
    }
}

fn route_contract_text(route: &AgentRoute) -> &'static str {
    match route {
        AgentRoute::Localhost => constants::tracking_config_update::ROUTE_LOCALHOST,
        AgentRoute::LocalNetwork => constants::tracking_config_update::ROUTE_LOCAL_NETWORK,
        AgentRoute::CloudRelay => constants::tracking_config_update::ROUTE_CLOUD_RELAY,
    }
}
