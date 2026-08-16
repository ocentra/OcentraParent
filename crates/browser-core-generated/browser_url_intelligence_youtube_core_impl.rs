use super::*;

pub(super) fn vimeo_shape(parsed: &ParsedBrowserUrl) -> Option<ParsedUrlShape> {
    let segments = path_segments(parsed);
    let video_id = match segments.as_slice() {
        [first, second, ..] if first.0 == SEGMENT_VIDEO => Some(second.0.clone()),
        [first, ..] => Some(first.0.clone()),
        [] => None,
    }?;

    if video_id.chars().all(|character| character.is_ascii_digit()) {
        Some(video_shape(
            PLATFORM_VIMEO,
            &video_id,
            vec![REASON_PARSED_URL],
        ))
    } else {
        None
    }
}

pub(super) fn youtube_shorts_shape(segments: &[BrowserUrlText]) -> Option<ParsedUrlShape> {
    match segments {
        [first, second, ..] if first.0 == SEGMENT_SHORTS => Some(ParsedUrlShape {
            target_kind: TARGET_SHORT_VIDEO,
            platform: PLATFORM_YOUTUBE_SHORTS,
            platform_ids: BrowserUrlShapePlatformIdsTemplate {
                video_id: Some(second.0.clone()),
                ..empty_platform_ids()
            },
            confidence: CONFIDENCE_HIGH,
            reason_codes: vec![REASON_PARSED_URL, REASON_YOUTUBE_SHORTS_ID],
        }),
        _ => None,
    }
}

pub(super) fn youtube_watch_shape(
    parsed: &ParsedBrowserUrl,
    segments: &[BrowserUrlText],
) -> Option<ParsedUrlShape> {
    let video_id = query_param(parsed, QUERY_VIDEO)?;
    if matches!(
        segments.first().map(|segment| segment.0.as_str()),
        Some(SEGMENT_WATCH) | None
    ) {
        Some(video_shape(
            PLATFORM_YOUTUBE,
            video_id.0.as_str(),
            vec![REASON_PARSED_URL, REASON_YOUTUBE_VIDEO_ID],
        ))
    } else {
        None
    }
}

pub(super) fn youtube_embed_or_live_shape(segments: &[BrowserUrlText]) -> Option<ParsedUrlShape> {
    match segments {
        [first, second, ..] if first.0 == SEGMENT_EMBED || first.0 == SEGMENT_LIVE => {
            Some(video_shape(
                PLATFORM_YOUTUBE,
                second.0.as_str(),
                vec![REASON_PARSED_URL, REASON_YOUTUBE_VIDEO_ID],
            ))
        }
        _ => None,
    }
}
