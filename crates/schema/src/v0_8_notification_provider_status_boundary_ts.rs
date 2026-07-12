pub fn v0_8_notification_provider_status_boundary_typescript() -> String {
    assemble_template_fragments(&[
        include_str!("v0_8_notification_provider_status_boundary.schemas.template.txt"),
        include_str!("v0_8_notification_provider_status_boundary.validation.template.txt"),
        include_str!("v0_8_notification_provider_status_boundary.sample.template.txt"),
        include_str!("v0_8_notification_provider_status_boundary.decoders.template.txt"),
    ])
}

fn assemble_template_fragments(fragments: &[&str]) -> String {
    fragments
        .iter()
        .map(|fragment| fragment.strip_suffix('\n').unwrap_or(fragment))
        .collect()
}
