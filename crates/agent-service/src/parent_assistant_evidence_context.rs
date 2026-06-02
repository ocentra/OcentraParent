use ocentra_parent_agent_protocol::{
    constants, ActivityHistoricalReportList, ActivityReadModelState, ActivityReportDocument,
    ActivityReportSectionKind, ActivitySavedReportState, AgentCommandEnvelope, LogFieldValue,
    ParentAssistantEvidenceContext, ParentEvidenceReference, ParentEvidenceReferenceKind,
};

use crate::{
    activity_surface_request::report_document_from_command,
    activity_surface_store::ActivitySurfaceStoreSnapshot,
};

pub(crate) fn evidence_contexts_from_command(
    command: &AgentCommandEnvelope,
    activity_snapshot: Option<ActivitySurfaceStoreSnapshot>,
    stored_report_history: Option<ActivityHistoricalReportList>,
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

    if let Some(report) = report_document_from_command(command)
        .or_else(|| report_document_from_history_command(command))
        .or_else(|| {
            stored_report_history
                .as_ref()
                .and_then(|history| history.reports.first())
                .map(|history_item| history_item.parsed_report.clone())
        })
    {
        contexts.push(report_evidence_context(&report));
    }

    contexts
}

fn report_document_from_history_command(
    command: &AgentCommandEnvelope,
) -> Option<ActivityReportDocument> {
    string_payload_field(command, constants::field::ACTIVITY_REPORTS)
        .and_then(|value| serde_json::from_str::<ActivityHistoricalReportList>(&value).ok())
        .and_then(|history| {
            history
                .reports
                .first()
                .map(|history_item| history_item.parsed_report.clone())
        })
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

fn report_evidence_context(report: &ActivityReportDocument) -> ParentAssistantEvidenceContext {
    ParentAssistantEvidenceContext {
        evidence: ParentEvidenceReference {
            evidence_reference_id: report.report_id.clone(),
            kind: ParentEvidenceReferenceKind::QueryStoreSummary,
            observed_at: report.generated_at.clone(),
        },
        citation_label: constants::parent_assistant::ACTIVITY_REPORT_CITATION_LABEL.to_string(),
        allowed_summary: report_context_summary(report),
    }
}

fn report_context_summary(report: &ActivityReportDocument) -> String {
    let mut summary = constants::parent_assistant::ACTIVITY_REPORT_SUMMARY_PREFIX.to_string();
    summary.push_str(constants::parent_assistant::ACTIVITY_REPORT_SUMMARY_ID_LABEL);
    summary.push_str(&report.report_id);
    summary.push_str(constants::parent_assistant::ACTIVITY_REPORT_SUMMARY_STATE_LABEL);
    summary.push_str(saved_state_label(report));
    summary.push_str(constants::parent_assistant::ACTIVITY_REPORT_SUMMARY_FILE_LABEL);
    summary.push_str(saved_metadata_value(report, SavedMetadataValue::FileName).as_str());
    summary.push_str(constants::parent_assistant::ACTIVITY_REPORT_SUMMARY_SAVED_AT_LABEL);
    summary.push_str(saved_metadata_value(report, SavedMetadataValue::SavedAt).as_str());
    summary.push_str(constants::parent_assistant::ACTIVITY_REPORT_SUMMARY_STORAGE_REASON_LABEL);
    summary.push_str(saved_metadata_value(report, SavedMetadataValue::StorageReason).as_str());
    summary.push_str(constants::parent_assistant::ACTIVITY_REPORT_SUMMARY_SECTIONS_LABEL);
    summary.push_str(&report.sections.len().to_string());
    summary.push_str(constants::parent_assistant::ACTIVITY_REPORT_SUMMARY_SOURCE_LABEL);
    summary.push_str(&report.source_states.len().to_string());
    summary.push_str(constants::parent_assistant::ACTIVITY_REPORT_SUMMARY_READY_SECTIONS_LABEL);
    summary.push_str(&count_sections_with_state(report, ActivityReadModelState::Ready).to_string());
    summary.push_str(constants::parent_assistant::ACTIVITY_REPORT_SUMMARY_OFFLINE_SOURCES_LABEL);
    summary
        .push_str(&count_sources_with_state(report, ActivityReadModelState::Offline).to_string());
    summary.push_str(constants::parent_assistant::ACTIVITY_REPORT_SUMMARY_STALE_SOURCES_LABEL);
    summary.push_str(&count_sources_with_state(report, ActivityReadModelState::Stale).to_string());
    summary
        .push_str(constants::parent_assistant::ACTIVITY_REPORT_SUMMARY_UNAVAILABLE_SOURCES_LABEL);
    summary.push_str(
        &count_sources_with_state(report, ActivityReadModelState::Unavailable).to_string(),
    );
    summary
        .push_str(constants::parent_assistant::ACTIVITY_REPORT_SUMMARY_UNREACHABLE_SOURCES_LABEL);
    summary.push_str(
        &count_sources_with_reachability(
            report,
            ocentra_parent_agent_protocol::ActivityReportSourceReachabilityState::Unreachable,
        )
        .to_string(),
    );
    summary.push_str(constants::parent_assistant::ACTIVITY_REPORT_SUMMARY_SECTION_KINDS_LABEL);
    summary.push_str(&section_kind_labels(report));
    summary.push_str(constants::parent_assistant::ACTIVITY_REPORT_SUMMARY_OFFLINE_SOURCE_IDS_LABEL);
    summary.push_str(&source_ids_with_state(
        report,
        ActivityReadModelState::Offline,
    ));
    summary.push_str(constants::parent_assistant::ACTIVITY_REPORT_SUMMARY_STALE_SOURCE_IDS_LABEL);
    summary.push_str(&source_ids_with_state(
        report,
        ActivityReadModelState::Stale,
    ));
    summary.push_str(
        constants::parent_assistant::ACTIVITY_REPORT_SUMMARY_UNREACHABLE_SOURCE_IDS_LABEL,
    );
    summary.push_str(&source_ids_with_reachability(
        report,
        ocentra_parent_agent_protocol::ActivityReportSourceReachabilityState::Unreachable,
    ));
    summary.push_str(
        constants::parent_assistant::ACTIVITY_REPORT_SUMMARY_UNAVAILABLE_SOURCE_IDS_LABEL,
    );
    summary.push_str(&source_ids_with_state(
        report,
        ActivityReadModelState::Unavailable,
    ));
    summary
}

fn count_sections_with_state(
    report: &ActivityReportDocument,
    state: ActivityReadModelState,
) -> usize {
    report
        .sections
        .iter()
        .filter(|section| section.state == state)
        .count()
}

fn count_sources_with_state(
    report: &ActivityReportDocument,
    state: ActivityReadModelState,
) -> usize {
    report
        .source_states
        .iter()
        .filter(|source| source.state == state)
        .count()
}

fn count_sources_with_reachability(
    report: &ActivityReportDocument,
    state: ocentra_parent_agent_protocol::ActivityReportSourceReachabilityState,
) -> usize {
    report
        .source_states
        .iter()
        .filter(|source| source.reachability_state == state)
        .count()
}

fn section_kind_labels(report: &ActivityReportDocument) -> String {
    joined_or_none(
        report
            .sections
            .iter()
            .map(|section| section_kind_label(section.section_kind)),
    )
}

fn source_ids_with_state(report: &ActivityReportDocument, state: ActivityReadModelState) -> String {
    joined_or_none(
        report
            .source_states
            .iter()
            .filter(|source| source.state == state)
            .map(|source| source.device_id.as_str()),
    )
}

fn source_ids_with_reachability(
    report: &ActivityReportDocument,
    state: ocentra_parent_agent_protocol::ActivityReportSourceReachabilityState,
) -> String {
    joined_or_none(
        report
            .source_states
            .iter()
            .filter(|source| source.reachability_state == state)
            .map(|source| source.device_id.as_str()),
    )
}

fn joined_or_none<'a>(values: impl Iterator<Item = &'a str>) -> String {
    let collected = values.collect::<Vec<_>>();
    if collected.is_empty() {
        return constants::parent_assistant::ACTIVITY_REPORT_SUMMARY_NONE.to_string();
    }

    collected.join(&constants::delimiter::LIST.to_string())
}

fn section_kind_label(kind: ActivityReportSectionKind) -> &'static str {
    match kind {
        ActivityReportSectionKind::Summary => constants::activity_surface::SECTION_SUMMARY,
        ActivityReportSectionKind::Screen => constants::activity_surface::SECTION_SCREEN,
        ActivityReportSectionKind::AppUse => constants::activity_surface::SECTION_APP_USE,
        ActivityReportSectionKind::Browser => constants::activity_surface::SECTION_BROWSER,
        ActivityReportSectionKind::Games => constants::activity_surface::SECTION_GAMES,
        ActivityReportSectionKind::Network => constants::activity_surface::SECTION_NETWORK,
    }
}

fn saved_state_label(report: &ActivityReportDocument) -> &'static str {
    match report
        .saved_metadata
        .as_ref()
        .map(|metadata| metadata.saved_state)
        .unwrap_or(ActivitySavedReportState::Draft)
    {
        ActivitySavedReportState::Draft => constants::activity_surface::SAVED_STATE_DRAFT,
        ActivitySavedReportState::Saved => constants::activity_surface::SAVED_STATE_SAVED,
        ActivitySavedReportState::StorageUnavailable => {
            constants::activity_surface::SAVED_STATE_STORAGE_UNAVAILABLE
        }
        ActivitySavedReportState::Degraded => constants::activity_surface::SAVED_STATE_DEGRADED,
        ActivitySavedReportState::ScaffoldOnly => {
            constants::activity_surface::SAVED_STATE_SCAFFOLD_ONLY
        }
    }
}

fn saved_metadata_value(report: &ActivityReportDocument, value: SavedMetadataValue) -> String {
    let Some(metadata) = report.saved_metadata.as_ref() else {
        return constants::parent_assistant::ACTIVITY_REPORT_SUMMARY_NONE.to_string();
    };

    match value {
        SavedMetadataValue::FileName => metadata.file_name.clone(),
        SavedMetadataValue::SavedAt => metadata.saved_at.clone().unwrap_or_else(|| {
            constants::parent_assistant::ACTIVITY_REPORT_SUMMARY_NONE.to_string()
        }),
        SavedMetadataValue::StorageReason => metadata.storage_reason.clone().unwrap_or_else(|| {
            constants::parent_assistant::ACTIVITY_REPORT_SUMMARY_NONE.to_string()
        }),
    }
}

enum SavedMetadataValue {
    FileName,
    SavedAt,
    StorageReason,
}

fn string_payload_field(command: &AgentCommandEnvelope, key: &str) -> Option<String> {
    match command.payload.get(key) {
        Some(LogFieldValue::String(value)) if !value.trim().is_empty() => {
            Some(value.trim().to_string())
        }
        _ => None,
    }
}
