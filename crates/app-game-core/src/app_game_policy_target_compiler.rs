pub fn app_game_policy_target_compiler_rules_typescript() -> String {
    include_str!(
        "../../../packages/schema-domain/src/generated-app-game-policy-target-compiler-rules.ts"
    )
    .to_string()
}
