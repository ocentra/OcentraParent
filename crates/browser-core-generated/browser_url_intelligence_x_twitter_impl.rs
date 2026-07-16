use super::*;

pub(super) fn x_twitter_shape(parsed: &ParsedBrowserUrl) -> ParsedUrlShape {
    let segments = path_segments(parsed);
    if matches!(
        segments.first().map(|segment| segment.0.as_str()),
        Some(SEGMENT_SEARCH)
    ) {
        if let Some(query) = query_param(parsed, QUERY_SEARCH) {
            return search_shape(PLATFORM_X_TWITTER, query.0.as_str());
        }
    }
    if matches!(
        segments.get(1).map(|segment| segment.0.as_str()),
        Some(SEGMENT_STATUS)
    ) {
        if let Some(post_id) = segments.get(2) {
            return social_post_shape(
                TARGET_SOCIAL_POST,
                PLATFORM_X_TWITTER,
                post_id.0.as_str(),
                CONFIDENCE_HIGH,
            );
        }
    }
    if segments.is_empty()
        || matches!(
            segments.first().map(|segment| segment.0.as_str()),
            Some(SEGMENT_HOME | SEGMENT_EXPLORE)
        )
    {
        return social_route_shape(
            ROUTE_SOCIAL_FEED,
            PLATFORM_X_TWITTER,
            CONFIDENCE_MEDIUM,
            vec![
                REASON_PARSED_URL,
                REASON_DYNAMIC_FEED,
                REASON_PARSED_SOCIAL_ROUTE,
            ],
        );
    }
    channel_shape(
        PLATFORM_X_TWITTER,
        segments[0].0.as_str(),
        CONFIDENCE_MEDIUM,
    )
}
