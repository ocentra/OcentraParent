use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScreenRuntimeInput {
    pub queue_job_id: String,
    pub screen_analysis_result_id: String,
    pub capture_reason: String,
    pub capture_scope: String,
    pub image_digest: String,
    pub summary: String,
    pub model_runtime_ref: String,
    pub model_id: String,
    pub prompt_or_template_version: String,
    pub policy_decision_ref: String,
    pub policy_action: String,
    pub parent_rule_ref: String,
    pub action_ref: String,
    pub deletion_proof_ref: String,
    pub portal_read_model_ref: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScreenRuntimeCaptureInput {
    pub queue_job_id: String,
    pub screen_analysis_result_id: String,
    pub capture_reason: String,
    pub capture_scope: String,
    pub image_digest: String,
    pub summary: String,
    pub model_runtime_ref: String,
    pub model_id: String,
    pub prompt_or_template_version: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScreenRuntimeDeletionInput {
    pub queue_job_id: String,
    pub screen_analysis_result_id: String,
    pub capture_reason: String,
    pub capture_scope: String,
    pub image_digest: String,
    pub summary: String,
    pub model_runtime_ref: String,
    pub model_id: String,
    pub prompt_or_template_version: String,
    pub deletion_proof_ref: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScreenRuntimeDegradedInput {
    pub queue_job_id: String,
    pub screen_analysis_result_id: String,
    pub capture_reason: String,
    pub capture_scope: String,
    pub image_digest: String,
    pub summary: String,
    pub model_runtime_ref: String,
    pub model_id: String,
    pub prompt_or_template_version: String,
    pub deletion_proof_ref: String,
    pub portal_read_model_ref: String,
}

impl From<&ScreenRuntimeInput> for ScreenRuntimeCaptureInput {
    fn from(input: &ScreenRuntimeInput) -> Self {
        Self {
            queue_job_id: input.queue_job_id.clone(),
            screen_analysis_result_id: input.screen_analysis_result_id.clone(),
            capture_reason: input.capture_reason.clone(),
            capture_scope: input.capture_scope.clone(),
            image_digest: input.image_digest.clone(),
            summary: input.summary.clone(),
            model_runtime_ref: input.model_runtime_ref.clone(),
            model_id: input.model_id.clone(),
            prompt_or_template_version: input.prompt_or_template_version.clone(),
        }
    }
}

impl From<&ScreenRuntimeInput> for ScreenRuntimeDeletionInput {
    fn from(input: &ScreenRuntimeInput) -> Self {
        Self {
            queue_job_id: input.queue_job_id.clone(),
            screen_analysis_result_id: input.screen_analysis_result_id.clone(),
            capture_reason: input.capture_reason.clone(),
            capture_scope: input.capture_scope.clone(),
            image_digest: input.image_digest.clone(),
            summary: input.summary.clone(),
            model_runtime_ref: input.model_runtime_ref.clone(),
            model_id: input.model_id.clone(),
            prompt_or_template_version: input.prompt_or_template_version.clone(),
            deletion_proof_ref: input.deletion_proof_ref.clone(),
        }
    }
}

impl From<&ScreenRuntimeDegradedInput> for ScreenRuntimeCaptureInput {
    fn from(input: &ScreenRuntimeDegradedInput) -> Self {
        Self {
            queue_job_id: input.queue_job_id.clone(),
            screen_analysis_result_id: input.screen_analysis_result_id.clone(),
            capture_reason: input.capture_reason.clone(),
            capture_scope: input.capture_scope.clone(),
            image_digest: input.image_digest.clone(),
            summary: input.summary.clone(),
            model_runtime_ref: input.model_runtime_ref.clone(),
            model_id: input.model_id.clone(),
            prompt_or_template_version: input.prompt_or_template_version.clone(),
        }
    }
}
