use ocentra_parent_agent_protocol::constants;

use crate::{BrowserRuntimeEventPayload, BrowserRuntimePhase, BrowserRuntimeReport};

pub(super) fn handoff_summary(
    report: &BrowserRuntimeReport,
) -> Option<(usize, String, String, String, String, String)> {
    if report.intervention_command_published() {
        return None;
    }
    let candidates = report
        .stored_events
        .iter()
        .filter_map(candidate_refs)
        .collect::<Vec<_>>();
    let (policy_preview_id, action_intent_id, event_ref, outbox_ref, handoff_ref) =
        candidates.first()?.clone();
    Some((
        candidates.len(),
        policy_preview_id,
        action_intent_id,
        event_ref,
        outbox_ref,
        handoff_ref,
    ))
}

fn candidate_refs(
    event: &ocentra_eventing::StoredEventEnvelope,
) -> Option<(String, String, String, String, String)> {
    let decoded = event.decode::<BrowserRuntimeEventPayload>().ok()?;
    let payload = decoded.payload;
    if payload.phase != BrowserRuntimePhase::PolicyDecisionCompleted || !payload.dry_run {
        return None;
    }
    let event_ref = browser_event_ref(&payload);
    Some((
        payload.policy_preview_id?,
        payload.action_intent_id?,
        event_ref,
        constants::browser::TEST_BROWSER_RUNTIME_ACTION_INTENT_OUTBOX_REF.to_string(),
        constants::browser::TEST_BROWSER_RUNTIME_ACTION_INTENT_HANDOFF_REF.to_string(),
    ))
}

fn browser_event_ref(payload: &BrowserRuntimeEventPayload) -> String {
    let mut value = String::from(constants::browser::CORRELATION_BROWSER_RUNTIME_PREFIX);
    value.push_str(&payload.evidence_ref);
    value.push(constants::delimiter::HYPHEN);
    value.push_str(&payload.observed_at);
    value.push(constants::delimiter::HYPHEN);
    value.push_str(payload.phase.event_type());
    value
}
