mod refs;
mod validation;

use serde::{Deserialize, Serialize};

use self::{
    refs::{normalize_ref, normalized_summary_refs},
    validation::{queue_status, validate_no_claims, validate_refs},
};

use crate::bundle::NetworkCrossSliceEvidenceBundle;
use crate::dns::types::NetworkEvidenceGrade;

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
