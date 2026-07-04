pub fn social_video_source_privacy_typescript() -> String {
    include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../browser-core-generated/social_video_source_privacy.ts"
    ))
    .to_string()
}
