use super::*;

pub(super) fn youtube_search_shape(
    parsed: &ParsedBrowserUrl,
    segments: &[BrowserUrlText],
) -> Option<ParsedUrlShape> {
    let query = query_param(parsed, QUERY_SEARCH)?;
    match segments {
        [first, ..] if first.0 == SEGMENT_RESULTS => Some(ParsedUrlShape {
            target_kind: TARGET_SEARCH,
            platform: PLATFORM_YOUTUBE,
            platform_ids: BrowserUrlShapePlatformIdsTemplate {
                query: Some(query.to_string()),
                ..empty_platform_ids()
            },
            confidence: CONFIDENCE_HIGH,
            reason_codes: vec![REASON_PARSED_URL, REASON_SEARCH_QUERY],
        }),
        _ => None,
    }
}
