use ocentra_browser_core::social_video_ai_signal_aggregate::social_video_ai_signal_aggregate_typescript;

#[test]
fn social_video_ai_signal_aggregate_stays_rust_owned_without_schema_value_owners() {
    let source = social_video_ai_signal_aggregate_typescript();

    assert_eq!(
        source
            .matches("@ocentra-parent/schema-domain/browser-ai-analysis-values';")
            .count(),
        0
    );
    assert_eq!(
        source
            .matches("@ocentra-parent/schema-domain/browser-schemas';")
            .count(),
        0
    );
    assert_eq!(
        source
            .matches("@ocentra-parent/schema-domain/browser-social-ai-analysis-values';")
            .count(),
        0
    );
    assert_eq!(
        source
            .matches("@ocentra-parent/schema-domain/browser-social-feed-video-route-gate-values';")
            .count(),
        0
    );
    assert_eq!(
        source
            .matches("@ocentra-parent/schema-domain/browser-social-riskbenefit-values';")
            .count(),
        0
    );
    assert_eq!(
        source
            .matches("@ocentra-parent/schema-domain/browser-social-video-metadata';")
            .count(),
        0
    );
    assert_eq!(
        source
            .matches("} from './social_video_ai_signal_aggregate_support';")
            .count(),
        1
    );
}
