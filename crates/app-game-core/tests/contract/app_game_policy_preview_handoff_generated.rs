use std::fs::read_to_string;
use std::path::PathBuf;

use ocentra_app_game_core::app_game_policy_preview_handoff_generated_ts::app_game_policy_preview_handoff_generated_typescript;
use ocentra_eventing::expect_value::ExpectValue;

#[test]
fn generated_app_game_policy_preview_handoff_ts_stays_checked_in() {
    let generated = app_game_policy_preview_handoff_generated_typescript();
    let file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../packages/schema-domain/src/generated-app-game-policy-preview-handoff.ts");
    let checked_in =
        read_to_string(file_path).expect_value("read generated app-game policy preview handoff ts");

    assert_eq!(generated, checked_in);
}
