use std::time::Duration;

use ocentra_eventing::{
    bus::subscriber::EventSubscriber, bus::EventBus, envelope::DomainEvent,
    envelope::EventContract, envelope::EventMetadata, envelope::EventSource, error::EventingError,
    ids::AggregateKey, ids::CorrelationId, ids::EventType, ids::IdempotencyKey, ids::RecordedAt,
    ids::RequestId, ids::RuntimeInstanceId, ids::SchemaVersion, ids::SourceComponent,
    ids::SourceService, ids::SubscriberId, ids::TargetHandler, request::EventResponseContract,
    request::RequestEvent, request::RequestOptions,
};
use ocentra_parent_agent_protocol::activity::policy_preview::PolicyPreviewReadModel;
use ocentra_parent_agent_protocol::browser_read_model::BrowserEvidenceReadModel;
use ocentra_parent_agent_protocol::constants;
use serde::{Deserialize, Serialize};

use crate::browser_runtime_stream_events::BrowserRuntimeText;
use crate::browser_runtime_stream_payload::{
    stream_browser_runtime_event_chain_for_read_model_with_policy_preview,
    BrowserRuntimeServiceStreamReport,
};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct BrowserRuntimeServiceStreamReportRequest {
    read_model: BrowserEvidenceReadModel,
    policy_preview: Option<PolicyPreviewReadModel>,
    request_id: RequestId,
}

impl DomainEvent for BrowserRuntimeServiceStreamReportRequest {
    fn contract(&self) -> Result<EventContract, EventingError> {
        Ok(EventContract::new(
            EventType::parse(constants::browser::EVENT_BROWSER_RUNTIME_STREAM_REPORT_REQUESTED)?,
            SchemaVersion::new(constants::browser::EVENT_SCHEMA_VERSION)?,
        ))
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        AggregateKey::parse(browser_runtime_stream_aggregate_key(&self.read_model))
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        let mut value =
            String::from(constants::browser::IDEMPOTENCY_BROWSER_RUNTIME_STREAM_REPORT_PREFIX);
        value.push_str(self.request_id.as_str());
        IdempotencyKey::parse(value)
    }
}

impl RequestEvent for BrowserRuntimeServiceStreamReportRequest {
    type Response = BrowserRuntimeServiceStreamReport;

    fn request_id(&self) -> Result<RequestId, EventingError> {
        Ok(self.request_id.clone())
    }
}

impl EventResponseContract for BrowserRuntimeServiceStreamReport {}

pub(crate) async fn request_browser_runtime_service_stream_report(
    read_model: BrowserEvidenceReadModel,
    policy_preview: Option<PolicyPreviewReadModel>,
) -> Result<BrowserRuntimeServiceStreamReport, EventingError> {
    let bus = EventBus::root();
    bus.subscribe::<BrowserRuntimeServiceStreamReportRequest, _, _>(
        EventSubscriber::new(
            SubscriberId::parse(constants::browser::SUBSCRIBER_BROWSER_RUNTIME_STREAM_REPORT)?,
            EventType::parse(constants::browser::EVENT_BROWSER_RUNTIME_STREAM_REPORT_REQUESTED)?,
            TargetHandler::parse(constants::browser::TARGET_BROWSER_RUNTIME_STREAM_REPORT)?,
        ),
        |context| async move {
            let response = stream_browser_runtime_event_chain_for_read_model_with_policy_preview(
                &context.payload().read_model,
                context.payload().policy_preview.as_ref(),
            )
            .await;
            context.complete_request(response).await?;
            Ok(())
        },
    )
    .await?;

    let request = BrowserRuntimeServiceStreamReportRequest {
        request_id: RequestId::parse(browser_runtime_stream_request_id(&read_model))?,
        read_model,
        policy_preview,
    };
    let metadata = browser_runtime_stream_metadata(&request.read_model)?;
    let request_report = bus
        .publish_request(
            request,
            metadata,
            RequestOptions::with_timeout(Duration::from_millis(
                constants::browser::REQUEST_BROWSER_RUNTIME_STREAM_REPORT_TIMEOUT_MS,
            ))?,
        )
        .await?;

    Ok(request_report.response)
}

fn browser_runtime_stream_metadata(
    read_model: &BrowserEvidenceReadModel,
) -> Result<EventMetadata, EventingError> {
    Ok(EventMetadata::from_parts(
        ocentra_eventing::ids::EventId::generated(),
        CorrelationId::parse(browser_runtime_stream_correlation_id(read_model))?,
        EventSource::new(
            ocentra_eventing::ids::EventCustody::parse(
                constants::eventing_source::CUSTODY_LOCAL_QUERY_STORE,
            )?,
            ocentra_eventing::ids::RuntimeRole::parse(constants::eventing_source::ROLE_CONTROLLER)?,
            SourceService::parse(constants::peer::LOCAL_DEV_AGENT)?,
            SourceComponent::parse(constants::browser::RUNTIME_COMPONENT_BROWSER_SPINE)?,
            RuntimeInstanceId::parse(constants::browser::RUNTIME_INSTANCE_LOCAL_BROWSER_RUNTIME)?,
        ),
        RecordedAt::parse(&read_model.generated_at)?,
        Some(TargetHandler::parse(
            constants::browser::TARGET_BROWSER_RUNTIME_STREAM_REPORT,
        )?),
    ))
}

fn browser_runtime_stream_request_id(read_model: &BrowserEvidenceReadModel) -> BrowserRuntimeText {
    let mut value = String::from(constants::browser::REQUEST_BROWSER_RUNTIME_STREAM_REPORT_PREFIX);
    value.push_str(&browser_runtime_stream_read_model_ref(read_model).0);
    BrowserRuntimeText(value)
}

fn browser_runtime_stream_correlation_id(
    read_model: &BrowserEvidenceReadModel,
) -> BrowserRuntimeText {
    let mut value = String::from(constants::browser::CORRELATION_BROWSER_RUNTIME_PREFIX);
    value.push_str(&browser_runtime_stream_read_model_ref(read_model).0);
    BrowserRuntimeText(value)
}

fn browser_runtime_stream_aggregate_key(
    read_model: &BrowserEvidenceReadModel,
) -> BrowserRuntimeText {
    let mut value = String::from(constants::browser::AGGREGATE_BROWSER_RUNTIME_PREFIX);
    value.push_str(&browser_runtime_stream_read_model_ref(read_model).0);
    BrowserRuntimeText(value)
}

fn browser_runtime_stream_read_model_ref(
    read_model: &BrowserEvidenceReadModel,
) -> BrowserRuntimeText {
    BrowserRuntimeText(
        read_model
            .latest_event_id
            .clone()
            .unwrap_or_else(|| read_model.generated_at.clone()),
    )
}
