use std::time::Duration;

use ocentra_eventing::{
    AggregateKey, CorrelationId, DomainEvent, EventBus, EventContract, EventMetadata,
    EventResponseContract, EventSource, EventSubscriber, EventType, EventingError, IdempotencyKey,
    RecordedAt, RequestEvent, RequestId, RequestOptions, RuntimeInstanceId, SchemaVersion,
    SourceComponent, SourceService, SubscriberId, TargetHandler,
};
use ocentra_parent_agent_protocol::{
    constants, AgentCommandEnvelope, AgentEventEnvelope, AgentEventName, LogFieldValue, LogFields,
    LogLevel, SocialParentNotificationDeliveryReadinessRow,
    SocialParentNotificationDeliveryReadinessSnapshot,
    SOCIAL_PARENT_NOTIFICATION_DELIVERY_AUDIT_REF,
    SOCIAL_PARENT_NOTIFICATION_DELIVERY_CAPABILITY_READY,
    SOCIAL_PARENT_NOTIFICATION_DELIVERY_EVIDENCE_REF,
    SOCIAL_PARENT_NOTIFICATION_DELIVERY_EXECUTION_REPORT_READY,
    SOCIAL_PARENT_NOTIFICATION_DELIVERY_EXTERNAL_RUNTIME_UNAVAILABLE,
    SOCIAL_PARENT_NOTIFICATION_DELIVERY_MANUAL_UI_PROOF_REQUIRED,
    SOCIAL_PARENT_NOTIFICATION_DELIVERY_NON_CLAIM_ENFORCEMENT,
    SOCIAL_PARENT_NOTIFICATION_DELIVERY_NON_CLAIM_EXTERNAL_RUNTIME,
    SOCIAL_PARENT_NOTIFICATION_DELIVERY_NON_CLAIM_FINAL_POLICY,
    SOCIAL_PARENT_NOTIFICATION_DELIVERY_NON_CLAIM_PARENT_NOTIFICATION_UI,
    SOCIAL_PARENT_NOTIFICATION_DELIVERY_NON_CLAIM_PROVIDER_DELIVERY,
    SOCIAL_PARENT_NOTIFICATION_DELIVERY_NON_CLAIM_PROVIDER_RECEIPT,
    SOCIAL_PARENT_NOTIFICATION_DELIVERY_PARENT_REPORT_REF,
    SOCIAL_PARENT_NOTIFICATION_DELIVERY_PARENT_VISIBLE_MANUAL_REQUIRED_REF,
    SOCIAL_PARENT_NOTIFICATION_DELIVERY_PARENT_VISIBLE_REPORT_STATUS_REF,
    SOCIAL_PARENT_NOTIFICATION_DELIVERY_POLICY_REF,
    SOCIAL_PARENT_NOTIFICATION_DELIVERY_READINESS_ID,
    SOCIAL_PARENT_NOTIFICATION_DELIVERY_REPORT_ARTIFACT_REF,
    SOCIAL_PARENT_NOTIFICATION_DELIVERY_REPORT_RECEIPT_REF,
    SOCIAL_PARENT_NOTIFICATION_DELIVERY_REPORT_WRITER_ROW_REF,
    SOCIAL_PARENT_NOTIFICATION_DELIVERY_ROW_MANUAL_REQUIRED,
    SOCIAL_PARENT_NOTIFICATION_DELIVERY_ROW_REPORT_READY,
    SOCIAL_PARENT_NOTIFICATION_DELIVERY_ROW_UNAVAILABLE,
    SOCIAL_PARENT_NOTIFICATION_DELIVERY_SCHEMA_VERSION,
    SOCIAL_PARENT_NOTIFICATION_DELIVERY_SOURCE_INTENT_REF,
    SOCIAL_PARENT_NOTIFICATION_DELIVERY_SOURCE_REPORT_WRITER_PROOF_REF,
    SOCIAL_PARENT_NOTIFICATION_DELIVERY_STATE_MANUAL_REQUIRED,
    SOCIAL_PARENT_NOTIFICATION_DELIVERY_STATE_REPORT_READY,
    SOCIAL_PARENT_NOTIFICATION_DELIVERY_STATE_UNAVAILABLE,
};
use serde::{Deserialize, Serialize};

use crate::{event_builder::build_event, fields::fields_from_pairs, time::timestamp_now};

type FieldPair = (&'static str, LogFieldValue);

pub fn social_parent_notification_delivery_read_model_from_service(
) -> SocialParentNotificationDeliveryReadinessSnapshot {
    let generated_at = timestamp_now();
    let rows = vec![
        report_ready_row(&generated_at),
        manual_required_row(&generated_at),
        unavailable_row(&generated_at),
    ];
    SocialParentNotificationDeliveryReadinessSnapshot {
        schema_version: SOCIAL_PARENT_NOTIFICATION_DELIVERY_SCHEMA_VERSION.to_string(),
        readiness_id: SOCIAL_PARENT_NOTIFICATION_DELIVERY_READINESS_ID.to_string(),
        generated_at,
        source_report_writer_proof_ref:
            SOCIAL_PARENT_NOTIFICATION_DELIVERY_SOURCE_REPORT_WRITER_PROOF_REF.to_string(),
        parent_report_status_ready_count: count_rows(
            &rows,
            SOCIAL_PARENT_NOTIFICATION_DELIVERY_STATE_REPORT_READY,
        ),
        manual_required_count: count_rows(
            &rows,
            SOCIAL_PARENT_NOTIFICATION_DELIVERY_STATE_MANUAL_REQUIRED,
        ),
        unavailable_count: count_rows(&rows, SOCIAL_PARENT_NOTIFICATION_DELIVERY_STATE_UNAVAILABLE),
        rows,
        non_claims: non_claims(),
        parent_notification_ui_delivery_claimed: false,
        external_runtime_report_delivery_claimed: false,
        final_policy_execution_claimed: false,
        enforcement_claimed: false,
    }
}

pub async fn request_social_parent_notification_delivery_read_model_from_service(
) -> Result<SocialParentNotificationDeliveryReadinessSnapshot, EventingError> {
    let bus = EventBus::new();
    bus.subscribe::<SocialParentNotificationDeliveryReadModelRequest, _, _>(
        EventSubscriber::new(
            SubscriberId::parse(
                constants::browser::SUBSCRIBER_BROWSER_SOCIAL_PARENT_NOTIFICATION_DELIVERY_STATUS,
            )?,
            EventType::parse(
                constants::browser::EVENT_BROWSER_SOCIAL_PARENT_NOTIFICATION_DELIVERY_STATUS_REQUESTED,
            )?,
            TargetHandler::parse(
                constants::browser::TARGET_BROWSER_SOCIAL_PARENT_NOTIFICATION_DELIVERY_STATUS,
            )?,
        ),
        |context| async move {
            context
                .complete_request(SocialParentNotificationDeliveryReadModelResponse {
                    read_model: social_parent_notification_delivery_read_model_from_service(),
                })
                .await?;
            Ok(())
        },
    )
    .await?;

    let requested_at = timestamp_now();
    let request = SocialParentNotificationDeliveryReadModelRequest {
        request_id: RequestId::parse(social_parent_notification_delivery_request_id(
            &requested_at,
        ))?,
        requested_at,
    };
    let metadata = social_parent_notification_delivery_metadata(&request)?;
    let report = bus
        .publish_request(
            request,
            metadata,
            RequestOptions::with_timeout(Duration::from_millis(
                constants::browser::REQUEST_BROWSER_SOCIAL_PARENT_NOTIFICATION_DELIVERY_STATUS_TIMEOUT_MS,
            ))?,
        )
        .await?;

    Ok(report.response.read_model)
}

pub fn social_parent_notification_delivery_read_model_payload(
    read_model: &SocialParentNotificationDeliveryReadinessSnapshot,
) -> LogFields {
    fields_from_pairs(read_model_pairs(read_model))
}

pub async fn build_browser_social_parent_notification_delivery_read_model_report(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    let read_model = request_social_parent_notification_delivery_read_model_from_service()
        .await
        .unwrap_or_else(|_| social_parent_notification_delivery_read_model_from_service());
    build_event(
        constants::event_id::BROWSER_SOCIAL_PARENT_NOTIFICATION_DELIVERY_READ_MODEL_REPORTED,
        &command.message_id,
        command.source,
        AgentEventName::AgentBrowserSocialParentNotificationDeliveryReadModelReported,
        LogLevel::Info,
        social_parent_notification_delivery_read_model_payload(&read_model),
        None,
    )
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct SocialParentNotificationDeliveryReadModelRequest {
    request_id: RequestId,
    requested_at: String,
}

impl DomainEvent for SocialParentNotificationDeliveryReadModelRequest {
    fn contract(&self) -> Result<EventContract, EventingError> {
        Ok(EventContract::new(
            EventType::parse(
                constants::browser::EVENT_BROWSER_SOCIAL_PARENT_NOTIFICATION_DELIVERY_STATUS_REQUESTED,
            )?,
            SchemaVersion::new(constants::browser::EVENT_SCHEMA_VERSION)?,
        ))
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        AggregateKey::parse(constants::browser::AGGREGATE_BROWSER_RUNTIME_PREFIX)
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        let mut value = String::from(
            constants::browser::IDEMPOTENCY_BROWSER_SOCIAL_PARENT_NOTIFICATION_DELIVERY_STATUS_PREFIX,
        );
        value.push_str(self.request_id.as_str());
        IdempotencyKey::parse(value)
    }
}

impl RequestEvent for SocialParentNotificationDeliveryReadModelRequest {
    type Response = SocialParentNotificationDeliveryReadModelResponse;

    fn request_id(&self) -> Result<RequestId, EventingError> {
        Ok(self.request_id.clone())
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct SocialParentNotificationDeliveryReadModelResponse {
    read_model: SocialParentNotificationDeliveryReadinessSnapshot,
}

impl EventResponseContract for SocialParentNotificationDeliveryReadModelResponse {}

fn social_parent_notification_delivery_metadata(
    request: &SocialParentNotificationDeliveryReadModelRequest,
) -> Result<EventMetadata, EventingError> {
    Ok(EventMetadata::from_parts(
        ocentra_eventing::EventId::generated(),
        CorrelationId::parse(social_parent_notification_delivery_correlation_id(
            &request.requested_at,
        ))?,
        EventSource::new(
            ocentra_eventing::EventCustody::parse(
                constants::eventing_source::CUSTODY_LOCAL_QUERY_STORE,
            )?,
            ocentra_eventing::RuntimeRole::parse(constants::eventing_source::ROLE_CONTROLLER)?,
            SourceService::parse(constants::peer::LOCAL_DEV_AGENT)?,
            SourceComponent::parse(constants::browser::RUNTIME_COMPONENT_BROWSER_SPINE)?,
            RuntimeInstanceId::parse(constants::browser::RUNTIME_INSTANCE_LOCAL_BROWSER_RUNTIME)?,
        ),
        RecordedAt::parse(&request.requested_at)?,
        Some(TargetHandler::parse(
            constants::browser::TARGET_BROWSER_SOCIAL_PARENT_NOTIFICATION_DELIVERY_STATUS,
        )?),
    ))
}

fn social_parent_notification_delivery_request_id(requested_at: &str) -> String {
    let mut value = String::from(
        constants::browser::REQUEST_BROWSER_SOCIAL_PARENT_NOTIFICATION_DELIVERY_STATUS_PREFIX,
    );
    value.push_str(requested_at);
    value
}

fn social_parent_notification_delivery_correlation_id(requested_at: &str) -> String {
    let mut value = String::from(constants::browser::CORRELATION_BROWSER_RUNTIME_PREFIX);
    value.push_str(requested_at);
    value
}

fn read_model_pairs(
    read_model: &SocialParentNotificationDeliveryReadinessSnapshot,
) -> Vec<FieldPair> {
    vec![
        (
            constants::field::GENERATED_AT,
            LogFieldValue::String(read_model.generated_at.clone()),
        ),
        (
            constants::field::CAPABILITY_STATUS,
            LogFieldValue::String(SOCIAL_PARENT_NOTIFICATION_DELIVERY_CAPABILITY_READY.to_string()),
        ),
        (
            constants::field::RETURNED,
            LogFieldValue::Number(read_model.rows.len() as f64),
        ),
        (
            constants::field::BROWSER_SOCIAL_PARENT_NOTIFICATION_DELIVERY_READ_MODEL,
            LogFieldValue::String(
                serde_json::to_string(read_model).expect(constants::error::AGENT_EVENT_SERIALIZES),
            ),
        ),
    ]
}

fn report_ready_row(created_at: &str) -> SocialParentNotificationDeliveryReadinessRow {
    row(
        SOCIAL_PARENT_NOTIFICATION_DELIVERY_ROW_REPORT_READY,
        Some(SOCIAL_PARENT_NOTIFICATION_DELIVERY_PARENT_VISIBLE_REPORT_STATUS_REF.to_string()),
        Some(SOCIAL_PARENT_NOTIFICATION_DELIVERY_PARENT_REPORT_REF.to_string()),
        Some(SOCIAL_PARENT_NOTIFICATION_DELIVERY_REPORT_ARTIFACT_REF.to_string()),
        Some(SOCIAL_PARENT_NOTIFICATION_DELIVERY_REPORT_RECEIPT_REF.to_string()),
        Vec::new(),
        SOCIAL_PARENT_NOTIFICATION_DELIVERY_STATE_REPORT_READY,
        SOCIAL_PARENT_NOTIFICATION_DELIVERY_EXECUTION_REPORT_READY,
        true,
        true,
        created_at,
    )
}

fn manual_required_row(created_at: &str) -> SocialParentNotificationDeliveryReadinessRow {
    row(
        SOCIAL_PARENT_NOTIFICATION_DELIVERY_ROW_MANUAL_REQUIRED,
        Some(SOCIAL_PARENT_NOTIFICATION_DELIVERY_PARENT_VISIBLE_MANUAL_REQUIRED_REF.to_string()),
        None,
        None,
        None,
        vec![SOCIAL_PARENT_NOTIFICATION_DELIVERY_MANUAL_UI_PROOF_REQUIRED.to_string()],
        SOCIAL_PARENT_NOTIFICATION_DELIVERY_STATE_MANUAL_REQUIRED,
        SOCIAL_PARENT_NOTIFICATION_DELIVERY_STATE_MANUAL_REQUIRED,
        false,
        false,
        created_at,
    )
}

fn unavailable_row(created_at: &str) -> SocialParentNotificationDeliveryReadinessRow {
    row(
        SOCIAL_PARENT_NOTIFICATION_DELIVERY_ROW_UNAVAILABLE,
        None,
        None,
        None,
        None,
        vec![SOCIAL_PARENT_NOTIFICATION_DELIVERY_EXTERNAL_RUNTIME_UNAVAILABLE.to_string()],
        SOCIAL_PARENT_NOTIFICATION_DELIVERY_STATE_UNAVAILABLE,
        SOCIAL_PARENT_NOTIFICATION_DELIVERY_STATE_UNAVAILABLE,
        false,
        false,
        created_at,
    )
}

#[allow(clippy::too_many_arguments)]
fn row(
    row_id: &'static str,
    parent_visible_report_status_ref: Option<String>,
    parent_report_ref: Option<String>,
    report_artifact_ref: Option<String>,
    report_receipt_ref: Option<String>,
    manual_proof_requirements: Vec<String>,
    notification_delivery_readiness_state: &'static str,
    report_delivery_execution_state: &'static str,
    parent_owned_report_artifact_written: bool,
    parent_owned_report_receipt_recorded: bool,
    created_at: &str,
) -> SocialParentNotificationDeliveryReadinessRow {
    SocialParentNotificationDeliveryReadinessRow {
        notification_delivery_readiness_row_id: row_id.to_string(),
        source_report_writer_delivery_row_ref:
            SOCIAL_PARENT_NOTIFICATION_DELIVERY_REPORT_WRITER_ROW_REF.to_string(),
        source_intent_ref: SOCIAL_PARENT_NOTIFICATION_DELIVERY_SOURCE_INTENT_REF.to_string(),
        parent_visible_report_status_ref,
        parent_notification_ui_ref: None,
        parent_report_ref,
        report_artifact_ref,
        report_receipt_ref,
        source_evidence_refs: vec![SOCIAL_PARENT_NOTIFICATION_DELIVERY_EVIDENCE_REF.to_string()],
        source_policy_refs: vec![SOCIAL_PARENT_NOTIFICATION_DELIVERY_POLICY_REF.to_string()],
        source_audit_refs: vec![SOCIAL_PARENT_NOTIFICATION_DELIVERY_AUDIT_REF.to_string()],
        manual_proof_requirements,
        notification_delivery_readiness_state: notification_delivery_readiness_state.to_string(),
        report_delivery_execution_state: report_delivery_execution_state.to_string(),
        parent_owned_report_artifact_written,
        parent_owned_report_receipt_recorded,
        parent_notification_ui_delivered: false,
        external_runtime_report_delivery_claimed: false,
        provider_delivery_attempted: false,
        provider_receipt_ingested: false,
        final_policy_decision_claimed: false,
        enforcement_claimed: false,
        created_at: created_at.to_string(),
    }
}

fn non_claims() -> Vec<String> {
    vec![
        SOCIAL_PARENT_NOTIFICATION_DELIVERY_NON_CLAIM_PARENT_NOTIFICATION_UI.to_string(),
        SOCIAL_PARENT_NOTIFICATION_DELIVERY_NON_CLAIM_EXTERNAL_RUNTIME.to_string(),
        SOCIAL_PARENT_NOTIFICATION_DELIVERY_NON_CLAIM_PROVIDER_DELIVERY.to_string(),
        SOCIAL_PARENT_NOTIFICATION_DELIVERY_NON_CLAIM_PROVIDER_RECEIPT.to_string(),
        SOCIAL_PARENT_NOTIFICATION_DELIVERY_NON_CLAIM_FINAL_POLICY.to_string(),
        SOCIAL_PARENT_NOTIFICATION_DELIVERY_NON_CLAIM_ENFORCEMENT.to_string(),
    ]
}

fn count_rows(rows: &[SocialParentNotificationDeliveryReadinessRow], state: &str) -> usize {
    rows.iter()
        .filter(|row| row.notification_delivery_readiness_state == state)
        .count()
}
