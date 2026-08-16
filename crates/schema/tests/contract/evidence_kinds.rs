use std::fs;
use std::path::PathBuf;

#[test]
fn evidence_kinds_generated_typescript_matches_checked_in_file() {
    let generated = ocentra_schema::evidence_kinds_ts::evidence_kinds_typescript();
    let checked_in = match fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../packages/schema-domain/src/generated-evidence-kinds.ts"),
    ) {
        Ok(value) => value,
        Err(_) => std::process::abort(),
    };

    assert_eq!(generated, checked_in);
    assert_eq!(
        generated.lines().next(),
        Some("/* generated from crates/schema/src/evidence_kinds_ts.rs */")
    );
    assert_eq!(
        generated
            .lines()
            .filter(|line| *line == "import { ActivityEventKindLiteral as ActivityEventKindLiteralSource } from './generated-activity-event-kind';")
            .count(),
        1
    );
    assert_eq!(
        generated
            .lines()
            .filter(|line| {
                *line == "const ActivityEventKindDefinition = defineLiteralKindGroup(ActivityEventKindLiteralSource);"
            })
            .count(),
        1
    );
}
