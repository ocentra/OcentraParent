#[path = "parent_assistant_evidence_context/activity_summary.rs"]
mod activity_summary;
#[path = "parent_assistant_evidence_context/report_context.rs"]
mod report_context;
#[path = "parent_assistant_evidence_context/report_context_counts.rs"]
mod report_context_counts;
#[path = "parent_assistant_evidence_context/report_context_labels.rs"]
mod report_context_labels;
#[path = "parent_assistant_evidence_context/report_context_saved_metadata.rs"]
mod report_context_saved_metadata;
#[path = "parent_assistant_evidence_context/report_context_saved_state.rs"]
mod report_context_saved_state;

use ocentra_parent_agent_protocol::activity::policy::ParentEvidenceReference;
use ocentra_parent_agent_protocol::activity::policy::ParentEvidenceReferenceKind;
use ocentra_parent_agent_protocol::activity_surface::ActivityHistoricalReportList;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::parent_assistant::ParentAssistantEvidenceContext;
use ocentra_parent_agent_protocol::transport::AgentCommandEnvelope;

use crate::parent_assistant_activity_snapshot::ParentAssistantActivitySnapshot;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ParentAssistantText(String);

pub(crate) trait IntoParentAssistantText {
    fn into_parent_assistant_text(self) -> ParentAssistantText;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ParentAssistantTextRef<'a>(&'a str);

impl IntoParentAssistantText for String {
    fn into_parent_assistant_text(self) -> ParentAssistantText {
        ParentAssistantText(self)
    }
}

impl IntoParentAssistantText for &str {
    fn into_parent_assistant_text(self) -> ParentAssistantText {
        ParentAssistantText(self.to_string())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ParentAssistantPayloadFieldName(&'static str);

struct ParentAssistantActivitySnapshotValue(Option<ParentAssistantActivitySnapshot>);

pub(crate) fn evidence_contexts_from_command(
    command: &AgentCommandEnvelope,
    activity_snapshot: Option<ParentAssistantActivitySnapshot>,
    stored_report_history: Option<ActivityHistoricalReportList>,
    observed_at: impl IntoParentAssistantText,
) -> Vec<ParentAssistantEvidenceContext> {
    let observed_at = observed_at.into_parent_assistant_text();
    let activity_snapshot = ParentAssistantActivitySnapshotValue(activity_snapshot);
    let allowed_summary =
        activity_summary::allowed_summary_from_command(command, activity_snapshot.0.as_ref());
    let mut contexts = vec![ParentAssistantEvidenceContext {
        evidence: ParentEvidenceReference {
            evidence_reference_id: activity_snapshot
                .0
                .as_ref()
                .and_then(|snapshot| snapshot.last_event_id.clone())
                .unwrap_or_else(|| constants::field::ACTIVITY_DIGEST.to_string()),
            kind: ParentEvidenceReferenceKind::QueryStoreSummary,
            observed_at: activity_snapshot
                .0
                .as_ref()
                .and_then(|snapshot| snapshot.last_observed_at.clone())
                .unwrap_or_else(|| observed_at.0.clone()),
        },
        citation_label: constants::parent_assistant::DEFAULT_CITATION_LABEL.to_string(),
        allowed_summary: allowed_summary.0,
        custody_label: constants::parent_assistant::EVIDENCE_CUSTODY_ACTIVITY_SUMMARY.to_string(),
        source_label: constants::parent_assistant::EVIDENCE_SOURCE_ACTIVITY_QUERY_STORE_SUMMARY
            .to_string(),
        raw_child_evidence_included: false,
        direct_enforcement_allowed: false,
    }];

    if let Some(context) =
        activity_summary::activity_event_context(activity_snapshot.0.as_ref(), &observed_at)
    {
        contexts.push(context);
    }

    if let Some(report) =
        activity_summary::report_document_from_sources(command, stored_report_history)
    {
        contexts.push(report_context::report_evidence_context(&report));
    }

    contexts
}
