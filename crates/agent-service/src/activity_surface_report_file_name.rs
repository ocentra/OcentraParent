use ocentra_parent_agent_protocol::activity_surface::{
    ActivityReportDocument, ActivitySurfaceScopeKind,
};
use ocentra_parent_agent_protocol::constants;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ReportFileName(pub(crate) String);

pub(crate) fn report_file_name(report: &ActivityReportDocument) -> ReportFileName {
    let (scope_name, scope_id) = match report.scope.scope_kind {
        ActivitySurfaceScopeKind::Family => (
            constants::activity_surface::SCOPE_FAMILY,
            report
                .scope
                .family_id
                .as_deref()
                .unwrap_or(constants::activity_surface::DEFAULT_FAMILY_ID),
        ),
        ActivitySurfaceScopeKind::Device => (
            constants::activity_surface::SCOPE_DEVICE,
            report
                .scope
                .device_id
                .as_deref()
                .unwrap_or(constants::activity_surface::DEFAULT_DEVICE_ID),
        ),
    };
    let mut raw_name = report.report_id.clone();
    raw_name.push(constants::delimiter::HYPHEN);
    raw_name.push_str(scope_name);
    raw_name.push(constants::delimiter::HYPHEN);
    raw_name.push_str(scope_id);

    let mut safe_file_name: String = raw_name
        .chars()
        .map(|value| {
            if value.is_ascii_alphanumeric()
                || value == constants::delimiter::HYPHEN
                || value == constants::delimiter::UNDERSCORE
            {
                value
            } else {
                constants::delimiter::HYPHEN
            }
        })
        .collect();
    if safe_file_name.is_empty() {
        safe_file_name.push_str(constants::activity_surface::REPORT_ID_FALLBACK);
    }
    safe_file_name.push(constants::delimiter::DOT);
    safe_file_name.push_str(constants::activity_surface::REPORT_FILE_EXTENSION);
    ReportFileName(safe_file_name)
}
