pub fn app_game_policy_target_compiler_generated_typescript() -> String {
    [
        include_str!("app_game_policy_target_compiler_generated.template.txt"),
        "\n",
        include_str!("app_game_policy_target_compiler_generated/request.template.txt"),
        include_str!("app_game_policy_target_compiler_generated/decision.template.txt"),
    ]
    .concat()
}
