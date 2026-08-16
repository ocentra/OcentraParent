use super::*;

pub(super) fn discord_shape(parsed: &ParsedBrowserUrl) -> ParsedUrlShape {
    let segments = path_segments(parsed);
    if matches!(
        segments.first().map(|segment| segment.0.as_str()),
        Some(SEGMENT_CHANNELS)
    ) {
        return social_route_shape(
            ROUTE_SOCIAL_MESSAGING,
            PLATFORM_DISCORD,
            CONFIDENCE_MEDIUM,
            vec![REASON_PARSED_URL, REASON_PARSED_SOCIAL_ROUTE],
        );
    }
    simple_shape(
        TARGET_UNKNOWN,
        PLATFORM_DISCORD,
        CONFIDENCE_LOW,
        vec![REASON_PARSED_URL, REASON_MANUAL_REQUIRED],
    )
}
