use ocentra_parent_agent_protocol::constants;
use sha2::{Digest, Sha256};

pub(crate) fn screen_queue_job(
    run_id: &str,
    capture_scope: &str,
    image_digest: &str,
    image_byte_size: usize,
) -> ocentra_parent_agent_protocol::ScreenAnalysisQueueJob {
    ocentra_parent_agent_protocol::ScreenAnalysisQueueJob {
        schema_version: ocentra_parent_agent_protocol::SCREEN_EVIDENCE_SCHEMA_VERSION,
        queue_job_id: format!("screen-capture-proof-{run_id}"),
        created_at: run_id.to_owned(),
        not_before: run_id.to_owned(),
        expires_at: run_id.to_owned(),
        last_attempt_at: None,
        capture_reason: ocentra_parent_agent_protocol::SCREEN_CAPTURE_REASON_MANUAL_PARENT_TEST
            .to_owned(),
        capture_scope: capture_scope.to_owned(),
        source_id: constants::activity_store::TEST_SCREEN_SOURCE_ID.to_owned(),
        adapter_id: constants::activity_store::TEST_SCREEN_ADAPTER_ID.to_owned(),
        device_ref: constants::peer::LOCAL_DEV_AGENT.to_owned(),
        local_user_ref: constants::activity_store::TEST_SCREEN_LOCAL_USER_REF.to_owned(),
        parent_setting_ref: constants::activity_store::TEST_SCREEN_PARENT_SETTING_REF.to_owned(),
        setting_version: 1,
        related_evidence_refs: Vec::new(),
        encrypted_image_ref: format!("screen-capture-proof-{run_id}.enc"),
        image_digest: image_digest.to_owned(),
        image_byte_size: image_byte_size as u64,
        image_format: ocentra_parent_agent_protocol::SCREEN_IMAGE_FORMAT_PNG.to_owned(),
        status: ocentra_parent_agent_protocol::SCREEN_QUEUE_STATUS_QUEUED.to_owned(),
        attempt_count: 0,
        max_retry_count: 1,
        failure_reason: None,
        unavailable_reason: None,
        deletion_required: true,
        deleted_at: None,
        deletion_status: ocentra_parent_agent_protocol::SCREEN_DELETION_REQUIRED.to_owned(),
        deletion_proof_ref: None,
        custody_state: ocentra_parent_agent_protocol::SCREEN_CUSTODY_TEMP_QUEUE.to_owned(),
    }
}

pub(crate) fn digest_hex(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    digest.iter().map(|byte| format!("{byte:02x}")).collect()
}
