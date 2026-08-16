use ocentra_parent_agent_core::screen_evidence_queue::{
    ScreenEvidenceExpiredQueueEntry, ScreenEvidenceOutboxFailure,
};
use ocentra_parent_agent_protocol::activity::ActivityEvent;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::screen_evidence::{
    SCREEN_DELETION_DELETE_FAILED, SCREEN_DELETION_DELETE_FAILED_SUMMARY,
};

use super::{expired_entry_event, ScreenAiObservedAt};

pub(super) fn outbox_failure_event(
    failure: &ScreenEvidenceOutboxFailure,
    observed_at: ScreenAiObservedAt,
) -> ActivityEvent {
    let entry = ScreenEvidenceExpiredQueueEntry {
        queue_job_id: failure.queue_job_id.clone(),
        image_digest: failure.malformed_record_digest.clone(),
        expires_at: observed_at.0.clone(),
        deletion_proof_ref: failure.deletion_proof_ref.clone(),
    };
    failed_deletion_event(&entry, observed_at)
}

fn failed_deletion_event(
    entry: &ScreenEvidenceExpiredQueueEntry,
    observed_at: ScreenAiObservedAt,
) -> ActivityEvent {
    let mut event = expired_entry_event(entry, observed_at);
    event.event_id.push(constants::delimiter::HYPHEN);
    event.event_id.push_str(SCREEN_DELETION_DELETE_FAILED);
    event.fields.insert(
        constants::field::SCREEN_IMAGE_DELETION_STATE.to_string(),
        LogFieldValue::String(SCREEN_DELETION_DELETE_FAILED.to_string()),
    );
    event.fields.insert(
        constants::field::SCREEN_SUMMARY.to_string(),
        LogFieldValue::String(SCREEN_DELETION_DELETE_FAILED_SUMMARY.to_string()),
    );
    event
}
