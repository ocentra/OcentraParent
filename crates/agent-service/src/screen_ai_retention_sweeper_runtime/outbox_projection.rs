use ocentra_parent_agent_core::screen_evidence_queue::{
    ScreenEvidenceOutboxFailure, ScreenEvidenceQueue,
};

use crate::activity_capture::ActivityCaptureError;

pub(super) fn acknowledge_projected_outbox_failures(
    queue: &ScreenEvidenceQueue,
    failures: &[ScreenEvidenceOutboxFailure],
) -> Result<(), ActivityCaptureError> {
    if failures.is_empty() {
        return Ok(());
    }
    let acknowledged = queue.acknowledge_outbox_failures(failures)?;
    if acknowledged != failures.len() as u64 {
        return Err(ActivityCaptureError::Io);
    }
    Ok(())
}
