use std::time::Duration;

use ocentra_eventing::{
    bus::subscriber::EventSubscriber, bus::EventBus, envelope::EventMetadata,
    envelope::EventSource, error::EventingError, ids::CorrelationId, ids::EventType,
    ids::RecordedAt, ids::RequestId, ids::RuntimeInstanceId, ids::SourceComponent,
    ids::SourceService, ids::SubscriberId, ids::TargetHandler, request::RequestOptions,
};
use ocentra_parent_agent_protocol::browser::social_report_writer_delivery_handoff::{
    SocialReportWriterDeliveryReadModel, SocialReportWriterDeliveryReadModelRow,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields, LogLevel};
use ocentra_parent_agent_protocol::social_parent_notification_delivery_read_model::{
    SocialParentNotificationDeliveryReadModelRequest,
    SocialParentNotificationDeliveryReadModelResponse,
    SocialParentNotificationDeliveryReadinessRow,
    SocialParentNotificationDeliveryReadinessSnapshot,
};
use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentEventEnvelope, AgentEventName,
};
use ocentra_parent_agent_protocol::SOCIAL_PARENT_NOTIFICATION_DELIVERY_CAPABILITY_READY;
use ocentra_parent_agent_protocol::SOCIAL_PARENT_NOTIFICATION_DELIVERY_EXECUTION_REPORT_READY;
use ocentra_parent_agent_protocol::SOCIAL_PARENT_NOTIFICATION_DELIVERY_NON_CLAIM_ENFORCEMENT;
use ocentra_parent_agent_protocol::SOCIAL_PARENT_NOTIFICATION_DELIVERY_NON_CLAIM_EXTERNAL_RUNTIME;
use ocentra_parent_agent_protocol::SOCIAL_PARENT_NOTIFICATION_DELIVERY_NON_CLAIM_FINAL_POLICY;
use ocentra_parent_agent_protocol::SOCIAL_PARENT_NOTIFICATION_DELIVERY_NON_CLAIM_PARENT_NOTIFICATION_UI;
use ocentra_parent_agent_protocol::SOCIAL_PARENT_NOTIFICATION_DELIVERY_NON_CLAIM_PROVIDER_DELIVERY;
use ocentra_parent_agent_protocol::SOCIAL_PARENT_NOTIFICATION_DELIVERY_NON_CLAIM_PROVIDER_RECEIPT;
use ocentra_parent_agent_protocol::SOCIAL_PARENT_NOTIFICATION_DELIVERY_READINESS_ID;
use ocentra_parent_agent_protocol::SOCIAL_PARENT_NOTIFICATION_DELIVERY_ROW_MANUAL_REQUIRED;
use ocentra_parent_agent_protocol::SOCIAL_PARENT_NOTIFICATION_DELIVERY_ROW_REPORT_READY;
use ocentra_parent_agent_protocol::SOCIAL_PARENT_NOTIFICATION_DELIVERY_ROW_UNAVAILABLE;
use ocentra_parent_agent_protocol::SOCIAL_PARENT_NOTIFICATION_DELIVERY_SCHEMA_VERSION;
use ocentra_parent_agent_protocol::SOCIAL_PARENT_NOTIFICATION_DELIVERY_STATE_MANUAL_REQUIRED;
use ocentra_parent_agent_protocol::SOCIAL_PARENT_NOTIFICATION_DELIVERY_STATE_REPORT_READY;
use ocentra_parent_agent_protocol::SOCIAL_PARENT_NOTIFICATION_DELIVERY_STATE_UNAVAILABLE;
use ocentra_parent_agent_protocol::SOCIAL_REPORT_WRITER_DELIVERY_RECEIPT_MANUAL_REQUIRED;
use ocentra_parent_agent_protocol::SOCIAL_REPORT_WRITER_DELIVERY_RECEIPT_RECORDED;
use ocentra_parent_agent_protocol::SOCIAL_REPORT_WRITER_DELIVERY_STATE_MANUAL_REQUIRED;
use ocentra_parent_agent_protocol::SOCIAL_REPORT_WRITER_DELIVERY_STATE_REPORT_READY;

use crate::{
    event_builder::build_event, fields::fields_from_pairs, json_contract::serialize_json_string,
    time::timestamp_now,
};

#[path = "social_parent_notification_delivery_read_model_payload/social_report_writer_delivery_event_handoff.rs"]
pub(crate) mod social_report_writer_delivery_event_handoff;

use self::social_report_writer_delivery_event_handoff::{
    request_social_report_writer_delivery_read_model_from_service,
    social_report_writer_delivery_read_model_from_service,
};

#[derive(Clone, Debug, PartialEq, Eq)]
struct RequestedAtText(String);

#[derive(Clone, Debug, PartialEq, Eq)]
struct CreatedAtText(String);

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct CorrelationIdText(pub(crate) String);

#[derive(Clone, Debug, PartialEq)]
struct FieldPairs(Vec<(&'static str, LogFieldValue)>);

#[derive(Clone, Debug, PartialEq, Eq)]
struct NonClaims(Vec<String>);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct StateText(&'static str);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct NotificationStates(pub(crate) (&'static str, &'static str, &'static str));

pub fn social_parent_notification_delivery_read_model_from_service(
) -> SocialParentNotificationDeliveryReadinessSnapshot {
    let report_writer_read_model = social_report_writer_delivery_read_model_from_service();
    social_parent_notification_delivery_read_model_from_report_writer(&report_writer_read_model)
}

fn social_parent_notification_delivery_read_model_from_report_writer(
    report_writer_read_model: &SocialReportWriterDeliveryReadModel,
) -> SocialParentNotificationDeliveryReadinessSnapshot {
    let generated_at: String = timestamp_now();
    let rows = report_writer_read_model
        .rows
        .iter()
        .map(|report_writer_row| {
            notification_row_from_report_writer(
                report_writer_row,
                CreatedAtText(generated_at.clone()),
            )
        })
        .collect::<Vec<_>>();
    SocialParentNotificationDeliveryReadinessSnapshot {
        schema_version: SOCIAL_PARENT_NOTIFICATION_DELIVERY_SCHEMA_VERSION.to_string(),
        readiness_id: SOCIAL_PARENT_NOTIFICATION_DELIVERY_READINESS_ID.to_string(),
        generated_at,
        source_report_writer_proof_ref: report_writer_read_model.proof_ref.clone(),
        parent_report_status_ready_count: count_rows(
            &rows,
            StateText(SOCIAL_PARENT_NOTIFICATION_DELIVERY_STATE_REPORT_READY),
        ),
        manual_required_count: count_rows(
            &rows,
            StateText(SOCIAL_PARENT_NOTIFICATION_DELIVERY_STATE_MANUAL_REQUIRED),
        ),
        unavailable_count: count_rows(
            &rows,
            StateText(SOCIAL_PARENT_NOTIFICATION_DELIVERY_STATE_UNAVAILABLE),
        ),
        rows,
        non_claims: non_claims().0,
        parent_notification_ui_delivery_claimed: false,
        external_runtime_report_delivery_claimed: report_writer_read_model
            .external_runtime_report_delivery_claimed,
        final_policy_execution_claimed: false,
        enforcement_claimed: false,
    }
}

pub async fn request_social_parent_notification_delivery_read_model_from_service(
) -> Result<SocialParentNotificationDeliveryReadinessSnapshot, EventingError> {
    let bus = EventBus::root();
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
            let report_writer_read_model =
                request_social_report_writer_delivery_read_model_from_service()
                    .await
                    .unwrap_or_else(|_| social_report_writer_delivery_read_model_from_service());
            context
                .complete_request(SocialParentNotificationDeliveryReadModelResponse {
                    read_model: social_parent_notification_delivery_read_model_from_report_writer(
                        &report_writer_read_model,
                    ),
                })
                .await?;
            Ok(())
        },
    )
    .await?;

    let requested_at: String = timestamp_now();
    let request = SocialParentNotificationDeliveryReadModelRequest {
        request_id: RequestId::parse(
            social_parent_notification_delivery_request_id(&RequestedAtText(requested_at.clone()))
                .0,
        )?,
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
    fields_from_pairs(read_model_pairs(read_model).0)
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

fn social_parent_notification_delivery_metadata(
    request: &SocialParentNotificationDeliveryReadModelRequest,
) -> Result<EventMetadata, EventingError> {
    Ok(EventMetadata::from_parts(
        ocentra_eventing::ids::EventId::generated(),
        CorrelationId::parse(
            social_parent_notification_delivery_correlation_id(&RequestedAtText(
                request.requested_at.clone(),
            ))
            .0,
        )?,
        EventSource::new(
            ocentra_eventing::ids::EventCustody::parse(
                constants::eventing_source::CUSTODY_LOCAL_QUERY_STORE,
            )?,
            ocentra_eventing::ids::RuntimeRole::parse(constants::eventing_source::ROLE_CONTROLLER)?,
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

fn social_parent_notification_delivery_request_id(
    requested_at: &RequestedAtText,
) -> RequestedAtText {
    let mut value = String::from(
        constants::browser::REQUEST_BROWSER_SOCIAL_PARENT_NOTIFICATION_DELIVERY_STATUS_PREFIX,
    );
    value.push_str(&requested_at.0);
    RequestedAtText(value)
}

fn social_parent_notification_delivery_correlation_id(
    requested_at: &RequestedAtText,
) -> CorrelationIdText {
    let mut value = String::from(constants::browser::CORRELATION_BROWSER_RUNTIME_PREFIX);
    value.push_str(&requested_at.0);
    CorrelationIdText(value)
}

fn read_model_pairs(read_model: &SocialParentNotificationDeliveryReadinessSnapshot) -> FieldPairs {
    FieldPairs(vec![
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
            LogFieldValue::String(serialize_json_string(read_model).0),
        ),
    ])
}

fn notification_row_from_report_writer(
    report_writer_row: &SocialReportWriterDeliveryReadModelRow,
    created_at: CreatedAtText,
) -> SocialParentNotificationDeliveryReadinessRow {
    let NotificationStates((row_id, readiness_state, execution_state)) =
        notification_states_from_report_writer(report_writer_row);
    SocialParentNotificationDeliveryReadinessRow {
        notification_delivery_readiness_row_id: row_id.to_string(),
        source_report_writer_delivery_row_ref: report_writer_row.row_id.clone(),
        source_intent_ref: report_writer_row.source_intent_ref.clone(),
        parent_visible_report_status_ref: report_writer_row
            .parent_visible_report_status_ref
            .clone(),
        parent_notification_ui_ref: None,
        parent_report_ref: report_writer_row.parent_report_ref.clone(),
        report_artifact_ref: report_writer_row.report_artifact_ref.clone(),
        report_receipt_ref: report_writer_row.report_receipt_ref.clone(),
        source_evidence_refs: report_writer_row.source_evidence_refs.clone(),
        source_policy_refs: report_writer_row.source_policy_refs.clone(),
        source_audit_refs: report_writer_row.source_audit_refs.clone(),
        manual_proof_requirements: report_writer_row.manual_proof_requirements.clone(),
        notification_delivery_readiness_state: readiness_state.to_string(),
        report_delivery_execution_state: execution_state.to_string(),
        parent_owned_report_artifact_written: report_writer_row
            .parent_owned_report_artifact_written,
        parent_owned_report_receipt_recorded: report_writer_row
            .parent_owned_report_receipt_recorded,
        parent_notification_ui_delivered: false,
        external_runtime_report_delivery_claimed: report_writer_row
            .external_runtime_report_delivery_claimed,
        provider_delivery_attempted: report_writer_row.provider_delivery_attempted,
        provider_receipt_ingested: report_writer_row.provider_receipt_ingested,
        final_policy_decision_claimed: report_writer_row.final_policy_decision_claimed,
        enforcement_claimed: report_writer_row.enforcement_claimed,
        created_at: created_at.0,
    }
}

fn notification_states_from_report_writer(
    report_writer_row: &SocialReportWriterDeliveryReadModelRow,
) -> NotificationStates {
    if report_writer_row.delivery_state == SOCIAL_REPORT_WRITER_DELIVERY_STATE_REPORT_READY
        && report_writer_row.receipt_state == SOCIAL_REPORT_WRITER_DELIVERY_RECEIPT_RECORDED
    {
        NotificationStates((
            SOCIAL_PARENT_NOTIFICATION_DELIVERY_ROW_REPORT_READY,
            SOCIAL_PARENT_NOTIFICATION_DELIVERY_STATE_REPORT_READY,
            SOCIAL_PARENT_NOTIFICATION_DELIVERY_EXECUTION_REPORT_READY,
        ))
    } else if report_writer_row.delivery_state
        == SOCIAL_REPORT_WRITER_DELIVERY_STATE_MANUAL_REQUIRED
        || report_writer_row.receipt_state == SOCIAL_REPORT_WRITER_DELIVERY_RECEIPT_MANUAL_REQUIRED
    {
        NotificationStates((
            SOCIAL_PARENT_NOTIFICATION_DELIVERY_ROW_MANUAL_REQUIRED,
            SOCIAL_PARENT_NOTIFICATION_DELIVERY_STATE_MANUAL_REQUIRED,
            SOCIAL_PARENT_NOTIFICATION_DELIVERY_STATE_MANUAL_REQUIRED,
        ))
    } else {
        NotificationStates((
            SOCIAL_PARENT_NOTIFICATION_DELIVERY_ROW_UNAVAILABLE,
            SOCIAL_PARENT_NOTIFICATION_DELIVERY_STATE_UNAVAILABLE,
            SOCIAL_PARENT_NOTIFICATION_DELIVERY_STATE_UNAVAILABLE,
        ))
    }
}

fn non_claims() -> NonClaims {
    NonClaims(vec![
        SOCIAL_PARENT_NOTIFICATION_DELIVERY_NON_CLAIM_PARENT_NOTIFICATION_UI.to_string(),
        SOCIAL_PARENT_NOTIFICATION_DELIVERY_NON_CLAIM_EXTERNAL_RUNTIME.to_string(),
        SOCIAL_PARENT_NOTIFICATION_DELIVERY_NON_CLAIM_PROVIDER_DELIVERY.to_string(),
        SOCIAL_PARENT_NOTIFICATION_DELIVERY_NON_CLAIM_PROVIDER_RECEIPT.to_string(),
        SOCIAL_PARENT_NOTIFICATION_DELIVERY_NON_CLAIM_FINAL_POLICY.to_string(),
        SOCIAL_PARENT_NOTIFICATION_DELIVERY_NON_CLAIM_ENFORCEMENT.to_string(),
    ])
}

fn count_rows(rows: &[SocialParentNotificationDeliveryReadinessRow], state: StateText) -> usize {
    rows.iter()
        .filter(|row| row.notification_delivery_readiness_state == state.0)
        .count()
}
