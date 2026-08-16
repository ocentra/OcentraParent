use super::*;

pub(super) fn twitch_shape(parsed: &ParsedBrowserUrl) -> ParsedUrlShape {
    let segments = path_segments(parsed);
    if matches!(
        segments.first().map(|segment| segment.0.as_str()),
        Some(SEGMENT_VIDEOS)
    ) {
        if let Some(video_id) = segments.get(1) {
            return video_shape(
                PLATFORM_TWITCH,
                video_id.0.as_str(),
                vec![REASON_PARSED_URL],
            );
        }
    }
    if matches!(
        segments.first().map(|segment| segment.0.as_str()),
        Some(SEGMENT_DIRECTORY | SEGMENT_FOLLOWING)
    ) {
        return social_route_shape(
            ROUTE_SOCIAL_FEED,
            PLATFORM_TWITCH,
            CONFIDENCE_MEDIUM,
            vec![
                REASON_PARSED_URL,
                REASON_DYNAMIC_FEED,
                REASON_PARSED_SOCIAL_ROUTE,
            ],
        );
    }
    if let Some(channel_id) = segments.first() {
        return channel_shape(PLATFORM_TWITCH, channel_id.0.as_str(), CONFIDENCE_MEDIUM);
    }
    social_route_shape(
        ROUTE_SOCIAL_FEED,
        PLATFORM_TWITCH,
        CONFIDENCE_MEDIUM,
        vec![
            REASON_PARSED_URL,
            REASON_DYNAMIC_FEED,
            REASON_PARSED_SOCIAL_ROUTE,
        ],
    )
}
