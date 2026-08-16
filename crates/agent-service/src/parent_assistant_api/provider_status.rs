use ocentra_parent_agent_protocol::activity::policy::ParentEvidenceReference;
use ocentra_parent_agent_protocol::activity::policy::ParentEvidenceReferenceKind;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::parent_assistant::ParentAssistantBackendState;
use ocentra_parent_agent_protocol::parent_assistant::ParentAssistantEvidenceContext;
use ocentra_parent_agent_protocol::parent_assistant::ParentAssistantProviderStatus;
use ocentra_parent_agent_protocol::policy_constants as policy;
use ocentra_parent_agent_protocol::transport::AgentCommandEnvelope;

#[path = "provider_run_state.rs"]
mod provider_run_state;
#[path = "provider_scheduler_state.rs"]
mod provider_scheduler_state;

use crate::local_ai_provider_scheduler::local_ai_provider_scheduler;
use crate::local_ai_runtime_config::LocalAiRuntimeConfigSnapshot;
use crate::local_ai_runtime_config_values::LocalAiRuntimeText;
use crate::local_ai_runtime_status::local_ai_runtime_status_for_model_from_config;
use crate::time::timestamp_now;

use super::api_boundary;
use super::payload_fields;
use super::ParentAssistantPayloadFieldName;
use super::ParentAssistantTextRef;

pub(super) fn provider_status_for_command(
    command: &AgentCommandEnvelope,
) -> ParentAssistantProviderStatus {
    let config = LocalAiRuntimeConfigSnapshot::from_environment();
    let default_model_id = config.model_id().0;
    let model_id = payload_fields::string_payload_field(
        command,
        ParentAssistantPayloadFieldName(constants::field::LOCAL_AI_MODEL_ID),
    )
    .unwrap_or_else(|| ParentAssistantTextRef(default_model_id.as_str()).into_text())
    .0;
    let (runtime, _, _) = local_ai_runtime_status_for_model_from_config(
        timestamp_now::<String>(),
        &config,
        Some(LocalAiRuntimeText(model_id)),
    );
    let scheduler_status = local_ai_provider_scheduler().status_snapshot();
    let queue_depth = scheduler_status.queue.total();
    let busy = scheduler_status.current_job_class.is_some() || queue_depth > 0;
    let runtime_unavailable = runtime.unavailable_reason.is_some();
    let provider_state = provider_scheduler_state::provider_state_for_status(
        runtime_unavailable,
        busy,
        &scheduler_status.degraded_state,
    );
    let scheduler_job_status = provider_scheduler_state::scheduler_job_status_for_status(
        runtime_unavailable,
        &scheduler_status.lifecycle_state,
    );
    let run_state = provider_run_state::run_state_for_status(
        runtime_unavailable,
        &scheduler_status.lifecycle_state,
    );
    let degraded_state = scheduler_status.degraded_state;
    let unavailable_reason = runtime
        .unavailable_reason
        .or_else(|| scheduler_status.unavailable_reason.clone());
    let citations = [default_evidence_context()];
    let api_provider_boundary =
        api_boundary::api_provider_boundary_for_command(command, &citations);
    let provider_route = api_boundary::provider_route(provider_state, &api_provider_boundary);

    ParentAssistantProviderStatus {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        backend_state: ParentAssistantBackendState::RuntimeBacked,
        provider_id: runtime.provider_id,
        model_id: runtime.model_id,
        provider_state,
        run_state,
        scheduler_job_status,
        scheduler_status,
        degraded_state,
        unavailable_reason,
        queue_depth,
        busy,
        api_provider_boundary,
        provider_route,
    }
}

fn default_evidence_context() -> ParentAssistantEvidenceContext {
    ParentAssistantEvidenceContext {
        evidence: ParentEvidenceReference {
            evidence_reference_id: constants::field::ACTIVITY_DIGEST.to_string(),
            kind: ParentEvidenceReferenceKind::QueryStoreSummary,
            observed_at: timestamp_now(),
        },
        citation_label: constants::parent_assistant::DEFAULT_CITATION_LABEL.to_string(),
        allowed_summary: constants::parent_assistant::DEFAULT_ALLOWED_SUMMARY.to_string(),
        custody_label: constants::parent_assistant::EVIDENCE_CUSTODY_ACTIVITY_SUMMARY.to_string(),
        source_label: constants::parent_assistant::EVIDENCE_SOURCE_ACTIVITY_QUERY_STORE_SUMMARY
            .to_string(),
        raw_child_evidence_included: false,
        direct_enforcement_allowed: false,
    }
}
