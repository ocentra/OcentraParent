use ocentra_parent_agent_protocol::activity::policy::ParentEvidenceReference;
use ocentra_parent_agent_protocol::activity::policy::ParentEvidenceReferenceKind;
use ocentra_parent_agent_protocol::activity_surface::ActivityReadModelState;
use ocentra_parent_agent_protocol::activity_surface::ActivityReportDocument;
use ocentra_parent_agent_protocol::activity_surface::ActivityReportSourceReachabilityState;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::parent_assistant::ParentAssistantEvidenceContext;

use super::report_context_counts::count_sections_with_state;
use super::report_context_counts::count_sources_with_reachability;
use super::report_context_counts::count_sources_with_state;
use super::report_context_counts::section_kind_labels;
use super::report_context_counts::source_ids_with_reachability;
use super::report_context_counts::source_ids_with_state;
use super::report_context_saved_metadata::saved_metadata_value;
use super::report_context_saved_metadata::SavedMetadataValue;
use super::report_context_saved_state::saved_state_label;
use super::ParentAssistantText;
use super::ParentAssistantTextRef;

pub(super) fn report_evidence_context(
    report: &ActivityReportDocument,
) -> ParentAssistantEvidenceContext {
    ParentAssistantEvidenceContext {
        evidence: ParentEvidenceReference {
            evidence_reference_id: report.report_id.clone(),
            kind: ParentEvidenceReferenceKind::QueryStoreSummary,
            observed_at: report.generated_at.clone(),
        },
        citation_label: constants::parent_assistant::ACTIVITY_REPORT_CITATION_LABEL.to_string(),
        allowed_summary: report_context_summary(report).0,
        custody_label: constants::parent_assistant::EVIDENCE_CUSTODY_ACTIVITY_REPORT.to_string(),
        source_label: constants::parent_assistant::EVIDENCE_SOURCE_SAVED_ACTIVITY_REPORT_HISTORY
            .to_string(),
        raw_child_evidence_included: false,
        direct_enforcement_allowed: false,
    }
}

pub(super) fn report_context_summary(report: &ActivityReportDocument) -> ParentAssistantText {
    let mut builder = ReportContextSummaryBuilder::new();
    let storage_labels = ReportContextStorageLabels::from_report(report);

    append_saved_metadata(&mut builder, report);
    append_section_counts(&mut builder, report);
    append_source_ids(&mut builder, report);
    append_storage_labels(&mut builder, &storage_labels);

    builder.build()
}

#[derive(Clone, Debug)]
struct ReportContextStorageLabels {
    custody_label: ParentAssistantText,
    source_label: ParentAssistantText,
    raw_child_evidence_included: bool,
}

impl ReportContextStorageLabels {
    fn from_report(report: &ActivityReportDocument) -> Self {
        let metadata = report.saved_metadata.as_ref();
        Self {
            custody_label: metadata
                .map(|_| {
                    ParentAssistantText(
                        constants::activity_surface::CUSTODY_PARENT_DEVICE_LOCAL_REPORT_JSON
                            .to_string(),
                    )
                })
                .unwrap_or_else(activity_report_summary_none),
            source_label: metadata
                .map(|_| {
                    ParentAssistantText(
                        constants::activity_surface::SOURCE_SAVED_REPORT_JSON.to_string(),
                    )
                })
                .unwrap_or_else(activity_report_summary_none),
            raw_child_evidence_included: metadata
                .map(|value| value.raw_child_evidence_included)
                .unwrap_or(false),
        }
    }
}

#[derive(Clone, Debug)]
struct ReportContextSummaryBuilder(ParentAssistantText);

impl ReportContextSummaryBuilder {
    fn new() -> Self {
        Self(ParentAssistantText(
            constants::parent_assistant::ACTIVITY_REPORT_SUMMARY_PREFIX.to_string(),
        ))
    }

    fn push_ref_value(
        &mut self,
        label: ParentAssistantTextRef<'_>,
        value: ParentAssistantTextRef<'_>,
    ) {
        self.0 .0.push_str(label.0);
        self.0 .0.push_str(value.0);
    }

    fn push_text_value(&mut self, label: ParentAssistantTextRef<'_>, value: &ParentAssistantText) {
        self.0 .0.push_str(label.0);
        self.0 .0.push_str(&value.0);
    }

    fn push_count(&mut self, label: ParentAssistantTextRef<'_>, value: usize) {
        let count_text = ParentAssistantText(value.to_string());
        self.push_text_value(label, &count_text);
    }

    fn push_bool(&mut self, label: ParentAssistantTextRef<'_>, value: bool) {
        let bool_text = ParentAssistantText(value.to_string());
        self.push_text_value(label, &bool_text);
    }

    fn build(self) -> ParentAssistantText {
        self.0
    }
}

fn append_saved_metadata(
    builder: &mut ReportContextSummaryBuilder,
    report: &ActivityReportDocument,
) {
    builder.push_ref_value(
        ParentAssistantTextRef(constants::parent_assistant::ACTIVITY_REPORT_SUMMARY_ID_LABEL),
        ParentAssistantTextRef(report.report_id.as_str()),
    );
    builder.push_ref_value(
        ParentAssistantTextRef(constants::parent_assistant::ACTIVITY_REPORT_SUMMARY_STATE_LABEL),
        saved_state_label(report),
    );
    builder.push_text_value(
        ParentAssistantTextRef(constants::parent_assistant::ACTIVITY_REPORT_SUMMARY_FILE_LABEL),
        &saved_metadata_value(report, &SavedMetadataValue::FileName),
    );
    builder.push_text_value(
        ParentAssistantTextRef(constants::parent_assistant::ACTIVITY_REPORT_SUMMARY_SAVED_AT_LABEL),
        &saved_metadata_value(report, &SavedMetadataValue::SavedAt),
    );
    builder.push_text_value(
        ParentAssistantTextRef(
            constants::parent_assistant::ACTIVITY_REPORT_SUMMARY_STORAGE_REASON_LABEL,
        ),
        &saved_metadata_value(report, &SavedMetadataValue::StorageReason),
    );
}

fn append_section_counts(
    builder: &mut ReportContextSummaryBuilder,
    report: &ActivityReportDocument,
) {
    builder.push_count(
        ParentAssistantTextRef(constants::parent_assistant::ACTIVITY_REPORT_SUMMARY_SECTIONS_LABEL),
        report.sections.len(),
    );
    builder.push_count(
        ParentAssistantTextRef(
            constants::parent_assistant::ACTIVITY_REPORT_SUMMARY_READY_SECTIONS_LABEL,
        ),
        count_sections_with_state(report, ActivityReadModelState::Ready),
    );
    builder.push_count(
        ParentAssistantTextRef(
            constants::parent_assistant::ACTIVITY_REPORT_SUMMARY_OFFLINE_SOURCES_LABEL,
        ),
        count_sources_with_state(report, ActivityReadModelState::Offline),
    );
    builder.push_count(
        ParentAssistantTextRef(
            constants::parent_assistant::ACTIVITY_REPORT_SUMMARY_STALE_SOURCES_LABEL,
        ),
        count_sources_with_state(report, ActivityReadModelState::Stale),
    );
    builder.push_count(
        ParentAssistantTextRef(
            constants::parent_assistant::ACTIVITY_REPORT_SUMMARY_UNAVAILABLE_SOURCES_LABEL,
        ),
        count_sources_with_state(report, ActivityReadModelState::Unavailable),
    );
    builder.push_count(
        ParentAssistantTextRef(
            constants::parent_assistant::ACTIVITY_REPORT_SUMMARY_UNREACHABLE_SOURCES_LABEL,
        ),
        count_sources_with_reachability(report, ActivityReportSourceReachabilityState::Unreachable),
    );
    builder.push_text_value(
        ParentAssistantTextRef(
            constants::parent_assistant::ACTIVITY_REPORT_SUMMARY_SECTION_KINDS_LABEL,
        ),
        &section_kind_labels(report),
    );
}

fn append_source_ids(builder: &mut ReportContextSummaryBuilder, report: &ActivityReportDocument) {
    builder.push_text_value(
        ParentAssistantTextRef(
            constants::parent_assistant::ACTIVITY_REPORT_SUMMARY_OFFLINE_SOURCE_IDS_LABEL,
        ),
        &source_ids_with_state(report, ActivityReadModelState::Offline),
    );
    builder.push_text_value(
        ParentAssistantTextRef(
            constants::parent_assistant::ACTIVITY_REPORT_SUMMARY_STALE_SOURCE_IDS_LABEL,
        ),
        &source_ids_with_state(report, ActivityReadModelState::Stale),
    );
    builder.push_text_value(
        ParentAssistantTextRef(
            constants::parent_assistant::ACTIVITY_REPORT_SUMMARY_UNREACHABLE_SOURCE_IDS_LABEL,
        ),
        &source_ids_with_reachability(report, ActivityReportSourceReachabilityState::Unreachable),
    );
    builder.push_text_value(
        ParentAssistantTextRef(
            constants::parent_assistant::ACTIVITY_REPORT_SUMMARY_UNAVAILABLE_SOURCE_IDS_LABEL,
        ),
        &source_ids_with_state(report, ActivityReadModelState::Unavailable),
    );
}

fn append_storage_labels(
    builder: &mut ReportContextSummaryBuilder,
    storage_labels: &ReportContextStorageLabels,
) {
    builder.push_text_value(
        ParentAssistantTextRef(constants::parent_assistant::ACTIVITY_REPORT_SUMMARY_CUSTODY_LABEL),
        &storage_labels.custody_label,
    );
    builder.push_text_value(
        ParentAssistantTextRef(
            constants::parent_assistant::ACTIVITY_REPORT_SUMMARY_SOURCE_DATA_LABEL,
        ),
        &storage_labels.source_label,
    );
    builder.push_bool(
        ParentAssistantTextRef(
            constants::parent_assistant::ACTIVITY_REPORT_SUMMARY_RAW_CHILD_EVIDENCE_LABEL,
        ),
        storage_labels.raw_child_evidence_included,
    );
}

fn activity_report_summary_none() -> ParentAssistantText {
    ParentAssistantText(constants::parent_assistant::ACTIVITY_REPORT_SUMMARY_NONE.to_string())
}
