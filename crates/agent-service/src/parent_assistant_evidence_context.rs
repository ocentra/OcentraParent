use ocentra_parent_agent_protocol::{
    constants, AgentCommandEnvelope, LogFieldValue, ParentAssistantEvidenceContext,
    ParentEvidenceReference, ParentEvidenceReferenceKind,
};

use crate::activity_surface_store::ActivitySurfaceStoreSnapshot;

pub(crate) fn evidence_contexts_from_command(
    command: &AgentCommandEnvelope,
    activity_snapshot: Option<ActivitySurfaceStoreSnapshot>,
    observed_at: String,
) -> Vec<ParentAssistantEvidenceContext> {
    let allowed_summary =
        string_payload_field(command, constants::field::PARENT_ASSISTANT_EVIDENCE_SUMMARY)
            .unwrap_or_else(|| activity_summary_from_snapshot(activity_snapshot.as_ref()));
    let mut contexts = vec![ParentAssistantEvidenceContext {
        evidence: ParentEvidenceReference {
            evidence_reference_id: activity_snapshot
                .as_ref()
                .and_then(|snapshot| snapshot.last_event_id.clone())
                .unwrap_or_else(|| constants::field::ACTIVITY_DIGEST.to_string()),
            kind: ParentEvidenceReferenceKind::QueryStoreSummary,
            observed_at: activity_snapshot
                .as_ref()
                .and_then(|snapshot| snapshot.last_observed_at.clone())
                .unwrap_or_else(|| observed_at.clone()),
        },
        citation_label: constants::parent_assistant::DEFAULT_CITATION_LABEL.to_string(),
        allowed_summary,
    }];

    if let Some(snapshot) = activity_snapshot {
        if let Some(last_event_id) = snapshot.last_event_id {
            contexts.push(ParentAssistantEvidenceContext {
                evidence: ParentEvidenceReference {
                    evidence_reference_id: last_event_id.clone(),
                    kind: ParentEvidenceReferenceKind::ActivityEvent,
                    observed_at: snapshot.last_observed_at.unwrap_or(observed_at),
                },
                citation_label: constants::parent_assistant::ACTIVITY_EVENT_CITATION_LABEL
                    .to_string(),
                allowed_summary: activity_event_summary(&last_event_id),
            });
        }
    }

    contexts
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
            activity_read_model_summary(snapshot)
        }
        Some(_) => constants::parent_assistant::ACTIVITY_CONTEXT_EMPTY.to_string(),
        None => constants::parent_assistant::ACTIVITY_CONTEXT_UNAVAILABLE.to_string(),
    }
}

fn activity_read_model_summary(snapshot: &ActivitySurfaceStoreSnapshot) -> String {
    let mut summary = constants::parent_assistant::ACTIVITY_CONTEXT_PREFIX.to_string();
    summary.push_str(constants::parent_assistant::ACTIVITY_CONTEXT_RECENT_LABEL);
    summary.push_str(&snapshot.recent_returned.to_string());
    summary.push_str(constants::parent_assistant::ACTIVITY_CONTEXT_SCREEN_LABEL);
    summary.push_str(&snapshot.screen_returned.to_string());
    summary.push_str(constants::parent_assistant::ACTIVITY_CONTEXT_BROWSER_LABEL);
    summary.push_str(&snapshot.browser_returned.to_string());
    summary.push_str(constants::parent_assistant::ACTIVITY_CONTEXT_GAMES_LABEL);
    summary.push_str(&snapshot.games_returned.to_string());
    summary.push_str(constants::parent_assistant::ACTIVITY_CONTEXT_NETWORK_LABEL);
    summary.push_str(&snapshot.network_returned.to_string());
    summary
}

fn activity_event_summary(last_event_id: &str) -> String {
    let mut summary = constants::parent_assistant::ACTIVITY_EVENT_SUMMARY_PREFIX.to_string();
    summary.push_str(last_event_id);
    summary
}

fn string_payload_field(command: &AgentCommandEnvelope, key: &str) -> Option<String> {
    match command.payload.get(key) {
        Some(LogFieldValue::String(value)) if !value.trim().is_empty() => {
            Some(value.trim().to_string())
        }
        _ => None,
    }
}
