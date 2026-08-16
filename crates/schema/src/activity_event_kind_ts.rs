use ocentra_parent_agent_protocol::constants::activity_event_kind;

pub fn activity_event_kind_typescript() -> String {
    let entries = activity_event_kind::ALL
        .iter()
        .map(|(key, kind)| format!("  {key}: '{}',\n", kind.as_protocol_str()))
        .collect::<String>();

    format!(
        "/* generated from crates/schema/src/activity_event_kind_ts.rs */\n\nexport const ActivityEventKindLiteral = {{\n{entries}}} as const;\n"
    )
}
