use super::*;

fn vimeo_shape(parsed: &ParsedBrowserUrl) -> Option<ParsedUrlShape> {
    let segments = path_segments(parsed);
    let video_id = match segments.as_slice() {
        [first, second, ..] if first == "video" => Some(second.to_string()),
        [first, ..] => Some(first.to_string()),
        [] => None,
    }?;

    if video_id.chars().all(|character| character.is_ascii_digit()) {
        Some(video_shape("vimeo", &video_id, vec!["parsed-url"]))
    } else {
        None
    }
}

fn youtube_shape(parsed: &ParsedBrowserUrl) -> ParsedUrlShape {
    let segments = path_segments(parsed);
    youtube_shorts_shape(&segments)
        .or_else(|| youtube_watch_shape(parsed, &segments))
        .or_else(|| youtube_embed_or_live_shape(&segments))
        .or_else(|| youtube_named_channel_shape(&segments))
        .or_else(|| youtube_handle_shape(&segments))
        .or_else(|| youtube_playlist_shape(parsed, &segments))
        .or_else(|| youtube_search_shape(parsed, &segments))
        .unwrap_or_else(|| simple_shape("unknown", "youtube", "low", vec!["parsed-url"]))
}

fn youtube_shorts_shape(segments: &[String]) -> Option<ParsedUrlShape> {
    match segments {
        [first, second, ..] if first == "shorts" => Some(ParsedUrlShape {
            target_kind: "short-video",
            platform: "youtube-shorts",
            platform_ids: BrowserUrlShapePlatformIdsTemplate {
                video_id: Some(second.to_string()),
                ..empty_platform_ids()
            },
            confidence: "high",
            reason_codes: vec!["parsed-url", "parsed-youtube-shorts-id"],
        }),
        _ => None,
    }
}

fn youtube_watch_shape(parsed: &ParsedBrowserUrl, segments: &[String]) -> Option<ParsedUrlShape> {
    let video_id = query_param(parsed, "v")?;
    if matches!(segments.first().map(String::as_str), Some("watch") | None) {
        Some(video_shape(
            "youtube",
            &video_id,
            vec!["parsed-url", "parsed-youtube-video-id"],
        ))
    } else {
        None
    }
}

fn youtube_embed_or_live_shape(segments: &[String]) -> Option<ParsedUrlShape> {
    match segments {
        [first, second, ..] if first == "embed" || first == "live" => Some(video_shape(
            "youtube",
            second,
            vec!["parsed-url", "parsed-youtube-video-id"],
        )),
        _ => None,
    }
}

fn youtube_named_channel_shape(segments: &[String]) -> Option<ParsedUrlShape> {
    match segments {
        [first, second, ..] if matches!(first.as_str(), "channel" | "c" | "user" | "@") => {
            Some(youtube_channel_shape(second))
        }
        _ => None,
    }
}

fn youtube_handle_shape(segments: &[String]) -> Option<ParsedUrlShape> {
    match segments {
        [first, ..] if first.starts_with('@') => Some(youtube_channel_shape(first)),
        _ => None,
    }
}

fn youtube_playlist_shape(
    parsed: &ParsedBrowserUrl,
    segments: &[String],
) -> Option<ParsedUrlShape> {
    let playlist_id = query_param(parsed, "list")?;
    match segments {
        [first, ..] if first == "playlist" => Some(ParsedUrlShape {
            target_kind: "playlist",
            platform: "youtube",
            platform_ids: BrowserUrlShapePlatformIdsTemplate {
                playlist_id: Some(playlist_id),
                ..empty_platform_ids()
            },
            confidence: "high",
            reason_codes: vec!["parsed-url", "parsed-playlist-id"],
        }),
        _ => None,
    }
}

fn youtube_search_shape(parsed: &ParsedBrowserUrl, segments: &[String]) -> Option<ParsedUrlShape> {
    let query = query_param(parsed, "search_query")?;
    match segments {
        [first, ..] if first == "results" => Some(ParsedUrlShape {
            target_kind: "search",
            platform: "youtube",
            platform_ids: BrowserUrlShapePlatformIdsTemplate {
                query: Some(query),
                ..empty_platform_ids()
            },
            confidence: "high",
            reason_codes: vec!["parsed-url", "parsed-search-query"],
        }),
        _ => None,
    }
}

fn youtube_channel_shape(channel_id: &str) -> ParsedUrlShape {
    ParsedUrlShape {
        target_kind: "channel",
        platform: "youtube",
        platform_ids: BrowserUrlShapePlatformIdsTemplate {
            channel_id: Some(channel_id.to_string()),
            ..empty_platform_ids()
        },
        confidence: "high",
        reason_codes: vec!["parsed-url", "parsed-channel-id"],
    }
}


