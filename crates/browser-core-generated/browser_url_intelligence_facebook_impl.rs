use super::*;

pub(super) fn facebook_shape(parsed: &ParsedBrowserUrl) -> ParsedUrlShape {
    let segments = path_segments(parsed);
    if matches!(
        segments.first().map(|segment| segment.0.as_str()),
        Some(SEGMENT_WATCH)
    ) {
        if let Some(video_id) = query_param(parsed, QUERY_VIDEO) {
            return video_shape(
                PLATFORM_FACEBOOK,
                video_id.0.as_str(),
                vec![REASON_PARSED_URL],
            );
        }
    }
    if matches!(
        segments.first().map(|segment| segment.0.as_str()),
        Some(SEGMENT_REEL | SEGMENT_VIDEOS)
    ) {
        if let Some(post_id) = segments.get(1) {
            let target_kind = if segments[0].0 == SEGMENT_REEL {
                TARGET_SHORT_VIDEO
            } else {
                TARGET_VIDEO
            };
            return social_post_shape(
                target_kind,
                PLATFORM_FACEBOOK,
                post_id.0.as_str(),
                CONFIDENCE_HIGH,
            );
        }
    }
    if segments.is_empty()
        || matches!(
            segments.first().map(|segment| segment.0.as_str()),
            Some(SEGMENT_HOME | SEGMENT_FEED | SEGMENT_WATCH)
        )
    {
        return social_route_shape(
            ROUTE_SOCIAL_FEED,
            PLATFORM_FACEBOOK,
            CONFIDENCE_MEDIUM,
            vec![
                REASON_PARSED_URL,
                REASON_DYNAMIC_FEED,
                REASON_PARSED_SOCIAL_ROUTE,
            ],
        );
    }
    if matches!(
        segments.first().map(|segment| segment.0.as_str()),
        Some(SEGMENT_LIVE)
    ) {
        return social_route_shape(
            ROUTE_SOCIAL_LIVESTREAM,
            PLATFORM_FACEBOOK,
            CONFIDENCE_MEDIUM,
            vec![REASON_PARSED_URL, REASON_PARSED_SOCIAL_ROUTE],
        );
    }
    channel_shape(
        PLATFORM_FACEBOOK,
        segments
            .first()
            .map(|segment| segment.0.as_str())
            .unwrap_or(parsed.domain.as_str()),
        CONFIDENCE_MEDIUM,
    )
}
