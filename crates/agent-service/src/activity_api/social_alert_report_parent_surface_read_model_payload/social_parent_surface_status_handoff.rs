use std::time::Duration;

use ocentra_eventing::{
    bus::subscriber::EventSubscriber, bus::EventBus, envelope::EventMetadata,
    envelope::EventSource, error::EventingError, ids::CorrelationId, ids::EventType,
    ids::RecordedAt, ids::RequestId, ids::RuntimeInstanceId, ids::SourceComponent,
    ids::SourceService, ids::SubscriberId, ids::TargetHandler, request::RequestOptions,
};
use ocentra_parent_agent_protocol::browser::social_parent_surface_status_handoff::{
    SocialPreferenceStatusHandoffReadModel, SocialPreferenceStatusHandoffRequest,
    SocialPreferenceStatusHandoffResponse, SocialPreferenceStatusHandoffRow,
    SocialProviderStatusHandoffReadModel, SocialProviderStatusHandoffRequest,
    SocialProviderStatusHandoffResponse, SocialProviderStatusHandoffRow,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_PARENT_SURFACE_AUDIT_HIGH_RISK_REF;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_PARENT_SURFACE_AUDIT_MANUAL_REF;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_PARENT_SURFACE_AUDIT_UNAVAILABLE_REF;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_PARENT_SURFACE_MANUAL_ACTION_PROOF_REF;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_PARENT_SURFACE_MANUAL_HIGH_RISK_PROOF_REF;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_PARENT_SURFACE_MANUAL_UNAVAILABLE_PROOF_REF;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_PARENT_SURFACE_NOTIFICATION_STATUS_HIGH_RISK_REF;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_PARENT_SURFACE_NOTIFICATION_STATUS_MANUAL_REF;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_PARENT_SURFACE_NOTIFICATION_STATUS_UNAVAILABLE_REF;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_PARENT_SURFACE_PREFERENCE_HANDOFF_ID;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_PARENT_SURFACE_PREFERENCE_ROW_HIGH_RISK_REF;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_PARENT_SURFACE_PREFERENCE_ROW_MANUAL_REF;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_PARENT_SURFACE_PREFERENCE_ROW_UNAVAILABLE_REF;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_PARENT_SURFACE_PREFERENCE_STATUS_HIGH_RISK_REF;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_PARENT_SURFACE_PREFERENCE_STATUS_MANUAL_REF;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_PARENT_SURFACE_PREFERENCE_STATUS_UNAVAILABLE_REF;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_PARENT_SURFACE_PROVIDER_HANDOFF_ID;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_PARENT_SURFACE_PROVIDER_ROW_HIGH_RISK_REF;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_PARENT_SURFACE_PROVIDER_ROW_MANUAL_REF;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_PARENT_SURFACE_PROVIDER_ROW_UNAVAILABLE_REF;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_PARENT_SURFACE_SOURCE_INTENT_HIGH_RISK_REF;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_PARENT_SURFACE_SOURCE_INTENT_MANUAL_REF;
use ocentra_parent_agent_protocol::SOCIAL_ALERT_REPORT_PARENT_SURFACE_SOURCE_INTENT_UNAVAILABLE_REF;

use crate::time::timestamp_now;

pub async fn request_social_provider_status_handoff_from_service(
) -> Result<SocialProviderStatusHandoffReadModel, EventingError> {
    let bus = EventBus::new();
    bus.subscribe::<SocialProviderStatusHandoffRequest, _, _>(
        EventSubscriber::new(
            SubscriberId::parse(
                constants::browser::SUBSCRIBER_BROWSER_SOCIAL_PROVIDER_RECEIPT_STATUS,
            )?,
            EventType::parse(
                constants::browser::EVENT_BROWSER_SOCIAL_PROVIDER_RECEIPT_STATUS_REQUESTED,
            )?,
            TargetHandler::parse(
                constants::browser::TARGET_BROWSER_SOCIAL_PROVIDER_RECEIPT_STATUS,
            )?,
        ),
        |context| async move {
            context
                .complete_request(SocialProviderStatusHandoffResponse {
                    read_model: social_provider_status_handoff_from_service(),
                })
                .await?;
            Ok(())
        },
    )
    .await?;

    let requested_at = timestamp_now();
    let request = SocialProviderStatusHandoffRequest {
        request_id: RequestId::parse(provider_status_request_id(&requested_at))?,
        requested_at,
    };
    let metadata = status_handoff_metadata(
        &request.requested_at,
        constants::browser::TARGET_BROWSER_SOCIAL_PROVIDER_RECEIPT_STATUS,
    )?;
    let report = bus
        .publish_request(
            request,
            metadata,
            RequestOptions::with_timeout(Duration::from_millis(
                constants::browser::REQUEST_BROWSER_SOCIAL_PROVIDER_RECEIPT_STATUS_TIMEOUT_MS,
            ))?,
        )
        .await?;
    Ok(report.response.read_model)
}

pub async fn request_social_preference_status_handoff_from_service(
) -> Result<SocialPreferenceStatusHandoffReadModel, EventingError> {
    let bus = EventBus::new();
    bus.subscribe::<SocialPreferenceStatusHandoffRequest, _, _>(
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
                .complete_request(SocialPreferenceStatusHandoffResponse {
                    read_model: social_preference_status_handoff_from_service(),
                })
                .await?;
            Ok(())
        },
    )
    .await?;

    let requested_at = timestamp_now();
    let request = SocialPreferenceStatusHandoffRequest {
        request_id: RequestId::parse(preference_status_request_id(&requested_at))?,
        requested_at,
    };
    let metadata = status_handoff_metadata(
        &request.requested_at,
        constants::browser::TARGET_BROWSER_SOCIAL_REPORT_WRITER_DELIVERY_STATUS,
    )?;
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

pub fn social_provider_status_handoff_from_service() -> SocialProviderStatusHandoffReadModel {
    SocialProviderStatusHandoffReadModel {
        handoff_id: SOCIAL_ALERT_REPORT_PARENT_SURFACE_PROVIDER_HANDOFF_ID.to_string(),
        rows: vec![
            provider_status_row(
                SOCIAL_ALERT_REPORT_PARENT_SURFACE_PROVIDER_ROW_HIGH_RISK_REF,
                SOCIAL_ALERT_REPORT_PARENT_SURFACE_SOURCE_INTENT_HIGH_RISK_REF,
                SOCIAL_ALERT_REPORT_PARENT_SURFACE_NOTIFICATION_STATUS_HIGH_RISK_REF,
                SOCIAL_ALERT_REPORT_PARENT_SURFACE_AUDIT_HIGH_RISK_REF,
                SOCIAL_ALERT_REPORT_PARENT_SURFACE_MANUAL_HIGH_RISK_PROOF_REF,
                false,
            ),
            provider_status_row(
                SOCIAL_ALERT_REPORT_PARENT_SURFACE_PROVIDER_ROW_MANUAL_REF,
                SOCIAL_ALERT_REPORT_PARENT_SURFACE_SOURCE_INTENT_MANUAL_REF,
                SOCIAL_ALERT_REPORT_PARENT_SURFACE_NOTIFICATION_STATUS_MANUAL_REF,
                SOCIAL_ALERT_REPORT_PARENT_SURFACE_AUDIT_MANUAL_REF,
                SOCIAL_ALERT_REPORT_PARENT_SURFACE_MANUAL_ACTION_PROOF_REF,
                false,
            ),
            provider_status_row(
                SOCIAL_ALERT_REPORT_PARENT_SURFACE_PROVIDER_ROW_UNAVAILABLE_REF,
                SOCIAL_ALERT_REPORT_PARENT_SURFACE_SOURCE_INTENT_UNAVAILABLE_REF,
                SOCIAL_ALERT_REPORT_PARENT_SURFACE_NOTIFICATION_STATUS_UNAVAILABLE_REF,
                SOCIAL_ALERT_REPORT_PARENT_SURFACE_AUDIT_UNAVAILABLE_REF,
                SOCIAL_ALERT_REPORT_PARENT_SURFACE_MANUAL_UNAVAILABLE_PROOF_REF,
                true,
            ),
        ],
    }
}

pub fn social_preference_status_handoff_from_service() -> SocialPreferenceStatusHandoffReadModel {
    SocialPreferenceStatusHandoffReadModel {
        handoff_id: SOCIAL_ALERT_REPORT_PARENT_SURFACE_PREFERENCE_HANDOFF_ID.to_string(),
        rows: vec![
            preference_status_row(
                SOCIAL_ALERT_REPORT_PARENT_SURFACE_PREFERENCE_ROW_HIGH_RISK_REF,
                SOCIAL_ALERT_REPORT_PARENT_SURFACE_PREFERENCE_STATUS_HIGH_RISK_REF,
                SOCIAL_ALERT_REPORT_PARENT_SURFACE_AUDIT_HIGH_RISK_REF,
                SOCIAL_ALERT_REPORT_PARENT_SURFACE_MANUAL_HIGH_RISK_PROOF_REF,
                false,
            ),
            preference_status_row(
                SOCIAL_ALERT_REPORT_PARENT_SURFACE_PREFERENCE_ROW_MANUAL_REF,
                SOCIAL_ALERT_REPORT_PARENT_SURFACE_PREFERENCE_STATUS_MANUAL_REF,
                SOCIAL_ALERT_REPORT_PARENT_SURFACE_AUDIT_MANUAL_REF,
                SOCIAL_ALERT_REPORT_PARENT_SURFACE_MANUAL_ACTION_PROOF_REF,
                false,
            ),
            preference_status_row(
                SOCIAL_ALERT_REPORT_PARENT_SURFACE_PREFERENCE_ROW_UNAVAILABLE_REF,
                SOCIAL_ALERT_REPORT_PARENT_SURFACE_PREFERENCE_STATUS_UNAVAILABLE_REF,
                SOCIAL_ALERT_REPORT_PARENT_SURFACE_AUDIT_UNAVAILABLE_REF,
                SOCIAL_ALERT_REPORT_PARENT_SURFACE_MANUAL_UNAVAILABLE_PROOF_REF,
                true,
            ),
        ],
    }
}

fn provider_status_row(
    handoff_row_id: &str,
    source_intent_ref: &str,
    notification_status_ref: &str,
    audit_ref: &str,
    manual_proof_requirement: &str,
    unavailable: bool,
) -> SocialProviderStatusHandoffRow {
    SocialProviderStatusHandoffRow {
        handoff_row_id: handoff_row_id.to_string(),
        source_intent_ref: source_intent_ref.to_string(),
        notification_status_ref: notification_status_ref.to_string(),
        audit_ref: audit_ref.to_string(),
        manual_proof_requirement: manual_proof_requirement.to_string(),
        unavailable,
    }
}

fn preference_status_row(
    handoff_row_id: &str,
    source_preference_status_ref: &str,
    audit_ref: &str,
    manual_proof_requirement: &str,
    preference_disabled: bool,
) -> SocialPreferenceStatusHandoffRow {
    SocialPreferenceStatusHandoffRow {
        handoff_row_id: handoff_row_id.to_string(),
        source_preference_status_ref: source_preference_status_ref.to_string(),
        audit_ref: audit_ref.to_string(),
        manual_proof_requirement: manual_proof_requirement.to_string(),
        preference_disabled,
    }
}

fn status_handoff_metadata(
    requested_at: &str,
    target_handler: &str,
) -> Result<EventMetadata, EventingError> {
    Ok(EventMetadata::from_parts(
        ocentra_eventing::ids::EventId::generated(),
        CorrelationId::parse(status_handoff_correlation_id(requested_at))?,
        EventSource::new(
            ocentra_eventing::ids::EventCustody::parse(
                constants::eventing_source::CUSTODY_LOCAL_QUERY_STORE,
            )?,
            ocentra_eventing::ids::RuntimeRole::parse(constants::eventing_source::ROLE_CONTROLLER)?,
            SourceService::parse(constants::peer::LOCAL_DEV_AGENT)?,
            SourceComponent::parse(constants::browser::RUNTIME_COMPONENT_BROWSER_SPINE)?,
            RuntimeInstanceId::parse(constants::browser::RUNTIME_INSTANCE_LOCAL_BROWSER_RUNTIME)?,
        ),
        RecordedAt::parse(requested_at)?,
        Some(TargetHandler::parse(target_handler)?),
    ))
}

fn provider_status_request_id(requested_at: &str) -> String {
    let mut value =
        String::from(constants::browser::REQUEST_BROWSER_SOCIAL_PROVIDER_RECEIPT_STATUS_PREFIX);
    value.push_str(requested_at);
    value
}

fn preference_status_request_id(requested_at: &str) -> String {
    let mut value = String::from(
        constants::browser::REQUEST_BROWSER_SOCIAL_REPORT_WRITER_DELIVERY_STATUS_PREFIX,
    );
    value.push_str(requested_at);
    value
}

fn status_handoff_correlation_id(requested_at: &str) -> String {
    let mut value = String::from(constants::browser::CORRELATION_BROWSER_RUNTIME_PREFIX);
    value.push_str(requested_at);
    value
}
