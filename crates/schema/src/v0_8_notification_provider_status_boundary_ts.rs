pub fn v0_8_notification_provider_status_boundary_typescript() -> String {
    assemble_template_fragments(
        include_str!("v0_8_notification_provider_status_boundary.schemas.template.txt"),
        include_str!("v0_8_notification_provider_status_boundary.validation.template.txt"),
        include_str!("v0_8_notification_provider_status_boundary.sample.template.txt"),
        include_str!("v0_8_notification_provider_status_boundary.decoders.template.txt"),
    )
}

fn assemble_template_fragments(
    schemas: &str,
    validation: &str,
    sample: &str,
    decoders: &str,
) -> String {
    format!(
        "{}\n\n{}\n\n{}\n{}\n",
        trim_trailing_newline(schemas),
        trim_trailing_newline(validation),
        trim_trailing_newline(sample),
        trim_trailing_newline(decoders),
    )
}

fn trim_trailing_newline(fragment: &str) -> &str {
    fragment.strip_suffix('\n').unwrap_or(fragment)
}
