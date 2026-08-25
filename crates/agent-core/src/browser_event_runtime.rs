use ocentra_eventing::bus::reports::dead_letter::DeadLetter;
use ocentra_eventing::bus::reports::handler::PublishReport;
use ocentra_eventing::{
    bus::publisher::RootEventPublisher, bus::EventBus, envelope::EventMetadata,
    envelope::EventSource, error::EventingError, ids::CorrelationId, ids::EventCustody,
    ids::EventId, ids::RecordedAt, ids::RuntimeInstanceId, ids::SourceComponent,
    ids::SourceService, ids::TargetHandler, topology::EventTopologyManifest,
};
use ocentra_parent_agent_protocol::browser::{
    BrowserRuntimeEventPayload as ProtocolBrowserRuntimeEventPayload, BrowserRuntimePhase,
};
use ocentra_parent_agent_protocol::constants;

#[path = "browser_event_runtime/helpers.rs"]
mod helpers;

pub mod action_handoff;
pub mod action_handoff_child_status;
pub mod action_handoff_child_status_types;
pub mod action_handoff_durable;
pub mod action_handoff_durable_types;
pub mod action_status;
pub mod delivery;
pub mod social_provider_receipt;
pub mod social_provider_receipt_durable;
pub mod social_provider_receipt_durable_types;
pub mod topology;

use crate::browser_event_runtime_refs::previous_phase_ref;

pub type BrowserRuntimeActionIntentHandoffReport =
    action_handoff::BrowserRuntimeActionIntentHandoffReport;
pub type BrowserRuntimeActionIntentHandoffResponse =
    action_handoff::BrowserRuntimeActionIntentHandoffResponse;
pub type BrowserRuntimeActionIntentStatusReport =
    action_status::BrowserRuntimeActionIntentStatusReport;
pub type BrowserRuntimeActionIntentStatusResponse =
    action_status::BrowserRuntimeActionIntentStatusResponse;
pub type BrowserRuntimeDeliveryDecisionError = delivery::BrowserRuntimeDeliveryDecisionError;
pub type BrowserRuntimeDeliveryDecisionReport = delivery::BrowserRuntimeDeliveryDecisionReport;
pub type BrowserRuntimeSocialProviderReceiptStatusReport =
    social_provider_receipt::BrowserRuntimeSocialProviderReceiptStatusReport;
pub type BrowserRuntimeSocialProviderReceiptStatusResponse =
    social_provider_receipt::BrowserRuntimeSocialProviderReceiptStatusResponse;

pub async fn request_browser_runtime_action_intent_handoff_for_input(
    input: BrowserRuntimeInput,
) -> Result<BrowserRuntimeActionIntentHandoffReport, EventingError> {
    action_handoff::request_browser_runtime_action_intent_handoff_for_input(input).await
}

pub fn browser_runtime_action_intent_handoff_topology_manifest(
) -> Result<EventTopologyManifest, EventingError> {
    action_handoff::browser_runtime_action_intent_handoff_topology_manifest()
}

pub async fn request_browser_runtime_action_intent_status_for_input(
    input: BrowserRuntimeInput,
) -> Result<BrowserRuntimeActionIntentStatusReport, EventingError> {
    action_status::request_browser_runtime_action_intent_status_for_input(input).await
}

pub fn browser_runtime_action_intent_status_topology_manifest(
) -> Result<EventTopologyManifest, EventingError> {
    action_status::browser_runtime_action_intent_status_topology_manifest()
}

pub fn prove_browser_runtime_delivery_decision(
) -> Result<BrowserRuntimeDeliveryDecisionReport, BrowserRuntimeDeliveryDecisionError> {
    delivery::prove_browser_runtime_delivery_decision()
}

pub async fn request_browser_runtime_social_provider_receipt_status_for_input(
    input: BrowserRuntimeInput,
) -> Result<BrowserRuntimeSocialProviderReceiptStatusReport, EventingError> {
    social_provider_receipt::request_browser_runtime_social_provider_receipt_status_for_input(input)
        .await
}

pub fn browser_runtime_social_provider_receipt_status_topology_manifest(
) -> Result<EventTopologyManifest, EventingError> {
    social_provider_receipt::browser_runtime_social_provider_receipt_status_topology_manifest()
}

pub fn browser_runtime_chain_topology_manifest() -> Result<EventTopologyManifest, EventingError> {
    topology::browser_runtime_chain_topology_manifest()
}

pub fn browser_runtime_stream_report_topology_manifest(
) -> Result<EventTopologyManifest, EventingError> {
    topology::browser_runtime_stream_report_topology_manifest()
}

pub type BrowserRuntimeEventPayload = ProtocolBrowserRuntimeEventPayload;

#[derive(Clone, Debug, PartialEq)]
pub struct BrowserRuntimeInput {
    pub source_ref: String,
    pub evidence_ref: String,
    pub capability_status: String,
    pub custody_label: String,
    pub query_visibility: String,
    pub degraded_reason: Option<String>,
    pub journal_ref: Option<String>,
    pub ai_request_ref: Option<String>,
    pub ai_analysis_ref: Option<String>,
    pub policy_evaluation_ref: Option<String>,
    pub policy_decision_ref: Option<String>,
    pub policy_preview_id: Option<String>,
    pub action_intent_id: Option<String>,
    pub intervention_command_ref: Option<String>,
    pub intervention_result_ref: Option<String>,
    pub audit_entry_ref: Option<String>,
    pub read_model_ref: Option<String>,
    pub observed_at: String,
    pub exact_url_claimed: bool,
    pub ai_authority: bool,
    pub policy_authority: bool,
    pub dry_run: bool,
    pub adapter_dispatch_claimed: bool,
    pub intervention_command_allowed: bool,
}

impl BrowserRuntimeInput {
    pub fn managed_decision_fixture() -> Self {
        Self {
            source_ref: constants::browser::BRIDGE_ENDPOINT_REF_LOOPBACK_DEVTOOLS.to_string(),
            evidence_ref: constants::browser::TEST_BROWSER_RUNTIME_EVIDENCE_REF.to_string(),
            capability_status: constants::browser::CAPABILITY_STATUS_AVAILABLE.to_string(),
            custody_label: constants::browser::CUSTODY_CHILD_DEVICE_LOCAL.to_string(),
            query_visibility: constants::browser::QUERY_VISIBILITY_LIVE_LOCAL.to_string(),
            degraded_reason: None,
            journal_ref: Some(constants::browser::TEST_BROWSER_RUNTIME_JOURNAL_REF.to_string()),
            ai_request_ref: Some(
                constants::browser::TEST_BROWSER_RUNTIME_AI_REQUEST_REF.to_string(),
            ),
            ai_analysis_ref: Some(
                constants::browser::TEST_BROWSER_RUNTIME_AI_ANALYSIS_REF.to_string(),
            ),
            policy_evaluation_ref: Some(
                constants::browser::TEST_BROWSER_RUNTIME_POLICY_EVALUATION_REF.to_string(),
            ),
            policy_decision_ref: Some(
                constants::browser::TEST_BROWSER_RUNTIME_POLICY_DECISION_REF.to_string(),
            ),
            policy_preview_id: None,
            action_intent_id: None,
            intervention_command_ref: Some(
                constants::browser::TEST_BROWSER_RUNTIME_INTERVENTION_COMMAND_REF.to_string(),
            ),
            intervention_result_ref: Some(
                constants::browser::TEST_BROWSER_RUNTIME_INTERVENTION_RESULT_REF.to_string(),
            ),
            audit_entry_ref: Some(
                constants::browser::TEST_BROWSER_RUNTIME_AUDIT_ENTRY_REF.to_string(),
            ),
            read_model_ref: Some(
                constants::browser::TEST_BROWSER_RUNTIME_READ_MODEL_REF.to_string(),
            ),
            observed_at: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
            exact_url_claimed: true,
            ai_authority: false,
            policy_authority: true,
            dry_run: false,
            adapter_dispatch_claimed: true,
            intervention_command_allowed: true,
        }
    }

    pub fn dry_run_action_handoff_fixture() -> Self {
        Self {
            policy_preview_id: Some(
                constants::browser::TEST_BROWSER_RUNTIME_POLICY_PREVIEW_ID.to_string(),
            ),
            action_intent_id: Some(
                constants::browser::TEST_BROWSER_RUNTIME_ACTION_INTENT_ID.to_string(),
            ),
            intervention_command_ref: None,
            intervention_result_ref: None,
            dry_run: true,
            adapter_dispatch_claimed: false,
            intervention_command_allowed: false,
            ..Self::managed_decision_fixture()
        }
    }

    pub fn manual_required_fixture() -> Self {
        Self {
            intervention_command_ref: None,
            intervention_result_ref: None,
            exact_url_claimed: false,
            policy_authority: false,
            adapter_dispatch_claimed: false,
            intervention_command_allowed: false,
            ..Self::managed_decision_fixture()
        }
    }
}

#[derive(Clone, Debug)]
pub struct BrowserRuntimeReport {
    pub publish_reports: Vec<PublishReport>,
    pub stored_events: Vec<ocentra_eventing::envelope::StoredEventEnvelope>,
    pub dead_letters: Vec<DeadLetter>,
}

impl BrowserRuntimeReport {
    pub fn intervention_command_published(&self) -> bool {
        self.stored_events.iter().any(|event| {
            event
                .decode::<BrowserRuntimeEventPayload>()
                .map(|envelope| {
                    envelope.payload().phase == BrowserRuntimePhase::InterventionCommandIssued
                })
                .unwrap_or(false)
        })
    }

    pub fn action_intent_handoff_summary(
        &self,
    ) -> Option<(usize, String, String, String, String, String)> {
        action_handoff::handoff_summary(self)
    }
}

pub async fn publish_browser_runtime_chain_for_input(
    input: BrowserRuntimeInput,
) -> Result<BrowserRuntimeReport, EventingError> {
    let spine = BrowserRuntimeSpine::without_owner_handlers();
    spine.publish_input_chain(input).await
}

struct BrowserRuntimeSpine {
    bus: RootEventPublisher,
}

impl BrowserRuntimeSpine {
    fn without_owner_handlers() -> Self {
        Self {
            bus: EventBus::new(),
        }
    }

    async fn publish_input_chain(
        &self,
        input: BrowserRuntimeInput,
    ) -> Result<BrowserRuntimeReport, EventingError> {
        let mut reports = Vec::new();
        for phase in BrowserRuntimePhase::ordered_chain()
            .iter()
            .copied()
            .filter(|phase| should_publish_phase(*phase, &input))
        {
            let payload = browser_runtime_event_payload_from_input(phase, &input);
            let metadata = browser_event_metadata(phase, &input, phase.target_handler())?;
            reports.push(self.bus.publish(payload, metadata).await?);
        }
        Ok(BrowserRuntimeReport {
            publish_reports: reports,
            stored_events: self.bus.journal().await,
            dead_letters: self.bus.dead_letters().await,
        })
    }
}

pub(crate) fn should_publish_phase(
    phase: BrowserRuntimePhase,
    input: &BrowserRuntimeInput,
) -> bool {
    helpers::should_publish_phase(phase, input)
}

fn browser_runtime_event_payload_from_input(
    phase: BrowserRuntimePhase,
    input: &BrowserRuntimeInput,
) -> BrowserRuntimeEventPayload {
    BrowserRuntimeEventPayload {
        phase,
        source_ref: input.source_ref.clone(),
        evidence_ref: input.evidence_ref.clone(),
        capability_status: input.capability_status.clone(),
        custody_label: input.custody_label.clone(),
        query_visibility: input.query_visibility.clone(),
        degraded_reason: input.degraded_reason.clone(),
        journal_ref: input.journal_ref.clone(),
        ai_request_ref: input.ai_request_ref.clone(),
        ai_analysis_ref: input.ai_analysis_ref.clone(),
        policy_evaluation_ref: input.policy_evaluation_ref.clone(),
        policy_decision_ref: input.policy_decision_ref.clone(),
        policy_preview_id: input.policy_preview_id.clone(),
        action_intent_id: input.action_intent_id.clone(),
        intervention_command_ref: input.intervention_command_ref.clone(),
        intervention_result_ref: input.intervention_result_ref.clone(),
        audit_entry_ref: input.audit_entry_ref.clone(),
        read_model_ref: input.read_model_ref.clone(),
        previous_phase_ref: previous_phase_ref(phase, input),
        exact_url_claimed: input.exact_url_claimed,
        ai_authority: input.ai_authority,
        policy_authority: input.policy_authority,
        dry_run: input.dry_run,
        adapter_dispatch_claimed: input.adapter_dispatch_claimed,
        intervention_command_allowed: input.intervention_command_allowed,
        observed_at: input.observed_at.clone(),
    }
}

fn browser_event_metadata(
    phase: BrowserRuntimePhase,
    input: &BrowserRuntimeInput,
    target_handler: &str,
) -> Result<EventMetadata, EventingError> {
    Ok(EventMetadata::from_parts(
        EventId::generated(),
        CorrelationId::parse(browser_correlation_id(input))?,
        browser_event_source(phase, input)?,
        RecordedAt::parse(&input.observed_at)?,
        Some(TargetHandler::parse(target_handler)?),
    ))
}

fn browser_event_source(
    phase: BrowserRuntimePhase,
    input: &BrowserRuntimeInput,
) -> Result<EventSource, EventingError> {
    Ok(EventSource::new(
        event_custody(input),
        phase.runtime_role()?,
        SourceService::parse(constants::peer::LOCAL_DEV_AGENT)?,
        SourceComponent::parse(constants::browser::RUNTIME_COMPONENT_BROWSER_SPINE)?,
        RuntimeInstanceId::parse(constants::browser::RUNTIME_INSTANCE_LOCAL_BROWSER_RUNTIME)?,
    ))
}

fn event_custody(input: &BrowserRuntimeInput) -> EventCustody {
    let value = if input.journal_ref.is_some() {
        constants::eventing_source::CUSTODY_LOCAL_QUERY_STORE
    } else {
        constants::eventing_source::CUSTODY_UNAVAILABLE
    };
    match EventCustody::parse(value) {
        Ok(custody) => custody,
        Err(_) => std::process::abort(),
    }
}

fn browser_aggregate_key(source_ref: &str) -> String {
    let mut value = String::from(constants::browser::AGGREGATE_BROWSER_RUNTIME_PREFIX);
    value.push_str(source_ref);
    value
}

pub(crate) fn browser_correlation_id(input: &BrowserRuntimeInput) -> String {
    let mut value = String::from(constants::browser::CORRELATION_BROWSER_RUNTIME_PREFIX);
    value.push_str(&input.evidence_ref);
    value.push(constants::delimiter::HYPHEN);
    value.push_str(&input.observed_at);
    value
}
