use std::time::Duration;

use ocentra_eventing::{
    bus::subscriber::EventSubscriber, bus::EventBus, envelope::EventMetadata,
    envelope::EventSource, error::EventingError, ids::CorrelationId, ids::EventType,
    ids::RecordedAt, ids::RequestId, ids::RuntimeInstanceId, ids::SourceComponent,
    ids::SourceService, ids::SubscriberId, ids::TargetHandler, request::RequestOptions,
};
use ocentra_parent_agent_protocol::browser::social_parent_surface_status_handoff::{
    SocialPreferenceStatusHandoffReadModel, SocialPreferenceStatusHandoffRow,
    SocialProviderStatusHandoffReadModel, SocialProviderStatusHandoffRow,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields, LogLevel};
use ocentra_parent_agent_protocol::social_alert_report_parent_surface_read_model::{
    SocialAlertReportParentSurfaceReadModelRequest,
    SocialAlertReportParentSurfaceReadModelResponse, SocialAlertReportParentSurfaceReadModelRow,
    SocialAlertReportParentSurfaceReadModelSnapshot,
};
use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentEventEnvelope, AgentEventName,
};
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_PARENT_SURFACE_HISTORY_UNAVAILABLE;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_PARENT_SURFACE_HISTORY_VISIBLE;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_PARENT_SURFACE_INTENT_ID;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_PARENT_SURFACE_MINIMAL_BOUNDARY;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_PARENT_SURFACE_NON_CLAIM_ADAPTER_DISPATCH;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_PARENT_SURFACE_NON_CLAIM_CHILD_DELIVERY;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_PARENT_SURFACE_NON_CLAIM_CLOUD_ROUTING;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_PARENT_SURFACE_NON_CLAIM_CONNECTOR_NATIVE;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_PARENT_SURFACE_NON_CLAIM_DURABLE_OUTBOX;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_PARENT_SURFACE_NON_CLAIM_ENFORCEMENT;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_PARENT_SURFACE_NON_CLAIM_FINAL_POLICY;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_PARENT_SURFACE_NON_CLAIM_FREQUENCY_UI;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_PARENT_SURFACE_NON_CLAIM_HISTORY_UI;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_PARENT_SURFACE_NON_CLAIM_NOTIFICATION_UI;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_PARENT_SURFACE_NON_CLAIM_PREFERENCE_UI;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_PARENT_SURFACE_NON_CLAIM_PROVIDER_CREDENTIALS;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_PARENT_SURFACE_NON_CLAIM_PROVIDER_DELIVERY;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_PARENT_SURFACE_NON_CLAIM_PROVIDER_RECEIPT;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_PARENT_SURFACE_NON_CLAIM_QUIET_HOURS;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_PARENT_SURFACE_NON_CLAIM_REPORT_DELIVERY;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_PARENT_SURFACE_NON_CLAIM_RETRY_WORKER;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_PARENT_SURFACE_PREFERENCE_DISABLED;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_PARENT_SURFACE_PREFERENCE_SETUP;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_PARENT_SURFACE_PROVIDER_ROW_HIGH_RISK_REF;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_PARENT_SURFACE_ROW_HIGH_RISK;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_PARENT_SURFACE_ROW_MANUAL;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_PARENT_SURFACE_ROW_UNAVAILABLE;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_PARENT_SURFACE_SCHEMA_VERSION;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_PARENT_SURFACE_STATE_MANUAL;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_PARENT_SURFACE_STATE_UNAVAILABLE;

use crate::{
    event_builder::build_event, fields::fields_from_pairs, json_contract::serialize_json_string,
    time::timestamp_now,
};

#[derive(Clone, Debug, Eq, PartialEq)]
struct ParentSurfaceText(String);

#[derive(Clone, Debug, Eq, PartialEq)]
struct ParentSurfaceTextList(Vec<String>);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ParentSurfaceFieldName(&'static str);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ParentSurfaceStatusSelection {
    row_id: ParentSurfaceFieldName,
    state: ParentSurfaceFieldName,
    history_visibility: ParentSurfaceFieldName,
}

impl<T> From<T> for ParentSurfaceText
where
    T: Into<String>,
{
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

impl<T> From<Vec<T>> for ParentSurfaceTextList
where
    T: Into<String>,
{
    fn from(value: Vec<T>) -> Self {
        Self(value.into_iter().map(Into::into).collect())
    }
}

#[path = "social_alert_report_parent_surface_read_model_payload/social_parent_surface_status_handoff.rs"]
mod social_parent_surface_status_handoff;

use social_parent_surface_status_handoff::{
    request_social_preference_status_handoff_from_service,
    request_social_provider_status_handoff_from_service,
    social_preference_status_handoff_from_service, social_provider_status_handoff_from_service,
};

pub fn social_alert_report_parent_surface_read_model_from_service(
) -> SocialAlertReportParentSurfaceReadModelSnapshot {
    social_alert_report_parent_surface_read_model_from_handoffs(
        &social_provider_status_handoff_from_service(),
        &social_preference_status_handoff_from_service(),
    )
}

fn social_alert_report_parent_surface_read_model_from_handoffs(
    provider_handoff: &SocialProviderStatusHandoffReadModel,
    preference_handoff: &SocialPreferenceStatusHandoffReadModel,
) -> SocialAlertReportParentSurfaceReadModelSnapshot {
    let generated_at: String = timestamp_now();
    let rows = provider_handoff
        .rows
        .iter()
        .zip(preference_handoff.rows.iter())
        .map(|(provider_row, preference_row)| parent_surface_row(provider_row, preference_row))
        .collect::<Vec<_>>();
    SocialAlertReportParentSurfaceReadModelSnapshot {
        schema_version: SOCIAL_ALERT_REPORT_PARENT_SURFACE_SCHEMA_VERSION.to_string(),
        intent_id: SOCIAL_ALERT_REPORT_PARENT_SURFACE_INTENT_ID.to_string(),
        generated_at,
        source_provider_status_handoff_id: provider_handoff.handoff_id.clone(),
        source_preference_status_handoff_id: preference_handoff.handoff_id.clone(),
        manual_action_required_count: count_rows(
            &rows,
            ParentSurfaceFieldName(SOCIAL_ALERT_REPORT_PARENT_SURFACE_STATE_MANUAL),
        ),
        unavailable_visible_count: count_rows(
            &rows,
            ParentSurfaceFieldName(SOCIAL_ALERT_REPORT_PARENT_SURFACE_STATE_UNAVAILABLE),
        ),
        history_visible_count: rows.len(),
        preference_setup_required_count: rows
            .iter()
            .filter(|row| {
                row.preference_visibility == SOCIAL_ALERT_REPORT_PARENT_SURFACE_PREFERENCE_SETUP
            })
            .count(),
        rows,
        parent_surface_non_claims: non_claims().0,
        parent_notification_ui_rendered: false,
        parent_notification_preference_ui_rendered: false,
        parent_frequency_control_ui_rendered: false,
        parent_notification_history_ui_rendered: false,
        provider_delivery_runtime_claimed: false,
        provider_receipt_ingestion_claimed: false,
        provider_credentials_claimed: false,
        cloud_routing_claimed: false,
        child_delivery_claimed: false,
        quiet_hours_timer_runtime_claimed: false,
        retry_execution_runtime_claimed: false,
        production_durable_outbox_storage_claimed: false,
        adapter_dispatch_claimed: false,
        report_delivery_execution_claimed: false,
        final_policy_execution_claimed: false,
        connector_native_runtime_claimed: false,
        enforcement_claimed: false,
    }
}

pub async fn request_social_alert_report_parent_surface_read_model_from_service(
) -> Result<SocialAlertReportParentSurfaceReadModelSnapshot, EventingError> {
    let bus = EventBus::new();
    bus.subscribe::<SocialAlertReportParentSurfaceReadModelRequest, _, _>(
        EventSubscriber::new(
            SubscriberId::parse(
                constants::browser::SUBSCRIBER_BROWSER_SOCIAL_ALERT_REPORT_PARENT_SURFACE_STATUS,
            )?,
            EventType::parse(
                constants::browser::EVENT_BROWSER_SOCIAL_ALERT_REPORT_PARENT_SURFACE_STATUS_REQUESTED,
            )?,
            TargetHandler::parse(
                constants::browser::TARGET_BROWSER_SOCIAL_ALERT_REPORT_PARENT_SURFACE_STATUS,
            )?,
        ),
        |context| async move {
            let provider_handoff = request_social_provider_status_handoff_from_service()
                .await
                .unwrap_or_else(|_| social_provider_status_handoff_from_service());
            let preference_handoff = request_social_preference_status_handoff_from_service()
                .await
                .unwrap_or_else(|_| social_preference_status_handoff_from_service());
            context
                .complete_request(SocialAlertReportParentSurfaceReadModelResponse {
                    read_model: social_alert_report_parent_surface_read_model_from_handoffs(
                        &provider_handoff,
                        &preference_handoff,
                    ),
                })
                .await?;
            Ok(())
        },
    )
    .await?;

    let requested_at: String = timestamp_now();
    let request = SocialAlertReportParentSurfaceReadModelRequest {
        request_id: RequestId::parse(parent_surface_request_id(requested_at.as_str()).0)?,
        requested_at,
    };
    let metadata = parent_surface_metadata(&request)?;
    let report = bus
        .publish_request(
            request,
            metadata,
            RequestOptions::with_timeout(Duration::from_millis(
                constants::browser::REQUEST_BROWSER_SOCIAL_ALERT_REPORT_PARENT_SURFACE_STATUS_TIMEOUT_MS,
            ))?,
        )
        .await?;

    Ok(report.response.read_model)
}

pub async fn build_browser_social_alert_report_parent_surface_read_model_report(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    let read_model = request_social_alert_report_parent_surface_read_model_from_service()
        .await
        .unwrap_or_else(|_| social_alert_report_parent_surface_read_model_from_service());
    build_event(
        constants::event_id::BROWSER_SOCIAL_ALERT_REPORT_PARENT_SURFACE_READ_MODEL_REPORTED,
        &command.message_id,
        command.source,
        AgentEventName::AgentBrowserSocialAlertReportParentSurfaceReadModelReported,
        LogLevel::Info,
        parent_surface_payload(&read_model),
        None,
    )
}

pub fn parent_surface_payload(
    read_model: &SocialAlertReportParentSurfaceReadModelSnapshot,
) -> LogFields {
    fields_from_pairs(vec![
        (
            constants::field::GENERATED_AT,
            LogFieldValue::String(read_model.generated_at.clone()),
        ),
        (
            constants::field::CAPABILITY_STATUS,
            LogFieldValue::String(SOCIAL_ALERT_REPORT_PARENT_SURFACE_SCHEMA_VERSION.to_string()),
        ),
        (
            constants::field::RETURNED,
            LogFieldValue::Number(read_model.rows.len() as f64),
        ),
        (
            constants::field::BROWSER_SOCIAL_ALERT_REPORT_PARENT_SURFACE_READ_MODEL,
            LogFieldValue::String(serialize_json_string(read_model).0),
        ),
    ])
}

fn parent_surface_metadata(
    request: &SocialAlertReportParentSurfaceReadModelRequest,
) -> Result<EventMetadata, EventingError> {
    status_handoff_metadata(
        request.requested_at.as_str(),
        ParentSurfaceFieldName(
            constants::browser::TARGET_BROWSER_SOCIAL_ALERT_REPORT_PARENT_SURFACE_STATUS,
        ),
    )
}

fn status_handoff_metadata(
    requested_at: impl Into<ParentSurfaceText>,
    target_handler: ParentSurfaceFieldName,
) -> Result<EventMetadata, EventingError> {
    let requested_at = requested_at.into();
    let correlation_id = parent_surface_correlation_id(requested_at.0.as_str());
    Ok(EventMetadata::from_parts(
        ocentra_eventing::ids::EventId::generated(),
        CorrelationId::parse(correlation_id.0.as_str())?,
        EventSource::new(
            ocentra_eventing::ids::EventCustody::parse(
                constants::eventing_source::CUSTODY_LOCAL_QUERY_STORE,
            )?,
            ocentra_eventing::ids::RuntimeRole::parse(constants::eventing_source::ROLE_CONTROLLER)?,
            SourceService::parse(constants::peer::LOCAL_DEV_AGENT)?,
            SourceComponent::parse(constants::browser::RUNTIME_COMPONENT_BROWSER_SPINE)?,
            RuntimeInstanceId::parse(constants::browser::RUNTIME_INSTANCE_LOCAL_BROWSER_RUNTIME)?,
        ),
        RecordedAt::parse(requested_at.0.as_str())?,
        Some(TargetHandler::parse(target_handler.0)?),
    ))
}

fn parent_surface_request_id(requested_at: impl Into<ParentSurfaceText>) -> ParentSurfaceText {
    let requested_at = requested_at.into();
    let mut value = String::from(
        constants::browser::REQUEST_BROWSER_SOCIAL_ALERT_REPORT_PARENT_SURFACE_STATUS_PREFIX,
    );
    value.push_str(requested_at.0.as_str());
    ParentSurfaceText(value)
}

fn parent_surface_correlation_id(requested_at: impl Into<ParentSurfaceText>) -> ParentSurfaceText {
    let requested_at = requested_at.into();
    let mut value = String::from(constants::browser::CORRELATION_BROWSER_RUNTIME_PREFIX);
    value.push_str(requested_at.0.as_str());
    ParentSurfaceText(value)
}

fn parent_surface_row(
    provider_row: &SocialProviderStatusHandoffRow,
    preference_row: &SocialPreferenceStatusHandoffRow,
) -> SocialAlertReportParentSurfaceReadModelRow {
    let ParentSurfaceStatusSelection {
        row_id,
        state,
        history_visibility,
    } = parent_surface_status(provider_row);
    let preference_visibility = parent_surface_preference_visibility(preference_row);
    SocialAlertReportParentSurfaceReadModelRow {
        surface_row_id: row_id.0.to_string(),
        source_provider_handoff_row_id: provider_row.handoff_row_id.clone(),
        source_preference_handoff_row_id: preference_row.handoff_row_id.clone(),
        source_intent_ref: provider_row.source_intent_ref.clone(),
        parent_surface_status: state.0.to_string(),
        history_visibility: history_visibility.0.to_string(),
        preference_visibility: preference_visibility.0.to_string(),
        notification_status_ref: provider_row.notification_status_ref.clone(),
        source_preference_status_ref: preference_row.source_preference_status_ref.clone(),
        drill_in_refs: vec![
            provider_row.notification_status_ref.clone(),
            preference_row.source_preference_status_ref.clone(),
        ],
        audit_refs: vec![
            provider_row.audit_ref.clone(),
            preference_row.audit_ref.clone(),
        ],
        manual_proof_requirements: vec![
            provider_row.manual_proof_requirement.clone(),
            preference_row.manual_proof_requirement.clone(),
        ],
        minimal_surface_payload_boundary: SOCIAL_ALERT_REPORT_PARENT_SURFACE_MINIMAL_BOUNDARY
            .to_string(),
        sensitive_detail_included: false,
        parent_notification_ui_rendered: false,
        parent_notification_preference_ui_rendered: false,
        parent_frequency_control_ui_rendered: false,
        parent_notification_history_ui_rendered: false,
        provider_delivery_claimed: false,
        provider_receipt_claimed: false,
        parent_preference_mutation_claimed: false,
        child_delivery_claimed: false,
        quiet_hours_timer_runtime_claimed: false,
        report_delivery_execution_claimed: false,
        final_policy_execution_claimed: false,
        adapter_dispatch_claimed: false,
        enforcement_claimed: false,
    }
}

fn parent_surface_status(
    provider_row: &SocialProviderStatusHandoffRow,
) -> ParentSurfaceStatusSelection {
    if provider_row.unavailable {
        ParentSurfaceStatusSelection {
            row_id: ParentSurfaceFieldName(SOCIAL_ALERT_REPORT_PARENT_SURFACE_ROW_UNAVAILABLE),
            state: ParentSurfaceFieldName(SOCIAL_ALERT_REPORT_PARENT_SURFACE_STATE_UNAVAILABLE),
            history_visibility: ParentSurfaceFieldName(
                SOCIAL_ALERT_REPORT_PARENT_SURFACE_HISTORY_UNAVAILABLE,
            ),
        }
    } else if provider_row.handoff_row_id
        == SOCIAL_ALERT_REPORT_PARENT_SURFACE_PROVIDER_ROW_HIGH_RISK_REF
    {
        ParentSurfaceStatusSelection {
            row_id: ParentSurfaceFieldName(SOCIAL_ALERT_REPORT_PARENT_SURFACE_ROW_HIGH_RISK),
            state: ParentSurfaceFieldName(SOCIAL_ALERT_REPORT_PARENT_SURFACE_STATE_MANUAL),
            history_visibility: ParentSurfaceFieldName(
                SOCIAL_ALERT_REPORT_PARENT_SURFACE_HISTORY_VISIBLE,
            ),
        }
    } else {
        ParentSurfaceStatusSelection {
            row_id: ParentSurfaceFieldName(SOCIAL_ALERT_REPORT_PARENT_SURFACE_ROW_MANUAL),
            state: ParentSurfaceFieldName(SOCIAL_ALERT_REPORT_PARENT_SURFACE_STATE_MANUAL),
            history_visibility: ParentSurfaceFieldName(
                SOCIAL_ALERT_REPORT_PARENT_SURFACE_HISTORY_VISIBLE,
            ),
        }
    }
}

fn parent_surface_preference_visibility(
    preference_row: &SocialPreferenceStatusHandoffRow,
) -> ParentSurfaceFieldName {
    if preference_row.preference_disabled {
        ParentSurfaceFieldName(SOCIAL_ALERT_REPORT_PARENT_SURFACE_PREFERENCE_DISABLED)
    } else {
        ParentSurfaceFieldName(SOCIAL_ALERT_REPORT_PARENT_SURFACE_PREFERENCE_SETUP)
    }
}

fn non_claims() -> ParentSurfaceTextList {
    ParentSurfaceTextList(vec![
        SOCIAL_ALERT_REPORT_PARENT_SURFACE_NON_CLAIM_NOTIFICATION_UI.to_string(),
        SOCIAL_ALERT_REPORT_PARENT_SURFACE_NON_CLAIM_PREFERENCE_UI.to_string(),
        SOCIAL_ALERT_REPORT_PARENT_SURFACE_NON_CLAIM_FREQUENCY_UI.to_string(),
        SOCIAL_ALERT_REPORT_PARENT_SURFACE_NON_CLAIM_HISTORY_UI.to_string(),
        SOCIAL_ALERT_REPORT_PARENT_SURFACE_NON_CLAIM_PROVIDER_DELIVERY.to_string(),
        SOCIAL_ALERT_REPORT_PARENT_SURFACE_NON_CLAIM_PROVIDER_RECEIPT.to_string(),
        SOCIAL_ALERT_REPORT_PARENT_SURFACE_NON_CLAIM_PROVIDER_CREDENTIALS.to_string(),
        SOCIAL_ALERT_REPORT_PARENT_SURFACE_NON_CLAIM_CLOUD_ROUTING.to_string(),
        SOCIAL_ALERT_REPORT_PARENT_SURFACE_NON_CLAIM_CHILD_DELIVERY.to_string(),
        SOCIAL_ALERT_REPORT_PARENT_SURFACE_NON_CLAIM_QUIET_HOURS.to_string(),
        SOCIAL_ALERT_REPORT_PARENT_SURFACE_NON_CLAIM_RETRY_WORKER.to_string(),
        SOCIAL_ALERT_REPORT_PARENT_SURFACE_NON_CLAIM_DURABLE_OUTBOX.to_string(),
        SOCIAL_ALERT_REPORT_PARENT_SURFACE_NON_CLAIM_ADAPTER_DISPATCH.to_string(),
        SOCIAL_ALERT_REPORT_PARENT_SURFACE_NON_CLAIM_REPORT_DELIVERY.to_string(),
        SOCIAL_ALERT_REPORT_PARENT_SURFACE_NON_CLAIM_FINAL_POLICY.to_string(),
        SOCIAL_ALERT_REPORT_PARENT_SURFACE_NON_CLAIM_CONNECTOR_NATIVE.to_string(),
        SOCIAL_ALERT_REPORT_PARENT_SURFACE_NON_CLAIM_ENFORCEMENT.to_string(),
    ])
}

fn count_rows(
    rows: &[SocialAlertReportParentSurfaceReadModelRow],
    parent_surface_state: ParentSurfaceFieldName,
) -> usize {
    rows.iter()
        .filter(|row| row.parent_surface_status == parent_surface_state.0)
        .count()
}
