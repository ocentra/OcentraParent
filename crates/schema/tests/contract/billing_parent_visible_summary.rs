use std::fs::read_to_string;
use std::path::PathBuf;

#[path = "../../src/billing_parent_visible_summary_ts.rs"]
mod billing_parent_visible_summary_ts;

#[test]
fn billing_parent_visible_summary_generated_typescript_matches_checked_in_file() {
    let generated = billing_parent_visible_summary_ts::BILLING_PARENT_VISIBLE_SUMMARY_TYPESCRIPT;
    let file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../packages/schema-domain/src/generated-billing-parent-visible-summary.ts");
    let checked_in = match read_to_string(file_path) {
        Ok(value) => value,
        Err(_) => std::process::abort(),
    };

    assert!(generated.starts_with(
        "/* generated from crates/schema/src/billing_parent_visible_summary_ts.rs */"
    ));
    assert_eq!(generated, checked_in);
}
