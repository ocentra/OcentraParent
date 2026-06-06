use serde::{Deserialize, Serialize};

use crate::{NetworkLocalAiQueuePlan, NetworkLocalAiQueueStatus};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkLocalAiRuntimeGenerationState {
    Complete,
    Unavailable,
    Failed,
    TimedOut,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkLocalAiRuntimeBridgeState {
    ResultReady,
    RuntimeUnavailable,
    RuntimeFailed,
    RuntimeTimedOut,
    QueueNotReady,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkLocalAiRuntimeResultRef {
    pub local_ai_result_ref: String,
    pub runtime_reference_id: String,
    pub model_reference: String,
    pub model_version_ref: String,
    pub generation_state: NetworkLocalAiRuntimeGenerationState,
    pub output_summary_ref: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkLocalAiRuntimeResultInput {
    pub queue_plan: NetworkLocalAiQueuePlan,
    pub runtime_result: Option<NetworkLocalAiRuntimeResultRef>,
    pub prompt_template_ref: String,
    pub policy_context_ref: String,
    pub parent_rule_refs: Vec<String>,
    pub remote_ai_claimed: bool,
    pub raw_pcap_input_claimed: bool,
    pub decrypted_payload_claimed: bool,
    pub page_content_claimed: bool,
    pub exact_url_claimed: bool,
    pub private_message_claimed: bool,
    pub search_query_claimed: bool,
    pub policy_authority_claimed: bool,
    pub adapter_authority_claimed: bool,
    pub enforcement_command_claimed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkLocalAiRuntimeResultBridge {
    pub bridge_state: NetworkLocalAiRuntimeBridgeState,
    pub queue_status: NetworkLocalAiQueueStatus,
    pub trigger_ref: String,
    pub queue_job_ref: Option<String>,
    pub queue_ref: Option<String>,
    pub model_runtime_ref: Option<String>,
    pub local_ai_result_ref: Option<String>,
    pub runtime_reference_id: Option<String>,
    pub model_reference: Option<String>,
    pub model_version_ref: Option<String>,
    pub prompt_template_ref: String,
    pub policy_context_ref: String,
    pub parent_rule_refs: Vec<String>,
    pub evidence_refs: Vec<String>,
    pub summary_refs: Vec<String>,
    pub managed_browser_exact_url_evidence_refs: Vec<String>,
    pub output_summary_ref: Option<String>,
    pub local_runtime_result_observed: bool,
    pub audit_input_ready: bool,
    pub local_model_output_available: bool,
    pub model_execution_proved: bool,
    pub raw_pcap_available: bool,
    pub exact_url_claimed: bool,
    pub decrypted_payload_available: bool,
    pub page_content_available: bool,
    pub private_message_available: bool,
    pub search_query_available: bool,
    pub remote_ai_used: bool,
    pub policy_authority: bool,
    pub adapter_authority: bool,
    pub enforcement_commands_published: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkLocalAiRuntimeResultError {
    EmptyPromptTemplateRef,
    EmptyPolicyContextRef,
    EmptyParentRuleRefs,
    EmptyParentRuleRef,
    ResultWithoutQueuedJob,
    MissingQueuedJob,
    MissingRuntimeResultForQueuedJob,
    EmptyLocalAiResultRef,
    EmptyRuntimeReferenceId,
    EmptyModelReference,
    EmptyModelVersionRef,
    CompleteMissingOutputSummaryRef,
    NonCompleteOutputSummaryRef,
    RemoteAiClaimRejected,
    RawPcapInputRejected,
    DecryptedPayloadClaimRejected,
    PageContentClaimRejected,
    ExactUrlClaimRejected,
    PrivateMessageClaimRejected,
    SearchQueryClaimRejected,
    PolicyAuthorityClaimRejected,
    AdapterAuthorityClaimRejected,
    EnforcementCommandClaimRejected,
}

pub fn bridge_network_local_ai_runtime_result(
    input: NetworkLocalAiRuntimeResultInput,
) -> Result<NetworkLocalAiRuntimeResultBridge, NetworkLocalAiRuntimeResultError> {
    reject_claims(&input)?;
    let prompt_template_ref = normalize_ref(&input.prompt_template_ref)
        .ok_or(NetworkLocalAiRuntimeResultError::EmptyPromptTemplateRef)?;
    let policy_context_ref = normalize_ref(&input.policy_context_ref)
        .ok_or(NetworkLocalAiRuntimeResultError::EmptyPolicyContextRef)?;
    let parent_rule_refs = normalized_refs(
        &input.parent_rule_refs,
        NetworkLocalAiRuntimeResultError::EmptyParentRuleRefs,
        NetworkLocalAiRuntimeResultError::EmptyParentRuleRef,
    )?;

    if input.queue_plan.status != NetworkLocalAiQueueStatus::Queued {
        if input.runtime_result.is_some() {
            return Err(NetworkLocalAiRuntimeResultError::ResultWithoutQueuedJob);
        }
        return Ok(queue_not_ready_bridge(
            input.queue_plan,
            prompt_template_ref,
            policy_context_ref,
            parent_rule_refs,
        ));
    }

    let job = input
        .queue_plan
        .job
        .as_ref()
        .ok_or(NetworkLocalAiRuntimeResultError::MissingQueuedJob)?;
    let runtime_result = input
        .runtime_result
        .ok_or(NetworkLocalAiRuntimeResultError::MissingRuntimeResultForQueuedJob)?;
    let normalized_result = normalize_runtime_result(runtime_result)?;
    let bridge_state = bridge_state(normalized_result.generation_state);
    let audit_input_ready = bridge_state == NetworkLocalAiRuntimeBridgeState::ResultReady;
    let local_model_output_available = normalized_result.output_summary_ref.is_some();

    Ok(NetworkLocalAiRuntimeResultBridge {
        bridge_state,
        queue_status: input.queue_plan.status,
        trigger_ref: job.trigger_ref.clone(),
        queue_job_ref: Some(job.queue_job_ref.clone()),
        queue_ref: Some(job.queue_ref.clone()),
        model_runtime_ref: Some(job.model_runtime_ref.clone()),
        local_ai_result_ref: Some(normalized_result.local_ai_result_ref),
        runtime_reference_id: Some(normalized_result.runtime_reference_id),
        model_reference: Some(normalized_result.model_reference),
        model_version_ref: Some(normalized_result.model_version_ref),
        prompt_template_ref,
        policy_context_ref,
        parent_rule_refs,
        evidence_refs: job.evidence_refs.clone(),
        summary_refs: job.summary_refs.clone(),
        managed_browser_exact_url_evidence_refs: job.exact_url_evidence_refs.clone(),
        output_summary_ref: normalized_result.output_summary_ref,
        local_runtime_result_observed: true,
        audit_input_ready,
        local_model_output_available,
        model_execution_proved: false,
        raw_pcap_available: false,
        exact_url_claimed: false,
        decrypted_payload_available: false,
        page_content_available: false,
        private_message_available: false,
        search_query_available: false,
        remote_ai_used: false,
        policy_authority: false,
        adapter_authority: false,
        enforcement_commands_published: 0,
    })
}

fn queue_not_ready_bridge(
    queue_plan: NetworkLocalAiQueuePlan,
    prompt_template_ref: String,
    policy_context_ref: String,
    parent_rule_refs: Vec<String>,
) -> NetworkLocalAiRuntimeResultBridge {
    NetworkLocalAiRuntimeResultBridge {
        bridge_state: NetworkLocalAiRuntimeBridgeState::QueueNotReady,
        queue_status: queue_plan.status,
        trigger_ref: queue_plan.trigger_ref,
        queue_job_ref: None,
        queue_ref: None,
        model_runtime_ref: None,
        local_ai_result_ref: None,
        runtime_reference_id: None,
        model_reference: None,
        model_version_ref: None,
        prompt_template_ref,
        policy_context_ref,
        parent_rule_refs,
        evidence_refs: queue_plan.evidence_refs,
        summary_refs: queue_plan.summary_refs,
        managed_browser_exact_url_evidence_refs: Vec::new(),
        output_summary_ref: None,
        local_runtime_result_observed: false,
        audit_input_ready: false,
        local_model_output_available: false,
        model_execution_proved: false,
        raw_pcap_available: false,
        exact_url_claimed: false,
        decrypted_payload_available: false,
        page_content_available: false,
        private_message_available: false,
        search_query_available: false,
        remote_ai_used: false,
        policy_authority: false,
        adapter_authority: false,
        enforcement_commands_published: 0,
    }
}

fn normalize_runtime_result(
    result: NetworkLocalAiRuntimeResultRef,
) -> Result<NetworkLocalAiRuntimeResultRef, NetworkLocalAiRuntimeResultError> {
    let local_ai_result_ref = normalize_ref(&result.local_ai_result_ref)
        .ok_or(NetworkLocalAiRuntimeResultError::EmptyLocalAiResultRef)?;
    let runtime_reference_id = normalize_ref(&result.runtime_reference_id)
        .ok_or(NetworkLocalAiRuntimeResultError::EmptyRuntimeReferenceId)?;
    let model_reference = normalize_ref(&result.model_reference)
        .ok_or(NetworkLocalAiRuntimeResultError::EmptyModelReference)?;
    let model_version_ref = normalize_ref(&result.model_version_ref)
        .ok_or(NetworkLocalAiRuntimeResultError::EmptyModelVersionRef)?;
    let output_summary_ref = normalized_optional_ref(result.output_summary_ref.as_deref())?;

    match (result.generation_state, output_summary_ref.as_ref()) {
        (NetworkLocalAiRuntimeGenerationState::Complete, None) => {
            return Err(NetworkLocalAiRuntimeResultError::CompleteMissingOutputSummaryRef);
        }
        (NetworkLocalAiRuntimeGenerationState::Unavailable, Some(_))
        | (NetworkLocalAiRuntimeGenerationState::Failed, Some(_))
        | (NetworkLocalAiRuntimeGenerationState::TimedOut, Some(_)) => {
            return Err(NetworkLocalAiRuntimeResultError::NonCompleteOutputSummaryRef);
        }
        _ => {}
    }

    Ok(NetworkLocalAiRuntimeResultRef {
        local_ai_result_ref,
        runtime_reference_id,
        model_reference,
        model_version_ref,
        generation_state: result.generation_state,
        output_summary_ref,
    })
}

fn bridge_state(
    generation_state: NetworkLocalAiRuntimeGenerationState,
) -> NetworkLocalAiRuntimeBridgeState {
    match generation_state {
        NetworkLocalAiRuntimeGenerationState::Complete => {
            NetworkLocalAiRuntimeBridgeState::ResultReady
        }
        NetworkLocalAiRuntimeGenerationState::Unavailable => {
            NetworkLocalAiRuntimeBridgeState::RuntimeUnavailable
        }
        NetworkLocalAiRuntimeGenerationState::Failed => {
            NetworkLocalAiRuntimeBridgeState::RuntimeFailed
        }
        NetworkLocalAiRuntimeGenerationState::TimedOut => {
            NetworkLocalAiRuntimeBridgeState::RuntimeTimedOut
        }
    }
}

fn reject_claims(
    input: &NetworkLocalAiRuntimeResultInput,
) -> Result<(), NetworkLocalAiRuntimeResultError> {
    if input.remote_ai_claimed {
        return Err(NetworkLocalAiRuntimeResultError::RemoteAiClaimRejected);
    }
    if input.raw_pcap_input_claimed {
        return Err(NetworkLocalAiRuntimeResultError::RawPcapInputRejected);
    }
    if input.decrypted_payload_claimed {
        return Err(NetworkLocalAiRuntimeResultError::DecryptedPayloadClaimRejected);
    }
    if input.page_content_claimed {
        return Err(NetworkLocalAiRuntimeResultError::PageContentClaimRejected);
    }
    if input.exact_url_claimed {
        return Err(NetworkLocalAiRuntimeResultError::ExactUrlClaimRejected);
    }
    if input.private_message_claimed {
        return Err(NetworkLocalAiRuntimeResultError::PrivateMessageClaimRejected);
    }
    if input.search_query_claimed {
        return Err(NetworkLocalAiRuntimeResultError::SearchQueryClaimRejected);
    }
    if input.policy_authority_claimed {
        return Err(NetworkLocalAiRuntimeResultError::PolicyAuthorityClaimRejected);
    }
    if input.adapter_authority_claimed {
        return Err(NetworkLocalAiRuntimeResultError::AdapterAuthorityClaimRejected);
    }
    if input.enforcement_command_claimed {
        return Err(NetworkLocalAiRuntimeResultError::EnforcementCommandClaimRejected);
    }
    Ok(())
}

fn normalized_refs(
    values: &[String],
    empty_values_error: NetworkLocalAiRuntimeResultError,
    empty_value_error: NetworkLocalAiRuntimeResultError,
) -> Result<Vec<String>, NetworkLocalAiRuntimeResultError> {
    if values.is_empty() {
        return Err(empty_values_error);
    }

    let mut refs = Vec::new();
    for value in values {
        let Some(normalized) = normalize_ref(value) else {
            return Err(empty_value_error);
        };
        if !refs.contains(&normalized) {
            refs.push(normalized);
        }
    }
    Ok(refs)
}

fn normalized_optional_ref(
    value: Option<&str>,
) -> Result<Option<String>, NetworkLocalAiRuntimeResultError> {
    match value {
        Some(value) => normalize_ref(value)
            .map(Some)
            .ok_or(NetworkLocalAiRuntimeResultError::NonCompleteOutputSummaryRef),
        None => Ok(None),
    }
}

fn normalize_ref(value: &str) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_owned())
    }
}
