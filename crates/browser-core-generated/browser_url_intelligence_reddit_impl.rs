use super::*;

pub(super) fn reddit_shape(parsed: &ParsedBrowserUrl) -> ParsedUrlShape {
    let segments = path_segments(parsed);
    if matches!(
        segments.first().map(|segment| segment.0.as_str()),
        Some(SEGMENT_REDDIT)
    ) && segments.len() >= 4
        && segments[2].0 == SEGMENT_COMMENTS
    {
        return social_post_shape(
            TARGET_SOCIAL_POST,
            PLATFORM_REDDIT,
            segments[3].0.as_str(),
            CONFIDENCE_MEDIUM,
        );
    }
    if matches!(
        segments.first().map(|segment| segment.0.as_str()),
        Some(SEGMENT_REDDIT)
    ) && segments.get(1).is_some()
    {
        return ParsedUrlShape {
            target_kind: TARGET_FORUM,
            platform: PLATFORM_REDDIT,
            platform_ids: BrowserUrlShapePlatformIdsTemplate {
                channel_id: Some(segments[1].0.clone()),
                ..empty_platform_ids()
            },
            confidence: CONFIDENCE_MEDIUM,
            reason_codes: vec![
                REASON_PARSED_URL,
                REASON_PARSED_CHANNEL_ID,
                REASON_PARSED_SOCIAL_ROUTE,
            ],
        };
    }
    if segments.is_empty() {
        return social_route_shape(
            ROUTE_SOCIAL_FEED,
            PLATFORM_REDDIT,
            CONFIDENCE_MEDIUM,
            vec![
                REASON_PARSED_URL,
                REASON_DYNAMIC_FEED,
                REASON_PARSED_SOCIAL_ROUTE,
            ],
        );
    }
    simple_shape(
        TARGET_UNKNOWN,
        PLATFORM_REDDIT,
        CONFIDENCE_LOW,
        vec![REASON_PARSED_URL, REASON_MANUAL_REQUIRED],
    )
}
