use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::logging::LogFields;

pub(crate) fn log_field(
    payload: &LogFields,
    field: impl std::fmt::Display,
    context: impl std::fmt::Display,
) -> LogFieldValue {
    let _ = context;
    let field_name = field.to_string();
    payload
        .get(field_name.as_str())
        .cloned()
        .unwrap_or_else(|| std::process::abort())
}
