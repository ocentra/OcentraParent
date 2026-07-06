use ocentra_app_game_core::app_game_policy_target_compiler::app_game_policy_target_compiler_rules_typescript;

#[test]
fn generated_app_game_policy_target_compiler_rules_stay_checked_in() {
    let checked_in = include_str!(
        "../../../../packages/schema-domain/src/generated-app-game-policy-target-compiler-rules.ts"
    );
    let generated = app_game_policy_target_compiler_rules_typescript();

    assert_eq!(checked_in, generated);
}
