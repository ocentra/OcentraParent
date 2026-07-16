pub fn social_video_ai_signal_aggregate_typescript() -> String {
    include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../browser-core-generated/social_video_ai_signal_aggregate.ts"
    ))
    .to_string()
}
