use ocentra_parent_agent_protocol::activity::policy::ParentEvidenceReferenceKind;
use ocentra_parent_agent_protocol::activity_surface::ActivityHistoricalReportList;
use ocentra_parent_agent_protocol::activity_surface::ActivityReportDocument;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::parent_assistant::ParentAssistantEvidenceContext;
use ocentra_parent_agent_protocol::transport::AgentCommandEnvelope;

use super::ParentAssistantPayloadFieldName;
use super::ParentAssistantText;
use super::ParentAssistantTextRef;
use crate::activity_surface_request::report_document_from_command;
use crate::activity_surface_store::ActivitySurfaceStoreSnapshot;

pub(super) fn allowed_summary_from_command(
    command: &AgentCommandEnvelope,
    activity_snapshot: Option<&ActivitySurfaceStoreSnapshot>,
) -> ParentAssistantText {
    string_payload_field(
        command,
        ParentAssistantPayloadFieldName(constants::field::PARENT_ASSISTANT_EVIDENCE_SUMMARY),
    )
    .unwrap_or_else(|| activity_summary_from_snapshot(activity_snapshot))
}

pub(super) fn activity_summary_from_snapshot(
    snapshot: Option<&ActivitySurfaceStoreSnapshot>,
) -> ParentAssistantText {
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
        Some(_) => {
            ParentAssistantText(constants::parent_assistant::ACTIVITY_CONTEXT_EMPTY.to_string())
        }
        None => ParentAssistantText(
            constants::parent_assistant::ACTIVITY_CONTEXT_UNAVAILABLE.to_string(),
        ),
    }
}

pub(super) fn activity_read_model_summary(
    snapshot: &ActivitySurfaceStoreSnapshot,
) -> ParentAssistantText {
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
    ParentAssistantText(summary)
}

pub(super) fn activity_event_context(
    snapshot: Option<&ActivitySurfaceStoreSnapshot>,
    observed_at: &ParentAssistantText,
) -> Option<ParentAssistantEvidenceContext> {
    let snapshot = snapshot?;
    let last_event_id = snapshot.last_event_id.as_ref()?;
    Some(ParentAssistantEvidenceContext {
        evidence: ocentra_parent_agent_protocol::activity::policy::ParentEvidenceReference {
            evidence_reference_id: last_event_id.clone(),
            kind: ParentEvidenceReferenceKind::ActivityEvent,
            observed_at: snapshot
                .last_observed_at
                .clone()
                .unwrap_or_else(|| observed_at.0.clone()),
        },
        citation_label: constants::parent_assistant::ACTIVITY_EVENT_CITATION_LABEL.to_string(),
        allowed_summary: activity_event_summary(ParentAssistantTextRef(last_event_id.as_str())).0,
        custody_label: constants::parent_assistant::EVIDENCE_CUSTODY_ACTIVITY_EVENT.to_string(),
        source_label: constants::parent_assistant::EVIDENCE_SOURCE_ACTIVITY_EVENT_CITATION
            .to_string(),
        raw_child_evidence_included: false,
        direct_enforcement_allowed: false,
    })
}

pub(super) fn report_document_from_sources(
    command: &AgentCommandEnvelope,
    stored_report_history: Option<ActivityHistoricalReportList>,
) -> Option<ActivityReportDocument> {
    report_document_from_command(command)
        .or_else(|| report_document_from_history_command(command))
        .or_else(|| {
            stored_report_history.and_then(|history| {
                history
                    .reports
                    .first()
                    .map(|history_item| history_item.parsed_report.clone())
            })
        })
}

pub(super) fn report_document_from_history_command(
    command: &AgentCommandEnvelope,
) -> Option<ActivityReportDocument> {
    string_payload_field(
        command,
        ParentAssistantPayloadFieldName(constants::field::ACTIVITY_REPORTS),
    )
    .and_then(|value| serde_json::from_str::<ActivityHistoricalReportList>(&value.0).ok())
    .and_then(|history| {
        history
            .reports
            .first()
            .map(|history_item| history_item.parsed_report.clone())
    })
}

pub(super) fn string_payload_field(
    command: &AgentCommandEnvelope,
    payload_field_name: ParentAssistantPayloadFieldName,
) -> Option<ParentAssistantText> {
    match command.payload.get(payload_field_name.0) {
        Some(LogFieldValue::String(value)) if !value.trim().is_empty() => {
            Some(ParentAssistantText(value.trim().to_string()))
        }
        _ => None,
    }
}

fn activity_event_summary(last_event_id: ParentAssistantTextRef<'_>) -> ParentAssistantText {
    let mut summary = constants::parent_assistant::ACTIVITY_EVENT_SUMMARY_PREFIX.to_string();
    summary.push_str(last_event_id.0);
    ParentAssistantText(summary)
}
