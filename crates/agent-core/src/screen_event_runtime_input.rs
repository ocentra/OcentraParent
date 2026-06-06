use ocentra_parent_agent_protocol::constants;

#[derive(Clone, Debug, PartialEq, Eq)]
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

#[derive(Clone, Debug, PartialEq, Eq)]
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

#[derive(Clone, Debug, PartialEq, Eq)]
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

impl ScreenRuntimeInput {
    pub fn proof_fixture() -> Self {
        Self {
            queue_job_id: constants::activity_store::TEST_SCREEN_QUEUE_JOB_ID.to_string(),
            screen_analysis_result_id: constants::activity_store::TEST_SCREEN_RESULT_ID.to_string(),
            capture_reason: constants::activity_capture::SCREEN_TRIGGER_TIMED_CADENCE.to_string(),
            capture_scope: constants::activity_capture::OBSERVATION_MODE_ACTIVE_WINDOW.to_string(),
            image_digest: constants::activity_store::TEST_SCREEN_IMAGE_DIGEST.to_string(),
            summary: constants::activity_store::TEST_SCREEN_SUMMARY.to_string(),
            model_runtime_ref: constants::activity_store::TEST_SCREEN_MODEL_RUNTIME_REF.to_string(),
            model_id: constants::activity_store::TEST_SCREEN_MODEL_ID.to_string(),
            prompt_or_template_version: constants::activity_store::TEST_SCREEN_TEMPLATE_VERSION
                .to_string(),
            policy_decision_ref: constants::activity_store::TEST_POLICY_DECISION_ID.to_string(),
            policy_action: constants::activity_store::TEST_POLICY_ACTION_ALLOW.to_string(),
            parent_rule_ref: constants::screen_flow::TEST_SCREEN_POLICY_RULE_REF.to_string(),
            action_ref: constants::screen_flow::TEST_SCREEN_ACTION_REF.to_string(),
            deletion_proof_ref: constants::activity_store::TEST_SCREEN_DELETION_REASONS.to_string(),
            portal_read_model_ref: constants::screen_flow::TEST_SCREEN_PORTAL_READ_MODEL_REF
                .to_string(),
        }
    }
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
