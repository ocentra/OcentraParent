use std::fs;
use std::path::PathBuf;

use ocentra_parent_agent_protocol::constants::activity_event_kind;

#[test]
fn activity_event_kind_generated_typescript_matches_agent_protocol_constants() {
    let generated = ocentra_schema::activity_event_kind_ts::activity_event_kind_typescript();
    let checked_in = match fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../packages/schema-domain/src/generated-activity-event-kind.ts"),
    ) {
        Ok(value) => value,
        Err(_) => std::process::abort(),
    };

    let expected = format!(
        "/* generated from crates/schema/src/activity_event_kind_ts.rs */\n\nexport const ActivityEventKindLiteral = {{\n{}}} as const;\n",
        activity_event_kind::ALL
            .iter()
            .map(|(key, kind)| format!("  {key}: '{}',\n", kind.as_protocol_str()))
            .collect::<String>()
    );

    assert_eq!(expected, generated);
    assert_eq!(generated, checked_in);
}
