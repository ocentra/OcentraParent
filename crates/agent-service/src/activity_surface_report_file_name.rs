use ocentra_parent_agent_protocol::{constants, ActivityReportDocument, ActivitySurfaceScopeKind};

pub(crate) fn report_file_name(report: &ActivityReportDocument) -> String {
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

    let mut name: String = raw_name
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
    if name.is_empty() {
        name.push_str(constants::activity_surface::REPORT_ID_FALLBACK);
    }
    name.push(constants::delimiter::DOT);
    name.push_str(constants::activity_surface::REPORT_FILE_EXTENSION);
    name
}
