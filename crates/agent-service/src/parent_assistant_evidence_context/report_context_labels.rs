use ocentra_parent_agent_protocol::activity_surface::ActivityReportSectionKind;
use ocentra_parent_agent_protocol::constants;

use super::ParentAssistantTextRef;

pub(super) fn section_kind_label(
    kind: ActivityReportSectionKind,
) -> ParentAssistantTextRef<'static> {
    match kind {
        ActivityReportSectionKind::Summary => {
            ParentAssistantTextRef(constants::activity_surface::SECTION_SUMMARY)
        }
        ActivityReportSectionKind::Screen => {
            ParentAssistantTextRef(constants::activity_surface::SECTION_SCREEN)
        }
        ActivityReportSectionKind::AppUse => {
            ParentAssistantTextRef(constants::activity_surface::SECTION_APP_USE)
        }
        ActivityReportSectionKind::Browser => {
            ParentAssistantTextRef(constants::activity_surface::SECTION_BROWSER)
        }
        ActivityReportSectionKind::Games => {
            ParentAssistantTextRef(constants::activity_surface::SECTION_GAMES)
        }
        ActivityReportSectionKind::Network => {
            ParentAssistantTextRef(constants::activity_surface::SECTION_NETWORK)
        }
    }
}
