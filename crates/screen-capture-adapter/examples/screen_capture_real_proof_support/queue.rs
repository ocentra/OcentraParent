use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::screen_evidence::{
    ScreenAnalysisQueueJob, SCREEN_CAPTURE_REASON_MANUAL_PARENT_TEST, SCREEN_CUSTODY_TEMP_QUEUE,
    SCREEN_DELETION_REQUIRED, SCREEN_IMAGE_FORMAT_PNG, SCREEN_QUEUE_STATUS_QUEUED,
};
use ocentra_parent_agent_protocol::SCREEN_EVIDENCE_SCHEMA_VERSION;
use sha2::{Digest, Sha256};

use super::{
    screen_capture_real_proof_support_impl::{
        SCREEN_CAPTURE_PROOF_SCOPE_ACTIVE_WINDOW, SCREEN_CAPTURE_PROOF_SCOPE_PRIMARY_DISPLAY,
        SCREEN_CAPTURE_PROOF_SCOPE_SELECTED_WINDOW,
    },
    ScreenCaptureProofRunId, ScreenCaptureProofScopeLabel,
};

pub(crate) fn screen_queue_job(
    run_id: &ScreenCaptureProofRunId,
    capture_scope: ScreenCaptureProofScopeLabel,
    image_digest: &ScreenCaptureProofDigestHex,
    image_byte_size: usize,
) -> ScreenAnalysisQueueJob {
    ScreenAnalysisQueueJob {
        schema_version: SCREEN_EVIDENCE_SCHEMA_VERSION,
        queue_job_id: format!("screen-capture-proof-{}", run_id.0.as_str()),
        created_at: run_id.0.as_str().to_owned(),
        not_before: run_id.0.as_str().to_owned(),
        expires_at: run_id.0.as_str().to_owned(),
        last_attempt_at: None,
        capture_reason: SCREEN_CAPTURE_REASON_MANUAL_PARENT_TEST.to_owned(),
        capture_scope: match capture_scope {
            ScreenCaptureProofScopeLabel::ActiveWindow => SCREEN_CAPTURE_PROOF_SCOPE_ACTIVE_WINDOW,
            ScreenCaptureProofScopeLabel::SelectedWindow => {
                SCREEN_CAPTURE_PROOF_SCOPE_SELECTED_WINDOW
            }
            ScreenCaptureProofScopeLabel::PrimaryDisplay => {
                SCREEN_CAPTURE_PROOF_SCOPE_PRIMARY_DISPLAY
            }
        }
        .to_owned(),
        source_id: constants::activity_store::TEST_SCREEN_SOURCE_ID.to_owned(),
        adapter_id: constants::activity_store::TEST_SCREEN_ADAPTER_ID.to_owned(),
        device_ref: constants::peer::LOCAL_DEV_AGENT.to_owned(),
        local_user_ref: constants::activity_store::TEST_SCREEN_LOCAL_USER_REF.to_owned(),
        parent_setting_ref: constants::activity_store::TEST_SCREEN_PARENT_SETTING_REF.to_owned(),
        setting_version: 1,
        related_evidence_refs: Vec::new(),
        encrypted_image_ref: format!("screen-capture-proof-{}.enc", run_id.0.as_str()),
        image_digest: image_digest.0.as_str().to_owned(),
        image_byte_size: image_byte_size as u64,
        image_format: SCREEN_IMAGE_FORMAT_PNG.to_owned(),
        status: SCREEN_QUEUE_STATUS_QUEUED.to_owned(),
        attempt_count: 0,
        max_retry_count: 1,
        failure_reason: None,
        unavailable_reason: None,
        deletion_required: true,
        deleted_at: None,
        deletion_status: SCREEN_DELETION_REQUIRED.to_owned(),
        deletion_proof_ref: None,
        custody_state: SCREEN_CUSTODY_TEMP_QUEUE.to_owned(),
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ScreenCaptureProofDigestHex(pub(crate) String);

pub(crate) fn digest_hex(bytes: &[u8]) -> ScreenCaptureProofDigestHex {
    let digest = Sha256::digest(bytes);
    ScreenCaptureProofDigestHex(digest.iter().map(|byte| format!("{byte:02x}")).collect())
}
