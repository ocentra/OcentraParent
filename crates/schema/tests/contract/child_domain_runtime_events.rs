use std::fs;
use std::path::PathBuf;

#[path = "../../src/child_domain_runtime_events_ts.rs"]
mod child_domain_runtime_events_ts;

#[test]
fn child_domain_runtime_events_generated_typescript_matches_checked_in_file() {
    let generated = child_domain_runtime_events_ts::child_domain_runtime_events_typescript();
    let checked_in = match fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../packages/schema-domain/src/child-domain-runtime-events.ts"),
    ) {
        Ok(value) => value,
        Err(_) => std::process::abort(),
    };

    assert_eq!(generated, checked_in);
    assert!(generated
        .starts_with("/* generated from crates/schema/src/child_domain_runtime_events_ts.rs */"));
}
