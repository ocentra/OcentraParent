use super::*;

#[path = "browser_url_intelligence_youtube_search_impl.rs"]
mod search;

pub(super) fn youtube_named_channel_shape(segments: &[BrowserUrlText]) -> Option<ParsedUrlShape> {
    match segments {
        [first, second, ..]
            if matches!(
                first.0.as_str(),
                SEGMENT_CHANNEL | SEGMENT_CUSTOM | SEGMENT_USER | SEGMENT_HANDLE
            ) =>
        {
            Some(youtube_channel_shape(second.0.as_str()))
        }
        _ => None,
    }
}

pub(super) fn youtube_handle_shape(segments: &[BrowserUrlText]) -> Option<ParsedUrlShape> {
    match segments {
        [first, ..] if first.0.starts_with(SEGMENT_HANDLE) => {
            Some(youtube_channel_shape(first.0.as_str()))
        }
        _ => None,
    }
}

pub(super) fn youtube_playlist_shape(
    parsed: &ParsedBrowserUrl,
    segments: &[BrowserUrlText],
) -> Option<ParsedUrlShape> {
    let playlist_id = query_param(parsed, QUERY_PLAYLIST)?;
    match segments {
        [first, ..] if first.0 == SEGMENT_PLAYLIST => Some(ParsedUrlShape {
            target_kind: TARGET_PLAYLIST,
            platform: PLATFORM_YOUTUBE,
            platform_ids: BrowserUrlShapePlatformIdsTemplate {
                playlist_id: Some(playlist_id.to_string()),
                ..empty_platform_ids()
            },
            confidence: CONFIDENCE_HIGH,
            reason_codes: vec![REASON_PARSED_URL, REASON_PLAYLIST_ID],
        }),
        _ => None,
    }
}

pub(super) fn youtube_search_shape(
    parsed: &ParsedBrowserUrl,
    segments: &[BrowserUrlText],
) -> Option<ParsedUrlShape> {
    search::youtube_search_shape(parsed, segments)
}

fn youtube_channel_shape(channel_id: impl std::fmt::Display) -> ParsedUrlShape {
    ParsedUrlShape {
        target_kind: TARGET_CHANNEL,
        platform: PLATFORM_YOUTUBE,
        platform_ids: BrowserUrlShapePlatformIdsTemplate {
            channel_id: Some(channel_id.to_string()),
            ..empty_platform_ids()
        },
        confidence: CONFIDENCE_HIGH,
        reason_codes: vec![REASON_PARSED_URL, REASON_CHANNEL_ID],
    }
}
