use super::*;

const SEGMENT_VIDEO: &str = "video";
const SEGMENT_SHORTS: &str = "shorts";
const SEGMENT_WATCH: &str = "watch";
const SEGMENT_EMBED: &str = "embed";
const SEGMENT_LIVE: &str = "live";
const SEGMENT_CHANNEL: &str = "channel";
const SEGMENT_CUSTOM: &str = "c";
const SEGMENT_USER: &str = "user";
const SEGMENT_HANDLE: &str = "@";
const SEGMENT_PLAYLIST: &str = "playlist";
const SEGMENT_RESULTS: &str = "results";
const QUERY_VIDEO: &str = "v";
const QUERY_PLAYLIST: &str = "list";
const QUERY_SEARCH: &str = "search_query";
const PLATFORM_VIMEO: &str = "vimeo";
const PLATFORM_YOUTUBE: &str = "youtube";
const PLATFORM_YOUTUBE_SHORTS: &str = "youtube-shorts";
const TARGET_UNKNOWN: &str = "unknown";
const TARGET_SHORT_VIDEO: &str = "short-video";
const TARGET_CHANNEL: &str = "channel";
const TARGET_PLAYLIST: &str = "playlist";
const TARGET_SEARCH: &str = "search";
const CONFIDENCE_HIGH: &str = "high";
const CONFIDENCE_LOW: &str = "low";
const REASON_PARSED_URL: &str = "parsed-url";
const REASON_YOUTUBE_SHORTS_ID: &str = "parsed-youtube-shorts-id";
const REASON_YOUTUBE_VIDEO_ID: &str = "parsed-youtube-video-id";
const REASON_PLAYLIST_ID: &str = "parsed-playlist-id";
const REASON_SEARCH_QUERY: &str = "parsed-search-query";
const REASON_CHANNEL_ID: &str = "parsed-channel-id";

#[path = "../../browser-core-generated/browser_url_intelligence_youtube_core_impl.rs"]
mod core;
#[path = "../../browser-core-generated/browser_url_intelligence_youtube_routes_impl.rs"]
mod routes;

pub(super) fn vimeo_shape(parsed: &ParsedBrowserUrl) -> Option<ParsedUrlShape> {
    core::vimeo_shape(parsed)
}

pub(super) fn youtube_shape(parsed: &ParsedBrowserUrl) -> ParsedUrlShape {
    let segments = path_segments(parsed);
    core::youtube_shorts_shape(&segments)
        .or_else(|| core::youtube_watch_shape(parsed, &segments))
        .or_else(|| core::youtube_embed_or_live_shape(&segments))
        .or_else(|| routes::youtube_named_channel_shape(&segments))
        .or_else(|| routes::youtube_handle_shape(&segments))
        .or_else(|| routes::youtube_playlist_shape(parsed, &segments))
        .or_else(|| routes::youtube_search_shape(parsed, &segments))
        .unwrap_or_else(|| {
            simple_shape(
                TARGET_UNKNOWN,
                PLATFORM_YOUTUBE,
                CONFIDENCE_LOW,
                vec![REASON_PARSED_URL],
            )
        })
}
