mod status;
mod validation;

use serde::{Deserialize, Serialize};

use self::{
    status::{privacy_mode_for, screen_summary_status},
    validation::{normalize_ref, validate_screen_summary_trigger_input},
};
use crate::bundle::NetworkCrossSliceEvidenceBundle;
use crate::cascade::NetworkCascadeNextCheck;
use crate::dns::types::NetworkEvidenceGrade;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkScreenSummaryTriggerStatus {
    Queued,
    NotRecommended,
    DisabledByParent,
    QueueUnavailable,
    CustodyManualRequired,
    ProtectedSurfaceUnavailable,
    Debounced,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkScreenSummaryPrivacyMode {
    NetworkOnly,
    ActiveWindowScreenIfEnabled,
    ScreenManualRequired,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkScreenSummaryTriggerInput {
    pub queue_job_ref: String,
    pub screen_queue_ref: String,
    pub parent_setting_ref: String,
    pub retention_policy_ref: String,
    pub bundle: NetworkCrossSliceEvidenceBundle,
    pub screen_summary_enabled: bool,
    pub queue_available: bool,
    pub encrypted_temporary_custody_available: bool,
    pub delete_after_analysis_available: bool,
    pub local_only_runtime_available: bool,
    pub protected_surface_detected: bool,
    pub debounce_clear: bool,
    pub raw_image_retention_requested: bool,
    pub remote_upload_requested: bool,
    pub screen_content_available: bool,
    pub policy_action_authority: bool,
    pub adapter_action_authority: bool,
    pub enforcement_command_published: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkScreenSummaryTriggerJob {
    pub queue_job_ref: String,
    pub screen_queue_ref: String,
    pub parent_setting_ref: String,
    pub retention_policy_ref: String,
    pub trigger_ref: String,
    pub source_evidence_refs: Vec<String>,
    pub exact_url_evidence_refs: Vec<String>,
    pub privacy_mode: NetworkScreenSummaryPrivacyMode,
    pub evidence_grade: NetworkEvidenceGrade,
    pub encrypted_temporary_custody_required: bool,
    pub delete_after_analysis_required: bool,
    pub capture_executed: bool,
    pub raw_image_available: bool,
    pub raw_image_retained: bool,
    pub remote_upload_authorized: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkScreenSummaryTriggerPlan {
    pub status: NetworkScreenSummaryTriggerStatus,
    pub privacy_mode: NetworkScreenSummaryPrivacyMode,
    pub trigger_ref: String,
    pub source_evidence_refs: Vec<String>,
    pub exact_url_evidence_refs: Vec<String>,
    pub screen_summary_recommended: bool,
    pub job: Option<NetworkScreenSummaryTriggerJob>,
    pub capture_executed: bool,
    pub raw_image_available: bool,
    pub raw_image_retained: bool,
    pub remote_upload_authorized: bool,
    pub policy_action_authority: bool,
    pub adapter_action_authority: bool,
    pub enforcement_command_published: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkScreenSummaryTriggerError {
    EmptyQueueJobRef,
    EmptyScreenQueueRef,
    EmptyParentSettingRef,
    EmptyRetentionPolicyRef,
    RawImageRetentionRejected,
    RemoteUploadRejected,
    ScreenContentRejected,
    DecryptedPayloadRejected,
    PolicyAuthorityRejected,
    AdapterAuthorityRejected,
    EnforcementCommandRejected,
}

pub fn plan_network_screen_summary_trigger(
    input: NetworkScreenSummaryTriggerInput,
) -> Result<NetworkScreenSummaryTriggerPlan, NetworkScreenSummaryTriggerError> {
    validate_screen_summary_trigger_input(&input)?;

    let screen_summary_recommended = input
        .bundle
        .next_checks
        .contains(&NetworkCascadeNextCheck::ScreenSummary);
    let status = screen_summary_status(&input, screen_summary_recommended);
    let privacy_mode = privacy_mode_for(status, screen_summary_recommended);
    let trigger_ref = input.bundle.trigger_ref.clone();
    let source_evidence_refs = input.bundle.evidence_refs.clone();
    let exact_url_evidence_refs = input.bundle.exact_url_evidence_refs.clone();
    let job = if status == NetworkScreenSummaryTriggerStatus::Queued {
        Some(NetworkScreenSummaryTriggerJob {
            queue_job_ref: normalize_ref(&input.queue_job_ref)
                .ok_or(NetworkScreenSummaryTriggerError::EmptyQueueJobRef)?,
            screen_queue_ref: normalize_ref(&input.screen_queue_ref)
                .ok_or(NetworkScreenSummaryTriggerError::EmptyScreenQueueRef)?,
            parent_setting_ref: normalize_ref(&input.parent_setting_ref)
                .ok_or(NetworkScreenSummaryTriggerError::EmptyParentSettingRef)?,
            retention_policy_ref: normalize_ref(&input.retention_policy_ref)
                .ok_or(NetworkScreenSummaryTriggerError::EmptyRetentionPolicyRef)?,
            trigger_ref: trigger_ref.clone(),
            source_evidence_refs: source_evidence_refs.clone(),
            exact_url_evidence_refs: exact_url_evidence_refs.clone(),
            privacy_mode,
            evidence_grade: input.bundle.evidence_grade,
            encrypted_temporary_custody_required: true,
            delete_after_analysis_required: true,
            capture_executed: false,
            raw_image_available: false,
            raw_image_retained: false,
            remote_upload_authorized: false,
        })
    } else {
        None
    };
    drop(input);

    Ok(NetworkScreenSummaryTriggerPlan {
        status,
        privacy_mode,
        trigger_ref,
        source_evidence_refs,
        exact_url_evidence_refs,
        screen_summary_recommended,
        job,
        capture_executed: false,
        raw_image_available: false,
        raw_image_retained: false,
        remote_upload_authorized: false,
        policy_action_authority: false,
        adapter_action_authority: false,
        enforcement_command_published: false,
    })
}
