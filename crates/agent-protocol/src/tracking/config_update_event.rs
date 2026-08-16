use ocentra_eventing::envelope::{DomainEvent, EventContract};
use ocentra_eventing::error::EventingError;
use ocentra_eventing::expect_value::ExpectValue;
use ocentra_eventing::ids::{AggregateKey, EventType, IdempotencyKey, RequestId, SchemaVersion};
use ocentra_eventing::request::{EventResponseContract, RequestEvent};
use serde::{Deserialize, Serialize};

use super::identifiers::{
    TrackingAcceptedAt, TrackingPolicyRuleRef, TrackingReadModelEventId,
    TrackingRetentionCommandId, TrackingSourceMessageId, TrackingSourcePeerId,
    TrackingTargetDeviceId, TrackingTargetPlatform,
};
use super::retention_settings_write_command::{
    default_tracking_retention_settings_write_request, TrackingDurableSettingsPersistenceState,
    TrackingRetentionSettingsWriteRequest,
};
use super::runtime_event::{default_tracking_runtime_config, TrackingRuntimeConfig};
use crate::{constants, AgentCommandEnvelope, AgentRoute, AGENT_PROTOCOL_SCHEMA_VERSION};

pub const TRACKING_CONFIG_UPDATE_SCHEMA_VERSION: u16 = crate::AGENT_PROTOCOL_SCHEMA_VERSION;

fn parse_or_panic<T, E>(result: Result<T, E>, message: &'static str) -> T {
    result.expect_value(message)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[repr(u8)]
pub enum TrackingConfigUpdateEventName {
    Parent,
    Child,
    Applied,
}

impl TrackingConfigUpdateEventName {
    const PROTOCOL_STRINGS: [&'static str; 3] = [
        constants::tracking_config_update::PARENT_EVENT_TYPE,
        constants::tracking_config_update::CHILD_EVENT_TYPE,
        constants::tracking_config_update::APPLIED_EVENT_TYPE,
    ];

    pub fn as_contract_text(&self) -> &'static str {
        Self::PROTOCOL_STRINGS[*self as usize]
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TrackingConfigUpdateTargetScope {
    Family,
    ChildProfile,
    ChildDevice,
    DeviceGroup,
}

impl TrackingConfigUpdateTargetScope {
    const PROTOCOL_STRINGS: [&'static str; 4] = [
        constants::tracking_config_update::TARGET_SCOPE_FAMILY,
        constants::tracking_config_update::TARGET_SCOPE_CHILD_PROFILE,
        constants::tracking_config_update::TARGET_SCOPE_CHILD_DEVICE,
        constants::tracking_config_update::TARGET_SCOPE_DEVICE_GROUP,
    ];

    fn as_contract_text(&self) -> &'static str {
        Self::PROTOCOL_STRINGS[*self as usize]
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
            device_id: parse_or_panic(
                TrackingTargetDeviceId::parse(command.target.device_id.clone()),
                constants::peer::LOCAL_DEV_AGENT,
            ),
            platform: parse_or_panic(
                TrackingTargetPlatform::parse(command.target.platform.clone()),
                constants::tracking_config_update::TARGET_SCOPE_CHILD_DEVICE,
            ),
            route: command.target.route.clone(),
        }
    }

    fn aggregate_key_text(&self) -> String {
        format!("{}:{}", self.scope.as_contract_text(), self.device_id)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackingConfigUpdateRequest {
    pub command_id: TrackingRetentionCommandId,
    pub runtime_config: TrackingRuntimeConfig,
    pub retention_settings: TrackingRetentionSettingsWriteRequest,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentTrackingConfigUpdatedEvent {
    pub source_command_id: TrackingRetentionCommandId,
    pub source_message_id: TrackingSourceMessageId,
    pub source_peer_id: TrackingSourcePeerId,
    pub target: TrackingConfigUpdateTarget,
    pub config: TrackingConfigUpdateRequest,
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
    pub config: TrackingConfigUpdateRequest,
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
        (self.schema_version == AGENT_PROTOCOL_SCHEMA_VERSION)
            .then_some(())
            .ok_or(EventingError::InvalidVersion)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrackingConfigPolicyDecisionState {
    #[serde(rename = "approved")]
    Approved,
    #[serde(rename = "rejected")]
    Rejected,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrackingConfigAuditOutcome {
    #[serde(rename = "committed")]
    Committed,
    #[serde(rename = "failed")]
    Failed,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrackingConfigPortalUpdateKind {
    #[serde(rename = "tracking-config-state")]
    TrackingConfigState,
    #[serde(rename = "manual-required-state")]
    ManualRequiredState,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackingConfigChangeRequestedEvent {
    pub change_requested_event_ref: String,
    pub previous_event_ref: String,
    pub source_command_id: TrackingRetentionCommandId,
    pub source_message_id: TrackingSourceMessageId,
    pub source_peer_id: TrackingSourcePeerId,
    pub target: TrackingConfigUpdateTarget,
    pub config: TrackingConfigUpdateRequest,
    pub requested_at: TrackingAcceptedAt,
}

impl DomainEvent for TrackingConfigChangeRequestedEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        Ok(EventContract::new(
            EventType::parse(constants::tracking_config_update::CHANGE_REQUESTED_EVENT_TYPE)?,
            SchemaVersion::new(AGENT_PROTOCOL_SCHEMA_VERSION)?,
        ))
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        AggregateKey::parse(self.target.aggregate_key_text())
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        tracking_config_flow_idempotency_key(
            constants::tracking_config_update::CHANGE_REQUESTED_EVENT_TYPE,
            &self.source_command_id,
            &self.target,
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackingConfigPolicyEvaluationRequestedEvent {
    pub policy_evaluation_ref: String,
    pub previous_event_ref: String,
    pub source_command_id: TrackingRetentionCommandId,
    pub target: TrackingConfigUpdateTarget,
    pub parent_rule_refs: Vec<TrackingPolicyRuleRef>,
    pub dry_run: bool,
}

impl DomainEvent for TrackingConfigPolicyEvaluationRequestedEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        Ok(EventContract::new(
            EventType::parse(constants::network_flow::EVENT_POLICY_EVALUATION_REQUESTED)?,
            SchemaVersion::new(AGENT_PROTOCOL_SCHEMA_VERSION)?,
        ))
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        AggregateKey::parse(self.target.aggregate_key_text())
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        tracking_config_flow_idempotency_key(
            constants::network_flow::EVENT_POLICY_EVALUATION_REQUESTED,
            &self.source_command_id,
            &self.target,
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackingConfigPolicyDecisionCompletedEvent {
    pub policy_decision_ref: String,
    pub previous_event_ref: String,
    pub source_command_id: TrackingRetentionCommandId,
    pub target: TrackingConfigUpdateTarget,
    pub decision_state: TrackingConfigPolicyDecisionState,
    pub parent_rule_refs: Vec<TrackingPolicyRuleRef>,
    pub child_runtime_publish_required: bool,
}

impl DomainEvent for TrackingConfigPolicyDecisionCompletedEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        Ok(EventContract::new(
            EventType::parse(constants::network_flow::EVENT_POLICY_DECISION_COMPLETED)?,
            SchemaVersion::new(AGENT_PROTOCOL_SCHEMA_VERSION)?,
        ))
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        AggregateKey::parse(self.target.aggregate_key_text())
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        tracking_config_flow_idempotency_key(
            constants::network_flow::EVENT_POLICY_DECISION_COMPLETED,
            &self.source_command_id,
            &self.target,
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackingConfigChangeApprovedEvent {
    pub change_approved_event_ref: String,
    pub previous_event_ref: String,
    pub source_command_id: TrackingRetentionCommandId,
    pub target: TrackingConfigUpdateTarget,
    pub approved_at: TrackingAcceptedAt,
    pub child_runtime_publish_required: bool,
}

impl DomainEvent for TrackingConfigChangeApprovedEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        Ok(EventContract::new(
            EventType::parse(constants::tracking_config_update::CHANGE_APPROVED_EVENT_TYPE)?,
            SchemaVersion::new(AGENT_PROTOCOL_SCHEMA_VERSION)?,
        ))
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        AggregateKey::parse(self.target.aggregate_key_text())
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        tracking_config_flow_idempotency_key(
            constants::tracking_config_update::CHANGE_APPROVED_EVENT_TYPE,
            &self.source_command_id,
            &self.target,
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackingConfigChangeRejectedEvent {
    pub change_rejected_event_ref: String,
    pub previous_event_ref: String,
    pub source_command_id: TrackingRetentionCommandId,
    pub target: TrackingConfigUpdateTarget,
    pub rejected_at: TrackingAcceptedAt,
    pub rejection_reason_code: String,
}

impl DomainEvent for TrackingConfigChangeRejectedEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        Ok(EventContract::new(
            EventType::parse(constants::tracking_config_update::CHANGE_REJECTED_EVENT_TYPE)?,
            SchemaVersion::new(AGENT_PROTOCOL_SCHEMA_VERSION)?,
        ))
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        AggregateKey::parse(self.target.aggregate_key_text())
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        tracking_config_flow_idempotency_key(
            constants::tracking_config_update::CHANGE_REJECTED_EVENT_TYPE,
            &self.source_command_id,
            &self.target,
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackingConfigAuditEntryCommittedEvent {
    pub audit_entry_ref: String,
    pub previous_event_ref: String,
    pub source_command_id: TrackingRetentionCommandId,
    pub policy_decision_ref: String,
    pub target: TrackingConfigUpdateTarget,
    pub audit_outcome: TrackingConfigAuditOutcome,
}

impl DomainEvent for TrackingConfigAuditEntryCommittedEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        Ok(EventContract::new(
            EventType::parse(constants::network_flow::EVENT_AUDIT_ENTRY_COMMITTED)?,
            SchemaVersion::new(AGENT_PROTOCOL_SCHEMA_VERSION)?,
        ))
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        AggregateKey::parse(self.target.aggregate_key_text())
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        tracking_config_flow_idempotency_key(
            constants::network_flow::EVENT_AUDIT_ENTRY_COMMITTED,
            &self.source_command_id,
            &self.target,
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackingConfigPortalReadModelUpdatedEvent {
    pub read_model_ref: TrackingReadModelEventId,
    pub previous_event_ref: String,
    pub audit_entry_ref: String,
    pub source_command_id: TrackingRetentionCommandId,
    pub target: TrackingConfigUpdateTarget,
    pub update_kind: TrackingConfigPortalUpdateKind,
    pub visible_manual_required: bool,
    pub visible_unavailable: bool,
}

impl DomainEvent for TrackingConfigPortalReadModelUpdatedEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        Ok(EventContract::new(
            EventType::parse(constants::network_flow::EVENT_PORTAL_READ_MODEL_UPDATED)?,
            SchemaVersion::new(AGENT_PROTOCOL_SCHEMA_VERSION)?,
        ))
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        AggregateKey::parse(self.target.aggregate_key_text())
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        tracking_config_flow_idempotency_key(
            constants::network_flow::EVENT_PORTAL_READ_MODEL_UPDATED,
            &self.source_command_id,
            &self.target,
        )
    }
}

pub fn tracking_config_change_requested_event(
    parent_action_event_ref: impl Into<String>,
    parent_event: &ParentTrackingConfigUpdatedEvent,
) -> TrackingConfigChangeRequestedEvent {
    TrackingConfigChangeRequestedEvent {
        change_requested_event_ref: tracking_config_flow_event_ref(
            &parent_event.source_command_id,
            "change-requested",
        ),
        previous_event_ref: parent_action_event_ref.into(),
        source_command_id: parent_event.source_command_id.clone(),
        source_message_id: parent_event.source_message_id.clone(),
        source_peer_id: parent_event.source_peer_id.clone(),
        target: parent_event.target.clone(),
        config: parent_event.config.clone(),
        requested_at: tracking_config_accepted_at(),
    }
}

pub fn tracking_config_policy_evaluation_requested_event(
    requested_event: &TrackingConfigChangeRequestedEvent,
    parent_rule_refs: Vec<TrackingPolicyRuleRef>,
    dry_run: bool,
) -> TrackingConfigPolicyEvaluationRequestedEvent {
    TrackingConfigPolicyEvaluationRequestedEvent {
        policy_evaluation_ref: tracking_config_flow_event_ref(
            &requested_event.source_command_id,
            "policy-evaluation-requested",
        ),
        previous_event_ref: requested_event.change_requested_event_ref.clone(),
        source_command_id: requested_event.source_command_id.clone(),
        target: requested_event.target.clone(),
        parent_rule_refs,
        dry_run,
    }
}

pub fn tracking_config_policy_decision_completed_event(
    evaluation_event: &TrackingConfigPolicyEvaluationRequestedEvent,
    decision_state: TrackingConfigPolicyDecisionState,
    child_runtime_publish_required: bool,
) -> TrackingConfigPolicyDecisionCompletedEvent {
    TrackingConfigPolicyDecisionCompletedEvent {
        policy_decision_ref: tracking_config_flow_event_ref(
            &evaluation_event.source_command_id,
            "policy-decision-completed",
        ),
        previous_event_ref: evaluation_event.policy_evaluation_ref.clone(),
        source_command_id: evaluation_event.source_command_id.clone(),
        target: evaluation_event.target.clone(),
        decision_state,
        parent_rule_refs: evaluation_event.parent_rule_refs.clone(),
        child_runtime_publish_required,
    }
}

pub fn tracking_config_change_approved_event(
    decision_event: &TrackingConfigPolicyDecisionCompletedEvent,
) -> TrackingConfigChangeApprovedEvent {
    TrackingConfigChangeApprovedEvent {
        change_approved_event_ref: tracking_config_flow_event_ref(
            &decision_event.source_command_id,
            "change-approved",
        ),
        previous_event_ref: decision_event.policy_decision_ref.clone(),
        source_command_id: decision_event.source_command_id.clone(),
        target: decision_event.target.clone(),
        approved_at: tracking_config_accepted_at(),
        child_runtime_publish_required: decision_event.child_runtime_publish_required,
    }
}

pub fn tracking_config_change_rejected_event(
    decision_event: &TrackingConfigPolicyDecisionCompletedEvent,
    rejection_reason_code: impl Into<String>,
) -> TrackingConfigChangeRejectedEvent {
    TrackingConfigChangeRejectedEvent {
        change_rejected_event_ref: tracking_config_flow_event_ref(
            &decision_event.source_command_id,
            "change-rejected",
        ),
        previous_event_ref: decision_event.policy_decision_ref.clone(),
        source_command_id: decision_event.source_command_id.clone(),
        target: decision_event.target.clone(),
        rejected_at: tracking_config_accepted_at(),
        rejection_reason_code: rejection_reason_code.into(),
    }
}

pub fn tracking_config_audit_entry_committed_event(
    decision_event: &TrackingConfigPolicyDecisionCompletedEvent,
    previous_event_ref: impl Into<String>,
    audit_outcome: TrackingConfigAuditOutcome,
) -> TrackingConfigAuditEntryCommittedEvent {
    TrackingConfigAuditEntryCommittedEvent {
        audit_entry_ref: tracking_config_flow_event_ref(
            &decision_event.source_command_id,
            "audit-entry-committed",
        ),
        previous_event_ref: previous_event_ref.into(),
        source_command_id: decision_event.source_command_id.clone(),
        policy_decision_ref: decision_event.policy_decision_ref.clone(),
        target: decision_event.target.clone(),
        audit_outcome,
    }
}

pub fn tracking_config_portal_read_model_updated_event(
    audit_event: &TrackingConfigAuditEntryCommittedEvent,
    update_kind: TrackingConfigPortalUpdateKind,
    visible_manual_required: bool,
    visible_unavailable: bool,
) -> TrackingConfigPortalReadModelUpdatedEvent {
    TrackingConfigPortalReadModelUpdatedEvent {
        read_model_ref: parse_or_panic(
            TrackingReadModelEventId::parse(tracking_config_flow_event_ref(
                &audit_event.source_command_id,
                "portal-read-model-updated",
            )),
            constants::tracking_config_update::READ_MODEL_UPDATE_KIND_TRACKING_CONFIG_STATE,
        ),
        previous_event_ref: audit_event.audit_entry_ref.clone(),
        audit_entry_ref: audit_event.audit_entry_ref.clone(),
        source_command_id: audit_event.source_command_id.clone(),
        target: audit_event.target.clone(),
        update_kind,
        visible_manual_required,
        visible_unavailable,
    }
}

pub fn parent_tracking_config_updated_event_from_command(
    command: &AgentCommandEnvelope,
    request: TrackingConfigUpdateRequest,
) -> ParentTrackingConfigUpdatedEvent {
    ParentTrackingConfigUpdatedEvent {
        source_command_id: request.command_id.clone(),
        source_message_id: parse_or_panic(
            TrackingSourceMessageId::parse(command.message_id.clone()),
            constants::tracking_retention_settings_write::COMMAND_ID,
        ),
        source_peer_id: parse_or_panic(
            TrackingSourcePeerId::parse(command.source.peer_id.clone()),
            constants::peer::PORTAL_DEV,
        ),
        target: TrackingConfigUpdateTarget::from_command(command),
        config: request,
    }
}

pub fn default_tracking_config_update_request() -> TrackingConfigUpdateRequest {
    let retention_settings = default_tracking_retention_settings_write_request();
    TrackingConfigUpdateRequest {
        command_id: retention_settings.command_id.clone(),
        runtime_config: default_tracking_runtime_config(),
        retention_settings,
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
        parent_event_type: child_event.parent_event_type,
        child_event_type: TrackingConfigUpdateEventName::Child,
        source_command_id: child_event.source_command_id.clone(),
        target: child_event.target.clone(),
        response_state,
        effective_tracking_state,
        local_service_state_revision,
        durable_settings_persistence_state,
    }
}

fn tracking_config_flow_event_ref(
    source_command_id: &TrackingRetentionCommandId,
    suffix: &str,
) -> String {
    format!("event.{}.{}", source_command_id, suffix)
}

fn tracking_config_flow_idempotency_key(
    event_type: &str,
    source_command_id: &TrackingRetentionCommandId,
    target: &TrackingConfigUpdateTarget,
) -> Result<IdempotencyKey, EventingError> {
    IdempotencyKey::parse(format!(
        "{}:{}:{}",
        event_type,
        source_command_id,
        target.aggregate_key_text()
    ))
}

fn tracking_config_accepted_at() -> TrackingAcceptedAt {
    parse_or_panic(
        TrackingAcceptedAt::parse(constants::tracking_retention_settings_write::ACCEPTED_AT),
        constants::tracking_retention_settings_write::ACCEPTED_AT,
    )
}
