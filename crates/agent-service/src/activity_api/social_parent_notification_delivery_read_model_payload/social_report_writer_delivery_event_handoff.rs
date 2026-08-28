use std::time::Duration;

use ocentra_eventing::{
    bus::subscriber::EventSubscriber, bus::EventBus, envelope::EventMetadata,
    envelope::EventSource, error::EventingError, ids::CorrelationId, ids::EventType,
    ids::RecordedAt, ids::RequestId, ids::RuntimeInstanceId, ids::SourceComponent,
    ids::SourceService, ids::SubscriberId, ids::TargetHandler, request::RequestOptions,
};
use ocentra_parent_agent_protocol::browser::social_report_writer_delivery_handoff::{
    SocialReportWriterDeliveryReadModel, SocialReportWriterDeliveryReadModelRequest,
    SocialReportWriterDeliveryReadModelResponse, SocialReportWriterDeliveryReadModelRow,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::SOCIAL_PARENT_NOTIFICATION_DELIVERY_AUDIT_REF;
use ocentra_parent_agent_protocol::SOCIAL_PARENT_NOTIFICATION_DELIVERY_EVIDENCE_REF;
use ocentra_parent_agent_protocol::SOCIAL_PARENT_NOTIFICATION_DELIVERY_EXTERNAL_RUNTIME_UNAVAILABLE;
use ocentra_parent_agent_protocol::SOCIAL_PARENT_NOTIFICATION_DELIVERY_MANUAL_UI_PROOF_REQUIRED;
use ocentra_parent_agent_protocol::SOCIAL_PARENT_NOTIFICATION_DELIVERY_POLICY_REF;
use ocentra_parent_agent_protocol::SOCIAL_PARENT_NOTIFICATION_DELIVERY_SOURCE_REPORT_WRITER_PROOF_REF;
use ocentra_parent_agent_protocol::SOCIAL_REPORT_WRITER_DELIVERY_PARENT_REPORT_REF;
use ocentra_parent_agent_protocol::SOCIAL_REPORT_WRITER_DELIVERY_PARENT_VISIBLE_MANUAL_REQUIRED_REF;
use ocentra_parent_agent_protocol::SOCIAL_REPORT_WRITER_DELIVERY_PARENT_VISIBLE_REPORT_STATUS_REF;
use ocentra_parent_agent_protocol::SOCIAL_REPORT_WRITER_DELIVERY_RECEIPT_MANUAL_REQUIRED;
use ocentra_parent_agent_protocol::SOCIAL_REPORT_WRITER_DELIVERY_RECEIPT_NOT_RECORDED;
use ocentra_parent_agent_protocol::SOCIAL_REPORT_WRITER_DELIVERY_RECEIPT_RECORDED;
use ocentra_parent_agent_protocol::SOCIAL_REPORT_WRITER_DELIVERY_REPORT_ARTIFACT_REF;
use ocentra_parent_agent_protocol::SOCIAL_REPORT_WRITER_DELIVERY_REPORT_RECEIPT_REF;
use ocentra_parent_agent_protocol::SOCIAL_REPORT_WRITER_DELIVERY_ROW_MANUAL_REQUIRED;
use ocentra_parent_agent_protocol::SOCIAL_REPORT_WRITER_DELIVERY_ROW_REPORT_READY;
use ocentra_parent_agent_protocol::SOCIAL_REPORT_WRITER_DELIVERY_ROW_UNAVAILABLE;
use ocentra_parent_agent_protocol::SOCIAL_REPORT_WRITER_DELIVERY_SOURCE_INTENT_REF;
use ocentra_parent_agent_protocol::SOCIAL_REPORT_WRITER_DELIVERY_STATE_MANUAL_REQUIRED;
use ocentra_parent_agent_protocol::SOCIAL_REPORT_WRITER_DELIVERY_STATE_REPORT_READY;
use ocentra_parent_agent_protocol::SOCIAL_REPORT_WRITER_DELIVERY_STATE_UNAVAILABLE;

use crate::time::timestamp_now;

#[derive(Clone, Debug, PartialEq, Eq)]
struct RequestedAtText(String);

#[derive(Clone, Debug, PartialEq, Eq)]
struct CreatedAtText(String);

#[derive(Clone, Debug, PartialEq, Eq)]
struct RequestIdText(String);

#[derive(Clone, Debug, PartialEq, Eq)]
struct CorrelationIdText(String);

pub fn social_report_writer_delivery_read_model_from_service() -> SocialReportWriterDeliveryReadModel
{
    let generated_at: String = timestamp_now();
    SocialReportWriterDeliveryReadModel {
        generated_at: generated_at.clone(),
        proof_ref: SOCIAL_PARENT_NOTIFICATION_DELIVERY_SOURCE_REPORT_WRITER_PROOF_REF.to_string(),
        rows: vec![
            report_writer_ready_row(CreatedAtText(generated_at.clone())),
            report_writer_manual_required_row(CreatedAtText(generated_at.clone())),
            report_writer_unavailable_row(CreatedAtText(generated_at)),
        ],
        non_claims: super::non_claims().0,
        external_runtime_report_delivery_claimed: false,
        final_policy_execution_claimed: false,
        enforcement_claimed: false,
    }
}

pub async fn request_social_report_writer_delivery_read_model_from_service(
) -> Result<SocialReportWriterDeliveryReadModel, EventingError> {
    let bus = EventBus::root();
    bus.subscribe::<SocialReportWriterDeliveryReadModelRequest, _, _>(
        EventSubscriber::new(
            SubscriberId::parse(
                constants::browser::SUBSCRIBER_BROWSER_SOCIAL_REPORT_WRITER_DELIVERY_STATUS,
            )?,
            EventType::parse(
                constants::browser::EVENT_BROWSER_SOCIAL_REPORT_WRITER_DELIVERY_STATUS_REQUESTED,
            )?,
            TargetHandler::parse(
                constants::browser::TARGET_BROWSER_SOCIAL_REPORT_WRITER_DELIVERY_STATUS,
            )?,
        ),
        |context| async move {
            context
                .complete_request(SocialReportWriterDeliveryReadModelResponse {
                    read_model: social_report_writer_delivery_read_model_from_service(),
                })
                .await?;
            Ok(())
        },
    )
    .await?;

    let requested_at: String = timestamp_now();
    let request = SocialReportWriterDeliveryReadModelRequest {
        request_id: RequestId::parse(
            social_report_writer_delivery_request_id(&RequestedAtText(requested_at.clone())).0,
        )?,
        requested_at,
    };
    let metadata = social_report_writer_delivery_metadata(&request)?;
    let report = bus
        .publish_request(
            request,
            metadata,
            RequestOptions::with_timeout(Duration::from_millis(
                constants::browser::REQUEST_BROWSER_SOCIAL_REPORT_WRITER_DELIVERY_STATUS_TIMEOUT_MS,
            ))?,
        )
        .await?;

    Ok(report.response.read_model)
}

fn social_report_writer_delivery_metadata(
    request: &SocialReportWriterDeliveryReadModelRequest,
) -> Result<EventMetadata, EventingError> {
    Ok(EventMetadata::from_parts(
        ocentra_eventing::ids::EventId::generated(),
        CorrelationId::parse(
            social_report_writer_delivery_correlation_id(&RequestedAtText(
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
            constants::browser::TARGET_BROWSER_SOCIAL_REPORT_WRITER_DELIVERY_STATUS,
        )?),
    ))
}

fn social_report_writer_delivery_request_id(requested_at: &RequestedAtText) -> RequestIdText {
    let mut value = String::from(
        constants::browser::REQUEST_BROWSER_SOCIAL_REPORT_WRITER_DELIVERY_STATUS_PREFIX,
    );
    value.push_str(&requested_at.0);
    RequestIdText(value)
}

fn social_report_writer_delivery_correlation_id(
    requested_at: &RequestedAtText,
) -> CorrelationIdText {
    let mut value = String::from(constants::browser::CORRELATION_BROWSER_RUNTIME_PREFIX);
    value.push_str(&requested_at.0);
    CorrelationIdText(value)
}

fn report_writer_ready_row(created_at: CreatedAtText) -> SocialReportWriterDeliveryReadModelRow {
    report_writer_row(SocialReportWriterDeliveryRowParts {
        row_id: SOCIAL_REPORT_WRITER_DELIVERY_ROW_REPORT_READY,
        parent_visible_report_status_ref: Some(
            SOCIAL_REPORT_WRITER_DELIVERY_PARENT_VISIBLE_REPORT_STATUS_REF.to_string(),
        ),
        parent_report_ref: Some(SOCIAL_REPORT_WRITER_DELIVERY_PARENT_REPORT_REF.to_string()),
        report_artifact_ref: Some(SOCIAL_REPORT_WRITER_DELIVERY_REPORT_ARTIFACT_REF.to_string()),
        report_receipt_ref: Some(SOCIAL_REPORT_WRITER_DELIVERY_REPORT_RECEIPT_REF.to_string()),
        manual_proof_requirements: Vec::new(),
        delivery_state: SOCIAL_REPORT_WRITER_DELIVERY_STATE_REPORT_READY,
        receipt_state: SOCIAL_REPORT_WRITER_DELIVERY_RECEIPT_RECORDED,
        parent_owned_report_artifact_written: true,
        parent_owned_report_receipt_recorded: true,
        created_at,
    })
}

fn report_writer_manual_required_row(
    created_at: CreatedAtText,
) -> SocialReportWriterDeliveryReadModelRow {
    report_writer_row(SocialReportWriterDeliveryRowParts {
        row_id: SOCIAL_REPORT_WRITER_DELIVERY_ROW_MANUAL_REQUIRED,
        parent_visible_report_status_ref: Some(
            SOCIAL_REPORT_WRITER_DELIVERY_PARENT_VISIBLE_MANUAL_REQUIRED_REF.to_string(),
        ),
        parent_report_ref: None,
        report_artifact_ref: None,
        report_receipt_ref: None,
        manual_proof_requirements: vec![
            SOCIAL_PARENT_NOTIFICATION_DELIVERY_MANUAL_UI_PROOF_REQUIRED.to_string(),
        ],
        delivery_state: SOCIAL_REPORT_WRITER_DELIVERY_STATE_MANUAL_REQUIRED,
        receipt_state: SOCIAL_REPORT_WRITER_DELIVERY_RECEIPT_MANUAL_REQUIRED,
        parent_owned_report_artifact_written: false,
        parent_owned_report_receipt_recorded: false,
        created_at,
    })
}

fn report_writer_unavailable_row(
    created_at: CreatedAtText,
) -> SocialReportWriterDeliveryReadModelRow {
    report_writer_row(SocialReportWriterDeliveryRowParts {
        row_id: SOCIAL_REPORT_WRITER_DELIVERY_ROW_UNAVAILABLE,
        parent_visible_report_status_ref: None,
        parent_report_ref: None,
        report_artifact_ref: None,
        report_receipt_ref: None,
        manual_proof_requirements: vec![
            SOCIAL_PARENT_NOTIFICATION_DELIVERY_EXTERNAL_RUNTIME_UNAVAILABLE.to_string(),
        ],
        delivery_state: SOCIAL_REPORT_WRITER_DELIVERY_STATE_UNAVAILABLE,
        receipt_state: SOCIAL_REPORT_WRITER_DELIVERY_RECEIPT_NOT_RECORDED,
        parent_owned_report_artifact_written: false,
        parent_owned_report_receipt_recorded: false,
        created_at,
    })
}

struct SocialReportWriterDeliveryRowParts {
    row_id: &'static str,
    parent_visible_report_status_ref: Option<String>,
    parent_report_ref: Option<String>,
    report_artifact_ref: Option<String>,
    report_receipt_ref: Option<String>,
    manual_proof_requirements: Vec<String>,
    delivery_state: &'static str,
    receipt_state: &'static str,
    parent_owned_report_artifact_written: bool,
    parent_owned_report_receipt_recorded: bool,
    created_at: CreatedAtText,
}

fn report_writer_row(
    parts: SocialReportWriterDeliveryRowParts,
) -> SocialReportWriterDeliveryReadModelRow {
    SocialReportWriterDeliveryReadModelRow {
        row_id: parts.row_id.to_string(),
        source_intent_ref: SOCIAL_REPORT_WRITER_DELIVERY_SOURCE_INTENT_REF.to_string(),
        parent_visible_report_status_ref: parts.parent_visible_report_status_ref,
        parent_report_ref: parts.parent_report_ref,
        report_artifact_ref: parts.report_artifact_ref,
        report_receipt_ref: parts.report_receipt_ref,
        source_evidence_refs: vec![SOCIAL_PARENT_NOTIFICATION_DELIVERY_EVIDENCE_REF.to_string()],
        source_policy_refs: vec![SOCIAL_PARENT_NOTIFICATION_DELIVERY_POLICY_REF.to_string()],
        source_audit_refs: vec![SOCIAL_PARENT_NOTIFICATION_DELIVERY_AUDIT_REF.to_string()],
        manual_proof_requirements: parts.manual_proof_requirements,
        delivery_state: parts.delivery_state.to_string(),
        receipt_state: parts.receipt_state.to_string(),
        parent_owned_report_artifact_written: parts.parent_owned_report_artifact_written,
        parent_owned_report_receipt_recorded: parts.parent_owned_report_receipt_recorded,
        external_runtime_report_delivery_claimed: false,
        provider_delivery_attempted: false,
        provider_receipt_ingested: false,
        final_policy_decision_claimed: false,
        enforcement_claimed: false,
        created_at: parts.created_at.0,
    }
}
