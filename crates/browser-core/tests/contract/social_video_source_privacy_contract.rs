use ocentra_browser_core::{
    social_video_ai_signal_aggregate::social_video_ai_signal_aggregate_typescript,
    social_video_source_privacy::social_video_source_privacy_typescript,
};

#[test]
fn social_video_privacy_generated_contracts_no_longer_depend_on_schema_domain_owner() {
    for source in [
        social_video_ai_signal_aggregate_typescript(),
        social_video_source_privacy_typescript(),
    ] {
        assert_eq!(
            source
                .matches("@ocentra-parent/schema-domain/agent-social-video-source-privacy';")
                .count(),
            0
        );
        assert!(
            source
                .matches("SocialVideoSourcePrivacyEvidenceIdSchema")
                .count()
                > 0
        );
    }
}
