use serde::{Deserialize, Serialize};

use crate::bundle::NetworkCrossSliceEvidenceBundle;
use crate::dns::NetworkEvidenceGrade;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkLocalAiQueueStatus {
    Queued,
    NotRecommended,
    DisabledByParent,
    ModelUnavailable,
    QueueUnavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkLocalAiQueueInputKind {
    EvidenceRefs,
    SummaryRefs,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkLocalAiQueueInput {
    pub queue_job_ref: String,
    pub queue_ref: String,
    pub model_runtime_ref: String,
    pub bundle: NetworkCrossSliceEvidenceBundle,
    pub summary_refs: Vec<String>,
    pub local_ai_enabled: bool,
    pub model_runtime_available: bool,
    pub queue_available: bool,
    pub raw_network_payload_available: bool,
    pub page_content_available: bool,
    pub policy_action_authority: bool,
    pub adapter_action_authority: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkLocalAiQueueJob {
    pub queue_job_ref: String,
    pub queue_ref: String,
    pub model_runtime_ref: String,
    pub trigger_ref: String,
    pub evidence_refs: Vec<String>,
    pub summary_refs: Vec<String>,
    pub exact_url_evidence_refs: Vec<String>,
    pub input_kinds: Vec<NetworkLocalAiQueueInputKind>,
    pub evidence_grade: NetworkEvidenceGrade,
    pub raw_network_payload_available: bool,
    pub page_content_available: bool,
    pub decrypted_payload_available: bool,
    pub policy_action_authority: bool,
    pub adapter_action_authority: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkLocalAiQueuePlan {
    pub status: NetworkLocalAiQueueStatus,
    pub trigger_ref: String,
    pub evidence_refs: Vec<String>,
    pub summary_refs: Vec<String>,
    pub local_ai_review_recommended: bool,
    pub job: Option<NetworkLocalAiQueueJob>,
    pub policy_action_authority: bool,
    pub adapter_action_authority: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkLocalAiQueueError {
    EmptyQueueJobRef,
    EmptyQueueRef,
    EmptyModelRuntimeRef,
    EmptySummaryRef,
    RawNetworkPayloadRejected,
    PageContentRejected,
    DecryptedPayloadRejected,
    PolicyAuthorityRejected,
    AdapterAuthorityRejected,
}

pub fn plan_network_local_ai_queue(
    input: NetworkLocalAiQueueInput,
) -> Result<NetworkLocalAiQueuePlan, NetworkLocalAiQueueError> {
    validate_refs(&input)?;
    validate_no_claims(&input)?;
    let status = queue_status(&input);

    let NetworkLocalAiQueueInput {
        queue_job_ref,
        queue_ref,
        model_runtime_ref,
        bundle,
        summary_refs,
        ..
    } = input;
    let trigger_ref = bundle.trigger_ref.clone();
    let evidence_refs = bundle.evidence_refs.clone();
    let summary_refs = normalized_summary_refs(&summary_refs)?;
    let job = if status == NetworkLocalAiQueueStatus::Queued {
        Some(NetworkLocalAiQueueJob {
            queue_job_ref: normalize_ref(&queue_job_ref)
                .ok_or(NetworkLocalAiQueueError::EmptyQueueJobRef)?,
            queue_ref: normalize_ref(&queue_ref).ok_or(NetworkLocalAiQueueError::EmptyQueueRef)?,
            model_runtime_ref: normalize_ref(&model_runtime_ref)
                .ok_or(NetworkLocalAiQueueError::EmptyModelRuntimeRef)?,
            trigger_ref: trigger_ref.clone(),
            evidence_refs: evidence_refs.clone(),
            summary_refs: summary_refs.clone(),
            exact_url_evidence_refs: bundle.exact_url_evidence_refs.clone(),
            input_kinds: vec![
                NetworkLocalAiQueueInputKind::EvidenceRefs,
                NetworkLocalAiQueueInputKind::SummaryRefs,
            ],
            evidence_grade: bundle.evidence_grade,
            raw_network_payload_available: false,
            page_content_available: false,
            decrypted_payload_available: false,
            policy_action_authority: false,
            adapter_action_authority: false,
        })
    } else {
        None
    };

    Ok(NetworkLocalAiQueuePlan {
        status,
        trigger_ref,
        evidence_refs,
        summary_refs,
        local_ai_review_recommended: bundle.local_ai_review_recommended,
        job,
        policy_action_authority: false,
        adapter_action_authority: false,
    })
}

fn validate_refs(input: &NetworkLocalAiQueueInput) -> Result<(), NetworkLocalAiQueueError> {
    if normalize_ref(&input.queue_job_ref).is_none() {
        return Err(NetworkLocalAiQueueError::EmptyQueueJobRef);
    }
    if normalize_ref(&input.queue_ref).is_none() {
        return Err(NetworkLocalAiQueueError::EmptyQueueRef);
    }
    if normalize_ref(&input.model_runtime_ref).is_none() {
        return Err(NetworkLocalAiQueueError::EmptyModelRuntimeRef);
    }
    normalized_summary_refs(&input.summary_refs)?;
    Ok(())
}

fn validate_no_claims(input: &NetworkLocalAiQueueInput) -> Result<(), NetworkLocalAiQueueError> {
    if input.raw_network_payload_available {
        return Err(NetworkLocalAiQueueError::RawNetworkPayloadRejected);
    }
    if input.page_content_available {
        return Err(NetworkLocalAiQueueError::PageContentRejected);
    }
    if input.bundle.decrypted_payload_available {
        return Err(NetworkLocalAiQueueError::DecryptedPayloadRejected);
    }
    if input.policy_action_authority || input.bundle.policy_action_authority {
        return Err(NetworkLocalAiQueueError::PolicyAuthorityRejected);
    }
    if input.adapter_action_authority || input.bundle.adapter_action_authorized {
        return Err(NetworkLocalAiQueueError::AdapterAuthorityRejected);
    }
    Ok(())
}

fn queue_status(input: &NetworkLocalAiQueueInput) -> NetworkLocalAiQueueStatus {
    if !input.bundle.local_ai_review_recommended {
        return NetworkLocalAiQueueStatus::NotRecommended;
    }
    if !input.local_ai_enabled {
        return NetworkLocalAiQueueStatus::DisabledByParent;
    }
    if !input.model_runtime_available {
        return NetworkLocalAiQueueStatus::ModelUnavailable;
    }
    if !input.queue_available {
        return NetworkLocalAiQueueStatus::QueueUnavailable;
    }
    NetworkLocalAiQueueStatus::Queued
}

fn normalized_summary_refs(
    summary_refs: &[String],
) -> Result<Vec<String>, NetworkLocalAiQueueError> {
    let mut refs = Vec::new();
    for summary_ref in summary_refs {
        let Some(normalized) = normalize_ref(summary_ref) else {
            return Err(NetworkLocalAiQueueError::EmptySummaryRef);
        };
        if !refs.contains(&normalized) {
            refs.push(normalized);
        }
    }
    Ok(refs)
}

fn normalize_ref(value: &str) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_owned())
    }
}
