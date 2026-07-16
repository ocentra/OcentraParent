use ocentra_parent_agent_protocol::activity_surface::ActivityReadModelState;
use ocentra_parent_agent_protocol::activity_surface::ActivityReportDocument;
use ocentra_parent_agent_protocol::constants;

use super::report_context_labels::section_kind_label;
use super::ParentAssistantText;
use super::ParentAssistantTextRef;

pub(super) fn count_sections_with_state(
    report: &ActivityReportDocument,
    state: ActivityReadModelState,
) -> usize {
    report
        .sections
        .iter()
        .filter(|section| section.state == state)
        .count()
}

pub(super) fn count_sources_with_state(
    report: &ActivityReportDocument,
    state: ActivityReadModelState,
) -> usize {
    report
        .source_states
        .iter()
        .filter(|source| source.state == state)
        .count()
}

pub(super) fn count_sources_with_reachability(
    report: &ActivityReportDocument,
    state: ocentra_parent_agent_protocol::activity_surface::ActivityReportSourceReachabilityState,
) -> usize {
    report
        .source_states
        .iter()
        .filter(|source| source.reachability_state == state)
        .count()
}

pub(super) fn section_kind_labels(report: &ActivityReportDocument) -> ParentAssistantText {
    joined_or_none(
        report
            .sections
            .iter()
            .map(|section| section_kind_label(section.section_kind)),
    )
}

pub(super) fn source_ids_with_state(
    report: &ActivityReportDocument,
    state: ActivityReadModelState,
) -> ParentAssistantText {
    joined_or_none(
        report
            .source_states
            .iter()
            .filter(|source| source.state == state)
            .map(|source| ParentAssistantTextRef(source.device_id.as_str())),
    )
}

pub(super) fn source_ids_with_reachability(
    report: &ActivityReportDocument,
    state: ocentra_parent_agent_protocol::activity_surface::ActivityReportSourceReachabilityState,
) -> ParentAssistantText {
    joined_or_none(
        report
            .source_states
            .iter()
            .filter(|source| source.reachability_state == state)
            .map(|source| ParentAssistantTextRef(source.device_id.as_str())),
    )
}

pub(super) fn joined_or_none<'a>(
    values: impl Iterator<Item = ParentAssistantTextRef<'a>>,
) -> ParentAssistantText {
    let collected = values.collect::<Vec<_>>();
    if collected.is_empty() {
        return ParentAssistantText(
            constants::parent_assistant::ACTIVITY_REPORT_SUMMARY_NONE.to_string(),
        );
    }

    let separator = constants::delimiter::LIST.to_string();
    ParentAssistantText(
        collected
            .iter()
            .map(|value| value.0)
            .collect::<Vec<_>>()
            .join(separator.as_str()),
    )
}
