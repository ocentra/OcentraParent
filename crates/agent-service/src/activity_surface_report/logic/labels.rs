use ocentra_parent_agent_protocol::activity_surface::{
    ActivityReportFrequency, ActivityReportSectionKind,
};
use ocentra_parent_agent_protocol::constants;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct ReportId(pub(super) String);

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct SummaryText(pub(super) String);

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct GeneratedAtText(pub(super) String);

impl std::fmt::Display for SummaryText {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl std::fmt::Display for GeneratedAtText {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

pub(super) fn report_section_summary(item_count: u64) -> SummaryText {
    if item_count > 0 {
        SummaryText(constants::activity_surface::SUMMARY_READY.to_string())
    } else {
        SummaryText(constants::activity_surface::SUMMARY_EMPTY.to_string())
    }
}

pub(super) fn section_title(kind: ActivityReportSectionKind) -> SummaryText {
    if kind == ActivityReportSectionKind::Summary {
        SummaryText(constants::activity_surface::SECTION_SUMMARY.to_string())
    } else if kind == ActivityReportSectionKind::Screen {
        SummaryText(constants::activity_surface::SECTION_SCREEN.to_string())
    } else if kind == ActivityReportSectionKind::AppUse {
        SummaryText(constants::activity_surface::SECTION_APP_USE.to_string())
    } else if kind == ActivityReportSectionKind::Browser {
        SummaryText(constants::activity_surface::SECTION_BROWSER.to_string())
    } else if kind == ActivityReportSectionKind::Games {
        SummaryText(constants::activity_surface::SECTION_GAMES.to_string())
    } else {
        SummaryText(constants::activity_surface::SECTION_NETWORK.to_string())
    }
}

pub(super) fn report_id(
    frequency: ActivityReportFrequency,
    generated_at: &GeneratedAtText,
) -> ReportId {
    let prefix = if frequency == ActivityReportFrequency::Daily {
        constants::activity_surface::REPORT_ID_DAILY
    } else if frequency == ActivityReportFrequency::Weekly {
        constants::activity_surface::REPORT_ID_WEEKLY
    } else {
        constants::activity_surface::REPORT_ID_MONTHLY
    };
    let mut id = String::from(prefix);
    id.push(constants::delimiter::HYPHEN);
    id.extend(generated_at.0.chars().filter(char::is_ascii_alphanumeric));
    ReportId(id)
}
