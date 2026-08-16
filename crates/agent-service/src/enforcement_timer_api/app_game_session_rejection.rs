use ocentra_parent_agent_protocol::constants;

use crate::app_game_dispatch_evidence::AppGameDispatchEvidenceRejection;

pub(super) fn rejection_reason(
    rejection: AppGameDispatchEvidenceRejection,
) -> super::rejection::EnforcementTimerRejectionReason {
    let value = match rejection {
        AppGameDispatchEvidenceRejection::Required => {
            constants::enforcement::REJECTION_APP_GAME_SESSION_EVIDENCE_REQUIRED
        }
        AppGameDispatchEvidenceRejection::Mismatch => {
            constants::enforcement::REJECTION_APP_GAME_RUNTIME_EVIDENCE_MISMATCH
        }
    };
    super::rejection::EnforcementTimerRejectionReason(value)
}
