use ocentra_parent_agent_protocol::{
    constants, AgentCommandEnvelope, LogFieldValue, ParentAssistantEvidenceContext,
    ParentEvidenceReference, ParentEvidenceReferenceKind,
};

use crate::activity_surface_store::ActivitySurfaceStoreSnapshot;

pub(crate) fn evidence_context_from_command(
    command: &AgentCommandEnvelope,
    activity_snapshot: Option<ActivitySurfaceStoreSnapshot>,
    observed_at: String,
) -> ParentAssistantEvidenceContext {
    let allowed_summary =
        string_payload_field(command, constants::field::PARENT_ASSISTANT_EVIDENCE_SUMMARY)
            .unwrap_or_else(|| activity_summary_from_snapshot(activity_snapshot.as_ref()));
    ParentAssistantEvidenceContext {
        evidence: ParentEvidenceReference {
            evidence_reference_id: activity_snapshot
                .and_then(|snapshot| snapshot.last_event_id)
                .unwrap_or_else(|| constants::field::ACTIVITY_DIGEST.to_string()),
            kind: ParentEvidenceReferenceKind::QueryStoreSummary,
            observed_at,
        },
        citation_label: constants::parent_assistant::DEFAULT_CITATION_LABEL.to_string(),
        allowed_summary,
    }
}

fn activity_summary_from_snapshot(snapshot: Option<&ActivitySurfaceStoreSnapshot>) -> String {
    match snapshot {
        Some(snapshot)
            if snapshot.recent_returned > 0
                || snapshot.browser_returned > 0
                || snapshot.network_returned > 0
                || snapshot.games_returned > 0
                || snapshot.screen_returned > 0 =>
        {
            constants::parent_assistant::ACTIVITY_CONTEXT_READY.to_string()
        }
        Some(_) => constants::parent_assistant::ACTIVITY_CONTEXT_EMPTY.to_string(),
        None => constants::parent_assistant::ACTIVITY_CONTEXT_UNAVAILABLE.to_string(),
    }
}

fn string_payload_field(command: &AgentCommandEnvelope, key: &str) -> Option<String> {
    match command.payload.get(key) {
        Some(LogFieldValue::String(value)) if !value.trim().is_empty() => {
            Some(value.trim().to_string())
        }
        _ => None,
    }
}
